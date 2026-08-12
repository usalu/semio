//! 🔺️ Sparse diff construction for `delete-tile`.
use super::mutation::DeleteTile;
use crate::artifacts::present::diff::{PresentDiff, PresentTilesDelta};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` removed-id delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &DeleteTile, _base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(PresentTilesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
