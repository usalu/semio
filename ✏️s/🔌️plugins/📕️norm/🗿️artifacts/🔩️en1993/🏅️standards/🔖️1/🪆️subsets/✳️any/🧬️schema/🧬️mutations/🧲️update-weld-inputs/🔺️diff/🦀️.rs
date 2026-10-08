//! 🧲️ `update-weld-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateWeldInputs;
use crate::diff::{En1993Diff, En1993MemberActionDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.member_actions.iter().position(|row| row.id == payload.member_action.id) {
        Some(index) if base.member_actions[index] == payload.member_action => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993MemberActionDelta::removal(&payload.member_action.id);
            replacement.absorb(En1993MemberActionDelta::insertion(&base.member_actions, index, payload.member_action.clone()));
            replacement
        }
        None => En1993MemberActionDelta::insertion(&base.member_actions, base.member_actions.len(), payload.member_action.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { member_actions: delta, ..Default::default() })
}
