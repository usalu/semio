//! ↩️ `insert-bar` inverse: removes the inserted row again.

use crate::insert_bar::InsertBar;
use crate::remove_bar::RemoveBar;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertBar, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bar::RemoveBar> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBar, base: &En1996Snapshot) -> Option<RemoveBar> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveBar { wall_id: payload.wall_id.clone(), id: payload.bar.id.clone() })
}
