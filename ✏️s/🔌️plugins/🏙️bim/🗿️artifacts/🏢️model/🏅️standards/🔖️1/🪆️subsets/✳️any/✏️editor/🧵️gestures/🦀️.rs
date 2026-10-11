//! 🧵️ BIM authoring gestures. A gesture is a retained tool statechart per window instance: the pointer commands (`canvasPointerDown/Move/Up`, `canvasDoubleClick`, `canvasCommitDraft`,
//! `canvasEscape`, `worldPointerDown/Move`) advance the session of the window they address, and every advance answers one history row of `create-*` / `set-*` / `move-*` mutations
//! (or none), a selection request, and the marks the window shows meanwhile. The marks travel in the window transient (ephemeral, local, never a document operation); the sessions
//! live in the app instance's operation owner, so a gesture survives from one pointer command to the next and dies with its window, its utility or its instance.
//!
//! Layout: `plane` (geometry), `snap` (what a pointer means), `session` (events, context, steps, previews), one node per tool family (`chain`, `area`, `point`, `opening`, `select`,
//! `transform`, `split`), `typed` (the keyboard twin of the pointer: typed points and, through `run_cursor`, the arrow-key cursor), `overlay` (marks to canvas records). This node owns the registry of tools, the owner and `run` and `run_typed`, the entries the
//! pointer and keyboard commands call.

#[path = "📏️annotate/🦀️.rs"]
pub mod annotate;
#[path = "🧩️area/🦀️.rs"]
pub mod area;
#[path = "🧱️chain/🦀️.rs"]
pub mod chain;
#[path = "🏗️frame/🦀️.rs"]
pub mod frame;
#[path = "🦴️structure/🦀️.rs"]
pub mod structure;
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
#[path = "🔪️split/🦀️.rs"]
pub mod split;
#[path = "🧷️sweep/🦀️.rs"]
pub mod sweep;
#[path = "🚚️transform/🦀️.rs"]
pub mod transform;
#[path = "👯️duplicate/🦀️.rs"]
pub mod duplicate;
#[path = "✂️reshape/🦀️.rs"]
pub mod reshape;
#[path = "⌨️typed/🦀️.rs"]
pub mod typed;
#[path = "🪑️place/🦀️.rs"]
pub mod place;
#[path = "🌀️route/🦀️.rs"]
pub mod route;
#[path = "🖼️viewports/🦀️.rs"]
pub mod viewports;

use self::plane::P;
use self::session::{GestureKey, Modifiers, Pointer, Preview, Step, Surface, Tool, ToolContext, ToolEvent};
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::modes::edit::windows::{plan, section, sheet, world};
use crate::editor::bim::transient::BimWindowTransient;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactInstanceOperationOwner, ArtifactView, Effect, Emit, Fault, NoConfigMutation, PluginLifecycleStep};
use std::collections::BTreeMap;

//#region 🔖️Registry
/// 🖱️ The commands that carry a pointer, a key or a typed line to the gesture of the addressed window.
pub const POINTER_COMMAND_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape", "worldPointerDown", "worldPointerMove", "engagementInput", "engagementSubmit", "cursorLeft", "cursorRight", "cursorUp", "cursorDown", "cursorLeftFar", "cursorRightFar", "cursorUpFar", "cursorDownFar", "cursorPlace"];

/// ⌨️ The keys that drive a gesture in progress: Escape cancels it, Enter finishes it, the arrow keys with the command key move the keyboard cursor by a fine step and with shift by a coarse one, and Enter with the command key clicks at the cursor.
pub const GESTURE_KEYBINDINGS: &[(&str, &str)] = &[("escape", "canvasEscape"), ("enter", "canvasCommitDraft"), ("mod+arrowleft", "cursorLeft"), ("mod+arrowright", "cursorRight"), ("mod+arrowup", "cursorUp"), ("mod+arrowdown", "cursorDown"), ("mod+shift+arrowleft", "cursorLeftFar"), ("mod+shift+arrowright", "cursorRightFar"), ("mod+shift+arrowup", "cursorUpFar"), ("mod+shift+arrowdown", "cursorDownFar"), ("mod+enter", "cursorPlace")];

