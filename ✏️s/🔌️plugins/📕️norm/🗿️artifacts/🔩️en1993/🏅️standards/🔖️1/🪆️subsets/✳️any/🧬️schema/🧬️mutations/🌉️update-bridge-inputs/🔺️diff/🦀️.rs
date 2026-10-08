//! 🌉️ `update-bridge-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateBridgeInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993BridgeFatigueEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.bridge_fatigue.iter().position(|row| row.id == payload.bridge_fatigue_item.id) {
        Some(index) if base.bridge_fatigue[index] == payload.bridge_fatigue_item => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993BridgeFatigueEdit::replace(index, payload.bridge_fatigue_item.id.clone(), payload.bridge_fatigue_item.clone()),
        None => En1993BridgeFatigueEdit::insert(base.bridge_fatigue.len(), payload.bridge_fatigue_item.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: vec![edit], ..Default::default() })
}
