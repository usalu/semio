use crate::change_member_support::ChangeMemberSupport;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberSupport, base: &En1995Snapshot) -> Option<crate::change_member_support::ChangeMemberSupport> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberSupport { member_id: payload.member_id.clone(), new_value: item.support })
}
