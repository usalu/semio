//! ↩️ `remove-foundation` inverse: inserts the removed row back at its position.

use crate::remove_foundation::RemoveFoundation;
use crate::insert_foundation::InsertFoundation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveFoundation, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_foundation::InsertFoundation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFoundation, base: &En1998Snapshot) -> Option<InsertFoundation> {
    let (index, row) = base.foundations.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFoundation { index: Some(index), foundation: row.clone() })
}
