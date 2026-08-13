//! 🔺️ Sparse diff construction for `resize-source-frame`.
use super::mutation::ResizeSourceFrame;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, swaps in `payload.new_frame`
/// on `source`, and mints a new content-addressed `presentation` handle for the result — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ResizeSourceFrame, base: &PresentSnapshot) -> PresentDiff {
    let (mut source, tiles) = crate::artifacts::present::present_working_scene(base);
    source.frame = payload.new_frame.clone();
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
