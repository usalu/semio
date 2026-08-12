//! 🔺️ Sparse diff construction for `replace-position-data`.
use super::mutation::ReplacePositionData;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturePatchEntry, GisMapFeaturesDelta};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `patched` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReplacePositionData, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        positions: Some(GisMapFeaturesDelta {
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
