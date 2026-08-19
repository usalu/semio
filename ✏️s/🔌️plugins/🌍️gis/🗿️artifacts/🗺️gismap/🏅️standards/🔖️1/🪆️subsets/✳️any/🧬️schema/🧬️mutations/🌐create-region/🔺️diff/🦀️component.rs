//! 🔺️ Sparse diff construction for `create-region`.
use super::mutation::CreateRegion;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `added` entry — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone. Fatal
/// `duplicate-id` when `item.id` already names a region.
pub async fn diff(payload: &CreateRegion, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if base.regions.iter().any(|feature| feature.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A region with id \"{}\" already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(GisMapDiff {
        regions: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔹Diff
