use super::InsertMemberAction;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995MemberActionDelta, En1995MemberDelta, En1995MemberPatch};
pub fn diff(payload: &InsertMemberAction, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.members.iter().position(|item| item.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member id.", vec![payload.member_id.clone()]);
    };
    let member = &base.members[idx];
    let at = payload.index.min(base.members[idx].actions.len());
    if member.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::modification(&member.id, En1995MemberPatch { actions: En1995MemberActionDelta::insertion(&member.actions, at, payload.action.clone()), ..Default::default() }), ..Default::default() })
}
