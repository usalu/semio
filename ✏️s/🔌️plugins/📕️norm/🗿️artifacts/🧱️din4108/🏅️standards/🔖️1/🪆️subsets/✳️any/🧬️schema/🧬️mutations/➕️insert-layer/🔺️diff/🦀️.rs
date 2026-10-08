//! ➕️ `insert-layer` diff — inserts the layer into the element's stack at its position, clamped to the end of the stack.

use super::InsertLayer;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let index = payload.index.unwrap_or(usize::MAX).min(element.layers.len());
    let nested = Din4108LayerDelta::insertion(index, payload.layer.clone());
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
