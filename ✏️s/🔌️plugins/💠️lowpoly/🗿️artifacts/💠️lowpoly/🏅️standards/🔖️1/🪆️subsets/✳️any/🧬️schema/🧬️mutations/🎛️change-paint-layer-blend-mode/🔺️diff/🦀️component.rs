//! 🔺️ `change-paint-layer-blend-mode` — sparse diff construction: one-field paint-layer patch.

use super::mutation::ChangePaintLayerBlendMode;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePaintLayerBlendMode, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { blend_mode: Some(payload.new_blend_mode.clone()), ..LowpolyPaintLayerPatch::default() })
}
//#endregion 🔖️Diff
