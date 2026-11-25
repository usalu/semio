use crate::change_member_label_en::ChangeMemberLabelEn;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberLabelEn, base: &En1995Snapshot) -> Option<crate::change_member_label_en::ChangeMemberLabelEn> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberLabelEn { member_id: payload.member_id.clone(), new_value: item.label_en.clone() })
}
