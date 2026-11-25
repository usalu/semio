use crate::change_cutout_kind::ChangeCutoutKind;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutKind, base: &En1995Snapshot) -> Option<crate::change_cutout_kind::ChangeCutoutKind> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutKind { cutout_id: payload.cutout_id.clone(), new_value: item.kind }),
        None => None,
    }
}
