//! ↩️ `insert-tank` inverse: removes the inserted row again.

use crate::insert_tank::InsertTank;
use crate::remove_tank::RemoveTank;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertTank, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tank::RemoveTank> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTank, base: &En1998Snapshot) -> Option<RemoveTank> {
    Some(RemoveTank { id: payload.tank.id.clone() })
}
