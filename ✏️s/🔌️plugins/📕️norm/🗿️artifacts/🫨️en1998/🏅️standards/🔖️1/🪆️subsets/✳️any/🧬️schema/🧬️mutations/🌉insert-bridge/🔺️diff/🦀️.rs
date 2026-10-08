//! 🌉 `insert-bridge` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertBridge;
use crate::diff::{En1998Diff, En1998BridgeDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.bridges.iter().any(|existing| existing.id == payload.bridge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge id {} already exists.", payload.bridge.id), [payload.bridge.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.bridges.len());
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::insertion(index, payload.bridge.clone()), ..Default::default() })
}
