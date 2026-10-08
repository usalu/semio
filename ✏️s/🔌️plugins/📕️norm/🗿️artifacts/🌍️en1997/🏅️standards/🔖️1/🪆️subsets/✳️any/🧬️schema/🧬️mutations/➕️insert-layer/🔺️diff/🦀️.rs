use super::InsertLayer;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997LayersRows};

pub fn diff(payload: &InsertLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.layers.iter().any(|existing| existing.id == payload.layer.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Layer id {} already exists.", payload.layer.id), [payload.layer.id.clone()]);
    }
    if payload.index > base.layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.layers.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997LayersRows::insertion(payload.index, payload.layer.clone())), ..Default::default() })
}
