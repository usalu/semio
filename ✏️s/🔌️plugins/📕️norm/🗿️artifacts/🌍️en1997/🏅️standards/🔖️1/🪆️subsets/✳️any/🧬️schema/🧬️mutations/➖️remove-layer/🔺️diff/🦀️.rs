use super::RemoveLayer;
use crate::diff::{En1997Diff, En1997SoilLayerList};
use crate::En1997Snapshot;
pub fn diff(payload: &RemoveLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    let mut layers = base.layers.clone();
    layers.remove(payload.index);
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997SoilLayerList { values: layers }), ..Default::default() })
}
