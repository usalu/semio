//! ↩️ `insert-local-fire` inverse: removes the inserted row again.

use crate::insert_local_fire::InsertLocalFire;
use crate::remove_local_fire::RemoveLocalFire;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertLocalFire, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_local_fire::RemoveLocalFire> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertLocalFire, base: &En1991Snapshot) -> Option<RemoveLocalFire> {
    Some(RemoveLocalFire { id: payload.item.id.clone() })
}
