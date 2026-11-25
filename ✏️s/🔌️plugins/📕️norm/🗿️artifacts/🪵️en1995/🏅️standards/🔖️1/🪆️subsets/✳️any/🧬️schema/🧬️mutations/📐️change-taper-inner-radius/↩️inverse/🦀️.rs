use crate::change_taper_inner_radius::ChangeTaperInnerRadius;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperInnerRadius, base: &En1995Snapshot) -> Option<crate::change_taper_inner_radius::ChangeTaperInnerRadius> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperInnerRadius { taper_id: payload.taper_id.clone(), new_value: item.inner_radius_m }),
        None => None,
    }
}
