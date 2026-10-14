//! 🔺️ Diff for `change-assumed-delta-t`.
use crate::change_assumed_delta_t::ChangeAssumedDeltaT;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedDeltaT, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_delta_t == payload.new_assumed_delta_t {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_delta_t: Some(payload.new_assumed_delta_t), ..Default::default() })
}
