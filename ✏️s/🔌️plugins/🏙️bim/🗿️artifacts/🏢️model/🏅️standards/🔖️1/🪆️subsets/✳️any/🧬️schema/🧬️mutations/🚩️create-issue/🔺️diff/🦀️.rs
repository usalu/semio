//! 🔺️ Diff constructor for `CreateIssue`: one created issue entry. The id must be free across every collection, then the issue must be writable (see `issue_problem`).

use super::super::elements;
use super::CreateIssue;
use crate::{issue_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateIssue, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = issue_problem(base, &payload.id, &payload.issue) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["issue", problem.field]);
    }
    MutationOutcome::new(ModelDiff::issues(payload.id.clone(), Entry::Created(payload.issue.clone())))
}
