//! 🔺️ Sparse diff construction for `replace-region-data`.
use super::mutation::ReplaceRegionData;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturePatchEntry, GisMapFeaturesDelta};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `patched` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a region; Warning `no-op` when `new_data` already
/// equals the region's current data.
pub async fn diff(payload: &ReplaceRegionData, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(existing) = base.regions.iter().find(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.data == payload.new_data {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Region \"{}\" data is already identical to the requested replacement.", payload.id));
    }
    protocol::MutationOutcome::new(GisMapDiff {
        regions: Some(GisMapFeaturesDelta { patched: vec![GisMapFeaturePatchEntry { id: payload.id.clone(), patch: MapFeaturePatch { data: Some(payload.new_data.clone()) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔹Diff
