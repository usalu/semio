//! ↩️ `change-assessment-as2-area-m2` inverse: restores the previous `compression_steel_area_m2`.

use crate::change_assessment_as2_area_m2::ChangeAssessmentAs2AreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentAs2AreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_as2_area_m2::ChangeAssessmentAs2AreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentAs2AreaM2, base: &En1998Snapshot) -> Option<ChangeAssessmentAs2AreaM2> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentAs2AreaM2 { id: payload.id.clone(), new_compression_steel_area_m2: r0.compression_steel_area_m2 })
}
