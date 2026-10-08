use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use super::ChangeMemberExposure;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberExposure, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    if m.exposure == payload.new_exposure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows { modified: vec![En1992MembersPatch { id: payload.member_id.clone(), exposure: Some(payload.new_exposure), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
