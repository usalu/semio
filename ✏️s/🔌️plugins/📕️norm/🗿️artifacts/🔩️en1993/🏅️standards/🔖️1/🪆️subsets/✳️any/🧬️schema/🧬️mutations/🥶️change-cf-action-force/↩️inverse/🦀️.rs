//! ↩️ `change-cf-action-force` inverse: restores the previous `force`.

use crate::change_cf_action_force::ChangeCfActionForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfActionForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_action_force::ChangeCfActionForce> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeCfActionForce { id: payload.id.clone(), element_id: payload.element_id.clone(), new_force: row.force })],
        None => Vec::new(),
    })
}
