//! 🔺️ Sparse diff construction for `resize-source-frame`.
use super::ResizeSourceFrame;
use crate::artifacts::presentation::diff::PresentationDiff;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, swaps in `payload.new_frame`
/// on `source`, and mints a new content-addressed `presentation` handle for the result — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ResizeSourceFrame, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let (mut source, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    let frame = &payload.new_frame;
    if !frame.x.is_finite() || !frame.y.is_finite() || !frame.width.is_finite() || !frame.height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Source frame must be finite, got ({}, {}, {}, {}).", frame.x, frame.y, frame.width, frame.height), ["source".to_string(), "frame".to_string()]);
    }
    if frame.width <= 0.0 || frame.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Source frame width/height must be positive, got ({}, {}).", frame.width, frame.height), ["source".to_string(), "frame".to_string()]);
    }
    if source.frame == payload.new_frame {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Source frame is already unchanged.".to_string());
    }
    source.frame = payload.new_frame.clone();
    protocol::MutationOutcome::new(crate::artifacts::presentation::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
