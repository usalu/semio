//! ↩️ `remove-floor` inverse: inserts the removed row back at its position.

use crate::remove_floor::RemoveFloor;
use crate::insert_floor::InsertFloor;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveFloor, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_floor::InsertFloor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFloor, base: &En1991Snapshot) -> Option<InsertFloor> {
    let (index, row) = base.floors.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFloor { index, item: row.clone() })
}
