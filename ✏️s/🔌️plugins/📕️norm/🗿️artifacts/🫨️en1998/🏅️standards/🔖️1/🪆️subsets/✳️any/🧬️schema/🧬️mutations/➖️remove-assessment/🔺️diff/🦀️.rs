//! Diff for `remove-assessment`.
use super::RemoveAssessment;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.assessments.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.assessments.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { assessments: Some(items), ..Default::default() })
}
