use crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberBucklingLengthZ, base: &En1995Snapshot) -> Option<crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberBucklingLengthZ { member_id: payload.member_id.clone(), new_value: item.buckling_length_z_m })
}
