//! Inverse for `change-elevation-regular`.
use super::ChangeElevationRegular;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeElevationRegular, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![En1998Mutation::ChangeElevationRegular(ChangeElevationRegular { building_index: payload.building_index, new_elevation_regular: b.elevation_regular })],
        None => Vec::new(),
    }

    })())
}
