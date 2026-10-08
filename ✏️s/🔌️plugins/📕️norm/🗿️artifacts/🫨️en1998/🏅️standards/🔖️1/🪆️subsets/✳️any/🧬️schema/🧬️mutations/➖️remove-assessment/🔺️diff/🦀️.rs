//! ➖️ `remove-assessment` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveAssessment;
use crate::diff::{En1998Diff, En1998AssessmentDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.assessments.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment #{}", payload.index), [payload.index.to_string()]);
    };
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::removal(&row.id), ..Default::default() })
}
