//! 🔺️ Sparse diff construction for `create-position`.
use super::mutation::CreatePosition;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload via the shared
/// `CollectionMutation` diff engine (`🧰️framework`'s `vcs::features_delta_from_collection_mutation`
/// mirror declared alongside this artifact's `GisMapDiff`) — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreatePosition, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        positions: Some(features_delta_from_collection_mutation(&base.positions, &CollectionMutation::Add { index: payload.index, item: payload.item.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