/// 🛠️ Every utility that owns a gesture, with the tool it arms.
pub fn build_tool(utility: &str, window_kind: &str) -> Box<dyn Tool> {
    use opening::Kind as Opening;
    match utility {
        "support" => Box::new(structure::Structure::new("support")),
        "load-point" => Box::new(structure::Structure::new("load-point")),
        "load-line" => Box::new(structure::Structure::new("load-line")),
        "load-area" => Box::new(structure::Structure::new("load-area")),
        "select" if window_kind == sheet::WINDOW_KIND_ID => Box::<viewports::Arrange>::default(),
        "viewport" => Box::<viewports::Place>::default(),
        "select" if window_kind != world::WINDOW_KIND_ID => Box::new(select::Select::default()),
        "move" => Box::new(transform::Transform::new(transform::Kind::Move)),
        "rotate" => Box::new(transform::Transform::new(transform::Kind::Rotate)),
        "wall" => Box::new(chain::Chain::new(chain::Kind::Wall)),
        "wall-arc" => Box::new(chain::Chain::new(chain::Kind::WallArc)),
        "curtain-wall" => Box::new(chain::Chain::new(chain::Kind::CurtainWall)),
        "beam" => Box::new(chain::Chain::new(chain::Kind::Beam)),
        "beam-arc" => Box::new(chain::Chain::new(chain::Kind::BeamArc)),
        "column-tilt" => Box::<frame::Tilt>::default(),
        "curtain-grid" => Box::<frame::Lines>::default(),
        "curtain-cell" => Box::<frame::Cell>::default(),
        "railing" => Box::new(chain::Chain::new(chain::Kind::Railing)),
        "ramp" => Box::new(chain::Chain::new(chain::Kind::Ramp)),
        "grid" => Box::new(chain::Chain::new(chain::Kind::Grid)),
        "measure" => Box::new(chain::Chain::new(chain::Kind::Measure)),
        "slab" => Box::new(area::Area::new(area::Kind::Slab)),
        "slab-walls" => Box::new(area::Area::new(area::Kind::SlabFromWalls)),
        "ceiling" => Box::new(area::Area::new(area::Kind::Ceiling)),
        "ceiling-space" => Box::new(area::Area::new(area::Kind::CeilingFromSpace)),
        "split-wall" => Box::<split::Split>::default(),
        "sweep" => Box::<sweep::Sweeping>::default(),
        "copy" => Box::new(duplicate::Duplicate::new(duplicate::Kind::Copy)),
        "mirror" => Box::new(duplicate::Duplicate::new(duplicate::Kind::Mirror)),
        "array" => Box::new(duplicate::Duplicate::new(duplicate::Kind::Array)),
        "array-radial" => Box::new(duplicate::Duplicate::new(duplicate::Kind::Radial)),
        "offset" => Box::new(reshape::Reshape::new(reshape::Kind::Offset)),
        "trim" => Box::new(reshape::Reshape::new(reshape::Kind::Trim)),
        "extend" => Box::new(reshape::Reshape::new(reshape::Kind::Extend)),
        "align" => Box::new(reshape::Reshape::new(reshape::Kind::Align)),
        "split" => Box::new(reshape::Reshape::new(reshape::Kind::Split)),
        "dimension" => Box::new(annotate::Annotate::new(annotate::Kind::Dimension)),
        "tag" => Box::new(annotate::Annotate::new(annotate::Kind::Tag)),
        "text-note" => Box::new(annotate::Annotate::new(annotate::Kind::Note)),
        "leader" => Box::new(annotate::Annotate::new(annotate::Kind::Leader)),
        "roof" => Box::new(area::Area::new(area::Kind::Roof)),
        "component" => Box::<place::Place>::default(),
        "route" => Box::<route::Route>::default(),
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
    last: Option<P>,
}

impl ToolSession {
    pub fn new(utility: &str, window_kind: &str) -> Self {
        Self { utility: utility.to_string(), tool: build_tool(utility, window_kind), last: None }
    }

    pub fn utility(&self) -> &str {
        &self.utility
    }

    /// ➡️ Advances the statechart by one event and projects the marks it shows afterwards.
    pub fn advance(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> (Step, Preview) {
        if let ToolEvent::Down(pointer) | ToolEvent::Move(pointer) | ToolEvent::Up(pointer) | ToolEvent::Double(pointer) = event {
            self.last = Some(pointer.at);
        }
        let step = self.tool.event(ctx, event);
        (step, self.tool.preview(ctx))
    }

    /// ⌨️ Feeds one typed line to the tool as a click at the point it names: a press, the release and a move to the same exact point, in one step. An empty line finishes the gesture, as
    /// Enter does; a line that names no point is refused and leaves the gesture untouched.
    pub fn enter(&mut self, ctx: &mut ToolContext<'_>, line: &str) -> (Step, Preview) {
        if let Some(step) = self.tool.line(ctx, line) {
            return (step, self.tool.preview(ctx));
        }
        let Some(entry) = typed::parse(line) else { return (Step::refuse(typed::INVALID), self.tool.preview(ctx)) };
        let Some(at) = typed::resolve(&entry, self.tool.anchor(), self.last) else { return self.advance(ctx, &ToolEvent::Finish) };
        self.click(ctx, at)
    }

    /// 🔑️ Offers a key to the tool; the flag says whether the tool took it (a tool that has no use for the key leaves it to its other meaning).
    pub fn key(&mut self, ctx: &mut ToolContext<'_>, key: GestureKey) -> (bool, Step, Preview) {
        let step = self.tool.key(ctx, key);
        (step.is_some(), step.unwrap_or_default(), self.tool.preview(ctx))
    }

    fn click(&mut self, ctx: &mut ToolContext<'_>, at: P) -> (Step, Preview) {
        let pointer = Pointer { at, modifiers: Modifiers::default(), tolerance: typed::REACH };
        let mut step = Step::default();
        for event in [ToolEvent::Down(pointer), ToolEvent::Up(pointer), ToolEvent::Move(pointer)] {
            step = step.then(self.advance(ctx, &event).0);
            if step.refused.is_some() {
                break;
            }
        }
        (step, self.tool.preview(ctx))
    }

    /// ⌨️ Where the keyboard cursor stands: where the pointer last was, else the point the gesture hangs on, else `start`.
    pub fn cursor(&self, start: P) -> P {
        self.last.or_else(|| self.tool.anchor()).unwrap_or(start)
    }

    /// ⌨️ Moves the keyboard cursor by `delta` metres from where the pointer last was (else the point the gesture hangs on, else `start`) and shows the tool's marks there, as a pointer move to the exact point would.
    pub fn nudge(&mut self, ctx: &mut ToolContext<'_>, delta: P, start: P) -> (Step, Preview) {
        let at = typed::resolve(&typed::Entry::Relative(delta), Some(self.cursor(start)), None).unwrap_or(start);
        self.advance(ctx, &ToolEvent::Move(Pointer { at, modifiers: Modifiers::default(), tolerance: typed::REACH }))
    }

    /// ⌨️ Clicks at the keyboard cursor, as the typed point `x, y` would.
    pub fn place(&mut self, ctx: &mut ToolContext<'_>, start: P) -> (Step, Preview) {
        let at = self.cursor(start);
        self.click(ctx, at)
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
        self.session(window, window_kind, utility).advance(ctx, event)
    }

    /// ⌨️ Feeds the typed `line` to the session of `window` for `utility`, started afresh when it was started for another utility.
    pub fn enter(&mut self, window: &str, window_kind: &str, utility: &str, ctx: &mut ToolContext<'_>, line: &str) -> (Step, Preview) {
        self.session(window, window_kind, utility).enter(ctx, line)
    }

    /// ⌨️ Moves the keyboard cursor of the session of `window` by `delta`, or clicks at it when there is no delta.
    #[allow(clippy::too_many_arguments)]
    pub fn cursor(&mut self, window: &str, window_kind: &str, utility: &str, ctx: &mut ToolContext<'_>, delta: Option<P>, start: P) -> (Step, Preview) {
        let session = self.session(window, window_kind, utility);
        match delta {
            Some(delta) => session.nudge(ctx, delta, start),
            None => session.place(ctx, start),
        }
    }

    /// 🔑️ Offers `key` to the session of `window` for `utility`; the flag says whether the tool took it. A window without a session of that utility has nothing to take it.
    pub fn key(&mut self, window: &str, utility: &str, ctx: &mut ToolContext<'_>, key: GestureKey) -> (bool, Step, Preview) {
        match self.sessions.get_mut(window).filter(|session| session.utility() == utility) {
            Some(session) => session.key(ctx, key),
            None => (false, Step::default(), Preview::default()),
        }
    }

    fn session(&mut self, window: &str, window_kind: &str, utility: &str) -> &mut ToolSession {
        if self.sessions.get(window).is_none_or(|session| session.utility() != utility) {
            self.sessions.insert(window.to_string(), ToolSession::new(utility, window_kind));
        }
        self.sessions.entry(window.to_string()).or_insert_with(|| ToolSession::new(utility, window_kind))
    }

    pub fn windows(&self) -> Vec<String> {
        self.sessions.keys().cloned().collect()
    }
}

impl ArtifactInstanceOperationOwner for GestureOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand::default())
    }

    fn maintenance_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn close_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        self.sessions.clear();
        Ok(PluginLifecycleStep::Complete(Default::default()))
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

