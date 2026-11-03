//! ↩️ `change-action-qk-line` inverse: restores the previous `q_k_line`.

use crate::change_action_q_k_line::ChangeActionQKLine;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionQKLine, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_q_k_line::ChangeActionQKLine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionQKLine, base: &En1992Snapshot) -> Option<ChangeActionQKLine> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionQKLine { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_q_k_line: r1.q_k_line })
}
