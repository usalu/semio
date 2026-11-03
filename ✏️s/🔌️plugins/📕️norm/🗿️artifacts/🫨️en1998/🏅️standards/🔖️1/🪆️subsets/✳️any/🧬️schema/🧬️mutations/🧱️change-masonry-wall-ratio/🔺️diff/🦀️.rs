//! 🧱️ `change-masonry-wall-ratio` diff: patches `masonry_wall_area_ratio` of the addressed building row.

use crate::change_masonry_wall_ratio::ChangeMasonryWallRatio;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_masonry_wall_area_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.masonry_wall_area_ratio == payload.new_masonry_wall_area_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { masonry_wall_area_ratio: Some(payload.new_masonry_wall_area_ratio), ..Default::default() }), ..Default::default() })
}
