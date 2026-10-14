//! 🔺️ `delete-shape-model` — sparse diff construction: always clears `shape_model`, built directly from
//! `(payload, base)` (idempotent even when `base.shape_model` is already `None`).

use super::DeleteShapeModel;
use semio_s_artifact_cad_cad_model_diff::{CadDiff, CadModelSlot};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteShapeModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.shape_model.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Shape-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { shape_model: Some(CadModelSlot { child: None }), ..Default::default() })
}
//#endregion 🔖️Diff
