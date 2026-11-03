//! ↩️ `change-storey-variable-category` inverse: restores the previous `category`.

use crate::change_storey_variable_category::ChangeStoreyVariableCategory;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyVariableCategory, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_variable_category::ChangeStoreyVariableCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyVariableCategory, base: &En1998Snapshot) -> Option<ChangeStoreyVariableCategory> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.storey_id)?;
    let r2 = r1.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyVariableCategory { building_id: payload.building_id.clone(), storey_id: payload.storey_id.clone(), id: payload.id.clone(), new_category: r2.category })
}
