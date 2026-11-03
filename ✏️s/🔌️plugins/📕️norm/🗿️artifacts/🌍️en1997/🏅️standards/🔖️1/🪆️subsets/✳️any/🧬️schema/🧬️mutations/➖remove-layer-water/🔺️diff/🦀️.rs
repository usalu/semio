use crate::remove_layer_water::RemoveLayerWater;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersRows};

pub fn diff(payload: &RemoveLayerWater, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.layer_waters.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer-water #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::removal(&base.layer_waters, payload.index)), ..Default::default() })
}
