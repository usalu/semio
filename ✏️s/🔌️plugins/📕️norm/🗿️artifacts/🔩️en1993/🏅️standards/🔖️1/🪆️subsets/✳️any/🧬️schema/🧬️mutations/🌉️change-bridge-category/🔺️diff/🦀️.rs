//! 🔺️ `change-bridge-category` diff: patches `category` of the addressed bridge fatigue row.

use crate::change_bridge_category::ChangeBridgeCategory;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BridgeFatigueDelta, En1993BridgeFatiguePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBridgeCategory, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.bridge_fatigue.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "bridge", Vec::<String>::new());
    };
    if row.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge fatigue already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: En1993BridgeFatigueDelta::modification(payload.id.clone(), En1993BridgeFatiguePatch { category: Some(payload.new_category), ..Default::default() }), ..Default::default() })
}
