//! 🪵️ `update-pile-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdatePileInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993PileEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdatePileInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.piles.iter().position(|row| row.id == payload.pile.id) {
        Some(index) if base.piles[index] == payload.pile => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993PileEdit::replace(index, payload.pile.id.clone(), payload.pile.clone()),
        None => En1993PileEdit::insert(base.piles.len(), payload.pile.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { piles: vec![edit], ..Default::default() })
}
