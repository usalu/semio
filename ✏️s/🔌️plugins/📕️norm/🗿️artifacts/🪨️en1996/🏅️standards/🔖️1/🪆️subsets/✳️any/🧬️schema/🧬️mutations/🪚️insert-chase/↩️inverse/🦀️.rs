//! ↩️ `insert-chase` inverse: removes the inserted row again.

use crate::insert_chase::InsertChase;
use crate::remove_chase::RemoveChase;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertChase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_chase::RemoveChase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertChase, base: &En1996Snapshot) -> Option<RemoveChase> {
    Some(RemoveChase { id: payload.item.id.clone() })
}
