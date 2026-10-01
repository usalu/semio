//! 🧭️ The gumball tool (design §5, §13): every viewport transform gesture is a `🔄️machine` statechart folded by the
//! tool-machine runner that yields ONE relative leaf (`drag-`/`rotate-`/`scale-transforms`) of the transform operators
//! the gesture composes into; the release commits it with the absolute splice rows that insert a missing operator as ONE
//! `ToolTransaction`. Tools are never history; the yielded leaf is, so editing it re-derives the operator on any base.

use crate::editor::generation3d::config::Generation3dConfigMutation;
use crate::editor::generation3d::selection::{component_group, ComponentTarget, DOMAIN};
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::drag_transforms;
use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::rotate_transforms;
use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::scale_transforms;
use crate::standards::v1::subsets::any::schema::transforms::{compose_scale, AxisAngle};
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, ensure_gumball_node, mutations::text::Generation3dMutation, with_host};
use machine::Command;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::FlowHost;
use semio_framework_plugin::{Emit, Fault, InteractionWrite};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolStep, ToolYield};

pub fn selection_ids(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() { fallback.to_vec() } else { ids.to_vec() }
}

/// 🧷️ Accepts pinned components only while the same set continues through its transforms.
pub fn validate_component_gesture(snapshot: &FlowHostSnapshot, pinned: &[String], selected: &[String]) -> Result<(), Fault> {
    if pinned.is_empty() { return Ok(()); }
    let (origin, pinned_components) = component_group(pinned).map_err(Fault::from)?;
    let (target, components) = component_group(selected).map_err(Fault::from)?;
    if origin.granularity != target.granularity || origin.index != target.index || pinned_components != components {
        return Err(Fault::from("The component selection changed during the transform"));
    }
    let mut widget_id = target.widget;
    let mut channel = target.channel;
    let mut visited = std::collections::BTreeSet::new();
    while visited.insert(widget_id) {
        if origin.widget == widget_id && origin.channel == channel { return Ok(()); }
        let Some(Widget::Neuron { neuron_kind, params, .. }) = snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == widget_id) else { break };
        if !matches!(neuron_kind.as_str(), "brep.mesh.translateComponents" | "brep.mesh.rotateComponents" | "brep.mesh.scaleComponents") || channel != "meshOut" { break; }
        let text = |key| params.get(key).and_then(|value| value.as_dictionary()).and_then(|value| value.get("value")).and_then(|value| value.as_atom()).and_then(|value| value.as_str());
        if text("mode") != Some(target.granularity) || text("selection").and_then(|value| serde_json::from_str::<Vec<u32>>(value).ok()).as_ref() != Some(&components) { break; }
        let mut inputs = snapshot.synapses.iter().filter(|wire| wire.to == widget_id && wire.to_port == "mesh");
        let Some(source) = inputs.next() else { break };
        if inputs.next().is_some() { break; }
        widget_id = &source.from;
        channel = &source.from_port;
    }
    Err(Fault::from("The component selection changed during the transform"))
}

//#region 🛠️GumballTool
/// 🎬️ How one gumball gesture moves its operators — the parameters of the relative leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GumballMotion {
    Translate([f64; 3]),
    Rotate(AxisAngle),
    Scale([f64; 3]),
}

