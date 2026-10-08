//! 🧵️ BIM authoring gestures. A gesture is a retained tool statechart per window instance: the pointer commands (`canvasPointerDown/Move/Up`, `canvasDoubleClick`, `canvasCommitDraft`,
//! `canvasEscape`, `worldPointerDown/Move`) advance the session of the window they address, and every advance answers one history row of `create-*` / `set-*` / `move-*` mutations
//! (or none), a selection request, and the marks the window shows meanwhile. The marks travel in the window transient (ephemeral, local, never a document operation); the sessions
//! live in the app instance's operation owner, so a gesture survives from one pointer command to the next and dies with its window, its utility or its instance.
//!
//! Layout: `plane` (geometry), `snap` (what a pointer means), `session` (events, context, steps, previews), one node per tool family (`chain`, `area`, `point`, `opening`, `select`,
//! `transform`), `overlay` (marks to canvas records). This node owns the registry of tools, the owner and `run`, the one entry the pointer commands call.

#[path = "🧩️area/🦀️.rs"]
pub mod area;
#[path = "🧱️chain/🦀️.rs"]
pub mod chain;
#[path = "🪟️opening/🦀️.rs"]
pub mod opening;
#[path = "🫧️overlay/🦀️.rs"]
pub mod overlay;
#[path = "📐️plane/🦀️.rs"]
pub mod plane;
#[path = "📍️point/🦀️.rs"]
pub mod point;
#[path = "🎯️select/🦀️.rs"]
pub mod select;
#[path = "🧭️session/🦀️.rs"]
pub mod session;
#[path = "🧲️snap/🦀️.rs"]
pub mod snap;
#[path = "🚚️transform/🦀️.rs"]
pub mod transform;

use self::session::{Modifiers, Pointer, Preview, Step, Surface, Tool, ToolContext, ToolEvent};
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::modes::edit::windows::{plan, section, world};
use crate::editor::bim::transient::BimWindowTransient;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactInstanceOperationOwner, ArtifactView, Effect, Emit, Fault, NoConfigMutation, PluginCloseStep};
use std::collections::BTreeMap;

//#region 🔖️Registry
/// 🖱️ The commands that carry a pointer or a key to the gesture of the addressed window.
pub const POINTER_COMMAND_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape", "worldPointerDown", "worldPointerMove"];

/// ⌨️ The keys that drive a gesture in progress: Escape cancels it, Enter finishes it.
pub const GESTURE_KEYBINDINGS: &[(&str, &str)] = &[("escape", "canvasEscape"), ("enter", "canvasCommitDraft")];

/// 🛠️ Every utility that owns a gesture, with the tool it arms.
pub fn build_tool(utility: &str, window_kind: &str) -> Box<dyn Tool> {
    use opening::Kind as Opening;
    match utility {
        "select" if window_kind != world::WINDOW_KIND_ID => Box::new(select::Select::default()),
        "move" => Box::new(transform::Transform::new(transform::Kind::Move)),
        "rotate" => Box::new(transform::Transform::new(transform::Kind::Rotate)),
        "wall" => Box::new(chain::Chain::new(chain::Kind::Wall)),
        "wall-arc" => Box::new(chain::Chain::new(chain::Kind::WallArc)),
        "curtain-wall" => Box::new(chain::Chain::new(chain::Kind::CurtainWall)),
        "beam" => Box::new(chain::Chain::new(chain::Kind::Beam)),
        "railing" => Box::new(chain::Chain::new(chain::Kind::Railing)),
        "grid" => Box::new(chain::Chain::new(chain::Kind::Grid)),
        "measure" => Box::new(chain::Chain::new(chain::Kind::Measure)),
        "slab" => Box::new(area::Area::new(area::Kind::Slab)),
        "slab-walls" => Box::new(area::Area::new(area::Kind::SlabFromWalls)),
        "roof" => Box::new(area::Area::new(area::Kind::Roof)),
        "column" => Box::new(point::Point::new(point::Kind::Column)),
        "space" => Box::new(point::Point::new(point::Kind::Space)),
        "stair" => Box::new(point::Point::new(point::Kind::Stair)),
        "window" => Box::new(opening::Placing::new(Opening::Window)),
        "door" => Box::new(opening::Placing::new(Opening::Door)),
        "opening" => Box::new(opening::Placing::new(Opening::Void)),
        _ => Box::new(session::Idle),
    }
}
//#endregion 🔖️Registry

