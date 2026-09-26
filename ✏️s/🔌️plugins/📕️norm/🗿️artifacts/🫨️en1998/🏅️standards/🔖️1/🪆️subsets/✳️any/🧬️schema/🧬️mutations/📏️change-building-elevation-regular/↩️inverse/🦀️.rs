//! Inverse for `change-building-elevation-regular`.
use super::ChangeBuildingElevationRegular;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeBuildingElevationRegular, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![En1998Mutation::ChangeBuildingElevationRegular(ChangeBuildingElevationRegular { building_index: payload.building_index, new_elevation_regular: b.elevation_regular })],
        None => Vec::new(),
    }
}
