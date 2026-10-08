//! 🔺️ `delete-building-model` — sparse diff construction: always clears `building_model`, built directly from
//! `(payload, base)` (idempotent even when `base.building_model` is already `None`).

use super::DeleteBuildingModel;
use crate::diff::{CadDiff, CadModelSlot};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteBuildingModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.building_model.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Building-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { building_model: Some(CadModelSlot { child: None }), ..Default::default() })
}
//#endregion 🔖️Diff
