//! 🔺️ Sparse diff construction for `delete-route`.
use super::mutation::DeleteRoute;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — a single `removed` id — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a route.
pub fn diff(payload: &DeleteRoute, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if !base.routes.iter().any(|feature| feature.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Route \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(GisMapDiff {
        routes: Some(GisMapFeaturesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔹Diff
