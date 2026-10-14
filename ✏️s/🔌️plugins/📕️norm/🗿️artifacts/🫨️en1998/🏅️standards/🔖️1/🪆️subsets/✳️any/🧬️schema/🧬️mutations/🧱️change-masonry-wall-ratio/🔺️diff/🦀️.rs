//! 🧱️ `change-masonry-wall-ratio` diff — patches the one field of the building at the index; a missing building is a `mutation.target-missing`.

use crate::change_masonry_wall_ratio::ChangeMasonryWallRatio;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let patch = En1998BuildingPatch { masonry_wall_area_ratio: Some(payload.new_masonry_wall_area_ratio), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
}
