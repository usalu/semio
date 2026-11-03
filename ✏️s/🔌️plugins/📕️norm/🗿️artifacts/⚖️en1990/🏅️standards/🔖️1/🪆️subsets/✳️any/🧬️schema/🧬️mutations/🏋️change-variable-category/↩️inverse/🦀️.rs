//! ↩️ `change-variable-category` inverse: restores the previous `category`.

use crate::change_variable_category::ChangeVariableCategory;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeVariableCategory, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_variable_category::ChangeVariableCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeVariableCategory, base: &En1990Snapshot) -> Option<ChangeVariableCategory> {
    let r0 = base.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeVariableCategory { id: payload.id.clone(), new_category: r0.category.clone() })
}
