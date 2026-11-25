use crate::change_member_label_de::ChangeMemberLabelDe;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberLabelDe, base: &En1995Snapshot) -> Option<crate::change_member_label_de::ChangeMemberLabelDe> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberLabelDe { member_id: payload.member_id.clone(), new_value: item.label_de.clone() })
}
