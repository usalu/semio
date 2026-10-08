//! 🧲️ `update-weld-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateWeldInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MemberActionEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.member_actions.iter().position(|row| row.id == payload.member_action.id) {
        Some(index) if base.member_actions[index] == payload.member_action => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993MemberActionEdit::replace(index, payload.member_action.id.clone(), payload.member_action.clone()),
        None => En1993MemberActionEdit::insert(base.member_actions.len(), payload.member_action.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { member_actions: vec![edit], ..Default::default() })
}
