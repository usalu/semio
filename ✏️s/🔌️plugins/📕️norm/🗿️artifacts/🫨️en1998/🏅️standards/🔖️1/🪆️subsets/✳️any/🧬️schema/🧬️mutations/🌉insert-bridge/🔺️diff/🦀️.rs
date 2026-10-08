//! 🌉 `insert-bridge` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertBridge;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998Diff, En1998BridgeEdit};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.bridges.iter().any(|existing| existing.id == payload.bridge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge id {} already exists.", payload.bridge.id), [payload.bridge.id.clone()]);
    }
    let index = payload.index.min(base.bridges.len());
    protocol::MutationOutcome::new(En1998Diff { bridges: vec![En1998BridgeEdit::insert(index, payload.bridge.clone())], ..Default::default() })
}
