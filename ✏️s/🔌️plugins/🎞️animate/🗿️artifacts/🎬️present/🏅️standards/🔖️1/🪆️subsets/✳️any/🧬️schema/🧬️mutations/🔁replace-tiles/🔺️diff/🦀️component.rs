//! 🔺️ Sparse diff construction for `replace-tiles`.
use super::mutation::ReplaceTiles;
use crate::artifacts::present::diff::{tiles_delta_from_set_tiles, PresentDiff};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` remove-all/add-all delta directly from the payload via the shared
/// whole-collection diff helper (`crate::artifacts::present::diff`'s `tiles_delta_from_set_tiles`)
/// — real handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplaceTiles, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff { tiles: Some(tiles_delta_from_set_tiles(&base.tiles, &payload.new_tiles)), ..Default::default() }
}
//#endregion 🔹Diff
