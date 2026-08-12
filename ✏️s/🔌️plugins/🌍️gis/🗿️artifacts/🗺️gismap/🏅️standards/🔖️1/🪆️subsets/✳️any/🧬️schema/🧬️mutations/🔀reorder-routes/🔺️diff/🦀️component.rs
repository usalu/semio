//! 🔺️ Sparse diff construction for `reorder-routes`.
use super::mutation::ReorderRoutes;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` reordered-ids delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderRoutes, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        routes: Some(features_delta_from_collection_mutation(&base.routes, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
