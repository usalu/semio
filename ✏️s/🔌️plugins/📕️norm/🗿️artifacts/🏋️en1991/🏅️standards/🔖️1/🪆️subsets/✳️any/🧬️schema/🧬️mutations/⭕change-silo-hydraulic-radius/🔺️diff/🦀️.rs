//! 🔺️ Diff for `change-silo-hydraulic-radius`.
use crate::change_silo_hydraulic_radius::ChangeSiloHydraulicRadius;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloHydraulicRadius, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_hydraulic_radius == payload.new_silo_hydraulic_radius {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_hydraulic_radius: Some(payload.new_silo_hydraulic_radius), ..Default::default() })
}
