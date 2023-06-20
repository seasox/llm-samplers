use std::{collections::HashMap, hash::Hash};

use num_traits::{Float, PrimInt};

pub use crate::types::*;

/// Presence and frequency penalty sampling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleFreqPresence<'a, TID, L> {
    alpha_frequency: L,
    alpha_presence: L,
    tokens: &'a [TID],
}

impl<'a, TID: PrimInt, L: Float> SampleFreqPresence<'a, TID, L> {
    pub fn new(alpha_frequency: L, alpha_presence: L, tokens: &'a [TID]) -> Self {
        Self {
            alpha_frequency,
            alpha_presence,
            tokens,
        }
    }
}

impl<'slf, TID: PrimInt + Hash, L: Float> Sampler<TID, L> for SampleFreqPresence<'slf, TID, L> {
    fn sample<'a>(&mut self, logits: &'a mut Logits<TID, L>) -> &'a mut Logits<TID, L> {
        let Self {
            alpha_frequency,
            alpha_presence,
            tokens,
        } = *self;
        let mut counts = HashMap::with_capacity(tokens.len());
        tokens.iter().for_each(|tid| {
            let cnt = counts.entry(tid).or_insert(L::zero());
            *cnt = *cnt + L::one()
        });

        logits.iter_mut().for_each(|l| {
            if let Some(cnt) = counts.get(&l.token_id) {
                l.logit = l.logit
                    - (*cnt * alpha_frequency
                        + if cnt > &L::zero() {
                            L::one()
                        } else {
                            L::zero()
                        } * alpha_presence);
            }
        });
        logits.set_sorted(false)
    }
}