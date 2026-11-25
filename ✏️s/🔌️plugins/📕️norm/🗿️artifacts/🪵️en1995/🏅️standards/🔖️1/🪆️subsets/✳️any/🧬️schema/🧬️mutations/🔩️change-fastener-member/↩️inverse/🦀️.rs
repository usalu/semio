use crate::change_fastener_member::ChangeFastenerMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerMember, base: &En1995Snapshot) -> Option<crate::change_fastener_member::ChangeFastenerMember> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerMember { fastener_id: payload.fastener_id.clone(), new_value: item.member_id.clone() }),
        None => None,
    }
}
