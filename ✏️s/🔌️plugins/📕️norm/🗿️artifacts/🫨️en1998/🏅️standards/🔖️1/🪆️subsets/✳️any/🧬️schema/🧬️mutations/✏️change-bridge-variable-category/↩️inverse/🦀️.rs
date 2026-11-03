//! ↩️ `change-bridge-variable-category` inverse: restores the previous `category`.

use crate::change_bridge_variable_category::ChangeBridgeVariableCategory;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeVariableCategory, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_variable_category::ChangeBridgeVariableCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeVariableCategory, base: &En1998Snapshot) -> Option<ChangeBridgeVariableCategory> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.bridge_id)?;
    let r1 = r0.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeVariableCategory { bridge_id: payload.bridge_id.clone(), id: payload.id.clone(), new_category: r1.category })
}
