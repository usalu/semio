//! ↩️ `change-assessment-gamma-el` inverse: restores the previous `gamma_el`.

use crate::change_assessment_gamma_el::ChangeAssessmentGammaEl;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentGammaEl, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_gamma_el::ChangeAssessmentGammaEl> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentGammaEl, base: &En1998Snapshot) -> Option<ChangeAssessmentGammaEl> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentGammaEl { id: payload.id.clone(), new_gamma_el: r0.gamma_el })
}
