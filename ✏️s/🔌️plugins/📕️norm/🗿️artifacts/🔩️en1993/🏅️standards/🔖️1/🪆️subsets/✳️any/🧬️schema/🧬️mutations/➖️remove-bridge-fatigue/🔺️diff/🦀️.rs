//! ➖️ `remove-bridge-fatigue` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveBridgeFatigue;
use crate::diff::{En1993Diff, En1993BridgeFatigueDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.bridge_fatigue.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("bridge-fatigue index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::removal(&row.id), ..Default::default() })
}
