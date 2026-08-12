//! 🔺️ Sparse diff construction for `create-route`.
use super::mutation::CreateRoute;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — a single `added` entry — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreateRoute, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        routes: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
