//! ↩️ `change-assessment-bar-diameter-mm` inverse: restores the previous `bar_diameter_mm`.

use crate::change_assessment_bar_diameter_mm::ChangeAssessmentBarDiameterMm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentBarDiameterMm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_bar_diameter_mm::ChangeAssessmentBarDiameterMm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentBarDiameterMm, base: &En1998Snapshot) -> Option<ChangeAssessmentBarDiameterMm> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentBarDiameterMm { id: payload.id.clone(), new_bar_diameter_mm: r0.bar_diameter_mm })
}
