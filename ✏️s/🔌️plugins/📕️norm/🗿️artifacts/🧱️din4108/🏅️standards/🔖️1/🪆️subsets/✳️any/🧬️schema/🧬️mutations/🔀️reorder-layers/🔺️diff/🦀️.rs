//! 🔀️ `reorder-layers` diff — moves the layer by removing it from `from` and inserting it at `to`.

use super::ReorderLayers;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ElementEdit, Din4108ElementPatch, Din4108LayerEdit};
use crate::Din4108Snapshot;

pub fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, element)) = base.elements.iter().enumerate().find(|(_, element)| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let (Some(layer), true) = (element.layers.get(payload.from), payload.to < element.layers.len()) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer reorder out of range", Vec::<String>::new());
    };
    let nested = vec![Din4108LayerEdit::remove(payload.from, layer.id.clone()), Din4108LayerEdit::insert(payload.to, layer.clone())];
    protocol::MutationOutcome::new(Din4108Diff { elements: vec![Din4108ElementEdit::patch(slot, element.id.clone(), Din4108ElementPatch { layers: nested, ..Default::default() })], ..Default::default() })
}
