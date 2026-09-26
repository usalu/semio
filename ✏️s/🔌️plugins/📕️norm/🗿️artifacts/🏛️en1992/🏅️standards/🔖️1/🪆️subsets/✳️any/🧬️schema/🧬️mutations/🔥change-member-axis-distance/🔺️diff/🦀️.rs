use crate::diff::{En1992Diff, En1992MemberList};
use super::ChangeMemberAxisDistance;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberAxisDistance, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    let Some(m) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(f) = m.fire.as_mut() else {
        return protocol::MutationOutcome::fatal("mutation.missing", "Member has no fire spec.", Vec::<String>::new());
    };
    if (f.axis_distance - payload.new_axis_distance).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    f.axis_distance = payload.new_axis_distance;
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
