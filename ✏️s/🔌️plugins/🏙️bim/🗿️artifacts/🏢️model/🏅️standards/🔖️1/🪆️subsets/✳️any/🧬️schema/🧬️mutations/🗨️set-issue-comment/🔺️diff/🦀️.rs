//! 🔺️ Diff constructor for `SetIssueComment`: a sparse comment patch of exactly the provided fields that differ. The comment that results must be writable (see `comment_problem`); a patch that restates the current values is
//! `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::SetIssueComment;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{comment_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
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
