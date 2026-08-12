//! 🔺️ `rename-paint-layer` — sparse diff construction: one-field paint-layer patch on `name`.

use super::mutation::RenamePaintLayer;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenamePaintLayer, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { name: Some(payload.new_name.clone()), ..LowpolyPaintLayerPatch::default() })
}
//#endregion 🔖️Diff
