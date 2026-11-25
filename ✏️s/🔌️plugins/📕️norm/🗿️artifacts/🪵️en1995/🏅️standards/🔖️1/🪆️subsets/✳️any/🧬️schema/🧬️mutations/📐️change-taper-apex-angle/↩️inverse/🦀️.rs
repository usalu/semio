use crate::change_taper_apex_angle::ChangeTaperApexAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperApexAngle, base: &En1995Snapshot) -> Option<crate::change_taper_apex_angle::ChangeTaperApexAngle> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperApexAngle { taper_id: payload.taper_id.clone(), new_value: item.apex_angle_rad }),
        None => None,
    }
}
