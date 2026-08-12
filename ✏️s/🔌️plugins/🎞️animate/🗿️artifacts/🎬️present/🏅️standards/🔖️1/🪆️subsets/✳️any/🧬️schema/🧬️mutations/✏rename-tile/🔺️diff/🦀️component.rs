//! 🔺️ Sparse diff construction for `rename-tile`.
use super::mutation::RenameTile;
use crate::artifacts::present::diff::{PresentDiff, PresentTilePatchEntry, PresentTilesDelta};
use crate::artifacts::present::{FigureTileDraftPatch, PresentSnapshot};

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` name-only patch delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &RenameTile, _base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(PresentTilesDelta {
            patched: vec![PresentTilePatchEntry {
                id: payload.id.clone(),
                patch: FigureTileDraftPatch { name: Some(payload.new_name.clone()), crop: None },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
