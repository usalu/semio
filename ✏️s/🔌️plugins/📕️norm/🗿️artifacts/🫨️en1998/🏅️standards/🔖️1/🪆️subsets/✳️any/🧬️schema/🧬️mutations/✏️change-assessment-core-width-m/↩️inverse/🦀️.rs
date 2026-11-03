//! ↩️ `change-assessment-core-width-m` inverse: restores the previous `core_width_m`.

use crate::change_assessment_core_width_m::ChangeAssessmentCoreWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentCoreWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_core_width_m::ChangeAssessmentCoreWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentCoreWidthM, base: &En1998Snapshot) -> Option<ChangeAssessmentCoreWidthM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentCoreWidthM { id: payload.id.clone(), new_core_width_m: r0.core_width_m })
}
