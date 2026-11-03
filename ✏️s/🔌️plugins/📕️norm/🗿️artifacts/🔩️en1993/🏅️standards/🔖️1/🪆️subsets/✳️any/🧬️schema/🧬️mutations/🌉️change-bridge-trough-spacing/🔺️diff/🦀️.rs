//! 🔺️ `change-bridge-trough-spacing` diff: patches `trough_spacing` of the addressed bridge fatigue row.

use crate::change_bridge_trough_spacing::ChangeBridgeTroughSpacing;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BridgeFatigueDelta, En1993BridgeFatiguePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBridgeTroughSpacing, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_trough_spacing.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.bridge_fatigue.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "bridge", Vec::<String>::new());
    };
    if row.trough_spacing == payload.new_trough_spacing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge fatigue already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::modification(payload.id.clone(), En1993BridgeFatiguePatch { trough_spacing: Some(payload.new_trough_spacing), ..Default::default() }), ..Default::default() })
}
