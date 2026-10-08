//! ➕️ `insert-member` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertMember;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MemberEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    let index = payload.index.min(base.members.len());
    protocol::MutationOutcome::new(En1993Diff { members: vec![En1993MemberEdit::insert(index, payload.member.clone())], ..Default::default() })
}
