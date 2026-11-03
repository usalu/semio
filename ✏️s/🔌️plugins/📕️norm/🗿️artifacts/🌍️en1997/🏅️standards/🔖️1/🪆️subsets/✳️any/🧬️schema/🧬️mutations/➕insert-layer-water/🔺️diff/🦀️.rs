use crate::insert_layer_water::InsertLayerWater;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersRows};

pub fn diff(payload: &InsertLayerWater, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.layer_waters.iter().any(|existing| existing.id == payload.layer_water.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("LayerWater id {} already exists.", payload.layer_water.id), [payload.layer_water.id.clone()]);
    }
    if payload.index > base.layer_waters.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.layer_waters.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::insertion(payload.index, payload.layer_water.clone())), ..Default::default() })
}
