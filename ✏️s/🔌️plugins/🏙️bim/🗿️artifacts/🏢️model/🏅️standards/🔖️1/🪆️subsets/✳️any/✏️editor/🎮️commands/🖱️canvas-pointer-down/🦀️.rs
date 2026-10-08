//! 🖱️ `canvasPointerDown`: a press in a plan window while the select utility is armed picks the topmost element of the plan under the pointer and selects it in the framework
//! `elements` domain (modifiers merge as the framework does: shift adds, ctrl or meta subtracts, both invert). Any other armed utility owns the press itself (the tool wave's
//! gesture jobs), so this command then does nothing.

use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::gestures::canvas_pointer;
use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::select_effect;
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::utilities::DEFAULT_UTILITY;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

/// 🎯️ The pick radius around a press, in canvas pixels.
const PICK_TOLERANCE_PIXELS: f64 = 6.0;

canvas_pointer! {
    /// 🖱️ The press position in canvas pixels with the canvas size and the held modifiers.
    CanvasPointerDown, "canvas-pointer-down"
}

/// 🔀️ The framework merge mode of the held modifiers.
pub fn merge_mode(shift: bool, ctrl: bool, meta: bool) -> &'static str {
    crate::editor::bim::gestures::session::Modifiers { shift, ctrl, meta }.merge()
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if ctx.gestures.is_some() {
        return crate::editor::bim::gestures::run(ctx, doc, |pointer| ToolEvent::Down(*pointer), &payload.raw());
    }
    if ctx.window_kind != plan::WINDOW_KIND_ID || ctx.utility != DEFAULT_UTILITY {
        return Ok(Emit::default());
    }
    let snapshot = doc.snapshot;
    let storey = plan::active_storey(snapshot, &ctx.plan);
    let at = plan::pixel_to_model(&ctx.plan.viewport, payload.x, payload.y, payload.width, payload.height);
    let tolerance = PICK_TOLERANCE_PIXELS / ctx.plan.viewport.zoom.max(0.01);
    let instance = doc.operation_optional().map(|operation| operation.app_instance_id);
    let hit = storey.and_then(|storey| crate::editor::bim::inference::with_inference(instance, snapshot, |inference| inference.plan_linework.get(&storey).and_then(|linework| plan::pick(linework, at, tolerance))));
    let merge = merge_mode(payload.shift, payload.ctrl, payload.meta);
    let mut emit = Emit::default();
    match hit.and_then(|id| kind_holding(snapshot, &id).map(|row| (row.kind.to_string(), id))) {
        Some(target) => emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &[target], merge)),
        None if merge == "replace" => emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &[], "replace")),
        None => {}
    }
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
