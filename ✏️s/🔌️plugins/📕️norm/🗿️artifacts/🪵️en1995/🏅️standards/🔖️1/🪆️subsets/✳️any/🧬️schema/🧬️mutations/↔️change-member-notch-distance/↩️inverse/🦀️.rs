use crate::change_member_notch_distance::ChangeMemberNotchDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberNotchDistance, base: &En1995Snapshot) -> Option<crate::change_member_notch_distance::ChangeMemberNotchDistance> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberNotchDistance { member_id: payload.member_id.clone(), new_value: item.notch_distance_m })
}
