//! Diff for `change-masonry-wall-ratio`.
use super::ChangeMasonryWallRatio;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    b.masonry_wall_area_ratio = payload.new_masonry_wall_area_ratio;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
