use crate::diff::{En1992Diff, En1992AnchorsRows};
use super::InsertAnchor;
use crate::En1992Snapshot;

pub fn diff(payload: &InsertAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.anchors.iter().any(|existing| existing.id == payload.anchor.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Anchor id {} already exists.", payload.anchor.id), [payload.anchor.id.clone()]);
    }
    let ids: Vec<String> = base.anchors.iter().map(|existing| existing.id.clone()).collect();
    let at = payload.index.min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.anchor.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows { added: vec![payload.anchor.clone()], order, ..Default::default() }), ..Default::default() })
}
