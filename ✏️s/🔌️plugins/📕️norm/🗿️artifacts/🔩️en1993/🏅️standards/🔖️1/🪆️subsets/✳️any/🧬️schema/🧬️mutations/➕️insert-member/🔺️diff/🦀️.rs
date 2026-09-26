use super::InsertMember;
use crate::diff::En1993MemberList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.members.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.member.clone());
    protocol::MutationOutcome::new(En1993Diff { members: Some(En1993MemberList { values }), ..Default::default() })
}
