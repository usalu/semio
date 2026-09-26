use super::RemoveLayer;
use crate::mutations::{insert_layer::InsertLayer, En1997Mutation};
use crate::En1997Snapshot;
pub fn inverse(payload: &RemoveLayer, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let layer = base.layers.get(payload.index).cloned().unwrap_or_else(|| base.layers[0].clone());
    vec![En1997Mutation::InsertLayer(InsertLayer { index: payload.index, layer })]
}
