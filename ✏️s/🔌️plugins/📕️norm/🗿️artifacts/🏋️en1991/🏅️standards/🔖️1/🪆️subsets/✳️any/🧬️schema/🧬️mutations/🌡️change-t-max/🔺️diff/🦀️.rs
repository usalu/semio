//! 🔺️ Diff for `change-t-max`.
use crate::change_t_max::ChangeTMax;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeTMax, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_max == payload.new_t_max {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_max: Some(payload.new_t_max), ..Default::default() })
}
