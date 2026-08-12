//! 🔺️ Sparse diff construction for `resize-tile-crop`.
use super::mutation::ResizeTileCrop;
use crate::artifacts::present::diff::{PresentDiff, PresentTilePatchEntry, PresentTilesDelta};
use crate::artifacts::present::{FigureTileDraftPatch, PresentSnapshot};

//#region 🔹Diff
/// 🔺️ Builds the sparse `tiles` crop-only patch delta directly from the payload — real
/// handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ResizeTileCrop, _base: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        tiles: Some(PresentTilesDelta {
            patched: vec![PresentTilePatchEntry {
                id: payload.id.clone(),
                patch: FigureTileDraftPatch { name: None, crop: Some(payload.new_crop.clone()) },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔹Diff
