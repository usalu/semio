//! 🔺️ Sparse diff construction for `reorder-positions`.
use super::ReorderPositions;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload: one `moved` row from the base index of `id` to `to_index` — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone. Error `target-missing` when `id` doesn't name a
/// position; Warning `no-op` when the resulting order is unchanged.
pub fn diff(payload: &ReorderPositions, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(from) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(base.positions.len() - 1);
    if from == to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Position \"{}\" is already at index {}.", payload.id, to));
    }
    protocol::MutationOutcome::new(GisMapDiff { positions: Some(GisMapFeaturesDelta::relocation(&base.positions, from, to)), ..Default::default() })
}
//#endregion 🔹Diff
