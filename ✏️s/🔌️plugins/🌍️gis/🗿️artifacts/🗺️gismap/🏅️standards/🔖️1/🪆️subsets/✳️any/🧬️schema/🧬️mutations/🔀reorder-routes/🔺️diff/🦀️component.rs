//! 🔺️ Sparse diff construction for `reorder-routes`.
use super::mutation::ReorderRoutes;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` reordered-ids delta directly from the payload: recomputes the
/// full id order from `base` by moving `id` to `to_index` — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderRoutes, base: &GisMapSnapshot) -> GisMapDiff {
    let mut ids: Vec<String> = base.routes.iter().map(|f| f.id.clone()).collect();
    if let Some(from) = ids.iter().position(|x| x == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    GisMapDiff {
        routes: Some(GisMapFeaturesDelta { reordered: Some(ids), ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
