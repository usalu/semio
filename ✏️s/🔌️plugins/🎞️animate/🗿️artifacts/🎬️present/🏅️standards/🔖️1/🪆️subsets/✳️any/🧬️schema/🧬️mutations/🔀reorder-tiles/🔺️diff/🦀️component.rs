//! 🔺️ Sparse diff construction for `reorder-tiles`.
use super::mutation::ReorderTiles;
use crate::artifacts::present::diff::{tiles_delta_from_collection_mutation, PresentDiff};
use crate::artifacts::present::PresentSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` reordered-ids delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderTiles, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(tiles_delta_from_collection_mutation(&base.tiles, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
