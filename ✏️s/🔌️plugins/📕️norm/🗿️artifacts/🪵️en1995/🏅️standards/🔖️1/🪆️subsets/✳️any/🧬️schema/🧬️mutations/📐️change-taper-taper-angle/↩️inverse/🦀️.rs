use crate::change_taper_taper_angle::ChangeTaperTaperAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperTaperAngle, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_taper_angle::ChangeTaperTaperAngle> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperTaperAngle { taper_id: payload.taper_id.clone(), new_value: item.taper_angle_rad })],
        None => Vec::new(),
    })
}