/// 🧊️ The storey the world window draws and picks on: the isolated one when it exists, else the lowest.
pub fn world_storey(snapshot: &ModelSnapshot, isolated: &str) -> String {
    Some(isolated.to_string()).filter(|storey| snapshot.storeys.contains_key(storey)).or_else(|| lowest_storey(snapshot)).unwrap_or_default()
}

/// 🪟️ The surface the addressed window shows and the pixels-to-metres reach there; none for a window without gestures.
pub fn surface_of(ctx: &BimDispatchCtx, snapshot: &ModelSnapshot) -> Option<(Surface, f64)> {
    match ctx.window_kind.as_str() {
        plan::WINDOW_KIND_ID => Some((Surface::Plan { storey: plan::active_storey(snapshot, &ctx.plan).unwrap_or_default() }, 1.0 / ctx.plan.viewport.zoom.max(0.01))),
        section::WINDOW_KIND_ID => {
            let (start, end) = section::plane_of(snapshot, &ctx.section).map_or(([0.0, 0.0], [0.0, 0.0]), |plane| ([plane.start.x, plane.start.y], [plane.end.x, plane.end.y]));
            Some((Surface::Section { start, end }, 1.0 / ctx.section.viewport.zoom.max(0.01)))
        }
        world::WINDOW_KIND_ID => Some((Surface::World { storey: world_storey(snapshot, &ctx.world.isolated_storey) }, WORLD_PIXEL_METRES)),
        sheet::WINDOW_KIND_ID => Some((Surface::Sheet { sheet: sheet::active_sheet(snapshot, &ctx.sheet).unwrap_or_default() }, 1.0 / ctx.sheet.viewport.zoom.max(0.01))),
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
        Surface::Sheet { .. } => {
            let zoom = ctx.sheet.viewport.zoom.max(0.01);
            [(raw.x - raw.width * 0.5) / zoom + ctx.sheet.viewport.x, (raw.y - raw.height * 0.5) / zoom + ctx.sheet.viewport.y]
        }
    };
    Pointer { at, modifiers: raw.modifiers, tolerance: reach }
}
//#endregion 🔖️Surface

