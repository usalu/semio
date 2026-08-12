//! 🔺️ Sparse diff construction for `create-region`.
use super::mutation::CreateRegion;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `added` entry — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreateRegion, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
