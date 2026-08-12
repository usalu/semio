//! 🔺️ Sparse diff construction for `rename-tile`.
use super::mutation::RenameTile;
use crate::artifacts::present::diff::{tiles_delta_from_collection_mutation, PresentDiff};
use crate::artifacts::present::{FigureTileDraftPatch, PresentSnapshot};
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` name-only patch delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &RenameTile, base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(tiles_delta_from_collection_mutation(
            &base.tiles,
            &CollectionMutation::Patch { id: payload.id.clone(), patch: FigureTileDraftPatch { name: Some(payload.new_name.clone()), crop: None } },
        )),
        ..Default::default()
    }
}
//#endregion 🔹Diff
