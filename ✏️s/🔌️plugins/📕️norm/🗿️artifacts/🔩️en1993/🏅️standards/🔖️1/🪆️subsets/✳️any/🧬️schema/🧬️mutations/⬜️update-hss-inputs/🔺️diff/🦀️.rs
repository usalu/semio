//! ⬜️ `update-hss-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateHssInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993LoadCaseEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateHssInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.load_cases.iter().position(|row| row.id == payload.load_case.id) {
        Some(index) if base.load_cases[index] == payload.load_case => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993LoadCaseEdit::replace(index, payload.load_case.id.clone(), payload.load_case.clone()),
        None => En1993LoadCaseEdit::insert(base.load_cases.len(), payload.load_case.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { load_cases: vec![edit], ..Default::default() })
}
