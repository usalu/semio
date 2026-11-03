//! ↩️ `insert-bulk-solid` inverse: removes the inserted row again.

use crate::insert_bulk_solid::InsertBulkSolid;
use crate::remove_bulk_solid::RemoveBulkSolid;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertBulkSolid, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bulk_solid::RemoveBulkSolid> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBulkSolid, base: &En1991Snapshot) -> Option<RemoveBulkSolid> {
    Some(RemoveBulkSolid { id: payload.item.id.clone() })
}
