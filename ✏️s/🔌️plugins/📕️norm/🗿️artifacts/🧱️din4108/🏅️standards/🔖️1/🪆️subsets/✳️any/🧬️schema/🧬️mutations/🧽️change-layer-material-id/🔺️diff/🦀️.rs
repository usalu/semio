//! 🧽️ `change-layer-material-id` diff — patches the layer's `material_id` inside its element; a missing element or layer is a `mutation.invariant`.

use super::ChangeLayerMaterialId;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch, Din4108LayerDelta, Din4108LayerPatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeLayerMaterialId, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let Some(layer) = element.layers.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "layer index out of range", Vec::<String>::new());
    };
    let nested = Din4108LayerDelta::modification(&layer.id, Din4108LayerPatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() });
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&element.id, Din4108ElementPatch { layers: nested, ..Default::default() }), ..Default::default() })
}