//#region 🔖️Owner
/// 🧭️ The gesture of one window: the utility it was started for and its statechart.
pub struct ToolSession {
    utility: String,
    tool: Box<dyn Tool>,
}

impl ToolSession {
    pub fn new(utility: &str, window_kind: &str) -> Self {
        Self { utility: utility.to_string(), tool: build_tool(utility, window_kind) }
    }

    pub fn utility(&self) -> &str {
        &self.utility
    }

    /// ➡️ Advances the statechart by one event and projects the marks it shows afterwards.
    pub fn advance(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> (Step, Preview) {
        let step = self.tool.event(ctx, event);
        (step, self.tool.preview(ctx))
    }
}

/// 🧳️ The sessions of every window of one app instance, retained between pointer commands and closed with the instance.
#[derive(Default)]
pub struct GestureOwner {
    sessions: BTreeMap<String, ToolSession>,
}

impl GestureOwner {
    /// ➡️ Advances the session of `window` for `utility`; a session started for another utility is dropped first (a utility switch cancels the gesture in progress).
    pub fn advance(&mut self, window: &str, window_kind: &str, utility: &str, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> (Step, Preview) {
        if self.sessions.get(window).is_none_or(|session| session.utility() != utility) {
            self.sessions.insert(window.to_string(), ToolSession::new(utility, window_kind));
        }
        match self.sessions.get_mut(window) {
            Some(session) => session.advance(ctx, event),
            None => (Step::default(), Preview::default()),
        }
    }

    pub fn windows(&self) -> Vec<String> {
        self.sessions.keys().cloned().collect()
    }
}

impl ArtifactInstanceOperationOwner for GestureOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(PluginCloseStep::Complete)
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.sessions.clear();
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
}
//#endregion 🔖️Owner

//#region 🔖️Surface
/// 🖱️ The raw pointer a window command carries: canvas pixels with the canvas size, the held modifiers and, in the world, the ground point the host ray-cast.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Raw {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub modifiers: Modifiers,
    pub ground: Option<[f64; 2]>,
}

/// 📐️ The metres one pixel spans in the world window, whose ray-cast gives no zoom.
const WORLD_PIXEL_METRES: f64 = 0.02;

fn lowest_storey(snapshot: &ModelSnapshot) -> Option<String> {
    snapshot.buildings.keys().find_map(|building| crate::editor::bim::entities::ordered_storeys(snapshot, building).into_iter().next())
}

/// 🪟️ The surface the addressed window shows and the pixels-to-metres reach there; none for a window without gestures.
pub fn surface_of(ctx: &BimDispatchCtx, snapshot: &ModelSnapshot) -> Option<(Surface, f64)> {
    match ctx.window_kind.as_str() {
        plan::WINDOW_KIND_ID => Some((Surface::Plan { storey: plan::active_storey(snapshot, &ctx.plan).unwrap_or_default() }, 1.0 / ctx.plan.viewport.zoom.max(0.01))),
        section::WINDOW_KIND_ID => Some((Surface::Section { start: [ctx.section.start_x, ctx.section.start_y], end: [ctx.section.end_x, ctx.section.end_y] }, 1.0 / ctx.section.viewport.zoom.max(0.01))),
        world::WINDOW_KIND_ID => {
            let isolated = Some(ctx.world.isolated_storey.clone()).filter(|storey| snapshot.storeys.contains_key(storey));
            Some((Surface::World { storey: isolated.or_else(|| lowest_storey(snapshot)).unwrap_or_default() }, WORLD_PIXEL_METRES))
        }
        _ => None,
    }
}

/// 🖱️ The pointer in the surface's own coordinates: plan metres, section `(u, v)` or the world ground point.
pub fn pointer_of(ctx: &BimDispatchCtx, surface: &Surface, reach: f64, raw: &Raw) -> Pointer {
    let at = match surface {
        Surface::Plan { .. } => {
            let (x, y) = plan::pixel_to_model(&ctx.plan.viewport, raw.x, raw.y, raw.width, raw.height);
            [x, y]
        }
        Surface::Section { .. } => {
            let zoom = ctx.section.viewport.zoom.max(0.01);
            [(raw.x - raw.width * 0.5) / zoom + ctx.section.viewport.x, -((raw.y - raw.height * 0.5) / zoom + ctx.section.viewport.y)]
        }
        Surface::World { .. } => raw.ground.unwrap_or([0.0, 0.0]),
    };
    Pointer { at, modifiers: raw.modifiers, tolerance: reach }
}
//#endregion 🔖️Surface

