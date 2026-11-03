//! 🔺️ `change-action-my` diff: patches `action.my` of the addressed member action row.

use crate::change_action_my::ChangeActionMy;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberActionDelta, En1993MemberActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeActionMy, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_action_my.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.member_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member-action", Vec::<String>::new());
    };
    if row.action.my == payload.new_action_my {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    let mut value = row.action.clone();
    value.my = payload.new_action_my;
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::modification(payload.id.clone(), En1993MemberActionPatch { action: Some(value), ..Default::default() }), ..Default::default() })
}
