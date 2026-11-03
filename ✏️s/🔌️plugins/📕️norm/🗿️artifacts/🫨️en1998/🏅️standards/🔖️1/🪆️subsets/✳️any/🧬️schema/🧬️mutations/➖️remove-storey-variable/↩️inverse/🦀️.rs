//! ↩️ `remove-storey-variable` inverse: inserts the removed row back at its position.

use crate::remove_storey_variable::RemoveStoreyVariable;
use crate::insert_storey_variable::InsertStoreyVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveStoreyVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_storey_variable::InsertStoreyVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveStoreyVariable, base: &En1998Snapshot) -> Option<InsertStoreyVariable> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.storey_id)?;
    let (index, row) = r1.variables.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertStoreyVariable { building_id: payload.building_id.clone(), storey_id: payload.storey_id.clone(), index: Some(index), variable: row.clone() })
}
