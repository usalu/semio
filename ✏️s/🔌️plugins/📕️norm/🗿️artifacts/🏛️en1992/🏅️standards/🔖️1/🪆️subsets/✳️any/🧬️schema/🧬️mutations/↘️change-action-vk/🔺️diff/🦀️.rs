use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch, En1992MembersActionsRows, En1992MembersActionsPatch};
use super::ChangeActionVk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeActionVk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(a) = m.actions.iter().find(|a| a.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Action {} not found.", payload.action_id), Vec::<String>::new());
    };
    if (a.v_k - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch {
            actions: Some(En1992MembersActionsRows::modification(&payload.action_id, En1992MembersActionsPatch { v_k: Some(payload.new_value), ..Default::default() })),
            ..Default::default()
        })),
        ..Default::default()
    })
}
