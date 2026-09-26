//! ↩️ `change-envelope-n50` inverse.
use super::ChangeEnvelopeN50;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeEnvelopeN50, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeEnvelopeN50(ChangeEnvelopeN50 { new_envelope_n50_h_inv: base.envelope_n50_h_inv })]
}
