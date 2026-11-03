//! ↩️ `insert-storey-variable` inverse: removes the inserted row again.

use crate::insert_storey_variable::InsertStoreyVariable;
use crate::remove_storey_variable::RemoveStoreyVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertStoreyVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_storey_variable::RemoveStoreyVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertStoreyVariable, base: &En1998Snapshot) -> Option<RemoveStoreyVariable> {
    let _r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let _r1 = _r0.storeys.iter().find(|row| row.id == payload.storey_id)?;
    Some(RemoveStoreyVariable { building_id: payload.building_id.clone(), storey_id: payload.storey_id.clone(), id: payload.variable.id.clone() })
}
