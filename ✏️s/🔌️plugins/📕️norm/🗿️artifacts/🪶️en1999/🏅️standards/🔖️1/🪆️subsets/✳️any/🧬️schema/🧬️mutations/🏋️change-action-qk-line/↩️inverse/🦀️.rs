//! ↩️ `change-action-qk-line` inverse: restores the previous `q_k_line`.

use crate::change_action_q_k_line::ChangeActionQKLine;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeActionQKLine, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_q_k_line::ChangeActionQKLine> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeActionQKLine { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_q_k_line: row.q_k_line })],
        None => Vec::new(),
    })
}
