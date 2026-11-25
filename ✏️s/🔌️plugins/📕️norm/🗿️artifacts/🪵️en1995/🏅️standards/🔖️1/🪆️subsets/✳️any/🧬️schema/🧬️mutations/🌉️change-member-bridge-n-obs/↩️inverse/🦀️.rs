use crate::change_member_bridge_n_obs::ChangeMemberBridgeNObs;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberBridgeNObs, base: &En1995Snapshot) -> Option<crate::change_member_bridge_n_obs::ChangeMemberBridgeNObs> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberBridgeNObs { member_id: payload.member_id.clone(), new_value: item.bridge_n_obs })
}
