//! 🔺️ `change-bridge-consequence` diff: patches `consequence` of the addressed bridge fatigue row.

use crate::change_bridge_consequence::ChangeBridgeConsequence;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BridgeFatigueDelta, En1993BridgeFatiguePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBridgeConsequence, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.bridge_fatigue.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "bridge", Vec::<String>::new());
    };
    if row.consequence == payload.new_consequence {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge fatigue already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::modification(payload.id.clone(), En1993BridgeFatiguePatch { consequence: Some(payload.new_consequence), ..Default::default() }), ..Default::default() })
}
