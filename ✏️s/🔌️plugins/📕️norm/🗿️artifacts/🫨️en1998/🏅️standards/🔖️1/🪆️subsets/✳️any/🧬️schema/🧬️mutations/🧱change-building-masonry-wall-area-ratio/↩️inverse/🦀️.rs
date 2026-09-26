//! Inverse for `change-building-masonry-wall-area-ratio`.
use super::ChangeBuildingMasonryWallAreaRatio;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeBuildingMasonryWallAreaRatio, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![En1998Mutation::ChangeBuildingMasonryWallAreaRatio(ChangeBuildingMasonryWallAreaRatio { building_index: payload.building_index, new_masonry_wall_area_ratio: b.masonry_wall_area_ratio })],
        None => Vec::new(),
    }
}
