//! ↩️ `remove-pier` inverse: inserts the removed row back at its position.

use crate::remove_pier::RemovePier;
use crate::insert_pier::InsertPier;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemovePier, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pier::InsertPier> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePier, base: &En1996Snapshot) -> Option<InsertPier> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.piers.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertPier { wall_id: payload.wall_id.clone(), index, pier: row.clone() })
}
