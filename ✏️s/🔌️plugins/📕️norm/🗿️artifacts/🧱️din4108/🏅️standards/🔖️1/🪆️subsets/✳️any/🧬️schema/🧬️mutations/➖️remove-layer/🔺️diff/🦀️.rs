//! ➖️ `remove-layer` diff — removes the layer at the index from the element's stack.

use super::RemoveLayer;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    if payload.index >= element.layers.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer index out of range", Vec::<String>::new());
    }
    let nested = Din4108LayerDelta::removal(&element.layers, payload.index);
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
