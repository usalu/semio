use crate::change_cutout_distance::ChangeCutoutDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutDistance, base: &En1995Snapshot) -> Option<crate::change_cutout_distance::ChangeCutoutDistance> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutDistance { cutout_id: payload.cutout_id.clone(), new_value: item.distance_m }),
        None => None,
    }
}
