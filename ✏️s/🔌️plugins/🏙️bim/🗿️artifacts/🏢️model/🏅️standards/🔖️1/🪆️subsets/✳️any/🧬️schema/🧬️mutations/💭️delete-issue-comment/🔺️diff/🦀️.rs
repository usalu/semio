//! 🔺️ Diff constructor for `DeleteIssueComment`: the comment leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteIssueComment;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteIssueComment, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.issue_comments.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Comment \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Comment", Some(&payload.id))
}
