//! ↩️ `remove-local-fire` inverse: inserts the removed row back at its position.

use crate::remove_local_fire::RemoveLocalFire;
use crate::insert_local_fire::InsertLocalFire;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveLocalFire, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_local_fire::InsertLocalFire> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveLocalFire, base: &En1991Snapshot) -> Option<InsertLocalFire> {
    let (index, row) = base.local_fires.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertLocalFire { index, item: row.clone() })
}
