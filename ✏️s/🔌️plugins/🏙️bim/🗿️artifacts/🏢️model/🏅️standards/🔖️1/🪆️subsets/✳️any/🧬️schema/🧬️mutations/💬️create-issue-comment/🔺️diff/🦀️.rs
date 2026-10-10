//! 🔺️ Diff constructor for `CreateIssueComment`: one created comment entry. The id must be free across every collection, then the comment must be writable (see `comment_problem`).

use super::super::elements;
use super::CreateIssueComment;
use crate::{comment_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateIssueComment, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = comment_problem(base, &payload.id, &payload.issue_comment) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["issue_comment", problem.field]);
    }
    MutationOutcome::new(ModelDiff::issue_comments(payload.id.clone(), Entry::Created(payload.issue_comment.clone())))
}
