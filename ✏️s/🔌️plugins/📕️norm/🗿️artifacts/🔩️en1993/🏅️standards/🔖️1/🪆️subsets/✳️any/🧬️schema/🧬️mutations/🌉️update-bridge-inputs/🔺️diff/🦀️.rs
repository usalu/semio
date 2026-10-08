//! 🌉️ `update-bridge-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateBridgeInputs;
use crate::diff::{En1993Diff, En1993BridgeFatigueDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.bridge_fatigue.iter().position(|row| row.id == payload.bridge_fatigue_item.id) {
        Some(index) if base.bridge_fatigue[index] == payload.bridge_fatigue_item => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993BridgeFatigueDelta::removal(&payload.bridge_fatigue_item.id);
            replacement.absorb(En1993BridgeFatigueDelta::insertion(&base.bridge_fatigue, index, payload.bridge_fatigue_item.clone()));
            replacement
        }
        None => En1993BridgeFatigueDelta::insertion(&base.bridge_fatigue, base.bridge_fatigue.len(), payload.bridge_fatigue_item.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: delta, ..Default::default() })
}
