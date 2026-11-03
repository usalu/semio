use crate::insert_layer_water::InsertLayerWater;
use crate::remove_layer_water::RemoveLayerWater;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertLayerWater, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_layer_water::RemoveLayerWater> {
    Ok((|| {
    let at = payload.index.min(base.layer_waters.len());
    vec![Op::from(RemoveLayerWater { index: at })]

    })())
}
