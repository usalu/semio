//! ↩️ `remove-paint-layer` — undo re-`insert-paint-layer`s the captured layer at the same index;
//! missing object/index ⇒ `Vec::new()`.

use super::RemovePaintLayer;
use crate::insert_paint_layer;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::insert_paint_layer::InsertPaintLayer>>(payload: &RemovePaintLayer, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![Op::from(insert_paint_layer::InsertPaintLayer { object_id: payload.object_id.clone(), index: payload.index, layer: layer.clone() })]

    })())
}
//#endregion 🔖️Inverse
