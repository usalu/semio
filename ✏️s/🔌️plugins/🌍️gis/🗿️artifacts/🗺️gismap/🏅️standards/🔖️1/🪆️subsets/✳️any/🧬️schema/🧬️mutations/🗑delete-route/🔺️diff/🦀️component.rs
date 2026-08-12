//! 🔺️ Sparse diff construction for `delete-route`.
use super::mutation::DeleteRoute;
use crate::artifacts::gismap::diff::{features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::CollectionMutation;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &DeleteRoute, base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        routes: Some(features_delta_from_collection_mutation(&base.routes, &CollectionMutation::Remove { id: payload.id.clone() })),
        ..Default::default()
    }
}
//#endregion 🔹Diff
