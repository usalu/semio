//! ↩️ `change-assessment-core-depth-m` inverse: restores the previous `core_depth_m`.

use crate::change_assessment_core_depth_m::ChangeAssessmentCoreDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentCoreDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_core_depth_m::ChangeAssessmentCoreDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentCoreDepthM, base: &En1998Snapshot) -> Option<ChangeAssessmentCoreDepthM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentCoreDepthM { id: payload.id.clone(), new_core_depth_m: r0.core_depth_m })
}
