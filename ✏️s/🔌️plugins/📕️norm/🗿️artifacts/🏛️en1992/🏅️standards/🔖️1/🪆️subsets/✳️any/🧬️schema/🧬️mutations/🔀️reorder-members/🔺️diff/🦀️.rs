use crate::diff::{En1992Diff, En1992MemberList};
use super::ReorderMembers;
use crate::En1992Snapshot;

pub fn diff(payload: &ReorderMembers, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    if payload.from_index >= members.len() {
        return protocol::MutationOutcome::fatal("mutation.missing", "from_index out of range", Vec::<String>::new());
    }
    let item = members.remove(payload.from_index);
    let to = payload.to_index.min(members.len());
    members.insert(to, item);
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
