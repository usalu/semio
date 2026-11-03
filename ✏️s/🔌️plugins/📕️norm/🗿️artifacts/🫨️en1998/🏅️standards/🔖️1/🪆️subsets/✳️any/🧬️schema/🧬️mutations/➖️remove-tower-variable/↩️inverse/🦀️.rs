//! ↩️ `remove-tower-variable` inverse: inserts the removed row back at its position.

use crate::remove_tower_variable::RemoveTowerVariable;
use crate::insert_tower_variable::InsertTowerVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveTowerVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tower_variable::InsertTowerVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTowerVariable, base: &En1998Snapshot) -> Option<InsertTowerVariable> {
    let r0 = base.towers.iter().find(|row| row.id == payload.tower_id)?;
    let (index, row) = r0.variables.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTowerVariable { tower_id: payload.tower_id.clone(), index: Some(index), variable: row.clone() })
}
