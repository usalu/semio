//! ↩️ `change-action-t` inverse: restores the previous `action.t`.

use crate::change_action_t::ChangeActionT;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionT, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_t::ChangeActionT> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionT { id: payload.id.clone(), new_action_t: row.action.t })],
        None => Vec::new(),
    })
}
