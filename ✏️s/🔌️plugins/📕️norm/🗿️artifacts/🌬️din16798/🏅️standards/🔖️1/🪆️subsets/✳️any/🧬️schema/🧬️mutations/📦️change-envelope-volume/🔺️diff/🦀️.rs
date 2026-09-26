//! 🔺️ `change-envelope-volume` diff.
use super::ChangeEnvelopeVolume;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeEnvelopeVolume, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.envelope_volume_m3 == payload.new_envelope_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { envelope_volume_m3: Some(payload.new_envelope_volume_m3), ..Default::default() })
}
