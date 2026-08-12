//! 🔺️ Sparse diff construction for `change-exaggeration`.
use super::mutation::ChangeExaggeration;
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `exaggeration` field delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeExaggeration, _base: &GisTerrainSnapshot) -> GisTerrainDiff {
    crate::artifacts::gisterrain::diff::diff_exaggeration(payload.new_exaggeration)
}
//#endregion 🔹Diff
