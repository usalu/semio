//! 🔩 `insert-member` diff — inserts the row at its position; a position past the collection's end inserts it last as a
//! `mutation.clamped` warning and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertMember;
use crate::diff::{En1990Diff, En1990MemberDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &InsertMember, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.members.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return MutationOutcome::fatal("mutation.duplicate-id", format!("The member '{key}' already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.members.len());
    let outcome = MutationOutcome::new(En1990Diff { members: En1990MemberDelta::insertion(index, payload.item.clone()), ..En1990Diff::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the member list; inserted at {index}.", payload.index.unwrap_or(index)))
}
