//! Diff for `insert-assessment`.
use super::InsertAssessment;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.assessments.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.assessment.clone());
    protocol::MutationOutcome::new(En1998Diff { assessments: Some(items), ..Default::default() })
}
