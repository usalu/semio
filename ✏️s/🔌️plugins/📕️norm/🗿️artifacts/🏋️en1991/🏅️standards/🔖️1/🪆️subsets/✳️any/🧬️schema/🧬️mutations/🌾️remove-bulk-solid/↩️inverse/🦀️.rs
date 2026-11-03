//! ↩️ `remove-bulk-solid` inverse: inserts the removed row back at its position.

use crate::remove_bulk_solid::RemoveBulkSolid;
use crate::insert_bulk_solid::InsertBulkSolid;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveBulkSolid, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bulk_solid::InsertBulkSolid> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBulkSolid, base: &En1991Snapshot) -> Option<InsertBulkSolid> {
    let (index, row) = base.bulk_solids.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBulkSolid { index, item: row.clone() })
}
