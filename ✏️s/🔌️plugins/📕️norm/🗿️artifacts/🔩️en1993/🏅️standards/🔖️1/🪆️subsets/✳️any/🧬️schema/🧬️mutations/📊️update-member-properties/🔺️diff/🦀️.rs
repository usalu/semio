//! 📊️ `update-member-properties` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateMemberProperties;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MemberEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.members.iter().position(|row| row.id == payload.member.id) {
        Some(index) if base.members[index] == payload.member => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993MemberEdit::replace(index, payload.member.id.clone(), payload.member.clone()),
        None => En1993MemberEdit::insert(base.members.len(), payload.member.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { members: vec![edit], ..Default::default() })
}
