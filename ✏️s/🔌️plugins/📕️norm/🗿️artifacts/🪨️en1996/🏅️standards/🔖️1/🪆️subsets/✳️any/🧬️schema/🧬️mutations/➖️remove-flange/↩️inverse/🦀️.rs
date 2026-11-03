//! ↩️ `remove-flange` inverse: inserts the removed row back at its position.

use crate::remove_flange::RemoveFlange;
use crate::insert_flange::InsertFlange;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveFlange, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_flange::InsertFlange> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFlange, base: &En1996Snapshot) -> Option<InsertFlange> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.flanges.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFlange { wall_id: payload.wall_id.clone(), index, flange: row.clone() })
}
