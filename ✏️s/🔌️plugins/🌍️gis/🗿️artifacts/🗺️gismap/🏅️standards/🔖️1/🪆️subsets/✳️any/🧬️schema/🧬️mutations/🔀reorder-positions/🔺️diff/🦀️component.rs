//! 🔺️ Sparse diff construction for `reorder-positions`.
use super::mutation::ReorderPositions;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` reordered-ids delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderPositions, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        positions: Some(features_delta_from_collection_mutation(&base.positions, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
