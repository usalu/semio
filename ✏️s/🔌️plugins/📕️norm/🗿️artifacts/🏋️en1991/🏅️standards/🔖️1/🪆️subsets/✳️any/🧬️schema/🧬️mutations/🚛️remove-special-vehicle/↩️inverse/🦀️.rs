//! ↩️ `remove-special-vehicle` inverse: inserts the removed row back at its position.

use crate::remove_special_vehicle::RemoveSpecialVehicle;
use crate::insert_special_vehicle::InsertSpecialVehicle;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveSpecialVehicle, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_special_vehicle::InsertSpecialVehicle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSpecialVehicle, base: &En1991Snapshot) -> Option<InsertSpecialVehicle> {
    let (index, row) = base.special_vehicles.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSpecialVehicle { index, item: row.clone() })
}
