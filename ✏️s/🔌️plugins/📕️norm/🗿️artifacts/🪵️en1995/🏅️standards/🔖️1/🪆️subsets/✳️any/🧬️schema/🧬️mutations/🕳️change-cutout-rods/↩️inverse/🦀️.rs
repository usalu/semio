use crate::change_cutout_rods::ChangeCutoutRods;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutRods, base: &En1995Snapshot) -> Option<crate::change_cutout_rods::ChangeCutoutRods> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutRods { cutout_id: payload.cutout_id.clone(), new_value: item.rods }),
        None => None,
    }
}
