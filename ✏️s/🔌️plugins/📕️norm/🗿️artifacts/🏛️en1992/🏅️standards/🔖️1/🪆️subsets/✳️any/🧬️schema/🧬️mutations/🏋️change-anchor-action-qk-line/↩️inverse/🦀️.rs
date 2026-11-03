//! ↩️ `change-anchor-action-qk-line` inverse: restores the previous `q_k_line`.

use crate::change_anchor_action_q_k_line::ChangeAnchorActionQKLine;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionQKLine, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_q_k_line::ChangeAnchorActionQKLine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionQKLine, base: &En1992Snapshot) -> Option<ChangeAnchorActionQKLine> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionQKLine { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_q_k_line: r1.q_k_line })
}
