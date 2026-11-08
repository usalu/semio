//! ↩️ `remove-pier-base` inverse: inserts the removed row back at its position.

use crate::remove_pier_base::RemovePierBase;
use crate::insert_pier_base::InsertPierBase;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemovePierBase, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pier_base::InsertPierBase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePierBase, base: &En1998Snapshot) -> Option<InsertPierBase> {
    let (index, row) = base.pier_bases.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertPierBase { index: Some(index), pier_base: row.clone() })
}
