//! ↩️ `change-assessment-as1-area-m2` inverse: restores the previous `tension_steel_area_m2`.

use crate::change_assessment_as1_area_m2::ChangeAssessmentAs1AreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentAs1AreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_as1_area_m2::ChangeAssessmentAs1AreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentAs1AreaM2, base: &En1998Snapshot) -> Option<ChangeAssessmentAs1AreaM2> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentAs1AreaM2 { id: payload.id.clone(), new_tension_steel_area_m2: r0.tension_steel_area_m2 })
}
