use crate::remove_layer_water::RemoveLayerWater;
use crate::insert_layer_water::InsertLayerWater;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveLayerWater, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_layer_water::InsertLayerWater> {
    Ok((|| {
    match base.layer_waters.get(payload.index).cloned() {
        Some(layer_water) => vec![Op::from(InsertLayerWater { index: payload.index, layer_water })],
        None => Vec::new(),
    }

    })())
}
