//! ↩️ `change-assessment-strp-leg-area-m2` inverse: restores the previous `stirrup_leg_area_m2`.

use crate::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentStrpLegAreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentStrpLegAreaM2, base: &En1998Snapshot) -> Option<ChangeAssessmentStrpLegAreaM2> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentStrpLegAreaM2 { id: payload.id.clone(), new_stirrup_leg_area_m2: r0.stirrup_leg_area_m2 })
}
