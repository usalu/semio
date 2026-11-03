//! ↩️ `insert-special-vehicle` inverse: removes the inserted row again.

use crate::insert_special_vehicle::InsertSpecialVehicle;
use crate::remove_special_vehicle::RemoveSpecialVehicle;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertSpecialVehicle, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_special_vehicle::RemoveSpecialVehicle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSpecialVehicle, base: &En1991Snapshot) -> Option<RemoveSpecialVehicle> {
    Some(RemoveSpecialVehicle { id: payload.item.id.clone() })
}
