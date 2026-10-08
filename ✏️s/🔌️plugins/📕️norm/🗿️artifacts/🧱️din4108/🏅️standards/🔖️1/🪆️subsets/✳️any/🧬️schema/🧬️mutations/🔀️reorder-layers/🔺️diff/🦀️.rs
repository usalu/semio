//! 🔀️ `reorder-layers` diff — moves the layer: it leaves its place and re-enters after the layer that precedes position `to` among the others.

use super::ReorderLayers;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let (Some(layer), true) = (element.layers.get(payload.from), payload.to < element.layers.len()) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer reorder out of range", Vec::<String>::new());
    };
    let rest: Vec<_> = element.layers.iter().filter(|other| other.id != layer.id).cloned().collect();
    let mut nested = Din4108LayerDelta::removal(&layer.id);
    nested.absorb(Din4108LayerDelta::insertion(&rest, payload.to, layer.clone()));
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
