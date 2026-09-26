use super::RemovePile;
use crate::mutations::{insert_pile::InsertPile, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &RemovePile, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let pile = base.piles.get(payload.index).cloned().unwrap_or_else(|| base.piles[0].clone());
    vec![En1997Mutation::InsertPile(InsertPile { index: payload.index, pile })]
}
