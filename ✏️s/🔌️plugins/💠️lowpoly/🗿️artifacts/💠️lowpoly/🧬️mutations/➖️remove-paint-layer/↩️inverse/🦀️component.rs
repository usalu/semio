//! ↩️ Inverse for `RemovePaintLayer`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolySnapshot, object_id: &str, index: usize) -> Vec<LowpolyMutation> {
    let layer = base.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(index)).cloned().unwrap_or_else(|| crate::artifacts::lowpoly::LowpolyPaintLayer::new("Layer"));
    vec![LowpolyMutation::AddPaintLayer { object_id: object_id.to_string(), index, layer }]
}
//#endregion 🔖️Inverse