impl GumballMotion {
    /// 🧭️ The operator family this motion composes into (`translate`, `rotate`, `scale`).
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Translate(_) => "translate",
            Self::Rotate(_) => "rotate",
            Self::Scale(_) => "scale",
        }
    }

    /// 🎚️ Whether the motion is one a leaf admits and that moves: finite numbers, a non-zero axis, factors above zero, not
    /// the identity.
    pub fn moves(&self) -> bool {
        match self {
            Self::Translate(offset) => offset.iter().all(|value| value.is_finite()) && *offset != [0.0; 3],
            Self::Rotate(rotation) => rotation.axis.iter().chain([&rotation.angle]).all(|value| value.is_finite()) && rotation.axis != [0.0; 3] && rotation.angle != 0.0,
            Self::Scale(factors) => factors.iter().all(|value| value.is_finite() && *value > 0.0) && *factors != [1.0; 3],
        }
    }

    /// ➕️ This motion followed by `tick` as ONE net motion of the same family: offsets add, rotations compose, factors
    /// multiply; `None` across families or for an inadmissible composition.
    pub fn then(&self, tick: &Self) -> Option<Self> {
        match (self, tick) {
            (Self::Translate(offset), Self::Translate(next)) => Some(Self::Translate(std::array::from_fn(|axis| offset[axis] + next[axis]))),
            (Self::Rotate(rotation), Self::Rotate(next)) => rotation.then(*next).ok().map(Self::Rotate),
            (Self::Scale(factors), Self::Scale(next)) => compose_scale(*factors, *next).ok().map(Self::Scale),
            _ => None,
        }
    }

    /// 🧮️ The relative leaf this motion yields over the operator `targets`.
    pub fn leaf(&self, targets: Vec<String>) -> Generation3dMutation {
        match self {
            Self::Translate(offset) => drag_transforms(targets, *offset),
            Self::Rotate(rotation) => rotate_transforms(targets, rotation.axis, rotation.angle),
            Self::Scale(factors) => scale_transforms(targets, *factors),
        }
    }

    /// 🧬️ The motion a relative leaf states — how an open gesture recovers its accumulated motion.
    pub fn of_leaf(leaf: &Generation3dMutation) -> Option<Self> {
        match leaf {
            Generation3dMutation::DragTransforms(leaf) => Some(Self::Translate([leaf.dx, leaf.dy, leaf.dz])),
            Generation3dMutation::RotateTransforms(leaf) => Some(Self::Rotate(AxisAngle { axis: [leaf.ax, leaf.ay, leaf.az], angle: leaf.angle })),
            Generation3dMutation::ScaleTransforms(leaf) => Some(Self::Scale([leaf.sx, leaf.sy, leaf.sz])),
            _ => None,
        }
    }
}

/// 🎬️ One gumball tick the tool yields: the operator ids the motion composes into and the motion.
#[derive(Clone, Debug, PartialEq)]
pub struct GumballRecord {
    pub targets: Vec<String>,
    pub motion: GumballMotion,
}

impl GumballRecord {
    fn moves(&self) -> bool {
        !self.targets.is_empty() && self.motion.moves()
    }
}

/// 🧰️ The gumball tool's context: the one record a streamed gesture has accumulated so far (`None` at rest) — tool state,
/// never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GumballToolContext {
    pub stream: Option<GumballRecord>,
}

/// 🔑️ The transaction key of a gesture's relative leaf — every tick upserts it, so an open transaction holds ONE net leaf.
pub const GENERATION3D_GUMBALL_LEAF_KEY: &str = "transform";

fn gumball_tool_context(input: GumballToolContext) -> GumballToolContext {
    input
}

fn record_moves(_context: &GumballToolContext, event: Option<&gumball_tool::Event>) -> bool {
    matches!(event, Some(gumball_tool::Event::Once(record) | gumball_tool::Event::Stream(record) | gumball_tool::Event::Finish(record)) if record.moves())
}

fn yield_once(_context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Once(record) | gumball_tool::Event::Finish(record)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(GENERATION3D_GUMBALL_LEAF_KEY, record.motion.leaf(record.targets.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Stream(record)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(GENERATION3D_GUMBALL_LEAF_KEY, record.motion.leaf(record.targets.clone()))));
    context.stream = Some(record.clone());
}

fn continue_stream(context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let (Some(gumball_tool::Event::Stream(tick)), Some(stream)) = (event, context.stream.as_mut()) else { return };
    if let Some(motion) = (stream.targets == tick.targets).then(|| stream.motion.then(&tick.motion)).flatten() {
        stream.motion = motion;
    }
    sink.push(Command::Effect(ToolYield::upsert(GENERATION3D_GUMBALL_LEAF_KEY, stream.motion.leaf(stream.targets.clone()))));
}

fn finish_stream(context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(mut stream) = context.stream.take() else { return };
    if let Some(gumball_tool::Event::Finish(tick)) = event {
        if let Some(motion) = (stream.targets == tick.targets).then(|| stream.motion.then(&tick.motion)).flatten() {
            stream.motion = motion;
        }
    }
    sink.push(Command::Effect(if stream.moves() { ToolYield::upsert(GENERATION3D_GUMBALL_LEAF_KEY, stream.motion.leaf(stream.targets)) } else { ToolYield::retract(GENERATION3D_GUMBALL_LEAF_KEY) }));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut GumballToolContext, _event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    context.stream = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine gumball_tool {
        context: GumballToolContext;
        event Event { Once(GumballRecord), Stream(GumballRecord), Finish(GumballRecord), Cancel }
        input: GumballToolContext;
        output: ();
        effect: ToolYield<Generation3dMutation>;
        context_from_input: gumball_tool_context;
        initial: idle;
        state idle {
            on Once if record_moves => idle do yield_once;
            on Finish if record_moves => idle do yield_once;
            on Stream if record_moves => streaming do begin_stream;
        }
        state streaming {
            on Stream => streaming do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
        }
    }
}

