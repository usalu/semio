//! 🏷️ `change-layer-compressive-class` diff — patches the layer's `compressive_class` inside its element; a missing element or layer is a `mutation.invariant`.

use super::ChangeLayerCompressiveClass;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta, Din4108LayerPatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeLayerCompressiveClass, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let Some(layer) = element.layers.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer index out of range", Vec::<String>::new());
    };
    let nested = Din4108LayerDelta::modification(&layer.id, Din4108LayerPatch { compressive_class: Some(payload.new_compressive_class.clone()), ..Default::default() });
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
