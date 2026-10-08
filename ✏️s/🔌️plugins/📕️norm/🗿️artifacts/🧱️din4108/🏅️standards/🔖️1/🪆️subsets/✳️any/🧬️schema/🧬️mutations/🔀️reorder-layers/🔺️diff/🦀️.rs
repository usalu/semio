//! 🔀️ `reorder-layers` diff — moves the layer from `from` to `to`.

use super::ReorderLayers;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    if payload.from >= element.layers.len() || payload.to >= element.layers.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer reorder out of range", Vec::<String>::new());
    }
    let nested = Din4108LayerDelta::relocation(&element.layers, payload.from, payload.to);
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
