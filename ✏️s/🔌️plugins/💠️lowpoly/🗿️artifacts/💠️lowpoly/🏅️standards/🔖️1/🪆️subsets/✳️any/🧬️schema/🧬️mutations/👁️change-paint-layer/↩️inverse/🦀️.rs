//! ↩️ `change-paint-layer-visible` — undo restores the base-state flag; missing object/index ⇒ `Vec::new()`.

use super::ChangePaintLayerVisible;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_paint_layer_visible::ChangePaintLayerVisible>>(payload: &ChangePaintLayerVisible, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![Op::from(ChangePaintLayerVisible { object_id: payload.object_id.clone(), index: payload.index, new_visible: layer.visible })]

    })())
}
//#endregion 🔖️Inverse
