use crate::change_member_action_vk::ChangeMemberActionVK;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberActionVK, base: &En1995Snapshot) -> Option<crate::change_member_action_vk::ChangeMemberActionVK> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return None; };
    Some(ChangeMemberActionVK { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: item.v_k_n })
}
