//! 🥶️ `update-cold-formed-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateColdFormedInputs;
use crate::diff::{En1993Diff, En1993ColdFormedMemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.cold_formed_members.iter().position(|row| row.id == payload.cold_formed_member.id) {
        Some(index) if base.cold_formed_members[index] == payload.cold_formed_member => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993ColdFormedMemberDelta::removal(&payload.cold_formed_member.id);
            replacement.absorb(En1993ColdFormedMemberDelta::insertion(&base.cold_formed_members, index, payload.cold_formed_member.clone()));
            replacement
        }
        None => En1993ColdFormedMemberDelta::insertion(&base.cold_formed_members, base.cold_formed_members.len(), payload.cold_formed_member.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: delta, ..Default::default() })
}
