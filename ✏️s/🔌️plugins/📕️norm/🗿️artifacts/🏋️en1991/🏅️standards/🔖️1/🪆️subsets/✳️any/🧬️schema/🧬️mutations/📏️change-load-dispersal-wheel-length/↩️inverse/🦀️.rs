//! ↩️ `change-load-dispersal-wheel-length` inverse: restores the previous `wheel_length`.

use crate::change_load_dispersal_wheel_length::ChangeLoadDispersalWheelLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadDispersalWheelLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_dispersal_wheel_length::ChangeLoadDispersalWheelLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLoadDispersalWheelLength, base: &En1991Snapshot) -> Option<ChangeLoadDispersalWheelLength> {
    let r0 = base.load_dispersals.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLoadDispersalWheelLength { id: payload.id.clone(), new_wheel_length: r0.wheel_length })
}
