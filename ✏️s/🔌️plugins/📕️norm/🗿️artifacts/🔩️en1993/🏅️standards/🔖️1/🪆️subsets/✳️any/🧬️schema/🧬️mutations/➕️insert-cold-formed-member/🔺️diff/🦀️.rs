//! ➕️ `insert-cold-formed-member` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertColdFormedMember;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993ColdFormedMemberEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.cold_formed_members.iter().any(|existing| existing.id == payload.cold_formed_member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cold formed member id {} already exists.", payload.cold_formed_member.id), [payload.cold_formed_member.id.clone()]);
    }
    let index = payload.index.min(base.cold_formed_members.len());
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: vec![En1993ColdFormedMemberEdit::insert(index, payload.cold_formed_member.clone())], ..Default::default() })
}
