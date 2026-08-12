//! 🔺️ Sparse diff construction for `delete-region`.
use super::mutation::DeleteRegion;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &DeleteRegion, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(features_delta_from_collection_mutation(&base.regions, &CollectionMutation::Remove { id: payload.id.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
