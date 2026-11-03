//! ↩️ `remove-bar` inverse: inserts the removed row back at its position.

use crate::remove_bar::RemoveBar;
use crate::insert_bar::InsertBar;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveBar, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bar::InsertBar> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBar, base: &En1996Snapshot) -> Option<InsertBar> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.bars.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBar { wall_id: payload.wall_id.clone(), index, bar: row.clone() })
}
