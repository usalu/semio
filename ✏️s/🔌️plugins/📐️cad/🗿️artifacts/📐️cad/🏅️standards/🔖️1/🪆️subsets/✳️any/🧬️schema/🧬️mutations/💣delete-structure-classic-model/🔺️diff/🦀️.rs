//! 🔺️ `delete-structure-classic-model` — sparse diff construction: always clears `structure_classic_model`, built directly from
//! `(payload, base)` (idempotent even when `base.structure_classic_model` is already `None`).

use super::DeleteStructureClassicModel;
use semio_s_artifact_cad_cad_model_diff::{CadDiff, CadModelSlot};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteStructureClassicModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.structure_classic_model.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Structure-classic-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { structure_classic_model: Some(CadModelSlot { child: None }), ..Default::default() })
}
//#endregion 🔖️Diff
