//! 🔺️ Sparse diff construction for `create-position`.
use super::CreatePosition;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `added` entry —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. Fatal
/// `duplicate-id` when `item.id` already names a position.
pub fn diff(payload: &CreatePosition, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    if base.positions.iter().any(|feature| feature.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A position with id \"{}\" already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    let reordered = (payload.index < base.positions.len()).then(|| {
        let mut order: Vec<String> = base.positions.iter().map(|feature| feature.id.clone()).collect();
        order.insert(payload.index, payload.item.id.clone());
        order
    });
    protocol::MutationOutcome::new(GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![payload.item.clone()], reordered, ..Default::default() }), ..Default::default() })
}
//#endregion 🔹Diff
