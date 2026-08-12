//! 🔺️ Sparse diff construction for `delete-tile`.
use super::mutation::DeleteTile;
use crate::artifacts::present::diff::{tiles_delta_from_collection_mutation, PresentDiff};
use crate::artifacts::present::PresentSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` delta directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &DeleteTile, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(tiles_delta_from_collection_mutation(&base.tiles, &CollectionMutation::Remove { id: payload.id.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
