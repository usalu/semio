//! 🔺️ Sparse diff construction for `resize-source-frame`.
use super::mutation::ResizeSourceFrame;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, swaps in `payload.new_frame`
/// on `source`, and mints a new content-addressed `presentation` handle for the result — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub async fn diff(payload: &ResizeSourceFrame, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (mut source, tiles) = crate::artifacts::present::present_working_scene(base);
    let frame = &payload.new_frame;
    if !frame.x.is_finite() || !frame.y.is_finite() || !frame.width.is_finite() || !frame.height.is_finite() {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Source frame must be finite, got ({}, {}, {}, {}).", frame.x, frame.y, frame.width, frame.height),
            ["source".to_string(), "frame".to_string()],
        );
    }
    if frame.width <= 0.0 || frame.height <= 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Source frame width/height must be positive, got ({}, {}).", frame.width, frame.height),
            ["source".to_string(), "frame".to_string()],
        );
    }
    if source.frame == payload.new_frame {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Source frame is already unchanged.".to_string());
    }
    source.frame = payload.new_frame.clone();
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