/// 🖱️ Declares the payload of a canvas pointer command: the pixel position, the canvas size, the modifiers and any extra fields, with `raw()` for the gesture entry.
macro_rules! canvas_pointer {
    ($(#[$meta:meta])* $name:ident, $keyword:literal $(, $extra:ident : $ty:ty)*) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[value(default)]
        #[dsl(keyword = $keyword)]
        pub struct $name {
            pub x: f64,
            pub y: f64,
            pub width: f64,
            pub height: f64,
            pub shift: bool,
            pub ctrl: bool,
            pub meta: bool,
            $(pub $extra: $ty,)*
        }

        impl $name {
            pub fn raw(&self) -> $crate::editor::bim::gestures::Raw {
                $crate::editor::bim::gestures::Raw { x: self.x, y: self.y, width: self.width, height: self.height, modifiers: $crate::editor::bim::gestures::session::Modifiers { shift: self.shift, ctrl: self.ctrl, meta: self.meta }, ground: None }
            }
        }
    };
}

pub(crate) use canvas_pointer;

//#region 🔖️Run
/// 🚫️ The refusal of a gesture command dispatched outside its retained route (no session owner).
pub const RETAINED_ROUTE: &str = "bim.gesture.retained-route";

/// 🧱️ The emit of a step: its mutations as one history row, its selection request and the utility it arms; a refused step is the command's fault.
pub fn emit_of(step: Step, ctx: &BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if let Some(code) = step.refused {
        return Err(fault(code, "the authoring tool could not write what the pointer meant"));
    }
    let mut emit = if step.mutations.is_empty() { Emit::default() } else { Emit::mutations(step.mutations) };
    if let Some(pick) = step.pick {
        emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &pick.targets, pick.merge));
    }
    if let (Some(utility), Some(window)) = (step.arm, ctx.view.as_ref().and_then(|view| view.window_id.clone())) {
        emit.effects.push(Effect::SetActiveUtility { window_id: window, utility_id: utility });
    }
    Ok(emit)
}

/// ➡️ Advances the addressed window's gesture by `event` and answers the emit; the new preview is left in `ctx.transient_out` when it differs from the window's current one.
pub fn run(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, event: impl FnOnce(&Pointer) -> ToolEvent, raw: &Raw) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let Some(owner) = ctx.gestures.clone() else { return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route")) };
    let snapshot = doc.snapshot;
    let Some((surface, reach)) = surface_of(ctx, snapshot) else { return Ok(Emit::default()) };
    let pointer = pointer_of(ctx, &surface, reach, raw);
    let event = event(&pointer);
    let operation = doc.operation_optional();
    let window = ctx.view.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
    let seed = operation.map_or("", |operation| operation.authoring_seed.as_str());
    let labels = ctx.labels();
    let (step, preview) = crate::editor::bim::inference::with_inference(operation.map(|operation| operation.app_instance_id), snapshot, |inference| {
        let mut tool = ToolContext::new(snapshot, inference, surface, seed);
        tool.selected = &ctx.selected;
        tool.library = &ctx.library_selected;
        tool.labels = labels;
        owner.with_mut::<GestureOwner, _>(|owner| Ok(owner.advance(&window, &ctx.window_kind, &ctx.utility, &mut tool, &event)))
    })?;
    let text = preview.to_text();
    let addressed = ctx.view.as_ref().is_some_and(|view| view.window_id.is_some());
    if addressed && text != ctx.window_transient.preview {
        ctx.transient_out = Some(BimWindowTransient { preview: text, pointer_generation: ctx.window_transient.pointer_generation + 1, ..ctx.window_transient.clone() });
    }
    emit_of(step, ctx)
}

/// ➡️ The event a cancelled or completed gesture command carries without a pointer.
pub fn run_keyed(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, event: ToolEvent) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    run(ctx, doc, |_| event, &Raw::default())
}
//#endregion 🔖️Run

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧷️app/🦀️.rs"]
mod app_tests;
