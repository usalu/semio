//! ↩️ `change-assessed element-intermediate-steel-area-m2` inverse: restores the previous `intermediate_steel_area_m2`.

use crate::change_assessment_mid_steel_area_m2::ChangeAssessmentMidSteelAreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentMidSteelAreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_mid_steel_area_m2::ChangeAssessmentMidSteelAreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentMidSteelAreaM2, base: &En1998Snapshot) -> Option<ChangeAssessmentMidSteelAreaM2> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentMidSteelAreaM2 { id: payload.id.clone(), new_intermediate_steel_area_m2: r0.intermediate_steel_area_m2 })
}
