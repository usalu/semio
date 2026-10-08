//! ➖️ `remove-cold-formed-member` diff — removes the row at the index.

use super::RemoveColdFormedMember;
use crate::diff::{En1993Diff, En1993ColdFormedMemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.cold_formed_members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("cold-formed-member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::removal(&base.cold_formed_members, payload.index), ..Default::default() })
}
