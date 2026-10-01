//! Diff for `insert-silo`.
use super::InsertSilo;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.silos.iter().any(|existing| existing.id == payload.silo.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Silo id {} already exists.", payload.silo.id), [payload.silo.id.clone()]);
    }
    let mut items = base.silos.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.silo.clone());
    protocol::MutationOutcome::new(En1998Diff { silos: Some(items), ..Default::default() })
}
