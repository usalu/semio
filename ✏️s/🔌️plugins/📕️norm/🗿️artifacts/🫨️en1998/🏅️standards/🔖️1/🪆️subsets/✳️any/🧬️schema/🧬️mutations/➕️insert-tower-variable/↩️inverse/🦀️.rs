//! ↩️ `insert-tower-variable` inverse: removes the inserted row again.

use crate::insert_tower_variable::InsertTowerVariable;
use crate::remove_tower_variable::RemoveTowerVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertTowerVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tower_variable::RemoveTowerVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTowerVariable, base: &En1998Snapshot) -> Option<RemoveTowerVariable> {
    let _r0 = base.towers.iter().find(|row| row.id == payload.tower_id)?;
    Some(RemoveTowerVariable { tower_id: payload.tower_id.clone(), id: payload.variable.id.clone() })
}
