//! ↩️ `remove-roof` inverse: inserts the removed row back at its position.

use crate::remove_roof::RemoveRoof;
use crate::insert_roof::InsertRoof;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRoof, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_roof::InsertRoof> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRoof, base: &En1991Snapshot) -> Option<InsertRoof> {
    let (index, row) = base.roofs.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRoof { index, item: row.clone() })
}
