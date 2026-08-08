//! ↩️ Inverse for `AddPaintLayer`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;

use crate::artifacts::lowpoly::LowpolyPaintLayer;

//#region 🔖️Inverse
pub fn inverse(_base: &LowpolyProjection, object_id: &str, index: usize, _layer: &LowpolyPaintLayer) -> Vec<LowpolyMutation> {
    vec![LowpolyMutation::RemovePaintLayer { object_id: object_id.to_string(), index }]
}
//#endregion 🔖️Inverse
