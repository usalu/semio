//! 🔺️ Diff for `change-silo-k`.
use crate::change_silo_k::ChangeSiloK;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloK, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_k == payload.new_silo_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_k: Some(payload.new_silo_k), ..Default::default() })
}
