//! ↩️ `remove-confined` inverse: inserts the removed row back at its position.

use crate::remove_confined::RemoveConfined;
use crate::insert_confined::InsertConfined;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveConfined, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_confined::InsertConfined> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveConfined, base: &En1996Snapshot) -> Option<InsertConfined> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.confined.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertConfined { wall_id: payload.wall_id.clone(), index, confined: row.clone() })
}
