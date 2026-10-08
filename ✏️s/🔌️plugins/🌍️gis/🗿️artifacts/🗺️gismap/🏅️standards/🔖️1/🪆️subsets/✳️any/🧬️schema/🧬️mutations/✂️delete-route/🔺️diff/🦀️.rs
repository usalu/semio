//! 🔺️ Sparse diff construction for `delete-route`.
use super::DeleteRoute;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — a single `removed` row at its base index — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a route.
pub fn diff(payload: &DeleteRoute, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(index) = base.routes.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Route \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(GisMapDiff { routes: Some(GisMapFeaturesDelta::removal(&base.routes, index)), ..Default::default() })
}
//#endregion 🔹Diff
