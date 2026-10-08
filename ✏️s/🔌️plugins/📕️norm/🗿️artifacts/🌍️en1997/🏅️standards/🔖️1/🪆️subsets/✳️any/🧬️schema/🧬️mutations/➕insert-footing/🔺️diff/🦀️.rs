use super::InsertFooting;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997FootingsRows};

pub fn diff(payload: &InsertFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.footings.iter().any(|existing| existing.id == payload.footing.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Footing id {} already exists.", payload.footing.id), [payload.footing.id.clone()]);
    }
    let ids: Vec<String> = base.footings.iter().map(|existing| existing.id.clone()).collect();
    let at = payload.index.min(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.footing.id.clone());
        order
    });
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows { added: vec![payload.footing.clone()], order, ..Default::default() }), ..Default::default() })
}
