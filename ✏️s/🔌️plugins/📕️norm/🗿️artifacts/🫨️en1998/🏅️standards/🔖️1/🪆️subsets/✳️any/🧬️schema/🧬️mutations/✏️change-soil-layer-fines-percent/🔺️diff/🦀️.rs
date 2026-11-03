//! ✏️ `change-soil-layer-fines-percent` diff: patches `fines_percent` of the addressed soil layer row.

use crate::change_soil_layer_fines_percent::ChangeSoilLayerFinesPercent;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta, En1998SoilLayerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSoilLayerFinesPercent, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_fines_percent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.soil_layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.fines_percent == payload.new_fines_percent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::modification(&payload.id, En1998SoilLayerPatch { fines_percent: Some(payload.new_fines_percent), ..Default::default() }), ..Default::default() })
}
