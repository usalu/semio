//! 🔺️ Sparse diff construction for `replace-position-data`.
use super::mutation::ReplacePositionData;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplacePositionData, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        positions: Some(features_delta_from_collection_mutation(&base.positions, &CollectionMutation::Patch { id: payload.id.clone(), patch: MapFeaturePatch { data: Some(payload.new_data.clone()) } })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
