//! 🔺️ Diff for `change-height`.
use crate::change_height::ChangeHeight;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { height: Some(payload.new_height), ..Default::default() })
}
