//! ➕️ `insert-soil-layer` diff: inserts the soil layer row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_soil_layer::InsertSoilLayer;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertSoilLayer, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.soil_layers.iter().any(|existing| existing.id == payload.layer.id) {
        let key = payload.layer.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("soil layer {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.soil_layers.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::insertion(index, payload.layer.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the soil layer list; inserted at {index}.", payload.index.unwrap_or(index)))
}
