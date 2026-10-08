//! 🔁️ `update-fatigue-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateFatigueInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993FatigueDetailEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.fatigue_details.iter().position(|row| row.id == payload.fatigue_detail.id) {
        Some(index) if base.fatigue_details[index] == payload.fatigue_detail => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993FatigueDetailEdit::replace(index, payload.fatigue_detail.id.clone(), payload.fatigue_detail.clone()),
        None => En1993FatigueDetailEdit::insert(base.fatigue_details.len(), payload.fatigue_detail.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: vec![edit], ..Default::default() })
}
