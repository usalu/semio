//! ✏️ `change-soil-layer-foundation-id` diff: patches `foundation_id` of the addressed soil layer row.

use crate::change_soil_layer_foundation_id::ChangeSoilLayerFoundationId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta, En1998SoilLayerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSoilLayerFoundationId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.soil_layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.foundation_id == payload.new_foundation_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::modification(&payload.id, En1998SoilLayerPatch { foundation_id: Some(payload.new_foundation_id.clone()), ..Default::default() }), ..Default::default() })
}
