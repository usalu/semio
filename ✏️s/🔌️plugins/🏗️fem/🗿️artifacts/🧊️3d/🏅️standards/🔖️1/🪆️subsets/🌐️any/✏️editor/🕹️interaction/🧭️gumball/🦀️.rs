//! 🧭️ Fem3d transform gumball — what the live selection resolves to geometrically (the nodes and solids a gesture
//! may move), the pivot the host anchors its gumball at, the `World3d` selection record that arms the host gumball
//! with live dispatch, and the gumball TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the
//! `🛠️tool-machine` runner. Every translate/rotate/scale dispatch builds one relative `move-selection` tick leaf; a
//! one-shot dispatch commits it as ONE `ToolTransaction`, a streamed gesture accumulates its ticks into the window's
//! ONE open transaction (persisted in the FEM gumball transient, previewed by every window, never history) and
//! commits the NET leaf once; an abort leaves zero trace
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::fem3d::interaction::{
    fem3d_element_endpoints, fem3d_entity_kind, fem3d_load_owner, Fem3dInteractionSnapshot, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT,
};
use crate::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig;
use crate::standards::v1::subsets::any::schema::mutations::move_selection::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{element_id, Fem3dSnapshot, FemLoad};
use machine::Command;
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolYield};
use semio_s_artifact_fem_2d::editor::fem2d::transient::{FemGumballGesture, FemGumballPhase, FemGumballTool};
use std::collections::HashSet;

//#region 🔖️Targets
/// 🎯️ The geometry a selection moves: node ids and solid ids, each once, in document order. Elements, supports and
/// loads resolve to the nodes and solids that carry them; materials, sections, cases and combinations carry no
/// geometry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fem3dTransformTargets {
    pub node_ids: Vec<String>,
    pub solid_ids: Vec<String>,
}

impl Fem3dTransformTargets {
    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty() && self.solid_ids.is_empty()
    }
}

