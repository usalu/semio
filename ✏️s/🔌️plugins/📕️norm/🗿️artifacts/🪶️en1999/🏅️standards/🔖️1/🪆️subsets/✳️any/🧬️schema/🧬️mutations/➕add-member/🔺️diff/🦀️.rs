//! 🔺️ `add-member` diff.

use crate::mutations::add_member::AddMember;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &AddMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    let ids: Vec<String> = base.members.iter().map(|existing| existing.id.clone()).collect();
    let at = (payload.index as usize).min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.member.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows { added: vec![payload.member.clone()], order, ..Default::default() }), ..Default::default() })
}
