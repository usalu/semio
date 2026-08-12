//! 🔺️ Sparse diff construction for `resize-source-frame`.
use super::mutation::ResizeSourceFrame;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `source` delta directly from the payload (base source with only `frame`
/// swapped in) — real handcrafted construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ResizeSourceFrame, base: &PresentSnapshot) -> PresentDiff {
    let mut source = base.source.clone();
    source.frame = payload.new_frame.clone();
    PresentDiff { source: Some(source), ..Default::default() }
}
//#endregion 🔹Diff
