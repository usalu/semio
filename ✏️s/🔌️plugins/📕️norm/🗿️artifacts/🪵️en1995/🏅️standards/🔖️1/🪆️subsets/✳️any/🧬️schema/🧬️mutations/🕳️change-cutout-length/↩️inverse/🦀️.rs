use crate::change_cutout_length::ChangeCutoutLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutLength, base: &En1995Snapshot) -> Option<crate::change_cutout_length::ChangeCutoutLength> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutLength { cutout_id: payload.cutout_id.clone(), new_value: item.length_m }),
        None => None,
    }
}
