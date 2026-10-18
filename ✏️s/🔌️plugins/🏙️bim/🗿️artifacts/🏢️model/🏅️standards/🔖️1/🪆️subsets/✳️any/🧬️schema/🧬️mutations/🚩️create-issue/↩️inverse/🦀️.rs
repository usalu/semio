//! ↩️ Inverse of `CreateIssue`: the concrete `DeleteIssue` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_issue::DeleteIssue;
use crate::standards::v1::subsets::any::schema::mutations::create_issue::CreateIssue;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateIssue, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_issue::DeleteIssue>,
{
    if base.issues.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteIssue { id: payload.id.clone() })]
}
