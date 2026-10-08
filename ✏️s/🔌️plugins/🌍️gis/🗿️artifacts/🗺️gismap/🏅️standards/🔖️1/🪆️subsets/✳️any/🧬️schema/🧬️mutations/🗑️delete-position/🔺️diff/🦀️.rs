//! 🔺️ Sparse diff construction for `delete-position`.
use super::DeletePosition;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — a single `removed` row at its base index —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. Error
/// `target-missing` when `id` doesn't name a position.
pub fn diff(payload: &DeletePosition, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(index) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(GisMapDiff { positions: Some(GisMapFeaturesDelta::removal(&base.positions, index)), ..Default::default() })
}
//#endregion 🔹Diff
