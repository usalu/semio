use crate::change_bracing_member::ChangeBracingMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingMember, base: &En1995Snapshot) -> Option<crate::change_bracing_member::ChangeBracingMember> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingMember { bracing_id: payload.bracing_id.clone(), new_value: item.member_id.clone() }),
        None => None,
    }
}
