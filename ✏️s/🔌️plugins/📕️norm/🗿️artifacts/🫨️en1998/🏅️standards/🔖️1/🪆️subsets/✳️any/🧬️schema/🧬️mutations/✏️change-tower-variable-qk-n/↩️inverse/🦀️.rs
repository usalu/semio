//! ↩️ `change-tower-variable-qk-n` inverse: restores the previous `qk_n`.

use crate::change_tower_variable_qk_n::ChangeTowerVariableQkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerVariableQkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_variable_qk_n::ChangeTowerVariableQkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerVariableQkN, base: &En1998Snapshot) -> Option<ChangeTowerVariableQkN> {
    let r0 = base.towers.iter().find(|row| row.id == payload.tower_id)?;
    let r1 = r0.variables.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerVariableQkN { tower_id: payload.tower_id.clone(), id: payload.id.clone(), new_qk_n: r1.qk_n })
}
