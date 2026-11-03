//! ↩️ `remove-concentrated` inverse: inserts the removed row back at its position.

use crate::remove_concentrated::RemoveConcentrated;
use crate::insert_concentrated::InsertConcentrated;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveConcentrated, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_concentrated::InsertConcentrated> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveConcentrated, base: &En1996Snapshot) -> Option<InsertConcentrated> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.load_case_id)?;
    let (index, row) = r1.concentrated.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertConcentrated { wall_id: payload.wall_id.clone(), load_case_id: payload.load_case_id.clone(), index, load: row.clone() })
}
