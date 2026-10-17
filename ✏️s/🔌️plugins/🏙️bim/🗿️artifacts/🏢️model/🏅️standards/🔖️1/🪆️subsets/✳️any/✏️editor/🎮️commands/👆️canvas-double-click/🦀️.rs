//! 👆️ `canvasDoubleClick`: a double click finishes the gesture in progress where it has an end of its own (a railing, a slab or roof polygon, a wall chain).

use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
use crate::editor::bim::gestures::run;
use crate::editor::bim::dispatch_context::BimDispatchCtx;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
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
