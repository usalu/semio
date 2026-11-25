use crate::change_member_bridge_beta::ChangeMemberBridgeBeta;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberBridgeBeta, base: &En1995Snapshot) -> Option<crate::change_member_bridge_beta::ChangeMemberBridgeBeta> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberBridgeBeta { member_id: payload.member_id.clone(), new_value: item.bridge_beta })
}
