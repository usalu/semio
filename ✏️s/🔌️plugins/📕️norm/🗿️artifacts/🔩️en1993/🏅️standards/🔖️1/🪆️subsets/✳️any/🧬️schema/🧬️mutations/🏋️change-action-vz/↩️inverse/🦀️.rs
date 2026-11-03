//! ↩️ `change-action-vz` inverse: restores the previous `action.vz`.

use crate::change_action_vz::ChangeActionVz;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionVz, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_vz::ChangeActionVz> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionVz { id: payload.id.clone(), new_action_vz: row.action.vz })],
        None => Vec::new(),
    })
}
