//! 🔺️ `change-member-my-ed` diff.

use crate::mutations::change_member_m_y_ed::ChangeMemberMYEd;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MembersRows, En1999MembersPatch, En1999MembersActionsRows, En1999MembersActionsPatch};

pub fn diff(payload: &ChangeMemberMYEd, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m_y_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M_y,Ed must be finite.", Vec::<String>::new());
    }
    let Some(member) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member {}", payload.member_id), Vec::<String>::new());
    };
    let Some(action) = member.actions.iter().find(|a| a.id == payload.action_id).or_else(|| member.actions.first()) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Member has no actions.", Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff {
        members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch {
            actions: Some(En1999MembersActionsRows::modification(&action.id, En1999MembersActionsPatch { m_y_k: Some(payload.new_m_y_k), ..Default::default() })),
            ..Default::default()
        })),
        ..Default::default()
    })
}
