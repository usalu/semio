//! ↩️ `change-envelope-volume` inverse.
use super::ChangeEnvelopeVolume;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeEnvelopeVolume, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeEnvelopeVolume(ChangeEnvelopeVolume { new_envelope_volume_m3: base.envelope_volume_m3 })]
}
