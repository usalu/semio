//! ↩️ `change-assessment-strp-spacing-m` inverse: restores the previous `stirrup_spacing_m`.

use crate::change_assessment_strp_spacing_m::ChangeAssessmentStrpSpacingM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentStrpSpacingM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_strp_spacing_m::ChangeAssessmentStrpSpacingM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentStrpSpacingM, base: &En1998Snapshot) -> Option<ChangeAssessmentStrpSpacingM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentStrpSpacingM { id: payload.id.clone(), new_stirrup_spacing_m: r0.stirrup_spacing_m })
}
