//! ↩️ `change-paint-layer-opacity` — undo restores the base-state opacity; missing object/index ⇒ `Vec::new()`.

use super::ChangePaintLayerOpacity;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_paint_layer_opacity::ChangePaintLayerOpacity>>(payload: &ChangePaintLayerOpacity, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![Op::from(ChangePaintLayerOpacity { object_id: payload.object_id.clone(), index: payload.index, new_opacity: layer.opacity })]

    })())
}
//#endregion 🔖️Inverse
