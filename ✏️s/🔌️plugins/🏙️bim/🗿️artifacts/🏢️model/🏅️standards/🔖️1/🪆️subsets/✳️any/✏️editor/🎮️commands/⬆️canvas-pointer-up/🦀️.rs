//! ⬆️ `canvasPointerUp`: the press ended. A drag commits (a marquee selects, a handle moves its wall, an opening slides, a rectangle becomes a slab); a release the host cancelled (the pointer
//! left the window, capture was lost) drops the gesture with zero trace.

use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::gestures::{canvas_pointer, run};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

canvas_pointer! {
    /// ⬆️ The release position in canvas pixels with the canvas size, the held modifiers and whether the host cancelled the gesture.
    CanvasPointerUp, "canvas-pointer-up", cancelled: bool
}

pub fn handle(payload: &CanvasPointerUp, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let cancelled = payload.cancelled;
    run(ctx, doc, move |pointer| if cancelled { ToolEvent::Lost } else { ToolEvent::Up(*pointer) }, &payload.raw())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
