//! 🔺️ `delete-shape-model` — sparse diff construction: always clears `shape_model`, built directly from
//! `(payload, base)` (idempotent even when `base.shape_model` is already `None`).

use super::DeleteShapeModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteShapeModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.shape_model.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Shape-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { shape_model: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
