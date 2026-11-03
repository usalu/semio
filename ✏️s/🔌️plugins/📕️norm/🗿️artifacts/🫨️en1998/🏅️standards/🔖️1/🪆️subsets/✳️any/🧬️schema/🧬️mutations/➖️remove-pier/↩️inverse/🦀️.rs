//! ↩️ `remove-pier` inverse: inserts the removed row back at its position.

use crate::remove_pier::RemovePier;
use crate::insert_pier::InsertPier;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemovePier, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pier::InsertPier> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePier, base: &En1998Snapshot) -> Option<InsertPier> {
    let (index, row) = base.piers.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertPier { index: Some(index), pier: row.clone() })
}
