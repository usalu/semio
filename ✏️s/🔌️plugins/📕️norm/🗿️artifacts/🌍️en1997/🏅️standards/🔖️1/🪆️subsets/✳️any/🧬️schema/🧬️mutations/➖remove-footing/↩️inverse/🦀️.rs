use super::RemoveFooting;
use crate::mutations::{insert_footing::InsertFooting, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &RemoveFooting, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let footing = base.footings.get(payload.index).cloned().unwrap_or_else(|| base.footings[0].clone());
    vec![En1997Mutation::InsertFooting(InsertFooting { index: payload.index, footing })]
}
