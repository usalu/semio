//! 🔺️ Sparse diff construction for `create-route`.
use super::mutation::CreateRoute;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — a single `added` entry — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Fatal
/// `duplicate-id` when `item.id` already names a route.
pub fn diff(payload: &CreateRoute, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if base.routes.iter().any(|feature| feature.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A route with id \"{}\" already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(GisMapDiff { routes: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔹Diff
