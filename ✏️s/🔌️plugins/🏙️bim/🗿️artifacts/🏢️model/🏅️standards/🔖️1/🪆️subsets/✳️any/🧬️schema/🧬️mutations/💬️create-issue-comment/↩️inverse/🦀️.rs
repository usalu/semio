//! ↩️ Inverse of `CreateIssueComment`: the concrete `DeleteIssueComment` of the id it created, none when the id was already taken.

use super::super::delete_issue_comment::DeleteIssueComment;
use super::CreateIssueComment;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateIssueComment, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.issue_comments.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteIssueComment(DeleteIssueComment { id: payload.id.clone() })]
}
