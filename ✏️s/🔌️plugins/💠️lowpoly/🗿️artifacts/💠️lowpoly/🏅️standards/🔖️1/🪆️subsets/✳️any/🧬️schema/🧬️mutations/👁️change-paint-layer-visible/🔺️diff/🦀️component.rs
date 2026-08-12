//! 🔺️ `change-paint-layer-visible` — sparse diff construction: one-field paint-layer patch.

use super::mutation::ChangePaintLayerVisible;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePaintLayerVisible, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { visible: Some(payload.new_visible), ..LowpolyPaintLayerPatch::default() })
}
//#endregion 🔖️Diff
