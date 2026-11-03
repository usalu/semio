//! ↩️ `insert-derailment` inverse: removes the inserted row again.

use crate::insert_derailment::InsertDerailment;
use crate::remove_derailment::RemoveDerailment;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertDerailment, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_derailment::RemoveDerailment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertDerailment, base: &En1991Snapshot) -> Option<RemoveDerailment> {
    Some(RemoveDerailment { id: payload.item.id.clone() })
}
