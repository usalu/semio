//! ✏️ `change-assessment-supported-building-id` diff: patches `supported_building_id` of the addressed assessment row.

use crate::change_assessment_supported_building_id::ChangeAssessmentSupportedBuildingId;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentSupportedBuildingId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.supported_building_id == payload.new_supported_building_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { supported_building_id: Some(payload.new_supported_building_id.clone()), ..Default::default() }), ..Default::default() })
}
