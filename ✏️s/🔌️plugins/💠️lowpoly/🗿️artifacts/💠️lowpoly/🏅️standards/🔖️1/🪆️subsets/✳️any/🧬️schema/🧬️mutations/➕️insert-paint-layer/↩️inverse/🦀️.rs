//! ↩️ `insert-paint-layer` — undo is `remove-paint-layer` at the same (now-final) index.

use super::InsertPaintLayer;
use crate::remove_paint_layer;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::remove_paint_layer::RemovePaintLayer>>(payload: &InsertPaintLayer, _base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(remove_paint_layer::RemovePaintLayer { object_id: payload.object_id.clone(), index: payload.index })]

    })())
}
//#endregion 🔖️Inverse
