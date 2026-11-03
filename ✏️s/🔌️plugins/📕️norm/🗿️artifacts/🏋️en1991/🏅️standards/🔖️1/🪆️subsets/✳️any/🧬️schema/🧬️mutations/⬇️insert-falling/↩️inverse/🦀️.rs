//! ↩️ `insert-falling` inverse: removes the inserted row again.

use crate::insert_falling::InsertFalling;
use crate::remove_falling::RemoveFalling;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertFalling, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_falling::RemoveFalling> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFalling, base: &En1991Snapshot) -> Option<RemoveFalling> {
    Some(RemoveFalling { id: payload.item.id.clone() })
}
