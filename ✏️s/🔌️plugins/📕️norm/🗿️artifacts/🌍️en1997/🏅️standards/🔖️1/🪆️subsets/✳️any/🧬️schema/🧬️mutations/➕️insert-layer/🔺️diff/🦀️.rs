use super::InsertLayer;
use crate::diff::{En1997Diff, En1997SoilLayerList};
use crate::En1997Snapshot;
pub fn diff(payload: &InsertLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.layers.iter().any(|existing| existing.id == payload.layer.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Layer id {} already exists.", payload.layer.id), [payload.layer.id.clone()]);
    }
    let mut layers = base.layers.clone();
    let at = payload.index.min(layers.len());
    layers.insert(at, payload.layer.clone());
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997SoilLayerList { values: layers }), ..Default::default() })
}
