//! ➖️ `remove-soil-layer` diff: removes the addressed soil layer row.

use crate::remove_soil_layer::RemoveSoilLayer;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveSoilLayer, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.soil_layers.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::removal(&base.soil_layers, index), ..Default::default() })
}
