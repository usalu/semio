//! ↩️ `remove-silo` inverse: inserts the removed row back at its position.

use crate::remove_silo::RemoveSilo;
use crate::insert_silo::InsertSilo;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveSilo, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_silo::InsertSilo> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSilo, base: &En1998Snapshot) -> Option<InsertSilo> {
    let (index, row) = base.silos.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSilo { index: Some(index), silo: row.clone() })
}
