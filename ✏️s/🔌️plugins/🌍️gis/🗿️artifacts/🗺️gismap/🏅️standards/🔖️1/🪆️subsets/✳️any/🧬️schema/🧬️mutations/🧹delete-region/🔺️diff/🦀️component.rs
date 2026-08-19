//! 🔺️ Sparse diff construction for `delete-region`.
use super::mutation::DeleteRegion;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `removed` id — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a region.
pub async fn diff(payload: &DeleteRegion, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if !base.regions.iter().any(|feature| feature.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(GisMapDiff {
        regions: Some(GisMapFeaturesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔹Diff
