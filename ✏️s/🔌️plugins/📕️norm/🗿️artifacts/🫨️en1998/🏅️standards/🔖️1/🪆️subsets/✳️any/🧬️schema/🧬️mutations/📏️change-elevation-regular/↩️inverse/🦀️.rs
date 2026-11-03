//! ↩️ `change-elevation-regular` inverse: restores the previous `elevation_regular`.

use crate::change_elevation_regular::ChangeElevationRegular;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeElevationRegular, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_elevation_regular::ChangeElevationRegular> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeElevationRegular, base: &En1998Snapshot) -> Option<ChangeElevationRegular> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeElevationRegular { id: payload.id.clone(), new_elevation_regular: r0.elevation_regular })
}
