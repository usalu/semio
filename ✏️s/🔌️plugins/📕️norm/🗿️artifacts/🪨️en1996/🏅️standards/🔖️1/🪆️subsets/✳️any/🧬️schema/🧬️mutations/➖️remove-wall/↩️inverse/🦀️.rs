//! ↩️ `remove-wall` inverse: inserts the removed row back at its position.

use crate::remove_wall::RemoveWall;
use crate::insert_wall::InsertWall;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveWall, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_wall::InsertWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveWall, base: &En1996Snapshot) -> Option<InsertWall> {
    let (index, row) = base.walls.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertWall { index, wall: row.clone() })
}
