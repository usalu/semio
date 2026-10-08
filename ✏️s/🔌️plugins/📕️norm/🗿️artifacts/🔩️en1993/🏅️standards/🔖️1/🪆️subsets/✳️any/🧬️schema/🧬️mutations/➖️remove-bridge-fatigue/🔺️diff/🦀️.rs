//! ➖️ `remove-bridge-fatigue` diff — removes the row at the index.

use super::RemoveBridgeFatigue;
use crate::diff::{En1993Diff, En1993BridgeFatigueDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.bridge_fatigue.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("bridge-fatigue index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::removal(&base.bridge_fatigue, payload.index), ..Default::default() })
}
