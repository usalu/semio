//! 🔺️ `remove-member-action` diff.

use crate::remove_member_action::RemoveMemberAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersPatch, En1999MembersRows, En1999MembersActionsRows};

pub fn diff(payload: &RemoveMemberAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown member id {}", payload.member_id), vec![payload.member_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member action id {}", payload.id), Vec::<String>::new());
    };
    let nested = En1999MembersActionsRows::removal(&parent.actions, index);
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(payload.member_id.clone(), En1999MembersPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
