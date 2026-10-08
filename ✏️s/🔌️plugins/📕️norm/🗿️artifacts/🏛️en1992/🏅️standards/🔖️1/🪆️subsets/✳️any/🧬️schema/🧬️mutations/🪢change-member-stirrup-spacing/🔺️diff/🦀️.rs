use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use super::ChangeMemberStirrupSpacing;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberStirrupSpacing, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(s) = m.stirrups.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Member has no stirrups.", Vec::<String>::new());
    };
    if (s.spacing - payload.new_spacing).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stirrups_spacing: Some(payload.new_spacing), ..Default::default() })),
        ..Default::default()
    })
}
