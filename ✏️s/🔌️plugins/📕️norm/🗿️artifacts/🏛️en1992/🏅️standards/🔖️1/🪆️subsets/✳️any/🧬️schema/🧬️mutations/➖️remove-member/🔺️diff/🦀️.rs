use crate::diff::{En1992Diff, En1992MembersRows};
use super::RemoveMember;
use crate::En1992Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(index) = base.members.iter().position(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::removal(&base.members, index)), ..Default::default() })
}
