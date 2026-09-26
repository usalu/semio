use super::InsertLayer;
use crate::mutations::{remove_layer::RemoveLayer, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &InsertLayer, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let at = payload.index.min(base.layers.len());
    vec![En1997Mutation::RemoveLayer(RemoveLayer { index: at })]
}