pub fn fem3d_transform_targets(doc: &Fem3dSnapshot, selection_ids: &[String]) -> Fem3dTransformTargets {
    let mut nodes = HashSet::new();
    let mut solids = HashSet::new();
    let member_nodes = |target: &str, nodes: &mut HashSet<String>| {
        if let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) {
            let (start, end) = fem3d_element_endpoints(element);
            nodes.insert(start.to_string());
            nodes.insert(end.to_string());
        }
    };
    for id in selection_ids {
        match fem3d_entity_kind(doc, id) {
            Some(FEM3D_GRANULARITY_NODE) => {
                nodes.insert(id.clone());
            }
            Some(FEM3D_GRANULARITY_SOLID) => {
                solids.insert(id.clone());
            }
            Some(FEM3D_GRANULARITY_ELEMENT) => member_nodes(id, &mut nodes),
            Some(FEM3D_GRANULARITY_SUPPORT) => {
                if let Some(support) = doc.supports.iter().find(|support| support.id == *id) {
                    nodes.insert(support.node_id.clone());
                }
            }
            Some(FEM3D_GRANULARITY_LOAD) => {
                if let Some((_, load)) = fem3d_load_owner(doc, id) {
                    match load {
                        FemLoad::Nodal { node_id, .. } => {
                            nodes.insert(node_id.clone());
                        }
                        FemLoad::MemberUdl { element_id: target, .. } => member_nodes(target, &mut nodes),
                        FemLoad::Area { solid_id, .. } => {
                            solids.insert(solid_id.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Fem3dTransformTargets {
        node_ids: doc.nodes.iter().filter(|node| nodes.contains(&node.id)).map(|node| node.id.clone()).collect(),
        solid_ids: doc.solids.iter().filter(|solid| solids.contains(&solid.id)).map(|solid| solid.id.clone()).collect(),
    }
}

/// 🎯️ The gumball pivot: the centroid of every moved node and every moved solid's own centroid.
pub fn fem3d_transform_pivot(doc: &Fem3dSnapshot, targets: &Fem3dTransformTargets) -> Option<[f64; 3]> {
    let mut points = Vec::new();
    for id in &targets.node_ids {
        if let Some(node) = doc.nodes.iter().find(|node| &node.id == id) {
            points.push([node.x, node.y, node.z]);
        }
    }
    for id in &targets.solid_ids {
        if let Some(point) = doc.solids.iter().find(|solid| &solid.id == id).and_then(crate::standards::v1::subsets::any::scene::fem3d_solid_centroid) {
            points.push(point);
        }
    }
    if points.is_empty() {
        return None;
    }
    let count = points.len() as f64;
    let sum = points.iter().fold([0.0; 3], |sum, point| [sum[0] + point[0], sum[1] + point[1], sum[2] + point[2]]);
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}
//#endregion 🔖️Targets

//#region 🛠️GumballTool
/// 🪪️ The editor app id every gumball transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const FEM3D_EDITOR_APP_ID: &str = "s.fem.fem3d@1/*#editor";

/// 🔑️ The transaction key of a gesture's leaf — every tick upserts it, so an open transaction holds ONE net leaf.
pub const FEM3D_GUMBALL_LEAF_KEY: &str = "selection:0";

/// 🎬️ How one gumball dispatch moves the selection — the parameters of the tick leaf it builds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fem3dGumballMotion {
    Translate { dx: f64, dy: f64, dz: f64 },
    Rotate { axis: [f64; 3], angle: f64 },
    Scale { sx: f64, sy: f64, sz: f64 },
}

/// 🧮️ The relative `move-selection` leaf one dispatch yields on `doc`: the selection's nodes and solids as literal
/// targets, their centroid as the pivot, and the motion; `None` when the selection moves no geometry.
pub fn fem3d_gumball_tick(doc: &Fem3dSnapshot, selection_ids: &[String], motion: Fem3dGumballMotion) -> Option<MoveSelection> {
    let targets = fem3d_transform_targets(doc, selection_ids);
    let [pivot_x, pivot_y, pivot_z] = fem3d_transform_pivot(doc, &targets)?;
    let identity = MoveSelection { node_ids: targets.node_ids, solid_ids: targets.solid_ids, pivot_x, pivot_y, pivot_z, dx: 0.0, dy: 0.0, dz: 0.0, axis_x: 0.0, axis_y: 0.0, axis_z: 1.0, angle: 0.0, sx: 1.0, sy: 1.0, sz: 1.0 };
    Some(match motion {
        Fem3dGumballMotion::Translate { dx, dy, dz } => MoveSelection { dx, dy, dz, ..identity },
        Fem3dGumballMotion::Rotate { axis: [axis_x, axis_y, axis_z], angle } => MoveSelection { axis_x, axis_y, axis_z, angle, ..identity },
        Fem3dGumballMotion::Scale { sx, sy, sz } => MoveSelection { sx, sy, sz, ..identity },
    })
}

/// ➕️ `net` followed by `tick` as ONE net leaf: offsets add, angles about the same axis add, factors multiply — only
/// for the same targets and pivot and the same kind of motion; an identity tick changes nothing; `None` otherwise.
pub fn fem3d_gumball_then(net: &MoveSelection, tick: &MoveSelection) -> Option<MoveSelection> {
    if (&net.node_ids, &net.solid_ids, [net.pivot_x, net.pivot_y, net.pivot_z]) != (&tick.node_ids, &tick.solid_ids, [tick.pivot_x, tick.pivot_y, tick.pivot_z]) {
        return None;
    }
    let unmoved = [0.0; 3];
    let unscaled = [1.0; 3];
    let offset = |leaf: &MoveSelection| [leaf.dx, leaf.dy, leaf.dz];
    let factors = |leaf: &MoveSelection| [leaf.sx, leaf.sy, leaf.sz];
    let translation = |leaf: &MoveSelection| leaf.angle == 0.0 && factors(leaf) == unscaled;
    let rotation = |leaf: &MoveSelection| offset(leaf) == unmoved && factors(leaf) == unscaled;
    let scaling = |leaf: &MoveSelection| offset(leaf) == unmoved && leaf.angle == 0.0;
    let same_axis = match (net.unit_axis(), tick.unit_axis()) {
        (Some(left), Some(right)) => left.iter().zip(right).all(|(left, right)| (left - right).abs() < 1e-9),
        _ => false,
    };
    if tick.is_identity() {
        Some(net.clone())
    } else if translation(net) && translation(tick) {
        Some(MoveSelection { dx: net.dx + tick.dx, dy: net.dy + tick.dy, dz: net.dz + tick.dz, ..net.clone() })
    } else if rotation(net) && rotation(tick) && (net.angle == 0.0 || same_axis) {
        Some(MoveSelection { axis_x: tick.axis_x, axis_y: tick.axis_y, axis_z: tick.axis_z, angle: net.angle + tick.angle, ..net.clone() })
    } else if scaling(net) && scaling(tick) {
        Some(MoveSelection { sx: net.sx * tick.sx, sy: net.sy * tick.sy, sz: net.sz * tick.sz, ..net.clone() })
    } else {
        None
    }
}

/// 🎚️ Whether a tick is one the leaf admits and that moves: finite numbers, positive factors, a rotation about a
/// non-zero axis, not the identity.
fn fem3d_gumball_moves(tick: &MoveSelection) -> bool {
    let finite = [tick.dx, tick.dy, tick.dz, tick.axis_x, tick.axis_y, tick.axis_z, tick.angle, tick.sx, tick.sy, tick.sz].iter().all(|value| value.is_finite());
    finite && tick.sx > 0.0 && tick.sy > 0.0 && tick.sz > 0.0 && (tick.angle == 0.0 || tick.unit_axis().is_some()) && !tick.is_identity()
}

/// 📨️ One gumball event: the tick leaf this dispatch built (`None` when the selection moves no geometry).
#[derive(Clone, Debug)]
pub struct Fem3dGumballRequest {
    pub tick: Option<MoveSelection>,
}

/// 🧰️ The gumball tool's context: the net leaf a streamed gesture has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem3dGumballContext {
    pub stream: Option<MoveSelection>,
}

fn gumball_context(input: Fem3dGumballContext) -> Fem3dGumballContext {
    input
}

fn tick_moves(_context: &Fem3dGumballContext, event: Option<&gumball_tool::Event>) -> bool {
    matches!(event, Some(gumball_tool::Event::Records(request) | gumball_tool::Event::Stream(request)) if request.tick.as_ref().is_some_and(fem3d_gumball_moves))
}

fn yield_tick(_context: &mut Fem3dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Records(Fem3dGumballRequest { tick: Some(tick) })) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(FEM3D_GUMBALL_LEAF_KEY, Fem3dMutation::MoveSelection(tick.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut Fem3dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Stream(Fem3dGumballRequest { tick: Some(tick) })) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(FEM3D_GUMBALL_LEAF_KEY, Fem3dMutation::MoveSelection(tick.clone()))));
    context.stream = Some(tick.clone());
}

fn continue_stream(context: &mut Fem3dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let (Some(gumball_tool::Event::Stream(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = request.tick.as_ref().filter(|tick| fem3d_gumball_moves(tick)).and_then(|tick| fem3d_gumball_then(&stream, tick)).unwrap_or(stream);
    sink.push(Command::Effect(ToolYield::upsert(FEM3D_GUMBALL_LEAF_KEY, Fem3dMutation::MoveSelection(net.clone()))));
    context.stream = Some(net);
}

fn finish_stream(context: &mut Fem3dGumballContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let (Some(gumball_tool::Event::Finish(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = request.tick.as_ref().filter(|tick| fem3d_gumball_moves(tick)).and_then(|tick| fem3d_gumball_then(&stream, tick)).unwrap_or(stream);
    if net.is_identity() {
        sink.push(Command::Effect(ToolYield::retract(FEM3D_GUMBALL_LEAF_KEY)));
    } else {
        sink.push(Command::Effect(ToolYield::upsert(FEM3D_GUMBALL_LEAF_KEY, Fem3dMutation::MoveSelection(net))));
    }
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut Fem3dGumballContext, _event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    context.stream = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine gumball_tool {
        context: Fem3dGumballContext;
        event Event { Records(Fem3dGumballRequest), Stream(Fem3dGumballRequest), Finish(Fem3dGumballRequest), Cancel }
        input: Fem3dGumballContext;
        output: ();
        effect: ToolYield<Fem3dMutation>;
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
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Fem3dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🛠️ One window's fem3d gumball tool for one dispatch, a `🛠️tool-machine` runner scoped `<appId>#<verb>`.
pub struct Fem3dGumballTool {
    runner: ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl FemGumballTool for Fem3dGumballTool {
    type Leaf = MoveSelection;
    type Mutation = Fem3dMutation;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{FEM3D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), Fem3dGumballContext::default(), GumballToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &FemGumballGesture) -> Result<Self, ToolRefusal> {
        let entries: Vec<(String, Fem3dMutation)> = gesture.entries()?;
        let stream = entries.iter().find_map(|(key, leaf)| match leaf {
            Fem3dMutation::MoveSelection(leaf) if key == FEM3D_GUMBALL_LEAF_KEY => Some(leaf.clone()),
            _ => None,
        });
        let snapshot = gesture.snapshot::<gumball_tool::GumballTool>(Fem3dGumballContext { stream })?;
        let runner = ToolMachineRunner::resume(format!("{FEM3D_EDITOR_APP_ID}#{}", gesture.verb), protocol::ActorId(gesture.authoring_seed.clone()), Fem3dGumballContext::default(), snapshot, Some(ToolTransaction::resume(gesture.transaction.clone(), entries)), GumballToolHost)?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: FemGumballPhase, tick: Option<MoveSelection>) -> Result<ToolStep<Fem3dMutation>, ToolRefusal> {
        let request = Fem3dGumballRequest { tick };
        let event = match phase {
            FemGumballPhase::Stream => gumball_tool::Event::Stream(request),
            FemGumballPhase::Commit if !self.runner.at_rest() => gumball_tool::Event::Finish(request),
            FemGumballPhase::Abort(_) => gumball_tool::Event::Cancel,
            FemGumballPhase::Once | FemGumballPhase::Commit => gumball_tool::Event::Records(request),
        };
        self.runner.send(event, protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 })
    }

    fn persist(self) -> Option<FemGumballGesture> {
        let (snapshot, transaction) = self.runner.into_parts();
        FemGumballGesture::persist(machine::persist(&snapshot).states, &self.verb, &self.authoring_seed, &self.base_revision, transaction)
    }
}
//#endregion 🛠️GumballTool

//#region 🔖️SelectionRecord
/// 🕹️ Whether the world gumball should render: the transform utility is armed, at least one handle
/// flag is on (an all-off gumball would draw nothing to grab), and the live selection moves something.
pub fn fem3d_gumball_active(doc: &Fem3dSnapshot, interaction: &Fem3dInteractionSnapshot, transform_armed: bool, config: &Fem3dGumballConfig) -> bool {
    transform_armed && config.any() && !fem3d_transform_targets(doc, &interaction.selected_ids).is_empty()
}

/// 🕹️ The host's `WorldSelectionRecord` for a fem3d window: the framework-owned `fem3d` selection
/// and hover projected onto the field names `World3dHost`'s `parseSelection` reads, plus — while the
/// transform utility is armed — the gumball descriptor. `gumballLiveDispatch` asks the host to stream
/// every drag step as an incremental `translateSelection`/`rotateSelection`/`scaleSelection` with
/// `phase: "stream"`, then `phase: "commit"` on release (or `phase: "abort"` on blur or a lost capture),
/// instead of previewing locally: the gumball tool's open transaction IS the preview, painted by both
/// windows while the results window re-solves it, and the release commits it as ONE edit.
pub fn fem3d_selection_json(doc: &Fem3dSnapshot, interaction: &Fem3dInteractionSnapshot, transform_armed: bool, config: &Fem3dGumballConfig) -> String {
    let hovered = interaction.hovered_ids.first().map(String::as_str);
    let mut value: dsl::json::Value = dsl::json::parse(&semio_framework_plugin::world3d_selection_json_with_granularity("rectangle", &interaction.selected_ids, hovered, Some(FEM3D_GRANULARITY_NODE))).unwrap_or_else(|_| dsl::json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("selectionMode", dsl::json!("object"));
        object.insert("targets", dsl::json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        if let Some(id) = interaction.selected_ids.first() {
            object.insert("activeObjectId", dsl::json!(id));
        }
        let active = fem3d_gumball_active(doc, interaction, transform_armed, config);
        object.insert("gumballActive", dsl::json!(active));
        if transform_armed {
            object.insert("transformMode", dsl::json!("transform"));
            object.insert(
                "gumballConfig",
                dsl::json!({
                    "moveAxes": config.move_axes,
                    "movePlanes": config.move_planes,
                    "rotate": config.rotate,
                    "scaleAxes": config.scale_axes,
                    "scalePlanes": false,
                    "scaleUniform": config.scale_uniform,
                }),
            );
            object.insert("gumballLiveDispatch", dsl::json!(true));
            if active {
                if let Some(pivot) = fem3d_transform_pivot(doc, &fem3d_transform_targets(doc, &interaction.selected_ids)) {
                    object.insert("gumballTarget", dsl::json!(pivot));
                }
            }
        }
    }
    dsl::json::to_string(&value)
}
//#endregion 🔖️SelectionRecord

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
