use crate::diff::{En1992Diff, En1992MemberList};
use super::InsertMember;
use crate::En1992Snapshot;

pub fn diff(payload: &InsertMember, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.members.iter().any(|m| m.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate", format!("Member id {} already exists.", payload.member.id), Vec::<String>::new());
    }
    let mut members = base.members.clone();
    let at = payload.index.min(members.len());
    members.insert(at, payload.member.clone());
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
