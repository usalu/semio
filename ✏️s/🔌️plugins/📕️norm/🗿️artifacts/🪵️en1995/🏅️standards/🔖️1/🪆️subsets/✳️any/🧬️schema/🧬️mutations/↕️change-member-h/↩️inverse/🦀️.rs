use crate::change_member_h::ChangeMemberH;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberH, base: &En1995Snapshot) -> Option<crate::change_member_h::ChangeMemberH> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberH { member_id: payload.member_id.clone(), new_value: item.h_m })
}
