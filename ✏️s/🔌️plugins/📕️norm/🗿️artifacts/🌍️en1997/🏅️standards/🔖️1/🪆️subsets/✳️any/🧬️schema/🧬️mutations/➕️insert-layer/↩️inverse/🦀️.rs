use crate::insert_layer::InsertLayer;
use crate::remove_layer::RemoveLayer;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertLayer, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_layer::RemoveLayer> {
    Ok((|| {
    let at = payload.index.min(base.layers.len());
    vec![Op::from(RemoveLayer { index: at })]

    })())
}
