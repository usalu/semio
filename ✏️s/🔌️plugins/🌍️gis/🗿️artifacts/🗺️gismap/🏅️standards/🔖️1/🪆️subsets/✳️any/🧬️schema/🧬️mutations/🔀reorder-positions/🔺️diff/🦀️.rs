//! 🔺️ Sparse diff construction for `reorder-positions`.
use super::ReorderPositions;
use crate::artifacts::gismap::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` reordered-ids delta directly from the payload: recomputes the
/// full id order from `base` by moving `id` to `to_index` — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone. Error `target-missing` when `id` doesn't name a
/// position; Warning `no-op` when the resulting order is unchanged.
pub fn diff(payload: &ReorderPositions, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let original: Vec<String> = base.positions.iter().map(|f| f.id.clone()).collect();
    let Some(from) = original.iter().position(|x| x == &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let mut ids = original.clone();
    let item = ids.remove(from);
    let to = payload.to_index.min(ids.len());
    ids.insert(to, item);
    if ids == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Position \"{}\" is already at index {}.", payload.id, to));
    }
    protocol::MutationOutcome::new(GisMapDiff { positions: Some(GisMapFeaturesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
//#endregion 🔹Diff
