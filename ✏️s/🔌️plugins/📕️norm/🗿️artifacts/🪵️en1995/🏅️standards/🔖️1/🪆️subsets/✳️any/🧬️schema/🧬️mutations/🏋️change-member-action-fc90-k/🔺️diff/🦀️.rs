use super::ChangeMemberActionFC90K;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995MemberActionDelta, En1995MemberActionPatch, En1995MemberDelta, En1995MemberPatch};
pub fn diff(payload: &ChangeMemberActionFC90K, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.members.iter().position(|item| item.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member id.", vec![payload.member_id.clone()]);
    };
    let Some(action_idx) = base.members[idx].actions.iter().position(|action| action.id == payload.action_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member action id.", vec![payload.member_id.clone(), payload.action_id.clone()]);
    };
    let member = &base.members[idx];
    let action = &member.actions[action_idx];
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::modification(&member.id, En1995MemberPatch { actions: En1995MemberActionDelta::modification(&action.id, En1995MemberActionPatch { f_c90_k_n: Some(payload.new_value), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
