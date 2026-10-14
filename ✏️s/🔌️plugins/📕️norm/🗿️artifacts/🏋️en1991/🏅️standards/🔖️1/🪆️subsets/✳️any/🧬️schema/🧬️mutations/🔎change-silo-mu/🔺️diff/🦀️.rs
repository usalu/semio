//! 🔺️ Diff for `change-silo-mu`.
use crate::change_silo_mu::ChangeSiloMu;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloMu, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_mu == payload.new_silo_mu {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_mu: Some(payload.new_silo_mu), ..Default::default() })
}
