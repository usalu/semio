//! 📊️ `update-member-properties` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use crate::update_member_properties::UpdateMemberProperties;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.members.iter().position(|row| row.id == payload.member.id) {
        Some(index) if base.members[index] == payload.member => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993MemberDelta::removal(&base.members, index);
            replacement.absorb(En1993MemberDelta::insertion(index, payload.member.clone()));
            replacement
        }
        None => En1993MemberDelta::insertion(base.members.len(), payload.member.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { members: delta, ..Default::default() })
}
