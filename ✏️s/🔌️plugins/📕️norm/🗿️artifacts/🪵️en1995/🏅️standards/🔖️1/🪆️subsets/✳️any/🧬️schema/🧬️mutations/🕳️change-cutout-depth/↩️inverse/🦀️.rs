use crate::change_cutout_depth::ChangeCutoutDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutDepth, base: &En1995Snapshot) -> Option<crate::change_cutout_depth::ChangeCutoutDepth> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutDepth { cutout_id: payload.cutout_id.clone(), new_value: item.depth_m }),
        None => None,
    }
}
