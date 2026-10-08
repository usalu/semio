//! 📊️ `update-member-properties` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateMemberProperties;
use crate::diff::{En1993Diff, En1993MemberDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.members.iter().position(|row| row.id == payload.member.id) {
        Some(index) if base.members[index] == payload.member => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993MemberDelta::removal(&payload.member.id);
            replacement.absorb(En1993MemberDelta::insertion(&base.members, index, payload.member.clone()));
            replacement
        }
        None => En1993MemberDelta::insertion(&base.members, base.members.len(), payload.member.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { members: delta, ..Default::default() })
}
