use crate::diff::{En1992Diff, En1992MemberList};
use crate::mutations::change_member_width::ChangeMemberWidth;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeMemberWidth, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    let Some(m) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    if (m.width - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    m.width = payload.new_value;
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
