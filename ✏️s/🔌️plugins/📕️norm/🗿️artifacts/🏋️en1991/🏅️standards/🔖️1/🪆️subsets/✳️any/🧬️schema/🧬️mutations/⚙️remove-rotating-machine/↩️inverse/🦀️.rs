//! ↩️ `remove-rotating-machine` inverse: inserts the removed row back at its position.

use crate::remove_rotating_machine::RemoveRotatingMachine;
use crate::insert_rotating_machine::InsertRotatingMachine;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRotatingMachine, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_rotating_machine::InsertRotatingMachine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRotatingMachine, base: &En1991Snapshot) -> Option<InsertRotatingMachine> {
    let (index, row) = base.rotating_machines.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRotatingMachine { index, item: row.clone() })
}
