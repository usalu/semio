use crate::diff::{En1992Diff, En1992MembersRows};
use super::ReorderMembers;
use crate::En1992Snapshot;

pub fn diff(payload: &ReorderMembers, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if payload.from_index >= base.members.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "from_index out of range", Vec::<String>::new());
    }
    let ids: Vec<String> = base.members.iter().map(|member| member.id.clone()).collect();
    let mut order = ids.clone();
    let item = order.remove(payload.from_index);
    let to = payload.to_index.min(order.len());
    order.insert(to, item);
    if order == ids {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Member order unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows { order: Some(order), ..Default::default() }), ..Default::default() })
}