/// 🧷️ The gumball tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct GumballToolHost;

impl machine::Host<gumball_tool::GumballTool> for GumballToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Generation3dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🎚️ Where one dispatch of a gumball verb sits in its gesture (the `World3dHost` live protocol): a one-shot `Once`, a
/// `Stream` tick into the window's open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GumballPhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl GumballPhase {
    /// 🧩️ Reads a gumball verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason` (absent =
    /// `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 💾️ One window's open gumball gesture, retained by the app instance between dispatches — ephemeral local tool state,
/// never history: the runner with its open transaction (ONE net relative leaf), the verb, the selection it transforms and
/// the document revision it opened on. The splice that inserts a missing operator is re-derived from the selection on the
/// committed base at the release and for every preview, so the open transaction never holds a structural row.
pub struct GumballGesture {
    runner: ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost>,
    verb: &'static str,
    ids: Vec<String>,
    base_revision: [u8; 32],
}

/// 📨️ One dispatch of a gumball verb: the verb, the selection (shape instance ids or mesh component ids) it transforms,
/// the tick's motion, its phase, and the admission it runs under.
pub struct GumballDispatch<'a> {
    pub verb: &'static str,
    pub window: &'a str,
    pub ids: Vec<String>,
    pub motion: GumballMotion,
    pub phase: GumballPhase,
    pub authoring_seed: &'a str,
    pub base_revision: [u8; 32],
}

/// 🗂️ Every window's open gumball gesture of one app instance, and a revision that moves whenever what the previews fold
/// in changes (a gesture opens, ticks, commits or aborts).
#[derive(Default)]
pub struct GumballGestures {
    open: std::collections::BTreeMap<String, GumballGesture>,
    revision: u64,
}

/// 🧩️ What one gumball selection needs on `host_snapshot`: the absolute splice rows that insert every missing transform
/// operator (none when they exist), the operator ids the motion composes into, and the selection writes that follow the
/// gesture onto them. A mesh-component selection splices ONE component operator for the whole component set.
pub fn gumball_splice(host_snapshot: &FlowHostSnapshot, ids: &[String], operation: &str) -> Result<(Vec<Generation3dMutation>, Vec<String>, Vec<InteractionWrite>), Fault> {
    with_host(host_snapshot, |host| {
        let (targets, writes) = if ids.iter().any(|id| ComponentTarget::parse(id).is_some()) {
            let (id, mode, components) = ensure_component_node(host, ids, operation).map_err(Fault::from)?;
            let writes = vec![InteractionWrite::replace("graph", "node", [id.clone()]), InteractionWrite::replace(DOMAIN, &mode, components.iter().map(|component| format!("{id}@meshOut#0.{mode}.{component}")))];
            (vec![id], writes)
        } else {
            let mut targets: Vec<String> = Vec::new();
            for id in ids {
                let next = ensure_gumball_node(host, id, operation).map_err(Fault::from)?;
                if !targets.contains(&next) {
                    targets.push(next);
                }
            }
            let writes = vec![InteractionWrite::replace("graph", "node", targets.clone())];
            (targets, writes)
        };
        Ok((commit_host_snapshot(host_snapshot, &host.host_snapshot), targets, writes))
    })
}

fn retire_rows(rows: Vec<Generation3dMutation>) {
    for row in rows {
        row.retire_cold();
    }
}

impl GumballGestures {
    /// 🔎️ Whether no window holds an open gesture.
    pub fn is_empty(&self) -> bool {
        self.open.is_empty()
    }

