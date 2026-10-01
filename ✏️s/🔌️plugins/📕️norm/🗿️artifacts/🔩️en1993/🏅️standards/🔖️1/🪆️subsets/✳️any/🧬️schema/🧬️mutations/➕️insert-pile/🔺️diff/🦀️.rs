use super::InsertPile;
use crate::diff::En1993PileList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertPile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let mut values = base.piles.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.pile.clone());
    protocol::MutationOutcome::new(En1993Diff { piles: Some(En1993PileList { values }), ..Default::default() })
}
