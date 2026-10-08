//! 🔧 `insert-assessment` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertAssessment;
use crate::diff::{En1998Diff, En1998AssessmentDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.assessments.iter().any(|existing| existing.id == payload.assessment.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Assessment id {} already exists.", payload.assessment.id), [payload.assessment.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.assessments.len());
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::insertion(index, payload.assessment.clone()), ..Default::default() })
}
