//! 🔺️ `change-action-gk-line` diff: patches `g_k_line` of the addressed member action row.

use crate::change_action_g_k_line::ChangeActionGKLine;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersActionsPatch, En1999MembersActionsRows, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeActionGKLine, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_g_k_line.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.g_k_line == payload.new_g_k_line {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { actions: Some(En1999MembersActionsRows::modification(&payload.action_id, En1999MembersActionsPatch { g_k_line: Some(payload.new_g_k_line), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
