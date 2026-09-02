//! ↩️ Inverse reconstruction for `change-imported-features` — reads the BASE value, never the diff.
use super::ChangeImportedFeatures;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.imported_features_json` — captured from pre-state, never from the
/// applied diff.
pub fn inverse(_payload: &ChangeImportedFeatures, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
    vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: base.imported_features_json.clone() })]
}
//#endregion 🔹Inverse
