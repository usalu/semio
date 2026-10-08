use super::RemoveLayer;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997LayersRows};

pub fn diff(payload: &RemoveLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997LayersRows::removal(&base.layers, payload.index)), ..Default::default() })
}
