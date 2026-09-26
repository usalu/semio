//! Diff for `insert-bridge`.
use super::InsertBridge;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.bridges.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.bridge.clone());
    protocol::MutationOutcome::new(En1998Diff { bridges: Some(items), ..Default::default() })
}
