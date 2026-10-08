use crate::diff::{En1992Diff, En1992MembersRows};
use super::InsertMember;
use crate::En1992Snapshot;

pub fn diff(payload: &InsertMember, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.members.iter().any(|m| m.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), Vec::<String>::new());
    }
    let ids: Vec<String> = base.members.iter().map(|existing| existing.id.clone()).collect();
    let at = payload.index.min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.member.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows { added: vec![payload.member.clone()], order, ..Default::default() }), ..Default::default() })
}
