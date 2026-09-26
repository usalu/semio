use super::InsertMember;
use crate::diff::En1995MemberList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &InsertMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let mut members = base.members.clone();
    let at = payload.index.min(members.len());
    members.insert(at, payload.member.clone());
    protocol::MutationOutcome::new(En1995Diff { members: Some(En1995MemberList { values: members }), ..Default::default() })
}