/// 🖱️ Declares the payload of a canvas pointer command: the pixel position, the canvas size, the modifiers and any extra fields, with `raw()` for the gesture entry.
macro_rules! canvas_pointer {
    ($(#[$meta:meta])* $name:ident, $keyword:literal $(, $extra:ident : $ty:ty)*) => {
        $(#[$meta])*
        #[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

/// ➡️ What drives one step of a gesture: a window event, or a typed line.
enum Feed<'a> {
    Event(ToolEvent),
    Line(&'a str),
    Cursor { delta: Option<P>, start: P },
    Key(GestureKey),
}

/// ➡️ Advances the addressed window's gesture by `event` and answers the emit; the new preview is left in `ctx.transient_out` when it differs from the window's current one.
pub fn run(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, event: impl FnOnce(&Pointer) -> ToolEvent, raw: &Raw) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if ctx.gestures.is_none() {
        return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route"));
    }
    let Some((surface, reach)) = surface_of(ctx, doc.snapshot) else { return Ok(Emit::default()) };
    let pointer = pointer_of(ctx, &surface, reach, raw);
    settle(ctx, doc, surface, Feed::Event(event(&pointer)))
}

/// ⌨️ Advances the addressed window's gesture by the typed `line` (a point, an offset, a length or nothing to finish) and answers the emit like [`run`].
pub fn run_typed(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, line: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if ctx.gestures.is_none() {
        return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route"));
    }
    let Some((surface, _)) = surface_of(ctx, doc.snapshot) else { return Ok(Emit::default()) };
    settle(ctx, doc, surface, Feed::Line(line))
}

fn settle(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, surface: Surface, feed: Feed<'_>) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    settle_taken(ctx, doc, surface, feed).map(|(emit, _)| emit)
}

fn settle_taken(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, surface: Surface, feed: Feed<'_>) -> Result<(Emit<ModelMutation, NoConfigMutation>, bool), Fault> {
    let Some(owner) = ctx.gestures.clone() else { return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route")) };
    let snapshot = doc.snapshot;
    let operation = doc.operation_optional();
    let window = ctx.view.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
    let seed = operation.map_or("", |operation| operation.authoring_seed.as_str());
    let labels = ctx.labels();
    let instance = Some(&owner);
    let (taken, step, preview) = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(instance, snapshot, |inference| {
        let mut tool = ToolContext::new(snapshot, inference, surface, seed);
        tool.instance = instance;
        tool.selected = &ctx.selected;
        tool.library = &ctx.library_selected;
        tool.labels = labels;
        owner.with_mut::<GestureOwner, _>(|owner| {
            Ok(match &feed {
                Feed::Event(event) => with_taken(owner.advance(&window, &ctx.window_kind, &ctx.utility, &mut tool, event)),
                Feed::Line(line) => with_taken(owner.enter(&window, &ctx.window_kind, &ctx.utility, &mut tool, line)),
                Feed::Cursor { delta, start } => with_taken(owner.cursor(&window, &ctx.window_kind, &ctx.utility, &mut tool, *delta, *start)),
                Feed::Key(key) => owner.key(&window, &ctx.utility, &mut tool, *key),
            })
        })
    })?;
    let text = preview.to_text();
    let addressed = ctx.view.as_ref().is_some_and(|view| view.window_id.is_some());
    if taken && addressed && text != ctx.window_transient.preview {
        ctx.transient_out = Some(BimWindowTransient { preview: text, pointer_generation: ctx.window_transient.pointer_generation + 1, ..ctx.window_transient.clone() });
    }
    emit_of(step, ctx).map(|emit| (emit, taken))
}

fn with_taken((step, preview): (Step, Preview)) -> (bool, Step, Preview) {
    (true, step, preview)
}

/// 🔑️ Offers `key` to the gesture of the addressed window and answers the emit of what the tool did, or `None` when the tool took no use of the key, so the key keeps its other meaning.
pub fn run_key(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, key: GestureKey) -> Result<Option<Emit<ModelMutation, NoConfigMutation>>, Fault> {
    if ctx.gestures.is_none() {
        return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route"));
    }
    let Some((surface, _)) = surface_of(ctx, doc.snapshot) else { return Ok(None) };
    settle_taken(ctx, doc, surface, Feed::Key(key)).map(|(emit, taken)| taken.then_some(emit))
}

/// ⌨️ The point a keyboard cursor starts from when the pointer never was in the window and the gesture hangs on nothing: the centre of the view (the plan and the section centre their camera, the world has no plan point).
pub fn cursor_start(ctx: &BimDispatchCtx) -> P {
    match ctx.window_kind.as_str() {
        plan::WINDOW_KIND_ID => [ctx.plan.viewport.x, -ctx.plan.viewport.y],
        section::WINDOW_KIND_ID => [ctx.section.viewport.x, -ctx.section.viewport.y],
        sheet::WINDOW_KIND_ID => [ctx.sheet.viewport.x, ctx.sheet.viewport.y],
        _ => [0.0, 0.0],
    }
}

/// ⌨️ Moves the keyboard cursor of the addressed window by `delta` metres, or clicks at it when `delta` is none, and answers the emit like [`run`]: the arrow keys move the cursor, the place key clicks, so every placement tool can be driven without a pointer or a typed line.
pub fn run_cursor(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, delta: Option<P>) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if ctx.gestures.is_none() {
        return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route"));
    }
    let Some((surface, _)) = surface_of(ctx, doc.snapshot) else { return Ok(Emit::default()) };
    let start = cursor_start(ctx);
    settle(ctx, doc, surface, Feed::Cursor { delta, start })
}

/// ⌨️ Keeps `line` as the entry typed into the addressed window: the window transient holds it for the field, the presence shares it with the other authors. Writes only what changes.
pub fn keep_entry(ctx: &mut BimDispatchCtx, line: &str) {
    if ctx.view.as_ref().is_none_or(|view| view.window_id.is_none()) {
        return;
    }
    let mut transient = ctx.transient_out.take().unwrap_or_else(|| ctx.window_transient.clone());
    transient.engagement_input = line.to_string();
    if transient != ctx.window_transient {
        ctx.transient_out = Some(transient);
    }
    if ctx.presence.engagement_input != line {
        ctx.presence_out.push(ctx.presence.typing(line));
    }
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
