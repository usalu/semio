//! 🔺️ Diff for `change-silo-height`.
use crate::change_silo_height::ChangeSiloHeight;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_height == payload.new_silo_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_height: Some(payload.new_silo_height), ..Default::default() })
}
