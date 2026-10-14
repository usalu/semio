//! 🔺️ Diff for `change-width`.
use crate::change_width::ChangeWidth;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeWidth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { width: Some(payload.new_width), ..Default::default() })
}
