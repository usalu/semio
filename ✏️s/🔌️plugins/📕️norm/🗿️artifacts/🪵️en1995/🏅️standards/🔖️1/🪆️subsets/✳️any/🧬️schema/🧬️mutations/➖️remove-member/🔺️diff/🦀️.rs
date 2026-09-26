use super::RemoveMember;
use crate::diff::En1995MemberList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &RemoveMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut members = base.members.clone();
    members.remove(payload.index);
    protocol::MutationOutcome::new(En1995Diff { members: Some(En1995MemberList { values: members }), ..Default::default() })
}
