//! ↩️ `change-anchor-action-gk-line` inverse: restores the previous `g_k_line`.

use crate::change_anchor_action_g_k_line::ChangeAnchorActionGKLine;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionGKLine, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_g_k_line::ChangeAnchorActionGKLine> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionGKLine, base: &En1992Snapshot) -> Option<ChangeAnchorActionGKLine> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionGKLine { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_g_k_line: r1.g_k_line })
}
