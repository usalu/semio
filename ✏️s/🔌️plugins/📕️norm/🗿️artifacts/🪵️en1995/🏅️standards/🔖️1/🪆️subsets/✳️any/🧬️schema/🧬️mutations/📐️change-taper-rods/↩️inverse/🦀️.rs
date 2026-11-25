use crate::change_taper_rods::ChangeTaperRods;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperRods, base: &En1995Snapshot) -> Option<crate::change_taper_rods::ChangeTaperRods> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperRods { taper_id: payload.taper_id.clone(), new_value: item.rods }),
        None => None,
    }
}
