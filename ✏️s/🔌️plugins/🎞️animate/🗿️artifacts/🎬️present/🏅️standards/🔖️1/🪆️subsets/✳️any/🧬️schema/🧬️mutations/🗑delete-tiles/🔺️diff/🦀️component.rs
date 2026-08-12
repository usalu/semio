//! 🔺️ Sparse diff construction for `delete-tiles`.
use super::mutation::DeleteTiles;
use crate::artifacts::present::diff::{PresentDiff, PresentTilesDelta};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` removed-ids delta directly from the payload, filtered to ids that
/// actually exist in `base` — real handcrafted construction, never apply-then-capture.
pub fn diff(payload: &DeleteTiles, base: &PresentSnapshot) -> PresentDiff {
    let removed: Vec<String> = payload.ids.iter().filter(|id| base.tiles.iter().any(|tile| &tile.id == *id)).cloned().collect();
    PresentDiff { tiles: Some(PresentTilesDelta { removed, ..Default::default() }), ..Default::default() }
}
//#endregion 🔹Diff
