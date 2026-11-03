//! ↩️ `insert-pier` inverse: removes the inserted row again.

use crate::insert_pier::InsertPier;
use crate::remove_pier::RemovePier;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertPier, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pier::RemovePier> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPier, base: &En1996Snapshot) -> Option<RemovePier> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemovePier { wall_id: payload.wall_id.clone(), id: payload.pier.id.clone() })
}
