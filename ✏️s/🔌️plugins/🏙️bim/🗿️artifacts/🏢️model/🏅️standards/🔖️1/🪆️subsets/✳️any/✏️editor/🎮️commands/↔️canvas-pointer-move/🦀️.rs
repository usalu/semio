//! ↔️ `canvasPointerMove`: the pointer moved over a plan, section or world window. The armed utility's gesture advances (a rubber band follows, a ghost slides, a marquee grows); nothing is written
//! until the gesture commits, and the marks it shows travel in the window transient, never in the document.

use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::run;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

canvas_pointer! {
    /// ↔️ The pointer position in canvas pixels with the canvas size and the held modifiers.
    CanvasPointerMove, "canvas-pointer-move"
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    run(ctx, doc, |pointer| ToolEvent::Move(*pointer), &payload.raw())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
