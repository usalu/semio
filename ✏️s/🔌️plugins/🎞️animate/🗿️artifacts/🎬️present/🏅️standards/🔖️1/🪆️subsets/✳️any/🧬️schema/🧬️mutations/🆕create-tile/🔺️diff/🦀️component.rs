//! 🔺️ Sparse diff construction for `create-tile`.
use super::mutation::CreateTile;
use crate::artifacts::present::diff::{tiles_delta_from_collection_mutation, PresentDiff};
use crate::artifacts::present::PresentSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` delta directly from the payload via the shared
/// `CollectionMutation` diff engine (`crate::artifacts::present::diff`'s
/// `tiles_delta_from_collection_mutation`) — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreateTile, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(tiles_delta_from_collection_mutation(&base.tiles, &CollectionMutation::Add { index: payload.index, item: payload.tile.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
