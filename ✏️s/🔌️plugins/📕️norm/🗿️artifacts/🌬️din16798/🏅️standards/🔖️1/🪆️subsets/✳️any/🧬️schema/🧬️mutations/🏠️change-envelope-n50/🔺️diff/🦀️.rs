//! 🔺️ `change-envelope-n50` diff.
use crate::change_envelope_n50::ChangeEnvelopeN50;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeEnvelopeN50, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.envelope_n50_h_inv == payload.new_envelope_n50_h_inv {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { envelope_n50_h_inv: Some(payload.new_envelope_n50_h_inv), ..Default::default() })
}
