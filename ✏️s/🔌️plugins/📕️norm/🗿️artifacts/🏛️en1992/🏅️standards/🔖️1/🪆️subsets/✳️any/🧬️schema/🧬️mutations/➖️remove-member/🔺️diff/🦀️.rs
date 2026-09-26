use crate::diff::{En1992Diff, En1992MemberList};
use super::RemoveMember;
use crate::En1992Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !base.members.iter().any(|m| m.id == payload.member_id) {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    }
    let members: Vec<_> = base.members.iter().filter(|m| m.id != payload.member_id).cloned().collect();
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
