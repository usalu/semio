use crate::change_member_support_length::ChangeMemberSupportLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberSupportLength, base: &En1995Snapshot) -> Option<crate::change_member_support_length::ChangeMemberSupportLength> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberSupportLength { member_id: payload.member_id.clone(), new_value: item.support_length_m })
}
