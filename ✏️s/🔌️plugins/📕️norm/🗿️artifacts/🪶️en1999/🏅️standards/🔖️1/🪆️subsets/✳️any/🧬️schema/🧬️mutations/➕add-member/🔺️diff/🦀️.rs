//! 🔺️ `add-member` diff.

use crate::diff::En1999Diff;
use crate::mutations::add_member::AddMember;
use crate::En1999Snapshot;

pub fn diff(payload: &AddMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    let mut members = base.members.clone();
    let idx = (payload.index as usize).min(members.len());
    members.insert(idx, payload.member.clone());
    protocol::MutationOutcome::new(En1999Diff { members: Some(members), ..Default::default() })
}