    /// 🔢️ Moves whenever what the previews fold in changes.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// 🧯️ Host abort of `window`'s open gesture: its transaction vanishes with zero trace. Answers whether one was open.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> bool {
        let Some(mut gesture) = self.open.remove(window) else { return false };
        gesture.runner.abort(reason);
        self.revision = self.revision.wrapping_add(1);
        true
    }

    /// 👁️ The rows a derived view folds onto `host_snapshot` to show every open gesture: each gesture's splice re-derived
    /// on that base, then its ONE net relative leaf — a preview, never history.
    pub fn provisional(&self, host_snapshot: &FlowHostSnapshot) -> Vec<Generation3dMutation> {
        let mut rows = Vec::new();
        for gesture in self.open.values() {
            let Some(leaf) = gesture.runner.transaction().and_then(|transaction| transaction.entries().iter().find(|(key, _)| key == GENERATION3D_GUMBALL_LEAF_KEY).map(|(_, leaf)| leaf.clone())) else { continue };
            let Some(motion) = GumballMotion::of_leaf(&leaf) else { continue };
            let Ok((splice, targets, _)) = gumball_splice(host_snapshot, &gesture.ids, motion.operation()) else { continue };
            rows.extend(splice);
            rows.push(motion.leaf(targets));
        }
        rows
    }

    /// 🛠️ Drives `request.window`'s gumball tool through ONE dispatch. `Once` commits the motion as one transaction;
    /// `Stream` upserts it into the window's open transaction — opening it on the first tick — which the app instance
    /// retains and every preview folds in; `Commit` folds the tail in and commits the whole gesture as ONE edit (the splice
    /// rows re-derived on the committed base, then the net relative leaf, every row stamped with the transaction); `Abort`
    /// drops the open gesture with zero trace. An open gesture another verb or a one-shot interrupts is aborted
    /// `captureLost`; one whose document moved under it is aborted `baseMoved`, and a tick or commit that finds it so is
    /// dropped with it.
    pub fn dispatch(&mut self, request: GumballDispatch<'_>, host_snapshot: &FlowHostSnapshot) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
        let open = self.open.remove(request.window);
        if open.is_some() {
            self.revision = self.revision.wrapping_add(1);
        }
        let open = match (open, request.phase) {
            (Some(mut gesture), GumballPhase::Abort(reason)) => {
                gesture.runner.abort(reason);
                return Ok(Emit::default());
            }
            (None, GumballPhase::Abort(_)) => return Ok(Emit::default()),
            (Some(mut gesture), phase) if gesture.base_revision != request.base_revision => {
                gesture.runner.abort(ToolAbortReason::BaseMoved);
                if phase != GumballPhase::Once {
                    return Ok(Emit::default());
                }
                None
            }
            (Some(mut gesture), phase) if gesture.verb != request.verb || phase == GumballPhase::Once => {
                gesture.runner.abort(ToolAbortReason::CaptureLost);
                None
            }
            (open, _) => open,
        };
        let mut gesture = match open {
            Some(gesture) => gesture,
            None => GumballGesture {
                runner: ToolMachineRunner::start(format!("{}#{}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID, request.verb), protocol::ActorId(request.authoring_seed.to_string()), GumballToolContext::default(), GumballToolHost).map_err(|refusal| Fault::from(refusal.code()))?,
                verb: request.verb,
                ids: request.ids.clone(),
                base_revision: request.base_revision,
            },
        };
        let (splice, targets, writes) = gumball_splice(host_snapshot, &gesture.ids, request.motion.operation())?;
        let record = GumballRecord { targets, motion: request.motion };
        let event = match request.phase {
            GumballPhase::Stream => gumball_tool::Event::Stream(record),
            GumballPhase::Commit if !gesture.runner.at_rest() => gumball_tool::Event::Finish(record),
            _ => gumball_tool::Event::Once(record),
        };
        let step = gesture.runner.send(event, crate::editor::generation3d::commands::node_graph_edit::generation3d_gesture_clock()).map_err(|refusal| Fault::from(refusal.code()))?;
        let emit = match step {
            ToolStep::Committed(transaction, leaves) => {
                let rows: Vec<Generation3dMutation> = splice.into_iter().chain(leaves).collect();
                let emit = if request.authoring_seed.is_empty() { Emit::mutations(rows) } else { Emit::commit_transaction(transaction, rows) };
                Emit { interaction_writes: writes, ..emit }
            }
            _ => {
                retire_rows(splice);
                Emit::default()
            }
        };
        if !gesture.runner.at_rest() {
            self.open.insert(request.window.to_string(), gesture);
            self.revision = self.revision.wrapping_add(1);
        }
        Ok(emit)
    }
}

/// 🎯️ The selection a gumball dispatch transforms: in component mode the component selection, pinned to the set the
/// gesture started on (`validate_component_gesture`); otherwise the explicit ids, else the `graph` selection.
pub fn gumball_ids(snapshot: &FlowHostSnapshot, explicit: &[String], components: Option<&[String]>, graph: &[String]) -> Result<Vec<String>, Fault> {
    match components {
        Some(selected) => {
            validate_component_gesture(snapshot, explicit, selected)?;
            Ok(selected.to_vec())
        }
        None => Ok(selection_ids(explicit, graph)),
    }
}

/// 🕹️ ONE one-shot gumball transaction of `ids` — the entry a view without a retained instance (the marks-free handler)
/// takes; a view without command authority publishes the rows plainly.
pub fn gumball_once(verb: &'static str, ids: Vec<String>, motion: GumballMotion, doc: &semio_framework_plugin::ArtifactView<'_, crate::Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if ids.is_empty() {
        return Ok(Emit::default());
    }
    let (authoring_seed, base_revision) = doc.operation().map(|operation| (operation.authoring_seed.clone(), operation.canonical_base_revision)).unwrap_or_default();
    GumballGestures::default().dispatch(GumballDispatch { verb, window: "", ids, motion, phase: GumballPhase::Once, authoring_seed: &authoring_seed, base_revision }, &doc.snapshot.host_snapshot)
}
//#endregion 🛠️GumballTool

/// 🎯️ Splices an adjustable component transform and reuses it only for its own component set.
pub fn ensure_component_node(host: &mut FlowHost, ids: &[String], operation: &str) -> Result<(String, String, Vec<u32>), String> {
    let (target, components) = component_group(ids)?;
    if !matches!(operation, "translate" | "rotate" | "scale") { return Err("Unknown component transform".into()); }
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    let kind = host.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::Neuron { id, neuron_kind, .. } if id == target.widget => Some(neuron_kind),
        _ => None,
    }).ok_or("The selected mesh no longer exists")?;
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let source = infos.get(kind).ok_or("The selected mesh operator is unavailable")?;
    if !source.outputs.iter().any(|port| port.name == target.channel && !port.cardinality.is_collection() && port.value_types.iter().any(|kind| kind == "mesh")) {
        return Err("Select a single indexed mesh output; convert B-Rep geometry to a mesh first".into());
    }
    let next_kind = format!("brep.mesh.{operation}Components");
    let selection = serde_json::to_string(&components).map_err(|error| error.to_string())?;
    let current = crate::standards::v1::subsets::any::schema::gumball_widget_json(host, target.widget);
    let params = current.as_ref().and_then(|value| value.get("params"));
    let parameter = |name| params.and_then(|value| value.get(name)).and_then(|value| value.get("value")).and_then(dsl::DslValue::as_str);
    let inputs = host.host_snapshot.synapses.iter().filter(|wire| wire.to == target.widget).collect::<Vec<_>>();
    if kind == &next_kind && parameter("mode") == Some(target.granularity) && parameter("selection") == Some(selection.as_str()) && (operation == "translate" || parameter("pivot") == Some("selection")) && inputs.len() == 1 && inputs[0].to_port == "mesh" {
        return Ok((target.widget.into(), target.granularity.into(), components));
    }
    let output = infos.get(&next_kind).and_then(|info| info.outputs.first()).ok_or("The component transform is unavailable")?;
    let base = format!("{}__{operation}Components", target.widget);
    let mut id = base.clone();
    let mut suffix = 2;
    while host.host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id) { id = format!("{base}_{suffix}"); suffix += 1; }
    let (x, y) = host.host_snapshot.layout.get(target.widget).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    host.add_widget(&serde_json::json!({"kind":"neuron","id":id,"neuronKind":next_kind}).to_string(), x + 220.0, y).map_err(|error| error.to_string())?;
    let params = serde_json::json!({"mode":{"$schema":"text","value":target.granularity},"selection":{"$schema":"text","value":selection},"pivot":{"$schema":"text","value":"selection"}});
    host.set_neuron_params(&id, &params.to_string()).map_err(|error| error.to_string())?;
    host.insert_between(target.widget, target.channel, &id, "mesh", &output.name).map_err(|error| error.to_string())?;
    for widget in &mut host.host_snapshot.widgets {
        if let Widget::Neuron { id: widget_id, preview, .. } = widget {
            if widget_id == &id { *preview = true; }
            if widget_id == target.widget { *preview = false; }
        }
    }
    Ok((id, target.granularity.into(), components))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
