//! ↩️ Inverse of `SetIssueComment`: an absolute `SetIssueComment` restoring the base value of exactly the fields the forward really changes, none when the comment is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::SetIssueComment;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetIssueComment, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::SetIssueComment>,
{
    let Some(record) = base.issue_comments.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetIssueComment::from_patch(payload.id.clone(), restore))]
}
