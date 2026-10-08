//! ⬜️ `update-hss-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateHssInputs;
use crate::diff::{En1993Diff, En1993LoadCaseDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateHssInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.load_cases.iter().position(|row| row.id == payload.load_case.id) {
        Some(index) if base.load_cases[index] == payload.load_case => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993LoadCaseDelta::removal(&base.load_cases, index);
            replacement.absorb(En1993LoadCaseDelta::insertion(index, payload.load_case.clone()));
            replacement
        }
        None => En1993LoadCaseDelta::insertion(base.load_cases.len(), payload.load_case.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { load_cases: delta, ..Default::default() })
}
