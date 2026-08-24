//! 🔺️ `delete-structure-classic-model` — sparse diff construction: always clears `structure_classic_model`, built directly from
//! `(payload, base)` (idempotent even when `base.structure_classic_model` is already `None`).

use super::mutation::DeleteStructureClassicModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteStructureClassicModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.structure_classic_model.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Structure-classic-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { structure_classic_model: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
