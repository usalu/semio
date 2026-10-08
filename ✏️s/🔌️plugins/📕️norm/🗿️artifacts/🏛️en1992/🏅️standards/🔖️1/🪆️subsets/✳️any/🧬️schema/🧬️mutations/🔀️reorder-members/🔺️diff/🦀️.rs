use crate::diff::{En1992Diff, En1992MembersRows};
use super::ReorderMembers;
use crate::En1992Snapshot;

pub fn diff(payload: &ReorderMembers, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if payload.from_index >= base.members.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "from_index out of range", Vec::<String>::new());
    }
    let to = payload.to_index.min(base.members.len() - 1);
    if to == payload.from_index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Member order unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::relocation(&base.members, payload.from_index, to)), ..Default::default() })
}
