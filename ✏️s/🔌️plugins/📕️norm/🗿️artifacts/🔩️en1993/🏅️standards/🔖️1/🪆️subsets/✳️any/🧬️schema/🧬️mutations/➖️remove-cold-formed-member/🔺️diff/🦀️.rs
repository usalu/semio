//! ➖️ `remove-cold-formed-member` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveColdFormedMember;
use crate::diff::{En1993Diff, En1993ColdFormedMemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.cold_formed_members.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("cold-formed-member index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::removal(&row.id), ..Default::default() })
}
