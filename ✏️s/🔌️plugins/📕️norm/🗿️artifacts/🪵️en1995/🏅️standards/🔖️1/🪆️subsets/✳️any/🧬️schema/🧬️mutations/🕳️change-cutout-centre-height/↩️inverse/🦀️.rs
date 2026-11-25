use crate::change_cutout_centre_height::ChangeCutoutCentreHeight;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutCentreHeight, base: &En1995Snapshot) -> Option<crate::change_cutout_centre_height::ChangeCutoutCentreHeight> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutCentreHeight { cutout_id: payload.cutout_id.clone(), new_value: item.centre_height_m }),
        None => None,
    }
}
