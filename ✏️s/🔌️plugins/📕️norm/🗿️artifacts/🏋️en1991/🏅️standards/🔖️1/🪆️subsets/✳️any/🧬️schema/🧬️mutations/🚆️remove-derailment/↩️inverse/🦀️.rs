//! ↩️ `remove-derailment` inverse: inserts the removed row back at its position.

use crate::remove_derailment::RemoveDerailment;
use crate::insert_derailment::InsertDerailment;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveDerailment, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_derailment::InsertDerailment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveDerailment, base: &En1991Snapshot) -> Option<InsertDerailment> {
    let (index, row) = base.derailment_cases.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertDerailment { index, item: row.clone() })
}
