//! 🔺️ `insert-member-action` diff.

use crate::insert_member_action::InsertMemberAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersPatch, En1999MembersRows, En1999MembersActionsRows};

pub fn diff(payload: &InsertMemberAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown member id {}", payload.member_id), vec![payload.member_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member action id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    if (payload.index as usize) > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    let nested = En1999MembersActionsRows::insertion(payload.index as usize, payload.action.clone());
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(payload.member_id.clone(), En1999MembersPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
