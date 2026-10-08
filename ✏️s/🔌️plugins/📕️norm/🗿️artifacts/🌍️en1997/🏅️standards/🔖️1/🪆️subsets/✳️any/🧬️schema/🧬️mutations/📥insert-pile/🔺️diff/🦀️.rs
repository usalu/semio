use super::InsertPile;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997PilesRows};

pub fn diff(payload: &InsertPile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    if payload.index > base.piles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.piles.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::insertion(payload.index, payload.pile.clone())), ..Default::default() })
}
