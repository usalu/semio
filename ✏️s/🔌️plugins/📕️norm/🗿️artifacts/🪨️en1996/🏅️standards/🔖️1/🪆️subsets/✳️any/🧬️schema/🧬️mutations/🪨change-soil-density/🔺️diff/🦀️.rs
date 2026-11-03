//! 🪨 `change-soil-density` diff: patches `soil_density_kg_m3` of the addressed wall row.

use crate::change_soil_density::ChangeSoilDensity;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeSoilDensity, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_soil_density_kg_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.soil_density_kg_m3 == payload.new_soil_density_kg_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { soil_density_kg_m3: Some(payload.new_soil_density_kg_m3), ..Default::default() }), ..Default::default() })
}
