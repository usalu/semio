//! ↩️ `remove-retaining-wall` inverse: inserts the removed row back at its position.

use crate::remove_retaining_wall::RemoveRetainingWall;
use crate::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_retaining_wall::InsertRetainingWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> Option<InsertRetainingWall> {
    let (index, row) = base.retaining_walls.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRetainingWall { index: Some(index), wall: row.clone() })
}
