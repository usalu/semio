//! ↩️ Inverse for `PatchPaintLayer`.
use crate::artifacts::lowpoly::mutations::{LowpolyMutation, LowpolyPaintLayerPatch};
use crate::artifacts::lowpoly::LowpolyProjection;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolyProjection, object_id: &str, index: usize, patch: &LowpolyPaintLayerPatch) -> Vec<LowpolyMutation> {
    let mut probe = base.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(index)).cloned().unwrap_or_else(|| crate::artifacts::lowpoly::LowpolyPaintLayer::new("Layer"));
    let inverse_patch = crate::artifacts::lowpoly::mutations::apply_paint_layer_patch(&mut probe, patch);
    vec![LowpolyMutation::PatchPaintLayer { object_id: object_id.to_string(), index, patch: inverse_patch }]
}
//#endregion 🔖️Inverse
