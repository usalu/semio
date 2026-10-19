//! 👆️ `canvasDoubleClick`: a double click finishes the gesture in progress where it has an end of its own (a railing, a slab or roof polygon, a wall chain).

use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::run;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

canvas_pointer! {
    /// 👆️ The double click position in canvas pixels with the canvas size.
    CanvasDoubleClick, "canvas-double-click"
}

pub fn handle(payload: &CanvasDoubleClick, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    run(ctx, doc, |pointer| ToolEvent::Double(*pointer), &payload.raw())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
