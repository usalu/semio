//! 🔺️ Diff for `change-t-min`.
use crate::change_t_min::ChangeTMin;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeTMin, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_min == payload.new_t_min {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_min: Some(payload.new_t_min), ..Default::default() })
}
