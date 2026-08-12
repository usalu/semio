//! 🔺️ `remove-paint-layer` — sparse diff construction (delegates to the existing remove-paint-layer
//! field-delta constructor).

use super::mutation::RemovePaintLayer;
use crate::artifacts::lowpoly::diff::diff_remove_paint_layer;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemovePaintLayer, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_remove_paint_layer(payload.object_id.clone(), payload.index)
}
//#endregion 🔖️Diff
