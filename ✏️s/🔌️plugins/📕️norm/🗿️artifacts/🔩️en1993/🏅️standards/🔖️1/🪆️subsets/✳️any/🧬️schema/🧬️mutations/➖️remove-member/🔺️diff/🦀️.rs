//! ➖️ `remove-member` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveMember;
use crate::diff::{En1993Diff, En1993MemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.members.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::removal(&row.id), ..Default::default() })
}
