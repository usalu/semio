//! 🔺️ `delete-shape-model` — sparse diff construction: always clears `shape_model`, built directly from
//! `(payload, base)` (idempotent even when `base.shape_model` is already `None`).

use super::mutation::DeleteShapeModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteShapeModel, _base: &CadSnapshot) -> CadDiff {
    CadDiff { shape_model: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
