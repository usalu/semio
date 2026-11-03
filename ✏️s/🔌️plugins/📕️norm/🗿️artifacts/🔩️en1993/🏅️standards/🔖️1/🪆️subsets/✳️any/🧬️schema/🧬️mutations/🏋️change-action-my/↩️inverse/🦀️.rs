//! ↩️ `change-action-my` inverse: restores the previous `action.my`.

use crate::change_action_my::ChangeActionMy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionMy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_my::ChangeActionMy> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionMy { id: payload.id.clone(), new_action_my: row.action.my })],
        None => Vec::new(),
    })
}
