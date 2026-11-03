//! ↩️ `remove-opening` inverse: inserts the removed row back at its position.

use crate::remove_opening::RemoveOpening;
use crate::insert_opening::InsertOpening;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveOpening, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_opening::InsertOpening> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveOpening, base: &En1996Snapshot) -> Option<InsertOpening> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.openings.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertOpening { wall_id: payload.wall_id.clone(), index, opening: row.clone() })
}
