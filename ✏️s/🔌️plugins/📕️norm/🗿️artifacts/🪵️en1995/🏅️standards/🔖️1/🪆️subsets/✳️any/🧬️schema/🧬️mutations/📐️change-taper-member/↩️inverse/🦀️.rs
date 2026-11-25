use crate::change_taper_member::ChangeTaperMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperMember, base: &En1995Snapshot) -> Option<crate::change_taper_member::ChangeTaperMember> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperMember { taper_id: payload.taper_id.clone(), new_value: item.member_id.clone() }),
        None => None,
    }
}
