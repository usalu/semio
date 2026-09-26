use super::InsertLayer;
use crate::diff::{En1997Diff, En1997SoilLayerList};
use crate::En1997Snapshot;
pub fn diff(payload: &InsertLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let mut layers = base.layers.clone();
    let at = payload.index.min(layers.len());
    layers.insert(at, payload.layer.clone());
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997SoilLayerList { values: layers }), ..Default::default() })
}
