//! ↩️ `remove-tie` inverse: inserts the removed row back at its position.

use crate::remove_tie::RemoveTie;
use crate::insert_tie::InsertTie;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveTie, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tie::InsertTie> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTie, base: &En1996Snapshot) -> Option<InsertTie> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.ties.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTie { wall_id: payload.wall_id.clone(), index, tie: row.clone() })
}
