//! ✏️ `change-soil-layer-n160` diff: patches `n1_60` of the addressed soil layer row.

use crate::change_soil_layer_n1_60::ChangeSoilLayerN160;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta, En1998SoilLayerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSoilLayerN160, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_n1_60.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.soil_layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.n1_60 == payload.new_n1_60 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::modification(&payload.id, En1998SoilLayerPatch { n1_60: Some(payload.new_n1_60), ..Default::default() }), ..Default::default() })
}
