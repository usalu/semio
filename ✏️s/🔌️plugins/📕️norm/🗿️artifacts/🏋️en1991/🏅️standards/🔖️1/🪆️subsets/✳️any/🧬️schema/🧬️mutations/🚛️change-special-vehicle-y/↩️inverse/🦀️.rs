//! ↩️ `change-special-vehicle-y` inverse: restores the previous `y`.

use crate::change_special_vehicle_y::ChangeSpecialVehicleY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSpecialVehicleY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_special_vehicle_y::ChangeSpecialVehicleY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSpecialVehicleY, base: &En1991Snapshot) -> Option<ChangeSpecialVehicleY> {
    let r0 = base.special_vehicles.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSpecialVehicleY { id: payload.id.clone(), new_y: r0.y })
}
