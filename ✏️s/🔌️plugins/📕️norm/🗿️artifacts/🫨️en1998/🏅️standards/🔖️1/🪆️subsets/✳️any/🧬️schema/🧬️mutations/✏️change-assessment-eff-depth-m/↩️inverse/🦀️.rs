//! ↩️ `change-assessment-eff-depth-m` inverse: restores the previous `effective_depth_m`.

use crate::change_assessment_eff_depth_m::ChangeAssessmentEffDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentEffDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_eff_depth_m::ChangeAssessmentEffDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentEffDepthM, base: &En1998Snapshot) -> Option<ChangeAssessmentEffDepthM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentEffDepthM { id: payload.id.clone(), new_effective_depth_m: r0.effective_depth_m })
}
