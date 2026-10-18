//! ↩️ Inverse of `CreateIssueComment`: the concrete `DeleteIssueComment` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_issue_comment::DeleteIssueComment;
use crate::standards::v1::subsets::any::schema::mutations::create_issue_comment::CreateIssueComment;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateIssueComment, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_issue_comment::DeleteIssueComment>,
{
    if base.issue_comments.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteIssueComment { id: payload.id.clone() })]
}
