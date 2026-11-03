//! ↩️ `change-variable-qk` inverse: restores the previous `qk`.

use crate::change_variable_qk::ChangeVariableQk;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeVariableQk, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_variable_qk::ChangeVariableQk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeVariableQk, base: &En1990Snapshot) -> Option<ChangeVariableQk> {
    let r0 = base.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeVariableQk { id: payload.id.clone(), new_qk: r0.qk })
}
