//! Inverse for `change-masonry-wall-ratio`.
use super::ChangeMasonryWallRatio;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![En1998Mutation::ChangeMasonryWallRatio(ChangeMasonryWallRatio { building_index: payload.building_index, new_masonry_wall_area_ratio: b.masonry_wall_area_ratio })],
        None => Vec::new(),
    }

    })())
}
