//! ↩️ `change-assessment-section-depth-m` inverse: restores the previous `section_depth_m`.

use crate::change_assessment_section_depth_m::ChangeAssessmentSectionDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentSectionDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_section_depth_m::ChangeAssessmentSectionDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentSectionDepthM, base: &En1998Snapshot) -> Option<ChangeAssessmentSectionDepthM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentSectionDepthM { id: payload.id.clone(), new_section_depth_m: r0.section_depth_m })
}
