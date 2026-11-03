//! ↩️ `insert-diaphragm` inverse: removes the inserted row again.

use crate::insert_diaphragm::InsertDiaphragm;
use crate::remove_diaphragm::RemoveDiaphragm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertDiaphragm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_diaphragm::RemoveDiaphragm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertDiaphragm, base: &En1998Snapshot) -> Option<RemoveDiaphragm> {
    Some(RemoveDiaphragm { id: payload.diaphragm.id.clone() })
}
