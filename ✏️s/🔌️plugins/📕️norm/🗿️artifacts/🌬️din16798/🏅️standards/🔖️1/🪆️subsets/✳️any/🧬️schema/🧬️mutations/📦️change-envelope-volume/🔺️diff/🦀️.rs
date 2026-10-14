//! 🔺️ `change-envelope-volume` diff.
use crate::change_envelope_volume::ChangeEnvelopeVolume;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeEnvelopeVolume, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.envelope_volume_m3 == payload.new_envelope_volume_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { envelope_volume_m3: Some(payload.new_envelope_volume_m3), ..Default::default() })
}
