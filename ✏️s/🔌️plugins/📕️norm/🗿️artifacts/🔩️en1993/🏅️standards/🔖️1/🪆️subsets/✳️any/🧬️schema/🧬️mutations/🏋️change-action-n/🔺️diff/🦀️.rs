//! 🔺️ `change-action-n` diff: patches `action.n` of the addressed member action row.

use crate::change_action_n::ChangeActionN;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberActionDelta, En1993MemberActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeActionN, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_action_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.member_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member-action", Vec::<String>::new());
    };
    if row.action.n == payload.new_action_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    let mut value = row.action.clone();
    value.n = payload.new_action_n;
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::modification(payload.id.clone(), En1993MemberActionPatch { action: Some(value), ..Default::default() }), ..Default::default() })
}
