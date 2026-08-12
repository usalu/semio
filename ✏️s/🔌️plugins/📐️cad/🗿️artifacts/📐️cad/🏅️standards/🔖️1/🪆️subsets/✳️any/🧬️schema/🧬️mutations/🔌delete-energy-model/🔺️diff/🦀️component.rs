//! 🔺️ `delete-energy-model` — sparse diff construction: always clears `energy_model`, built directly from
//! `(payload, base)` (idempotent even when `base.energy_model` is already `None`).

use super::mutation::DeleteEnergyModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteEnergyModel, _base: &CadSnapshot) -> CadDiff {
    CadDiff { energy_model: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
