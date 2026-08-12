//! 🔺️ Sparse diff construction for `resize-tile-crop`.
use super::mutation::ResizeTileCrop;
use crate::artifacts::present::diff::{tiles_delta_from_collection_mutation, PresentDiff};
use crate::artifacts::present::{FigureTileDraftPatch, PresentSnapshot};
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` crop-only patch delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ResizeTileCrop, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(tiles_delta_from_collection_mutation(
            &base.tiles,
            &CollectionMutation::Patch { id: payload.id.clone(), patch: FigureTileDraftPatch { name: None, crop: Some(payload.new_crop.clone()) } },
        )),
        ..Default::default()
    }
}
//#endregion 🔹Diff
