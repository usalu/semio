//! ➕️ `insert-pile` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_pile::InsertPile;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertPile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.piles.len());
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::insertion(index, payload.pile.clone()), ..Default::default() })
}
