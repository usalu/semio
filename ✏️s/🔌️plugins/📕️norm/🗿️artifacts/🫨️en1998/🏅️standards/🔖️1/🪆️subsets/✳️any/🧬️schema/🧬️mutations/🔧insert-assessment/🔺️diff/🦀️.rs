//! Diff for `insert-assessment`.
use super::InsertAssessment;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.assessments.iter().any(|existing| existing.id == payload.assessment.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Assessment id {} already exists.", payload.assessment.id), [payload.assessment.id.clone()]);
    }
    let mut items = base.assessments.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.assessment.clone());
    protocol::MutationOutcome::new(En1998Diff { assessments: Some(items), ..Default::default() })
}
