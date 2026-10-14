//! ➖️ `remove-assessment` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_assessment::RemoveAssessment;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998AssessmentDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.assessments.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::removal(&base.assessments, payload.index), ..Default::default() })
}
