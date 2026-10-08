//! 🫙 `insert-silo` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertSilo;
use crate::diff::{En1998Diff, En1998SiloDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.silos.iter().any(|existing| existing.id == payload.silo.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Silo id {} already exists.", payload.silo.id), [payload.silo.id.clone()]);
    }
    let index = payload.index.min(base.silos.len());
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::insertion(&base.silos, index, payload.silo.clone()), ..Default::default() })
}
