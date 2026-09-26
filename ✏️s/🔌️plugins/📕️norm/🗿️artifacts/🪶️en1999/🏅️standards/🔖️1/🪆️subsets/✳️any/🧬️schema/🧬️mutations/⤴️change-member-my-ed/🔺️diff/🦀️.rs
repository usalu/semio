//! 🔺️ `change-member-my-ed` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_member_m_y_ed::ChangeMemberMYEd;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeMemberMYEd, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m_y_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M_y,Ed must be finite.", Vec::<String>::new());
    }
    let mut members = base.members.clone();
    let Some(member) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member {}", payload.member_id), Vec::<String>::new());
    };
    let idx = member
        .actions
        .iter()
        .position(|a| a.id == payload.action_id)
        .or_else(|| if member.actions.is_empty() { None } else { Some(0) });
    let Some(idx) = idx else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Member has no actions.", Vec::<String>::new());
    };
    member.actions[idx].m_y_k = payload.new_m_y_k;
    protocol::MutationOutcome::new(En1999Diff { members: Some(members), ..Default::default() })
}
