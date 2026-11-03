//! ↩️ `change-anchor-action-tk` inverse: restores the previous `t_k`.

use crate::change_anchor_action_t_k::ChangeAnchorActionTK;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionTK, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_t_k::ChangeAnchorActionTK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionTK, base: &En1992Snapshot) -> Option<ChangeAnchorActionTK> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionTK { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_t_k: r1.t_k })
}
