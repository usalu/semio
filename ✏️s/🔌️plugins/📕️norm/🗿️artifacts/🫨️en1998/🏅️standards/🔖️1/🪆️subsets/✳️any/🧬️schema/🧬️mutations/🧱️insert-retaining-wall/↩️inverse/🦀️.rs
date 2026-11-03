//! ↩️ `insert-retaining-wall` inverse: removes the inserted row again.

use crate::insert_retaining_wall::InsertRetainingWall;
use crate::remove_retaining_wall::RemoveRetainingWall;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertRetainingWall, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_retaining_wall::RemoveRetainingWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRetainingWall, base: &En1998Snapshot) -> Option<RemoveRetainingWall> {
    Some(RemoveRetainingWall { id: payload.wall.id.clone() })
}
