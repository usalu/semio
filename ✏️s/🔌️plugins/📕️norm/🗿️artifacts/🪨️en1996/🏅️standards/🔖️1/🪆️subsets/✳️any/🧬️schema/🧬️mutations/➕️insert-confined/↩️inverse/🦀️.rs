//! ↩️ `insert-confined` inverse: removes the inserted row again.

use crate::insert_confined::InsertConfined;
use crate::remove_confined::RemoveConfined;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertConfined, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_confined::RemoveConfined> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertConfined, base: &En1996Snapshot) -> Option<RemoveConfined> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveConfined { wall_id: payload.wall_id.clone(), id: payload.confined.id.clone() })
}
