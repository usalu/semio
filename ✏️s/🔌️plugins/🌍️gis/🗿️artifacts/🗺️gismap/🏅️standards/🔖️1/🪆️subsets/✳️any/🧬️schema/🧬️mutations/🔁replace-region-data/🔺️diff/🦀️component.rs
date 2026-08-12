//! 🔺️ Sparse diff construction for `replace-region-data`.
use super::mutation::ReplaceRegionData;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplaceRegionData, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(features_delta_from_collection_mutation(&base.regions, &CollectionMutation::Patch { id: payload.id.clone(), patch: MapFeaturePatch { data: Some(payload.new_data.clone()) } })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
