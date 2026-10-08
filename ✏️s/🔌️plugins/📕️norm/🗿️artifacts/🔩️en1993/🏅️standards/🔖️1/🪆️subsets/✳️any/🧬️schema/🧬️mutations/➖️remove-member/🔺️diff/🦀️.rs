//! ➖️ `remove-member` diff — removes the row at the index.

use super::RemoveMember;
use crate::diff::{En1993Diff, En1993MemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::removal(&base.members, payload.index), ..Default::default() })
}
