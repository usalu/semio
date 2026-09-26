//! Diff for `insert-silo`.
use super::InsertSilo;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.silos.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.silo.clone());
    protocol::MutationOutcome::new(En1998Diff { silos: Some(items), ..Default::default() })
}
