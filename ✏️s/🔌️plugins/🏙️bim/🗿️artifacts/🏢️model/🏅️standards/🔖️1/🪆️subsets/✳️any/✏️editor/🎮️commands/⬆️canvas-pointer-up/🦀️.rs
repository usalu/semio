//! ⬆️ `canvasPointerUp`: the press ended. A drag commits (a marquee selects, a handle moves its wall, an opening slides, a rectangle becomes a slab); a release the host cancelled (the pointer
//! left the window, capture was lost) drops the gesture with zero trace.

use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::run;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
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
