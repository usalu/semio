//! 🔺️ `insert-paint-layer` — sparse diff construction (delegates to the existing add-paint-layer
//! field-delta constructor).

use super::mutation::InsertPaintLayer;
use crate::artifacts::lowpoly::diff::diff_add_paint_layer;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &InsertPaintLayer, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_add_paint_layer(payload.object_id.clone(), payload.index, payload.layer.clone())
}
//#endregion 🔖️Diff
