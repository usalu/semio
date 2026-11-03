//! ✏️ `change-assessment-as2-area-m2` diff: patches `compression_steel_area_m2` of the addressed assessment row.

use crate::change_assessment_as2_area_m2::ChangeAssessmentAs2AreaM2;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentAs2AreaM2, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_compression_steel_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.compression_steel_area_m2 == payload.new_compression_steel_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { compression_steel_area_m2: Some(payload.new_compression_steel_area_m2), ..Default::default() }), ..Default::default() })
}
