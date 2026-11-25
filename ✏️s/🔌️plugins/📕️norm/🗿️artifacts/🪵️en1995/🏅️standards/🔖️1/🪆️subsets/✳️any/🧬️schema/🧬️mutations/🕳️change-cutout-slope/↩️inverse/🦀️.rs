use crate::change_cutout_slope::ChangeCutoutSlope;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutSlope, base: &En1995Snapshot) -> Option<crate::change_cutout_slope::ChangeCutoutSlope> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutSlope { cutout_id: payload.cutout_id.clone(), new_value: item.slope }),
        None => None,
    }
}
