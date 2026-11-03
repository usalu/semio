//! ↩️ `insert-wall` inverse: removes the inserted row again.

use crate::insert_wall::InsertWall;
use crate::remove_wall::RemoveWall;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertWall, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_wall::RemoveWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertWall, base: &En1996Snapshot) -> Option<RemoveWall> {
    Some(RemoveWall { id: payload.wall.id.clone() })
}
