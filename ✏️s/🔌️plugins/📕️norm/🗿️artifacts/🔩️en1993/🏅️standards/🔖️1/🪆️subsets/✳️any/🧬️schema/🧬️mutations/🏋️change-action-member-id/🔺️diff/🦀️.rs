//! 🔺️ `change-action-member-id` diff: patches `member_id` of the addressed member action row.

use crate::change_action_member_id::ChangeActionMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberActionDelta, En1993MemberActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeActionMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.member_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member-action", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::modification(payload.id.clone(), En1993MemberActionPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
