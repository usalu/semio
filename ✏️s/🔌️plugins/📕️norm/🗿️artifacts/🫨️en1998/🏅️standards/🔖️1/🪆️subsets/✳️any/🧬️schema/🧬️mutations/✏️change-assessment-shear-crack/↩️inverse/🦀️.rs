//! ↩️ `change-assessment-shear-crack` inverse: restores the previous `shear_cracking_before_yield`.

use crate::change_assessment_shear_crack::ChangeAssessmentShearCrack;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentShearCrack, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_shear_crack::ChangeAssessmentShearCrack> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentShearCrack, base: &En1998Snapshot) -> Option<ChangeAssessmentShearCrack> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentShearCrack { id: payload.id.clone(), new_shear_cracking_before_yield: r0.shear_cracking_before_yield })
}
