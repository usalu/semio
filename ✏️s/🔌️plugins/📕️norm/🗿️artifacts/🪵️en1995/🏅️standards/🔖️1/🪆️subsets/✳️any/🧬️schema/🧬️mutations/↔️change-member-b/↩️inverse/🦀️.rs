use crate::change_member_b::ChangeMemberB;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberB, base: &En1995Snapshot) -> Option<crate::change_member_b::ChangeMemberB> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberB { member_id: payload.member_id.clone(), new_value: item.b_m })
}
