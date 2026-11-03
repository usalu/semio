//! ↩️ `change-bridge-variable-qk-n` inverse: restores the previous `qk_n`.

use crate::change_bridge_variable_qk_n::ChangeBridgeVariableQkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeVariableQkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_variable_qk_n::ChangeBridgeVariableQkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeVariableQkN, base: &En1998Snapshot) -> Option<ChangeBridgeVariableQkN> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.bridge_id)?;
    let r1 = r0.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeVariableQkN { bridge_id: payload.bridge_id.clone(), id: payload.id.clone(), new_qk_n: r1.qk_n })
}
