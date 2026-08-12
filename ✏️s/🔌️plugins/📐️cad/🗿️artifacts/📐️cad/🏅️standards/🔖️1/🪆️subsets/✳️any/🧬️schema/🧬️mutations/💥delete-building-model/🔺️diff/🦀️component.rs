//! 🔺️ `delete-building-model` — sparse diff construction: always clears `building_model`, built directly from
//! `(payload, base)` (idempotent even when `base.building_model` is already `None`).

use super::mutation::DeleteBuildingModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteBuildingModel, _base: &CadSnapshot) -> CadDiff {
    CadDiff { building_model: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
