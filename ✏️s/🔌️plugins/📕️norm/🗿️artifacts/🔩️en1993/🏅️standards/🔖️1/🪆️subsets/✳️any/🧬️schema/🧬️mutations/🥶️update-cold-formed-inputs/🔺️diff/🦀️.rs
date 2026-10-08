//! 🥶️ `update-cold-formed-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateColdFormedInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993ColdFormedMemberEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.cold_formed_members.iter().position(|row| row.id == payload.cold_formed_member.id) {
        Some(index) if base.cold_formed_members[index] == payload.cold_formed_member => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993ColdFormedMemberEdit::replace(index, payload.cold_formed_member.id.clone(), payload.cold_formed_member.clone()),
        None => En1993ColdFormedMemberEdit::insert(base.cold_formed_members.len(), payload.cold_formed_member.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: vec![edit], ..Default::default() })
}
