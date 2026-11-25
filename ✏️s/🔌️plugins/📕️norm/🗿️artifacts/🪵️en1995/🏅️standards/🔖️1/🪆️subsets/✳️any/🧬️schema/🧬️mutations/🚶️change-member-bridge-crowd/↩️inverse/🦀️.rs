use crate::change_member_bridge_crowd::ChangeMemberBridgeCrowd;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberBridgeCrowd, base: &En1995Snapshot) -> Option<crate::change_member_bridge_crowd::ChangeMemberBridgeCrowd> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberBridgeCrowd { member_id: payload.member_id.clone(), new_value: item.bridge_crowd_per_m2 })
}
