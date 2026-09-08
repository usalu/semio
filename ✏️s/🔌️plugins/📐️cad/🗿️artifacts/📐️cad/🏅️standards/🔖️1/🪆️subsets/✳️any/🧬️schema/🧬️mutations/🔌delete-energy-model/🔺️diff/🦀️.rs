//! 🔺️ `delete-energy-model` — sparse diff construction: always clears `energy_model`, built directly from
//! `(payload, base)` (idempotent even when `base.energy_model` is already `None`).

use super::DeleteEnergyModel;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteEnergyModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.energy_model.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Energy-model child is already empty.");
    }
    protocol::MutationOutcome::new(CadDiff { energy_model: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
