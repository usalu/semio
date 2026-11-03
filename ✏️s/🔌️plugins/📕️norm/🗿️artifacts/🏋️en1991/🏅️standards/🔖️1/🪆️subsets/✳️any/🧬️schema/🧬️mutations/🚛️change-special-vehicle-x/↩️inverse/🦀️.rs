//! ↩️ `change-special-vehicle-x` inverse: restores the previous `x`.

use crate::change_special_vehicle_x::ChangeSpecialVehicleX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSpecialVehicleX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_special_vehicle_x::ChangeSpecialVehicleX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSpecialVehicleX, base: &En1991Snapshot) -> Option<ChangeSpecialVehicleX> {
    let r0 = base.special_vehicles.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSpecialVehicleX { id: payload.id.clone(), new_x: r0.x })
}
