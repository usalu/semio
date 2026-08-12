//! 🔺️ Sparse diff construction for `create-tile`.
use super::mutation::CreateTile;
use crate::artifacts::present::diff::{PresentDiff, PresentTilesDelta};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` added-entry delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreateTile, _base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(PresentTilesDelta { added: vec![payload.tile.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
