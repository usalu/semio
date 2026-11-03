//! ↩️ `change-action-tk` inverse: restores the previous `t_k`.

use crate::change_action_t_k::ChangeActionTK;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionTK, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_t_k::ChangeActionTK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionTK, base: &En1992Snapshot) -> Option<ChangeActionTK> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionTK { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_t_k: r1.t_k })
}
