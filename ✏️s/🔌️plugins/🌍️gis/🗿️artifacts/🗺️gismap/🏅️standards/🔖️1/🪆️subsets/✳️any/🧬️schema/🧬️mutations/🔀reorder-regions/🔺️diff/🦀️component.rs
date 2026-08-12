//! 🔺️ Sparse diff construction for `reorder-regions`.
use super::mutation::ReorderRegions;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` reordered-ids delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderRegions, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(features_delta_from_collection_mutation(&base.regions, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
