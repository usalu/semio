//! 🔺️ `change-water-layer-id` diff: patches `layer_id` of the addressed layer water row.

use crate::change_water_layer_id::ChangeWaterLayerId;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersPatch, En1997LayerWatersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWaterLayerId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.layer_waters.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("water {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.layer_id == payload.new_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The layer water already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::modification(&payload.id, En1997LayerWatersPatch { layer_id: Some(payload.new_layer_id.clone()), ..Default::default() })), ..Default::default() })
}
