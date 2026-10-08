//! ➖️ `remove-layer` diff — removes the layer at the index from the element's stack, guarded by the layer's own id.

use super::RemoveLayer;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ElementEdit, Din4108ElementPatch, Din4108LayerEdit};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, element)) = base.elements.iter().enumerate().find(|(_, element)| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let Some(layer) = element.layers.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer index out of range", Vec::<String>::new());
    };
    let nested = vec![Din4108LayerEdit::remove(payload.index, layer.id.clone())];
    protocol::MutationOutcome::new(Din4108Diff { elements: vec![Din4108ElementEdit::patch(slot, element.id.clone(), Din4108ElementPatch { layers: nested, ..Default::default() })], ..Default::default() })
}
