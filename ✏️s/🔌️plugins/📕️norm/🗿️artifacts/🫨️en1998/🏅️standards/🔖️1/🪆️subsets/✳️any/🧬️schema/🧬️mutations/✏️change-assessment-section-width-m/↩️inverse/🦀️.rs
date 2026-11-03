//! ↩️ `change-assessment-section-width-m` inverse: restores the previous `section_width_m`.

use crate::change_assessment_section_width_m::ChangeAssessmentSectionWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentSectionWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_section_width_m::ChangeAssessmentSectionWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentSectionWidthM, base: &En1998Snapshot) -> Option<ChangeAssessmentSectionWidthM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentSectionWidthM { id: payload.id.clone(), new_section_width_m: r0.section_width_m })
}
