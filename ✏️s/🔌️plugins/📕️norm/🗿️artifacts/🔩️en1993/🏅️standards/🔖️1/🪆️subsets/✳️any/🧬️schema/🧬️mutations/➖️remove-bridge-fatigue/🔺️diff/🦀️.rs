//! ➖️ `remove-bridge-fatigue` diff — removes the row at the index.

use crate::remove_bridge_fatigue::RemoveBridgeFatigue;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BridgeFatigueDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.bridge_fatigue.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("bridge-fatigue index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::removal(&base.bridge_fatigue, payload.index), ..Default::default() })
}
