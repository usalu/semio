//! 🧭️ Fem2d transform gumball — config, pivot, meta layer, and the gumball TOOL: a `🔄️machine` statechart whose
//! effects are `ToolYield`s, driven by the `🛠️tool-machine` runner. Every translate/rotate/scale dispatch builds one
//! relative `move-selection` tick leaf; a one-shot dispatch commits it as ONE `ToolTransaction`, a streamed gesture
//! accumulates its ticks into the window's ONE open transaction (persisted in the FEM gumball transient, previewed by
//! every window, never history) and commits the NET leaf once; an abort leaves zero trace
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::fem2d::interaction::{fem2d_entity_kind, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SUPPORT};
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use crate::editor::fem2d::modes::edit::windows::model::{fem2d_element_endpoints, find_node_2d, screen_2d, ORIGIN_2D, SCALE_2D};
use crate::editor::fem2d::transient::FemGumballGesture;
use crate::standards::v1::subsets::any::schema::mutations::move_selection::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{element_id, Fem2dSnapshot, FemLoad};
use machine::Command;
use semio_framework_tool_machine::{GesturePhase, GestureTool, ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolYield};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

//#region 🔖️Config
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fem2dGumballConfig {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

impl Default for Fem2dGumballConfig {
    fn default() -> Self {
        Self { move_axes: true, rotate: true, scale_axes: true, scale_uniform: true }
    }
}

thread_local! {
    static GUMBALL_CONFIG: RefCell<HashMap<String, Fem2dGumballConfig>> = RefCell::new(HashMap::new());
}

pub fn gumball_config_for_window(window_id: &str) -> Fem2dGumballConfig {
    GUMBALL_CONFIG.with(|store| store.borrow().get(window_id).copied().unwrap_or_default())
}

pub fn set_gumball_flag(window_id: &str, flag: &str, pressed: Option<bool>) {
    GUMBALL_CONFIG.with(|store| {
        let mut map = store.borrow_mut();
        let entry = map.entry(window_id.to_string()).or_default();
        let toggle = |current: &mut bool| *current = pressed.unwrap_or(!*current);
        match flag {
            "move" | "moveAxes" => toggle(&mut entry.move_axes),
            "rotate" => toggle(&mut entry.rotate),
            "scaleAxes" => toggle(&mut entry.scale_axes),
            "scaleUniform" => toggle(&mut entry.scale_uniform),
            _ => {}
        }
    });
}
//#endregion 🔖️Config

//#region 🔖️Targets
/// 🎯️ The geometry a selection moves: node ids and region ids, each once, in document order. Elements, supports
/// and loads resolve to the nodes and regions that carry them; the rest carry no geometry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fem2dTransformTargets {
    pub node_ids: Vec<String>,
    pub region_ids: Vec<String>,
}

impl Fem2dTransformTargets {
    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty() && self.region_ids.is_empty()
    }
}

