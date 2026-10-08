use crate::diff::{En1992Diff, En1992MembersRows};
use super::InsertMember;
use crate::En1992Snapshot;

pub fn diff(payload: &InsertMember, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.members.iter().any(|m| m.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), Vec::<String>::new());
    }
    if payload.index > base.members.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.members.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::insertion(payload.index, payload.member.clone())), ..Default::default() })
}
