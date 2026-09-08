//! 🔺️ Sparse diff construction for `change-imported-features`.
use super::ChangeImportedFeatures;
use crate::diff::GisTerrainDiff;
use crate::GisTerrainSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `imported_features_json` field delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Warning `no-op`
/// when `new_imported_features_json` already equals `base.imported_features_json`.
pub fn diff(payload: &ChangeImportedFeatures, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
    if base.imported_features_json == payload.new_imported_features_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Imported features are already identical to the requested replacement.");
    }
    protocol::MutationOutcome::new(crate::diff::diff_imported_features_json(payload.new_imported_features_json.clone()))
}
//#endregion 🔹Diff
