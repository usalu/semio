//! ➖️ `remove-assessment` diff: removes the addressed assessment row.

use crate::remove_assessment::RemoveAssessment;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.assessments.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::removal(&base.assessments, index), ..Default::default() })
}
