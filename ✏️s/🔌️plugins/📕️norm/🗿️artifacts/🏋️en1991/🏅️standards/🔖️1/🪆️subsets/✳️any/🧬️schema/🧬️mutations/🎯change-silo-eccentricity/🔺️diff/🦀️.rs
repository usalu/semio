//! 🔺️ Diff for `change-silo-eccentricity`.
use crate::change_silo_eccentricity::ChangeSiloEccentricity;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloEccentricity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_eccentricity == payload.new_silo_eccentricity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_eccentricity: Some(payload.new_silo_eccentricity), ..Default::default() })
}
