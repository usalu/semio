//! 🔺️ Sparse diff construction for `create-region`.
use super::mutation::CreateRegion;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload via the shared
/// `CollectionMutation` diff engine (`🧰️framework`'s `vcs::features_delta_from_collection_mutation`
/// mirror declared alongside this artifact's `GisMapDiff`) — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreateRegion, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(features_delta_from_collection_mutation(&base.regions, &CollectionMutation::Add { index: payload.index, item: payload.item.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
