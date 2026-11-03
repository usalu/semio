//! ↩️ `change-tower-variable-category` inverse: restores the previous `category`.

use crate::change_tower_variable_category::ChangeTowerVariableCategory;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerVariableCategory, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_variable_category::ChangeTowerVariableCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerVariableCategory, base: &En1998Snapshot) -> Option<ChangeTowerVariableCategory> {
    let r0 = base.towers.iter().find(|row| row.id == payload.tower_id)?;
    let r1 = r0.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerVariableCategory { tower_id: payload.tower_id.clone(), id: payload.id.clone(), new_category: r1.category })
}
