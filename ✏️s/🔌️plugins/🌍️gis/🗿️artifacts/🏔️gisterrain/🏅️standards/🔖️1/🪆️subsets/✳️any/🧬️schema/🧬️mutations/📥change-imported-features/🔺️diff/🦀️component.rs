//! 🔺️ Sparse diff construction for `change-imported-features`.
use super::mutation::ChangeImportedFeatures;
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `imported_features_json` field delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeImportedFeatures, _base: &GisTerrainSnapshot) -> GisTerrainDiff {
    crate::artifacts::gisterrain::diff::diff_imported_features_json(payload.new_imported_features_json.clone())
}
//#endregion 🔹Diff
