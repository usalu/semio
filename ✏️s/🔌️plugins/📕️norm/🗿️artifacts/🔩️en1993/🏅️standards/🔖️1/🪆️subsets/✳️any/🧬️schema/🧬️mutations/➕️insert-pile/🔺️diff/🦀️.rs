//! ➕️ `insert-pile` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertPile;
use crate::diff::{En1993Diff, En1993PileDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertPile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let index = payload.index.min(base.piles.len());
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::insertion(&base.piles, index, payload.pile.clone()), ..Default::default() })
}
