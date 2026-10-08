use super::RemoveMemberAction;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995MemberActionDelta, En1995MemberDelta, En1995MemberPatch};
pub fn diff(payload: &RemoveMemberAction, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.members.iter().position(|item| item.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member id.", vec![payload.member_id.clone()]);
    };
    if payload.index >= base.members[idx].actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member action index {} out of range.", payload.index), vec![payload.member_id.clone()]);
    }
    let member = &base.members[idx];
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::modification(&member.id, En1995MemberPatch { actions: En1995MemberActionDelta::removal(&member.actions, payload.index), ..Default::default() }), ..Default::default() })
}
