//! 🔺️ Sparse diff construction for `replace-route-data`.
use super::mutation::ReplaceRouteData;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplaceRouteData, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        routes: Some(features_delta_from_collection_mutation(&base.routes, &CollectionMutation::Patch { id: payload.id.clone(), patch: MapFeaturePatch { data: Some(payload.new_data.clone()) } })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
