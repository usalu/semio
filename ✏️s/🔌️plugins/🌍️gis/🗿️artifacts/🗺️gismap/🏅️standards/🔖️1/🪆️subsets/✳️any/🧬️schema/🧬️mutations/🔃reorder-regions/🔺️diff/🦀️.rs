//! 🔺️ Sparse diff construction for `reorder-regions`.
use super::ReorderRegions;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload: one `moved` row from the base index of `id` to `to_index` — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone. Error `target-missing` when `id` doesn't name a
/// region; Warning `no-op` when the resulting order is unchanged.
pub fn diff(payload: &ReorderRegions, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(from) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(base.regions.len() - 1);
    if from == to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Region \"{}\" is already at index {}.", payload.id, to));
    }
    protocol::MutationOutcome::new(GisMapDiff { regions: Some(GisMapFeaturesDelta::relocation(&base.regions, from, to)), ..Default::default() })
}
//#endregion 🔹Diff
