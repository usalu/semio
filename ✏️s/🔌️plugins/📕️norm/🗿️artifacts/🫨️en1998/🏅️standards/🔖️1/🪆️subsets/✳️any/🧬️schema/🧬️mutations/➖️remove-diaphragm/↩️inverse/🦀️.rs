//! ↩️ `remove-diaphragm` inverse: inserts the removed row back at its position.

use crate::remove_diaphragm::RemoveDiaphragm;
use crate::insert_diaphragm::InsertDiaphragm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveDiaphragm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_diaphragm::InsertDiaphragm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveDiaphragm, base: &En1998Snapshot) -> Option<InsertDiaphragm> {
    let (index, row) = base.diaphragms.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertDiaphragm { index: Some(index), diaphragm: row.clone() })
}
