use super::InsertLayer;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997LayersRows};

pub fn diff(payload: &InsertLayer, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.layers.iter().any(|existing| existing.id == payload.layer.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Layer id {} already exists.", payload.layer.id), [payload.layer.id.clone()]);
    }
    let ids: Vec<String> = base.layers.iter().map(|existing| existing.id.clone()).collect();
    let at = payload.index.min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.layer.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997LayersRows { added: vec![payload.layer.clone()], order, ..Default::default() }), ..Default::default() })
}
