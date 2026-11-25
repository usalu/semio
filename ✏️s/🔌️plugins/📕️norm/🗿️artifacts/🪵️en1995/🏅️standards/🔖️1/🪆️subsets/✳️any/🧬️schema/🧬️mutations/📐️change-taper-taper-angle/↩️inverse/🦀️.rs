use crate::change_taper_taper_angle::ChangeTaperTaperAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperTaperAngle, base: &En1995Snapshot) -> Option<crate::change_taper_taper_angle::ChangeTaperTaperAngle> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperTaperAngle { taper_id: payload.taper_id.clone(), new_value: item.taper_angle_rad }),
        None => None,
    }
}
