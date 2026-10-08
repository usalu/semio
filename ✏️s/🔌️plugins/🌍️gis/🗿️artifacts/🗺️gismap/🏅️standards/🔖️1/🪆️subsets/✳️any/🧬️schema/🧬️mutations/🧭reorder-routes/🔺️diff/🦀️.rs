//! 🔺️ Sparse diff construction for `reorder-routes`.
use super::ReorderRoutes;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload: one `moved` row from the base index of `id` to `to_index` — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone. Error `target-missing` when `id` doesn't name a
/// route; Warning `no-op` when the resulting order is unchanged.
pub fn diff(payload: &ReorderRoutes, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(from) = base.routes.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Route \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(base.routes.len() - 1);
    if from == to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Route \"{}\" is already at index {}.", payload.id, to));
    }
    protocol::MutationOutcome::new(GisMapDiff { routes: Some(GisMapFeaturesDelta::relocation(&base.routes, from, to)), ..Default::default() })
}
//#endregion 🔹Diff
