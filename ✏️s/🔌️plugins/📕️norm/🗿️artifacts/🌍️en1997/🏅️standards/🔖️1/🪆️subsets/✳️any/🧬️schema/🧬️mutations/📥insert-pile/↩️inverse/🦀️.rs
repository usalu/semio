use super::InsertPile;
use crate::mutations::{remove_pile::RemovePile, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &InsertPile, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let at = payload.index.min(base.piles.len());
    vec![En1997Mutation::RemovePile(RemovePile { index: at })]
}
