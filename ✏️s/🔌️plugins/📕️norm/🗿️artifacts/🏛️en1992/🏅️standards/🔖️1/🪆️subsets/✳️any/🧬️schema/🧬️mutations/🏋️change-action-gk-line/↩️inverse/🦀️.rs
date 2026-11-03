//! ↩️ `change-action-gk-line` inverse: restores the previous `g_k_line`.

use crate::change_action_g_k_line::ChangeActionGKLine;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionGKLine, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_g_k_line::ChangeActionGKLine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionGKLine, base: &En1992Snapshot) -> Option<ChangeActionGKLine> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionGKLine { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_g_k_line: r1.g_k_line })
}
