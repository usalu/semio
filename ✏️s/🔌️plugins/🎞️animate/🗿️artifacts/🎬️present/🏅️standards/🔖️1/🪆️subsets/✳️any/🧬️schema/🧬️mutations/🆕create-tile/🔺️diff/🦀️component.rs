//! 🔺️ Sparse diff construction for `create-tile`.
use super::mutation::CreateTile;
use crate::artifacts::present::diff::{PresentDiff, PresentTilesDelta};
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` added-entry delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone. `index` is FINAL-state per the
/// taxonomy's index-addressing law, so this also sets `reordered` to the full id order with the new
/// tile spliced in at `index` (clamped) — `apply_tiles_delta`'s `added` handling is append-only, so
/// `reordered` is what actually places the new tile; every existing call site passes
/// `index: tiles.len()` (append), so this is behavior-preserving there and only changes the
/// previously-dead `index` for non-trailing inserts (needed for `delete-tiles`'s inverse to
/// reconstruct the exact pre-deletion order).
pub fn diff(payload: &CreateTile, base: &PresentSnapshot) -> PresentDiff {
    let mut ids: Vec<String> = base.tiles.iter().map(|item| item.id.clone()).collect();
    let at = payload.index.min(ids.len());
    ids.insert(at, payload.tile.id.clone());
    PresentDiff {
        tiles: Some(PresentTilesDelta { added: vec![payload.tile.clone()], reordered: Some(ids), ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
