//! ↩️ `remove-chase` inverse: inserts the removed row back at its position.

use crate::remove_chase::RemoveChase;
use crate::insert_chase::InsertChase;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveChase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_chase::InsertChase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveChase, base: &En1996Snapshot) -> Option<InsertChase> {
    let (index, row) = base.chases.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertChase { index, item: row.clone() })
}
