//! 🔺️ Diff for `change-fire-mode`.
use crate::change_fire_mode::ChangeFireMode;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireMode, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_mode == payload.new_fire_mode {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_mode: Some(payload.new_fire_mode), ..Default::default() })
}
