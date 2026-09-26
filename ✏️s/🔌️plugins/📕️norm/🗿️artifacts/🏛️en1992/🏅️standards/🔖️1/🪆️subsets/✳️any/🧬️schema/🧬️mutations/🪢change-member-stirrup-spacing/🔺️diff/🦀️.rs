use crate::diff::{En1992Diff, En1992MemberList};
use super::ChangeMemberStirrupSpacing;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberStirrupSpacing, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    let Some(m) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(s) = m.stirrups.as_mut() else {
        return protocol::MutationOutcome::fatal("mutation.missing", "Member has no stirrups.", Vec::<String>::new());
    };
    if (s.spacing - payload.new_spacing).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    s.spacing = payload.new_spacing;
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
