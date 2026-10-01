use super::InsertPile;
use crate::diff::{En1997Diff, En1997PileList};
use crate::En1997Snapshot;
pub fn diff(payload: &InsertPile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let mut piles = base.piles.clone();
    let at = payload.index.min(piles.len());
    piles.insert(at, payload.pile.clone());
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PileList { values: piles }), ..Default::default() })
}
