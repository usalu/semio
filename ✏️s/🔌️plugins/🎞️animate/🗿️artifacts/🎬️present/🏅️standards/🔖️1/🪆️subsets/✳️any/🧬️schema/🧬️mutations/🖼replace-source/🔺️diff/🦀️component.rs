//! 🔺️ Sparse diff construction for `replace-source`.
use super::mutation::ReplaceSource;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `source` delta directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplaceSource, _base: &PresentSnapshot) -> PresentDiff {
    PresentDiff { source: Some(payload.new_source.clone()), ..Default::default() }
}
//#endregion 🔹Diff
