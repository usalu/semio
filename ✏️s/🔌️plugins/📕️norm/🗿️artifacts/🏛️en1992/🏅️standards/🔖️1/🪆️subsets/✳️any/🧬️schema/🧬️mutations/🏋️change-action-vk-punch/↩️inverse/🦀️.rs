//! ↩️ `change-action-vk-punch` inverse: restores the previous `v_k_punch`.

use crate::change_action_v_k_punch::ChangeActionVKPunch;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionVKPunch, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_v_k_punch::ChangeActionVKPunch> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionVKPunch, base: &En1992Snapshot) -> Option<ChangeActionVKPunch> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionVKPunch { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_v_k_punch: r1.v_k_punch })
}
