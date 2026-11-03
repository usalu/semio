//! ↩️ `change-assessment-supported-building-id` inverse: restores the previous `supported_building_id`.

use crate::change_assessment_supported_building_id::ChangeAssessmentSupportedBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentSupportedBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_supported_building_id::ChangeAssessmentSupportedBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentSupportedBuildingId, base: &En1998Snapshot) -> Option<ChangeAssessmentSupportedBuildingId> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentSupportedBuildingId { id: payload.id.clone(), new_supported_building_id: r0.supported_building_id.clone() })
}
