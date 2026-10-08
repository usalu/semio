//! ➕️ `insert-cold-formed-member` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertColdFormedMember;
use crate::diff::{En1993Diff, En1993ColdFormedMemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.cold_formed_members.iter().any(|existing| existing.id == payload.cold_formed_member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cold formed member id {} already exists.", payload.cold_formed_member.id), [payload.cold_formed_member.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.cold_formed_members.len());
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::insertion(index, payload.cold_formed_member.clone()), ..Default::default() })
}
