use crate::change_action_vk::ChangeActionVk;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionVk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_vk::ChangeActionVk> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(a) = m.actions.iter().find(|a| a.id == payload.action_id) else { return vec![]; };
    vec![Op::from(ChangeActionVk { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: a.v_k })]

    })())
}
