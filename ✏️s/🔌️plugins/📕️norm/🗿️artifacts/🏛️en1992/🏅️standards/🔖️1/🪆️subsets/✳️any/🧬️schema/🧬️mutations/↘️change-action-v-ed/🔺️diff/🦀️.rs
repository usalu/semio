use crate::diff::{En1992Diff, En1992MemberList};
use crate::mutations::change_action_v_ed::ChangeActionVEd;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeActionVEd, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    let Some(m) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(a) = m.actions.iter_mut().find(|a| a.id == payload.action_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Action {} not found.", payload.action_id), Vec::<String>::new());
    };
    if (a.v_k - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    a.v_k = payload.new_value;
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
