use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch, En1992MembersActionsRows, En1992MembersActionsPatch};
use super::ChangeActionMk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeActionMk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(a) = m.actions.iter().find(|a| a.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Action {} not found.", payload.action_id), Vec::<String>::new());
    };
    if (a.m_k - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows {
            modified: vec![En1992MembersPatch {
                id: payload.member_id.clone(),
                actions: Some(En1992MembersActionsRows { modified: vec![En1992MembersActionsPatch { id: payload.action_id.clone(), m_k: Some(payload.new_value), ..Default::default() }] }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
