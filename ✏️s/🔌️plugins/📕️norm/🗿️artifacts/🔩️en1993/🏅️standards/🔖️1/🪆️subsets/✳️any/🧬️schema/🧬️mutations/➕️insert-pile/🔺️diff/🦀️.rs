//! ➕️ `insert-pile` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertPile;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993PileEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertPile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let index = payload.index.min(base.piles.len());
    protocol::MutationOutcome::new(En1993Diff { piles: vec![En1993PileEdit::insert(index, payload.pile.clone())], ..Default::default() })
}
