//! ↩️ `insert-rotating-machine` inverse: removes the inserted row again.

use crate::insert_rotating_machine::InsertRotatingMachine;
use crate::remove_rotating_machine::RemoveRotatingMachine;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRotatingMachine, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_rotating_machine::RemoveRotatingMachine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRotatingMachine, base: &En1991Snapshot) -> Option<RemoveRotatingMachine> {
    Some(RemoveRotatingMachine { id: payload.item.id.clone() })
}
