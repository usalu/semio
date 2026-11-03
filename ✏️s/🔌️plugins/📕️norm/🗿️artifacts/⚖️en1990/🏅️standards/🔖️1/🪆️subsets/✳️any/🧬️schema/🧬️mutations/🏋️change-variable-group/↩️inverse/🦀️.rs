//! ↩️ `change-variable-group` inverse: restores the previous `group`.

use crate::change_variable_group::ChangeVariableGroup;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeVariableGroup, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_variable_group::ChangeVariableGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeVariableGroup, base: &En1990Snapshot) -> Option<ChangeVariableGroup> {
    let r0 = base.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeVariableGroup { id: payload.id.clone(), new_group: r0.group.clone() })
}
