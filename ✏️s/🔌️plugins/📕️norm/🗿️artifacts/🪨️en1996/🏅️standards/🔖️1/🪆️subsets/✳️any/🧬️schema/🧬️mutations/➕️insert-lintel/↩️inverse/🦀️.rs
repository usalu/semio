//! ↩️ `insert-lintel` inverse: removes the inserted row again.

use crate::insert_lintel::InsertLintel;
use crate::remove_lintel::RemoveLintel;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertLintel, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_lintel::RemoveLintel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertLintel, base: &En1996Snapshot) -> Option<RemoveLintel> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveLintel { wall_id: payload.wall_id.clone(), id: payload.lintel.id.clone() })
}
