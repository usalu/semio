//! ↩️ `insert-roof` inverse: removes the inserted row again.

use crate::insert_roof::InsertRoof;
use crate::remove_roof::RemoveRoof;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRoof, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_roof::RemoveRoof> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRoof, base: &En1991Snapshot) -> Option<RemoveRoof> {
    Some(RemoveRoof { id: payload.item.id.clone() })
}
