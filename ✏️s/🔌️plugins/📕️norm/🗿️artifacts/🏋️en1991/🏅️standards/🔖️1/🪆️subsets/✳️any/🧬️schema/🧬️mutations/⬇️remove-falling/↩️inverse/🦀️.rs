//! ↩️ `remove-falling` inverse: inserts the removed row back at its position.

use crate::remove_falling::RemoveFalling;
use crate::insert_falling::InsertFalling;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveFalling, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_falling::InsertFalling> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFalling, base: &En1991Snapshot) -> Option<InsertFalling> {
    let (index, row) = base.falling_loads.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFalling { index, item: row.clone() })
}
