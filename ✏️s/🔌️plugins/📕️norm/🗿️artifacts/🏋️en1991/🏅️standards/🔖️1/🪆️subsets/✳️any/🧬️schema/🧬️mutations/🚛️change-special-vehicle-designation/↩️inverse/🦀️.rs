//! ↩️ `change-special-vehicle-designation` inverse: restores the previous `designation`.

use crate::change_special_vehicle_designation::ChangeSpecialVehicleDesignation;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSpecialVehicleDesignation, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_special_vehicle_designation::ChangeSpecialVehicleDesignation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSpecialVehicleDesignation, base: &En1991Snapshot) -> Option<ChangeSpecialVehicleDesignation> {
    let r0 = base.special_vehicles.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSpecialVehicleDesignation { id: payload.id.clone(), new_designation: r0.designation.clone() })
}
