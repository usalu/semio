//! 🔺️ Sparse diff construction for `create-position`.
use super::mutation::CreatePosition;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `added` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &CreatePosition, _base: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        positions: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
