use super::InsertFooting;
use crate::mutations::{remove_footing::RemoveFooting, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &InsertFooting, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let at = payload.index.min(base.footings.len());
    vec![En1997Mutation::RemoveFooting(RemoveFooting { index: at })]
}
