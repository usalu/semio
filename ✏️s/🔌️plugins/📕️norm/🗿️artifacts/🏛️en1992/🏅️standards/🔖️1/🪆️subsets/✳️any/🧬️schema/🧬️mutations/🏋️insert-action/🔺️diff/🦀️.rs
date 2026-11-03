//! 🏋️ `insert-action` diff: inserts the action row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_action::InsertAction;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersActionsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertAction, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.actions.iter().any(|existing| existing.id == payload.action.id) {
        let key = payload.action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Action {key} already exists."), [key]);
    }
    if payload.index > r0.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { actions: Some(En1992MembersActionsRows::insertion(payload.index, payload.action.clone())), ..Default::default() })), ..Default::default() })
}
