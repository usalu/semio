use super::InsertPile;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997PilesRows};

pub fn diff(payload: &InsertPile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.piles.iter().any(|existing| existing.id == payload.pile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile id {} already exists.", payload.pile.id), [payload.pile.id.clone()]);
    }
    let ids: Vec<String> = base.piles.iter().map(|existing| existing.id.clone()).collect();
    let at = payload.index.min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.pile.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows { added: vec![payload.pile.clone()], order, ..Default::default() }), ..Default::default() })
}
