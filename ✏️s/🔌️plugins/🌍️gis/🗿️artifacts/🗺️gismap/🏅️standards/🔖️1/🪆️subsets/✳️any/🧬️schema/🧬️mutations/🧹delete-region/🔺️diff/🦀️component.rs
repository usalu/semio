//! 🔺️ Sparse diff construction for `delete-region`.
use super::mutation::DeleteRegion;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `regions` delta directly from the payload — a single `removed` id — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &DeleteRegion, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        regions: Some(GisMapFeaturesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
