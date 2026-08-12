//! 🔺️ Sparse diff construction for `reorder-tiles`.
use super::mutation::ReorderTiles;
use crate::artifacts::present::diff::{PresentDiff, PresentTilesDelta};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` reordered-ids delta directly from the payload — recomputes the
/// full id order from `base` with `id` relocated to `to_index` (clamped) — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ReorderTiles, base: &PresentSnapshot) -> PresentDiff {
    let mut ids: Vec<String> = base.tiles.iter().map(|item| item.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    PresentDiff { tiles: Some(PresentTilesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔹Diff
