use crate::change_member_service_class::ChangeMemberServiceClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberServiceClass, base: &En1995Snapshot) -> Option<crate::change_member_service_class::ChangeMemberServiceClass> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberServiceClass { member_id: payload.member_id.clone(), new_value: item.service_class })
}
