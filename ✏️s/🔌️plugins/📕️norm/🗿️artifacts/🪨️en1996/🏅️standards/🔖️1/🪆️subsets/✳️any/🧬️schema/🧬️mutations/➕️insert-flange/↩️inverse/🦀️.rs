//! ↩️ `insert-flange` inverse: removes the inserted row again.

use crate::insert_flange::InsertFlange;
use crate::remove_flange::RemoveFlange;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertFlange, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_flange::RemoveFlange> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFlange, base: &En1996Snapshot) -> Option<RemoveFlange> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveFlange { wall_id: payload.wall_id.clone(), id: payload.flange.id.clone() })
}
