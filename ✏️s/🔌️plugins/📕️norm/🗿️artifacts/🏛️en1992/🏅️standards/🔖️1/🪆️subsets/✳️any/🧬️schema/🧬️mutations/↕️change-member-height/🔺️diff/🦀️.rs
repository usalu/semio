use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use super::ChangeMemberHeight;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberHeight, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    if (m.height - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { height: Some(payload.new_value), ..Default::default() })),
        ..Default::default()
    })
}
