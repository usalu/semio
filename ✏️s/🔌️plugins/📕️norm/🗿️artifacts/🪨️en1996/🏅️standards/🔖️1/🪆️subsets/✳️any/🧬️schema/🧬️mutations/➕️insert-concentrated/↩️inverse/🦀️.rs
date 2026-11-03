//! ↩️ `insert-concentrated` inverse: removes the inserted row again.

use crate::insert_concentrated::InsertConcentrated;
use crate::remove_concentrated::RemoveConcentrated;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertConcentrated, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_concentrated::RemoveConcentrated> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertConcentrated, base: &En1996Snapshot) -> Option<RemoveConcentrated> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let _r1 = _r0.load_cases.iter().find(|row| row.id == payload.load_case_id)?;
    Some(RemoveConcentrated { wall_id: payload.wall_id.clone(), load_case_id: payload.load_case_id.clone(), id: payload.load.id.clone() })
}
