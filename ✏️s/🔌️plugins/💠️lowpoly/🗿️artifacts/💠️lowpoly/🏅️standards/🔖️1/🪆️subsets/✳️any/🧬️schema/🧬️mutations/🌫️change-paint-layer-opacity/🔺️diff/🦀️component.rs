//! 🔺️ `change-paint-layer-opacity` — sparse diff construction: one-field paint-layer patch.

use super::mutation::ChangePaintLayerOpacity;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePaintLayerOpacity, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { opacity: Some(payload.new_opacity), ..LowpolyPaintLayerPatch::default() })
}
//#endregion 🔖️Diff
