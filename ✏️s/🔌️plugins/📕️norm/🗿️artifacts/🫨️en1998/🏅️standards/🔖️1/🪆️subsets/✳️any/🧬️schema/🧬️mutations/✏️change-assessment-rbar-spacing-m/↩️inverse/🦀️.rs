//! ↩️ `change-assessment-rbar-spacing-m` inverse: restores the previous `restrained_bar_spacing_m`.

use crate::change_assessment_rbar_spacing_m::ChangeAssessmentRbarSpacingM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentRbarSpacingM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_rbar_spacing_m::ChangeAssessmentRbarSpacingM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentRbarSpacingM, base: &En1998Snapshot) -> Option<ChangeAssessmentRbarSpacingM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentRbarSpacingM { id: payload.id.clone(), new_restrained_bar_spacing_m: r0.restrained_bar_spacing_m })
}
