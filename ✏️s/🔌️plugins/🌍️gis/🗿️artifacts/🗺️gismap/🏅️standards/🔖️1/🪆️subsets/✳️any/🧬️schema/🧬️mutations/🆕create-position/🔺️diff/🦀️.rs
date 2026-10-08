//! 🔺️ Sparse diff construction for `create-position`.
use super::CreatePosition;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `inserted` row at its after index —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. Fatal
/// `duplicate-id` when `item.id` already names a position.
pub fn diff(payload: &CreatePosition, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if base.positions.iter().any(|feature| feature.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A position with id \"{}\" already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    let index = payload.index.min(base.positions.len());
    protocol::MutationOutcome::new(GisMapDiff { positions: Some(GisMapFeaturesDelta::insertion(index, payload.item.clone())), ..Default::default() })
}
//#endregion 🔹Diff
