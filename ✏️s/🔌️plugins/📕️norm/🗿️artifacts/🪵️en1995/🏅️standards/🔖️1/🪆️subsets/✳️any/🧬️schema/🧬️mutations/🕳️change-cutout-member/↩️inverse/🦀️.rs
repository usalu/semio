use crate::change_cutout_member::ChangeCutoutMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutMember, base: &En1995Snapshot) -> Option<crate::change_cutout_member::ChangeCutoutMember> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutMember { cutout_id: payload.cutout_id.clone(), new_value: item.member_id.clone() }),
        None => None,
    }
}
