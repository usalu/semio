//! ↩️ `rename-paint-layer` — undo restores the base-state name; missing object/index ⇒ `Vec::new()`.

use super::RenamePaintLayer;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_paint_layer::RenamePaintLayer>>(payload: &RenamePaintLayer, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![Op::from(RenamePaintLayer { object_id: payload.object_id.clone(), index: payload.index, new_name: layer.name.clone() })]

    })())
}
//#endregion 🔖️Inverse
