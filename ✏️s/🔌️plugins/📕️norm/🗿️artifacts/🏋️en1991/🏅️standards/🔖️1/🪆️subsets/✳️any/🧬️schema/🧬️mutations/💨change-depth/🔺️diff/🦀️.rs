//! 🔺️ Diff for `change-depth`.
use crate::change_depth::ChangeDepth;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeDepth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.depth == payload.new_depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { depth: Some(payload.new_depth), ..Default::default() })
}
