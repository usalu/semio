use crate::change_member_action_fc90_k::ChangeMemberActionFC90K;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeMemberActionFC90K, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_action_fc90_k::ChangeMemberActionFC90K> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };
    vec![Op::from(ChangeMemberActionFC90K { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: item.f_c90_k_n })]

    })())
}
