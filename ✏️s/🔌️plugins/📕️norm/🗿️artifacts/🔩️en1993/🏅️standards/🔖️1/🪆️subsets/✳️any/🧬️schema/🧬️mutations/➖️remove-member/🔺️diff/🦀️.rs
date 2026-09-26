use super::RemoveMember;
use crate::diff::En1993MemberList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.members.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { members: Some(En1993MemberList { values }), ..Default::default() })
}
