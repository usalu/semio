//! 🔺️ Diff constructor for `SetIssueComment`: a sparse comment patch of exactly the provided fields that differ. The comment that results must be writable (see `comment_problem`); a patch that restates the current values is
//! `mutation.no-op`.

use super::SetIssueComment;
use crate::{comment_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetIssueComment, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.issue_comments.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Comment \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Comment \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = comment_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::issue_comments(payload.id.clone(), Entry::Patched(patch)))
}
