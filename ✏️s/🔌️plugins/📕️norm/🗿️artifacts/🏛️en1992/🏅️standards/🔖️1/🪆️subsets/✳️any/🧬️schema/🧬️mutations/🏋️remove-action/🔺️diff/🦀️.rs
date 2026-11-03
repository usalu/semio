//! 🏋️ `remove-action` diff: removes the addressed action row.

use crate::remove_action::RemoveAction;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersActionsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveAction, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "actions index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { actions: Some(En1992MembersActionsRows::removal(&r0.actions, index)), ..Default::default() })), ..Default::default() })
}