pub fn fem2d_transform_targets(doc: &Fem2dSnapshot, selection_ids: &[String]) -> Fem2dTransformTargets {
    let mut nodes = HashSet::new();
    let mut regions = HashSet::new();
    for id in selection_ids {
        match fem2d_entity_kind(doc, id) {
            Some(FEM2D_GRANULARITY_NODE) => {
                nodes.insert(id.clone());
            }
            Some(FEM2D_GRANULARITY_REGION) => {
                regions.insert(id.clone());
            }
            Some(FEM2D_GRANULARITY_ELEMENT) => {
                if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) {
                    let (start, end) = fem2d_element_endpoints(element);
                    nodes.insert(start.to_string());
                    nodes.insert(end.to_string());
                }
            }
            Some(FEM2D_GRANULARITY_SUPPORT) => {
                if let Some(support) = doc.supports.iter().find(|support| support.id == *id) {
                    nodes.insert(support.node_id.clone());
                }
            }
            Some(FEM2D_GRANULARITY_LOAD) => {
                if let Some((_, load)) = crate::editor::fem2d::interaction::fem2d_load_owner(doc, id) {
                    match load {
                        FemLoad::Nodal { node_id, .. } => {
                            nodes.insert(node_id.clone());
                        }
                        FemLoad::MemberUdl { element_id: target, .. } => {
                            if let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) {
                                let (start, end) = fem2d_element_endpoints(element);
                                nodes.insert(start.to_string());
                                nodes.insert(end.to_string());
                            }
                        }
                        FemLoad::Area { region_id, .. } => {
                            regions.insert(region_id.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Fem2dTransformTargets {
        node_ids: doc.nodes.iter().filter(|node| nodes.contains(&node.id)).map(|node| node.id.clone()).collect(),
        region_ids: doc.regions.iter().filter(|region| regions.contains(&region.id)).map(|region| region.id.clone()).collect(),
    }
}

pub fn fem2d_transform_pivot(doc: &Fem2dSnapshot, targets: &Fem2dTransformTargets) -> Option<(f64, f64)> {
    let mut points = Vec::new();
    for id in &targets.node_ids {
        if let Some(node) = find_node_2d(&doc.nodes, id) {
            points.push((node.x, node.y));
        }
    }
    for id in &targets.region_ids {
        if let Some(region) = doc.regions.iter().find(|region| region.id == *id) {
            for point in &region.outline {
                points.push((point[0], point[1]));
            }
        }
    }
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().fold((0.0, 0.0), |acc, point| (acc.0 + point.0, acc.1 + point.1));
    let count = points.len() as f64;
    Some((sum.0 / count, sum.1 / count))
}
//#endregion 🔖️Targets

//#region 🛠️GumballTool
/// 🪪️ The editor app id every gumball transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const FEM2D_EDITOR_APP_ID: &str = "s.fem.fem2d@1/*#editor";

/// 🔑️ The transaction key of a gesture's leaf — every tick upserts it, so an open transaction holds ONE net leaf.
pub const FEM2D_GUMBALL_LEAF_KEY: &str = "selection:0";

/// 🎬️ How one gumball dispatch moves the selection — the parameters of the tick leaf it builds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fem2dGumballMotion {
    Translate { dx: f64, dy: f64 },
    Rotate { angle: f64 },
    Scale { sx: f64, sy: f64 },
}

/// 🧮️ The relative `move-selection` leaf one dispatch yields on `doc`: the selection's nodes and regions as literal
/// targets, its centroid as the pivot, and the motion; `None` when the selection moves no geometry.
pub fn fem2d_gumball_tick(doc: &Fem2dSnapshot, selection_ids: &[String], motion: Fem2dGumballMotion) -> Option<MoveSelection> {
    let targets = fem2d_transform_targets(doc, selection_ids);
    let (pivot_x, pivot_y) = fem2d_transform_pivot(doc, &targets)?;
    let identity = MoveSelection { node_ids: targets.node_ids, region_ids: targets.region_ids, pivot_x, pivot_y, dx: 0.0, dy: 0.0, angle: 0.0, sx: 1.0, sy: 1.0 };
    Some(match motion {
        Fem2dGumballMotion::Translate { dx, dy } => MoveSelection { dx, dy, ..identity },
        Fem2dGumballMotion::Rotate { angle } => MoveSelection { angle, ..identity },
        Fem2dGumballMotion::Scale { sx, sy } => MoveSelection { sx, sy, ..identity },
    })
}

/// ➕️ `net` followed by `tick` as ONE net leaf: offsets add, angles add, factors multiply — only for the same targets
/// and pivot and the same kind of motion; an identity tick changes nothing; `None` otherwise.
pub fn fem2d_gumball_then(net: &MoveSelection, tick: &MoveSelection) -> Option<MoveSelection> {
    if (&net.node_ids, &net.region_ids, net.pivot_x, net.pivot_y) != (&tick.node_ids, &tick.region_ids, tick.pivot_x, tick.pivot_y) {
        return None;
    }
    let translation = |leaf: &MoveSelection| (leaf.angle, leaf.sx, leaf.sy) == (0.0, 1.0, 1.0);
    let rotation = |leaf: &MoveSelection| (leaf.dx, leaf.dy, leaf.sx, leaf.sy) == (0.0, 0.0, 1.0, 1.0);
    let scaling = |leaf: &MoveSelection| (leaf.dx, leaf.dy, leaf.angle) == (0.0, 0.0, 0.0);
    if tick.is_identity() {
        Some(net.clone())
    } else if translation(net) && translation(tick) {
        Some(MoveSelection { dx: net.dx + tick.dx, dy: net.dy + tick.dy, ..net.clone() })
    } else if rotation(net) && rotation(tick) {
        Some(MoveSelection { angle: net.angle + tick.angle, ..net.clone() })
    } else if scaling(net) && scaling(tick) {
        Some(MoveSelection { sx: net.sx * tick.sx, sy: net.sy * tick.sy, ..net.clone() })
    } else {
        None
    }
}

/// 🎚️ Whether a tick is one the leaf admits and that moves: finite numbers, positive factors, not the identity.
fn fem2d_gumball_moves(tick: &MoveSelection) -> bool {
    [tick.dx, tick.dy, tick.angle, tick.sx, tick.sy].iter().all(|value| value.is_finite()) && tick.sx > 0.0 && tick.sy > 0.0 && !tick.is_identity()
}

/// 📨️ One gumball event: the tick leaf this dispatch built (`None` when the selection moves no geometry).
#[derive(Clone, Debug)]
pub struct Fem2dGumballRequest {
    pub tick: Option<MoveSelection>,
}

/// 🧰️ The gumball tool's context: the net leaf a streamed gesture has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem2dGumballContext {
    pub stream: Option<MoveSelection>,
}

fn gumball_context(input: Fem2dGumballContext) -> Fem2dGumballContext {
    input
}

fn tick_moves(_context: &Fem2dGumballContext, event: Option<&gumball_tool::Event>) -> bool {
    matches!(event, Some(gumball_tool::Event::Records(request) | gumball_tool::Event::Stream(request)) if request.tick.as_ref().is_some_and(fem2d_gumball_moves))
}

fn yield_tick(_context: &mut Fem2dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Records(Fem2dGumballRequest { tick: Some(tick) })) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(FEM2D_GUMBALL_LEAF_KEY, Fem2dMutation::MoveSelection(tick.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut Fem2dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Stream(Fem2dGumballRequest { tick: Some(tick) })) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(FEM2D_GUMBALL_LEAF_KEY, Fem2dMutation::MoveSelection(tick.clone()))));
    context.stream = Some(tick.clone());
}

fn continue_stream(context: &mut Fem2dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let (Some(gumball_tool::Event::Stream(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = request.tick.as_ref().filter(|tick| fem2d_gumball_moves(tick)).and_then(|tick| fem2d_gumball_then(&stream, tick)).unwrap_or(stream);
    sink.push(Command::Effect(ToolYield::upsert(FEM2D_GUMBALL_LEAF_KEY, Fem2dMutation::MoveSelection(net.clone()))));
    context.stream = Some(net);
}

fn finish_stream(context: &mut Fem2dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let (Some(gumball_tool::Event::Finish(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = request.tick.as_ref().filter(|tick| fem2d_gumball_moves(tick)).and_then(|tick| fem2d_gumball_then(&stream, tick)).unwrap_or(stream);
    if net.is_identity() {
        sink.push(Command::Effect(ToolYield::retract(FEM2D_GUMBALL_LEAF_KEY)));
    } else {
        sink.push(Command::Effect(ToolYield::upsert(FEM2D_GUMBALL_LEAF_KEY, Fem2dMutation::MoveSelection(net))));
    }
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut Fem2dGumballContext, _event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    context.stream = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine gumball_tool {
        context: Fem2dGumballContext;
        event Event { Records(Fem2dGumballRequest), Stream(Fem2dGumballRequest), Finish(Fem2dGumballRequest), Cancel }
        input: Fem2dGumballContext;
        output: ();
        effect: ToolYield<Fem2dMutation>;
        context_from_input: gumball_context;
        initial: idle;
        state idle {
            on Records if tick_moves => idle do yield_tick;
            on Stream if tick_moves => streaming do begin_stream;
        }
        state streaming {
            on Stream => streaming do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
        }
    }
}

/// 🧷️ The gumball tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct GumballToolHost;

impl machine::Host<gumball_tool::GumballTool> for GumballToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Fem2dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🛠️ One window's fem2d gumball tool for one dispatch, a `🛠️tool-machine` runner scoped `<appId>#<verb>`.
pub struct Fem2dGumballTool {
    runner: ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl GestureTool for Fem2dGumballTool {
    type Gesture = FemGumballGesture;
    type Tick = MoveSelection;
    type Mutation = Fem2dMutation;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{FEM2D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string().into()), Fem2dGumballContext::default(), GumballToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &FemGumballGesture) -> Result<Self, ToolRefusal> {
        let entries: Vec<(String, Fem2dMutation)> = gesture.entries()?;
        let stream = entries.iter().find_map(|(key, leaf)| match leaf {
            Fem2dMutation::MoveSelection(leaf) if key == FEM2D_GUMBALL_LEAF_KEY => Some(leaf.clone()),
            _ => None,
        });
        let snapshot = gesture.snapshot::<gumball_tool::GumballTool>(Fem2dGumballContext { stream })?;
        let runner = ToolMachineRunner::resume(format!("{FEM2D_EDITOR_APP_ID}#{}", gesture.verb), protocol::ActorId(gesture.authoring_seed.clone().into()), Fem2dGumballContext::default(), snapshot, Some(ToolTransaction::resume(gesture.transaction.clone(), entries)), GumballToolHost)?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }


    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<MoveSelection>) -> Result<ToolStep<Fem2dMutation>, ToolRefusal> {
        let request = Fem2dGumballRequest { tick };
        let event = match phase {
            GesturePhase::Stream => gumball_tool::Event::Stream(request),
            GesturePhase::Commit if !self.runner.at_rest() => gumball_tool::Event::Finish(request),
            GesturePhase::Abort(_) => gumball_tool::Event::Cancel,
            GesturePhase::Once | GesturePhase::Commit => gumball_tool::Event::Records(request),
        };
        self.runner.send(event, semio_framework_tool_machine::authoring_clock(0))
    }

    fn persist(self) -> Option<FemGumballGesture> {
        let (snapshot, transaction) = self.runner.into_parts();
        FemGumballGesture::persist(machine::persist(&snapshot).states, &self.verb, &self.authoring_seed, &self.base_revision, transaction)
    }
}
//#endregion 🛠️GumballTool

//#region 🔖️MetaLayer
pub fn fem2d_gumball_active(active_utility: &str, selection_ids: &[String]) -> bool {
    active_utility == FEM2D_UTILITY_TRANSFORM && !selection_ids.is_empty()
}

/// 🧭️ The `meta:gumball` layer of the Transform utility (`📐️Canvas2dHost/🧬️schema/🔣️gumball-meta`): handles at the
/// selection pivot in layer units, the model→layer map of `screen_2d`, and `liveDispatch` — the hosts stream the gesture
/// so every window previews the open transaction (the results window re-solves while the drag goes on).
pub fn fem2d_gumball_meta_layer(doc: &Fem2dSnapshot, selection_ids: &[String], active_utility: &str, window_id: Option<&str>) -> Option<semio_framework_pack_json::Value> {
    if !fem2d_gumball_active(active_utility, selection_ids) {
        return None;
    }
    let targets = fem2d_transform_targets(doc, selection_ids);
    let pivot = fem2d_transform_pivot(doc, &targets)?;
    let (layer_x, layer_y) = screen_2d(pivot.0, pivot.1);
    let config = window_id.map(gumball_config_for_window).unwrap_or_default();
    Some(semio_framework_pack_json::json!({
        "id": "meta:gumball",
        "role": "meta",
        "gumball": {
            "active": true,
            "liveDispatch": true,
            "pivotLayer": [layer_x, layer_y],
            "modelToLayer": { "scale": [SCALE_2D, -SCALE_2D], "offset": [ORIGIN_2D, ORIGIN_2D] },
            "selectionIds": selection_ids,
            "config": {
                "moveAxes": config.move_axes,
                "rotate": config.rotate,
                "scaleAxes": config.scale_axes,
                "scaleUniform": config.scale_uniform,
            },
        },
    }))
}

//#endregion 🔖️MetaLayer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
