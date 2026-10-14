//! 🔺️ Diff for `change-mixed-terrain-upwind`.
use crate::change_mixed_terrain_upwind::ChangeMixedTerrainUpwind;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeMixedTerrainUpwind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.mixed_terrain_upwind == payload.new_mixed_terrain_upwind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { mixed_terrain_upwind: Some(payload.new_mixed_terrain_upwind), ..Default::default() })
}
