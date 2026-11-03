//! ↩️ `change-action-vzk` inverse: restores the previous `v_z_k`.

use crate::change_action_v_z_k::ChangeActionVZK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeActionVZK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_v_z_k::ChangeActionVZK> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeActionVZK { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_v_z_k: row.v_z_k })],
        None => Vec::new(),
    })
}
