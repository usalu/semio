use crate::change_taper_apex_angle::ChangeTaperApexAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperApexAngle, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_apex_angle::ChangeTaperApexAngle> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperApexAngle { taper_id: payload.taper_id.clone(), new_value: item.apex_angle_rad })],
        None => Vec::new(),
    })
}
