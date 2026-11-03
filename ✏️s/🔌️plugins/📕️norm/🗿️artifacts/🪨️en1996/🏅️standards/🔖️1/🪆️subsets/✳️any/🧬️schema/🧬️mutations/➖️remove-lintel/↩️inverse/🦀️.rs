//! ↩️ `remove-lintel` inverse: inserts the removed row back at its position.

use crate::remove_lintel::RemoveLintel;
use crate::insert_lintel::InsertLintel;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveLintel, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_lintel::InsertLintel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveLintel, base: &En1996Snapshot) -> Option<InsertLintel> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.lintels.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertLintel { wall_id: payload.wall_id.clone(), index, lintel: row.clone() })
}
