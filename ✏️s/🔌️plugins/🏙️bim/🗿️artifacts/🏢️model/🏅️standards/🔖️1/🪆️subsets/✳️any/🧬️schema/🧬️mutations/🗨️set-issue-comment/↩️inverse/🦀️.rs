//! ↩️ Inverse of `SetIssueComment`: an absolute `SetIssueComment` restoring the base value of exactly the fields the forward really changes, none when the comment is absent or nothing changes.

use super::SetIssueComment;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetIssueComment, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.issue_comments.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetIssueComment(SetIssueComment::from_patch(payload.id.clone(), restore))]
}
