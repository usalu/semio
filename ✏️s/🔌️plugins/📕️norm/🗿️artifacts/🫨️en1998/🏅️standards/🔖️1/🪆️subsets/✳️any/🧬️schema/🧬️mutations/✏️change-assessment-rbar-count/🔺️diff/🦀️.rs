//! ✏️ `change-assessment-rbar-count` diff: patches `restrained_bar_count` of the addressed assessment row.

use crate::change_assessment_rbar_count::ChangeAssessmentRbarCount;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentRbarCount, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_restrained_bar_count.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.restrained_bar_count == payload.new_restrained_bar_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { restrained_bar_count: Some(payload.new_restrained_bar_count), ..Default::default() }), ..Default::default() })
}
