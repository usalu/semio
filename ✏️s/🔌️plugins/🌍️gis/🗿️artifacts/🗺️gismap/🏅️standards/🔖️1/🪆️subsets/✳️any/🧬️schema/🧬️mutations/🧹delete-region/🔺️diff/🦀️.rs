//! 🔺️ Sparse diff construction for `delete-region`.
use super::DeleteRegion;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `removed` row at its base index — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a region.
pub fn diff(payload: &DeleteRegion, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(GisMapDiff { regions: Some(GisMapFeaturesDelta::removal(&base.regions, index)), ..Default::default() })
}
//#endregion 🔹Diff
