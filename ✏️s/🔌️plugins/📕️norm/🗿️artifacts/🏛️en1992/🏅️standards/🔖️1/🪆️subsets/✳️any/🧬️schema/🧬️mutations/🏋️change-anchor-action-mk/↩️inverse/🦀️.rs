//! ↩️ `change-anchor-action-mk` inverse: restores the previous `m_k`.

use crate::change_anchor_action_m_k::ChangeAnchorActionMK;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionMK, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_m_k::ChangeAnchorActionMK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionMK, base: &En1992Snapshot) -> Option<ChangeAnchorActionMK> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionMK { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_m_k: r1.m_k })
}
