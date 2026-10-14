//! Inverse for `change-elevation-regular`.
use crate::change_elevation_regular::ChangeElevationRegular;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeElevationRegular, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_elevation_regular::ChangeElevationRegular> {
    Ok((|| {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![Op::from(ChangeElevationRegular { building_index: payload.building_index, new_elevation_regular: b.elevation_regular })],
        None => Vec::new(),
    }

    })())
}
