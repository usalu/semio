//! 🔺️ Sparse diff construction for `change-imported-features`.
use super::ChangeImportedFeatures;
use crate::diff::GisTerrainDiff;
use crate::GisTerrainSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `imported_map` field delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Warning `no-op`
/// when `new_imported_map` already equals `base.imported_map`.
pub fn diff(payload: &ChangeImportedFeatures, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
    let identical=match(&base.imported_map,&payload.new_imported_map){(None,None)=>true,(Some(base),Some(requested))=>base.same(requested),_=>false};
    if identical {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Imported features are already identical to the requested replacement.");
    }
    protocol::MutationOutcome::new(crate::diff::diff_imported_map(payload.new_imported_map.clone()))
}
//#endregion 🔹Diff
