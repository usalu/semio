//! ↩️ `change-anchor-action-nk` inverse: restores the previous `n_k`.

use crate::change_anchor_action_n_k::ChangeAnchorActionNK;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionNK, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_n_k::ChangeAnchorActionNK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionNK, base: &En1992Snapshot) -> Option<ChangeAnchorActionNK> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionNK { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_n_k: r1.n_k })
}
