//! 🔺️ Sparse diff construction for `replace-route-data`.
use super::mutation::ReplaceRouteData;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturePatchEntry, GisMapFeaturesDelta};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — a single `patched` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplaceRouteData, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        routes: Some(GisMapFeaturesDelta {
            patched: vec![GisMapFeaturePatchEntry {
                id: payload.id.clone(),
                patch: MapFeaturePatch { data: Some(payload.new_data.clone()) },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
