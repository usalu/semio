use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use super::ChangeMemberAxisDistance;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberAxisDistance, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(f) = m.fire.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Member has no fire spec.", Vec::<String>::new());
    };
    if (f.axis_distance - payload.new_axis_distance).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows { modified: vec![En1992MembersPatch { id: payload.member_id.clone(), fire_axis_distance: Some(payload.new_axis_distance), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
