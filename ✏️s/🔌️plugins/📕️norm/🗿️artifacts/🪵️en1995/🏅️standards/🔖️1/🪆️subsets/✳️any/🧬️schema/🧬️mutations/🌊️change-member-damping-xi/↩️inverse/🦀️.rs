use crate::change_member_damping_xi::ChangeMemberDampingXi;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberDampingXi, base: &En1995Snapshot) -> Option<crate::change_member_damping_xi::ChangeMemberDampingXi> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberDampingXi { member_id: payload.member_id.clone(), new_value: item.damping_xi })
}
