use super::RemoveMember;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995MemberDelta};
pub fn diff(payload: &RemoveMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::removal(&base.members[payload.index].id), ..Default::default() })
}
