//! ↩️ `change-accidental-vehicle-mass` inverse: restores the previous `vehicle_mass`.

use crate::change_accidental_vehicle_mass::ChangeAccidentalVehicleMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalVehicleMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_vehicle_mass::ChangeAccidentalVehicleMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalVehicleMass, base: &En1991Snapshot) -> Option<ChangeAccidentalVehicleMass> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.impact.first()?;
    Some(ChangeAccidentalVehicleMass { id: payload.id.clone(), new_vehicle_mass: current.vehicle_mass })
}
