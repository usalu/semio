//! ➕️ `insert-layer` diff — inserts the layer into the element's stack at its position, clamped to the end of the stack.

use super::InsertLayer;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ElementEdit, Din4108ElementPatch, Din4108LayerEdit};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, element)) = base.elements.iter().enumerate().find(|(_, element)| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let index = payload.index.min(element.layers.len());
    let nested = vec![Din4108LayerEdit::insert(index, payload.layer.clone())];
    protocol::MutationOutcome::new(Din4108Diff { elements: vec![Din4108ElementEdit::patch(slot, element.id.clone(), Din4108ElementPatch { layers: nested, ..Default::default() })], ..Default::default() })
}
