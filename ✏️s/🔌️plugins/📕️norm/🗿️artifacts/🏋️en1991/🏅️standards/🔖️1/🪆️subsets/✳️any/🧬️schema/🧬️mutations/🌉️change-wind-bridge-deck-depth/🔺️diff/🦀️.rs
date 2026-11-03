//! 🌉 `change-wind-bridge-deck-depth` diff: patches `deck_depth` of the addressed bridge wind case row.

use crate::change_wind_bridge_deck_depth::ChangeWindBridgeDeckDepth;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta, En1991WindBridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindBridgeDeckDepth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_deck_depth.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge wind case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.deck_depth == payload.new_deck_depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge wind case already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::modification(&payload.id, En1991WindBridgePatch { deck_depth: Some(payload.new_deck_depth), ..Default::default() }), ..Default::default() })
}
