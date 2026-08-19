//! 🔺️ Sparse diff construction for `replace-position-data`.
use super::mutation::ReplacePositionData;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturePatchEntry, GisMapFeaturesDelta};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeaturePatch};

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `patched` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a position; Warning `no-op` when `new_data` already
/// equals the position's current data.
pub async fn diff(payload: &ReplacePositionData, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(existing) = base.positions.iter().find(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.data == payload.new_data {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Position \"{}\" data is already identical to the requested replacement.", payload.id));
    }
    protocol::MutationOutcome::new(GisMapDiff {
        positions: Some(GisMapFeaturesDelta {
            patched: vec![GisMapFeaturePatchEntry {
                id: payload.id.clone(),
                patch: MapFeaturePatch { data: Some(payload.new_data.clone()) },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔹Diff
