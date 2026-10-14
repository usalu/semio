use crate::remove_layer::RemoveLayer;
use crate::insert_layer::InsertLayer;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveLayer, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_layer::InsertLayer> {
    Ok((|| {
    let layer = base.layers.get(payload.index).cloned().unwrap_or_else(|| base.layers[0].clone());
    vec![Op::from(InsertLayer { index: payload.index, layer })]

    })())
}
