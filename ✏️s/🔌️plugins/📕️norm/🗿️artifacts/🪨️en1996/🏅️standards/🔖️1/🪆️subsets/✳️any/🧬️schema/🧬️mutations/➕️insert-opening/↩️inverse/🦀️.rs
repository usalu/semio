//! ↩️ `insert-opening` inverse: removes the inserted row again.

use crate::insert_opening::InsertOpening;
use crate::remove_opening::RemoveOpening;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertOpening, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_opening::RemoveOpening> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertOpening, base: &En1996Snapshot) -> Option<RemoveOpening> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveOpening { wall_id: payload.wall_id.clone(), id: payload.opening.id.clone() })
}
