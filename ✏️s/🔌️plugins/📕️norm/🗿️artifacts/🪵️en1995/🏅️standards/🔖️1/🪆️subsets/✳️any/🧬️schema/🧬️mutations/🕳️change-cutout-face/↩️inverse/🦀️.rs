use crate::change_cutout_face::ChangeCutoutFace;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutFace, base: &En1995Snapshot) -> Option<crate::change_cutout_face::ChangeCutoutFace> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutFace { cutout_id: payload.cutout_id.clone(), new_value: item.face }),
        None => None,
    }
}
