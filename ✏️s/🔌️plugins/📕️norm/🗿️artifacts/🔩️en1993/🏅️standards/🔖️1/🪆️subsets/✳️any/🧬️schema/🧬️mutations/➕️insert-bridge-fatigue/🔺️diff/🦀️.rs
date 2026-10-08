//! ➕️ `insert-bridge-fatigue` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertBridgeFatigue;
use crate::diff::{En1993Diff, En1993BridgeFatigueDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.bridge_fatigue.iter().any(|existing| existing.id == payload.bridge_fatigue_item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge fatigue item id {} already exists.", payload.bridge_fatigue_item.id), [payload.bridge_fatigue_item.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.bridge_fatigue.len());
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::insertion(index, payload.bridge_fatigue_item.clone()), ..Default::default() })
}
