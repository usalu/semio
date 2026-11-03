//! ↩️ `change-accidental-vehicle-speed` inverse: restores the previous `vehicle_speed`.

use crate::change_accidental_vehicle_speed::ChangeAccidentalVehicleSpeed;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalVehicleSpeed, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_vehicle_speed::ChangeAccidentalVehicleSpeed> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalVehicleSpeed, base: &En1991Snapshot) -> Option<ChangeAccidentalVehicleSpeed> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.impact.first()?;
    Some(ChangeAccidentalVehicleSpeed { id: payload.id.clone(), new_vehicle_speed: current.vehicle_speed })
}
