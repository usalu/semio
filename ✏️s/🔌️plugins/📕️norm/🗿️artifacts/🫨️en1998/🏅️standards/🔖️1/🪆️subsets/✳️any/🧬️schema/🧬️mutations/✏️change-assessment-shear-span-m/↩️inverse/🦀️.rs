//! ↩️ `change-assessment-shear-span-m` inverse: restores the previous `shear_span_m`.

use crate::change_assessment_shear_span_m::ChangeAssessmentShearSpanM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentShearSpanM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_shear_span_m::ChangeAssessmentShearSpanM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentShearSpanM, base: &En1998Snapshot) -> Option<ChangeAssessmentShearSpanM> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentShearSpanM { id: payload.id.clone(), new_shear_span_m: r0.shear_span_m })
}
