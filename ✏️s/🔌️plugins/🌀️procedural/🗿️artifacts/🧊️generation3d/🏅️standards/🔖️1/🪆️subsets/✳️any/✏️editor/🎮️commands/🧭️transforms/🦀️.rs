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
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue;
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, ensure_gumball_node, gumball_identity, mutations::text::Generation3dMutation, record_input_leaves, with_host, GumballRefusal};
use machine::Command;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::FlowHost;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, InteractionWrite};
use semio_framework_ui_locale::LocalizedLabel;
pub use semio_framework_tool_machine::GesturePhase;
use semio_framework_tool_machine::{drive_gesture, GestureTool, ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};

impl From<GumballRefusal> for Fault {
    fn from(refusal: GumballRefusal) -> Self {
        let fault = Fault::new(FaultOrigin::App, FaultCode::new(refusal.code()), refusal.detail());
        match refusal {
            GumballRefusal::KindUnavailable(kind) | GumballRefusal::TransformUnavailable(kind) => fault.with_param("kind", kind),
            _ => fault,
        }
    }
}

/// 📢️ The localized notice of every gumball refusal code (design §20.12), the editor's declared fault-notice table: one
/// fixed sentence per code, `{kind}` filled from the fault's `kind` param; the English developer detail is never shown.
pub fn gumball_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 12]> = std::sync::LazyLock::new(|| {
        [
            ("generation3d.gumball.unknown-operation", LocalizedLabel::native("This transform is not available.", "Diese Transformation ist nicht verfügbar.")),
            ("generation3d.gumball.no-shape-source", LocalizedLabel::native("Select a node that produces a shape.", "Einen Knoten auswählen, der eine Form erzeugt.")),
            ("generation3d.gumball.kind-unavailable", LocalizedLabel::native("The node kind {kind} is not available here.", "Die Knotenart {kind} ist hier nicht verfügbar.")),
            ("generation3d.gumball.no-shape-output", LocalizedLabel::native("The selected output holds no shape.", "Die ausgewählte Ausgabe enthält keine Form.")),
            ("generation3d.gumball.list-output", LocalizedLabel::native("Take one shape out of the list before transforming it.", "Vor dem Transformieren eine einzelne Form aus der Liste entnehmen.")),
            ("generation3d.gumball.identifier-occupied", LocalizedLabel::native("Another node already uses the transform's name.", "Ein anderer Knoten verwendet bereits den Namen der Transformation.")),
            ("generation3d.gumball.transform-unavailable", LocalizedLabel::native("The transform {kind} is not available here.", "Die Transformation {kind} ist hier nicht verfügbar.")),
            ("generation3d.gumball.mesh-missing", LocalizedLabel::native("The selected mesh no longer exists.", "Das ausgewählte Netz existiert nicht mehr.")),
            ("generation3d.gumball.not-indexed-mesh", LocalizedLabel::native("Select one indexed mesh output; convert B-Rep geometry to a mesh first.", "Eine einzelne indizierte Netzausgabe auswählen; B-Rep-Geometrie zuerst in ein Netz umwandeln.")),
            ("generation3d.gumball.selection-changed", LocalizedLabel::native("The component selection changed during the transform.", "Die Komponentenauswahl hat sich während der Transformation geändert.")),
            ("generation3d.gumball.component-selection", LocalizedLabel::native("Select components of a single mesh.", "Komponenten eines einzelnen Netzes auswählen.")),
            ("generation3d.gumball.host-edit", LocalizedLabel::native("The transform could not be added to the graph.", "Die Transformation konnte dem Graphen nicht hinzugefügt werden.")),
        ]
    });
    &*NOTICES
}

pub fn selection_ids(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() { fallback.to_vec() } else { ids.to_vec() }
}

/// 🧷️ Accepts pinned components only while the same set continues through its transforms.
pub fn validate_component_gesture(snapshot: &FlowHostSnapshot, pinned: &[String], selected: &[String]) -> Result<(), Fault> {
    if pinned.is_empty() { return Ok(()); }
    let (origin, pinned_components) = component_group(pinned).map_err(GumballRefusal::ComponentSelection)?;
    let (target, components) = component_group(selected).map_err(GumballRefusal::ComponentSelection)?;
    if origin.granularity != target.granularity || origin.index != target.index || pinned_components != components {
        return Err(GumballRefusal::SelectionChanged.into());
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
    Err(GumballRefusal::SelectionChanged.into())
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
    /// 🗺️ The operator family this motion composes into (`translate`, `rotate`, `scale`).
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

/// 📼️ One gumball tick the tool yields: the operator ids the motion composes into and the motion.
#[derive(Clone, Debug, PartialEq)]
pub struct GumballRecord {
    pub targets: Vec<String>,
    pub motion: GumballMotion,
}

impl GumballRecord {
    fn moves(&self) -> bool {
        !self.targets.is_empty() && self.motion.moves()
    }

    /// 🔬️ The record a relative leaf states — how a resumed gesture recovers the stream it accumulated.
    fn of_leaf(leaf: &Generation3dMutation) -> Option<Self> {
        let targets = match leaf {
            Generation3dMutation::DragTransforms(leaf) => &leaf.targets,
            Generation3dMutation::RotateTransforms(leaf) => &leaf.targets,
            Generation3dMutation::ScaleTransforms(leaf) => &leaf.targets,
            _ => return None,
        };
        Some(Self { targets: targets.clone(), motion: GumballMotion::of_leaf(leaf)? })
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

/// 🏠️ The gumball tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
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

/// 💾️ One window's open gumball gesture between dispatches — ephemeral local tool state the app instance retains, never
/// history: the statechart configuration, the verb, the admission, the document revision it opened on, its open transaction
/// (ONE net relative leaf) and the selection it transforms. The splice that inserts a missing operator is re-derived from
/// the selection on the committed base at the release and for every preview, so the open transaction never holds a
/// structural row.
#[derive(Clone, Debug, PartialEq)]
pub struct GumballGesture {
    states: Vec<String>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
    transaction: protocol::TransactionRef,
    entries: Vec<(String, Generation3dMutation)>,
    ids: Vec<String>,
}

impl GumballGesture {
    /// 🔁️ Whether a dispatch of `verb` in `phase` on `base_revision` continues this gesture — exactly the rule
    /// [`drive_gesture`] applies, read before the drive so the splice is derived from the selection the gesture transforms.
    fn continues(&self, verb: &str, phase: GesturePhase, base_revision: &str) -> bool {
        self.verb == verb && self.base_revision == base_revision && !matches!(phase, GesturePhase::Once | GesturePhase::Abort(_))
    }

    /// 🥅️ The gesture's ONE net relative leaf.
    fn leaf(&self) -> Option<&Generation3dMutation> {
        self.entries.iter().find(|(key, _)| key == GENERATION3D_GUMBALL_LEAF_KEY).map(|(_, leaf)| leaf)
    }
}

/// 🎫️ One gumball tick handed to the tool: the selection it transforms (kept by the gesture it opens) and its record.
pub struct GumballTick {
    pub ids: Vec<String>,
    pub record: GumballRecord,
}

/// 🤖️ One window's gumball tool for ONE dispatch on the shared streamed-gesture runner ([`drive_gesture`]), a
/// `🛠️tool-machine` runner scoped `<appId>#<verb>`.
pub struct Generation3dGumballTool {
    runner: ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
    ids: Vec<String>,
}

impl GestureTool for Generation3dGumballTool {
    type Gesture = GumballGesture;
    type Tick = GumballTick;
    type Mutation = Generation3dMutation;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{}#{verb}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID), protocol::ActorId(authoring_seed.to_string()), GumballToolContext::default(), GumballToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string(), ids: Vec::new() })
    }

    fn resume(gesture: &GumballGesture) -> Result<Self, ToolRefusal> {
        let stream = gesture.leaf().and_then(GumballRecord::of_leaf);
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: <gumball_tool::GumballTool as machine::Machine>::definition().fingerprint, states: gesture.states.clone(), history: Vec::new(), done: false };
        let snapshot = machine::restore::<gumball_tool::GumballTool, machine::NoMigrations>(&persisted, GumballToolContext { stream }, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(gesture.transaction.clone(), gesture.entries.clone());
        let runner = ToolMachineRunner::resume(format!("{}#{}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID, gesture.verb), protocol::ActorId(gesture.authoring_seed.clone()), GumballToolContext::default(), snapshot, Some(transaction), GumballToolHost)?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone(), ids: gesture.ids.clone() })
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

    fn send(&mut self, phase: GesturePhase, tick: Option<GumballTick>) -> Result<ToolStep<Generation3dMutation>, ToolRefusal> {
        let event = match (phase, tick) {
            (GesturePhase::Abort(_), _) => gumball_tool::Event::Cancel,
            (_, None) => return Ok(ToolStep::Idle),
            (phase, Some(tick)) => {
                if self.runner.at_rest() {
                    self.ids = tick.ids;
                }
                match phase {
                    GesturePhase::Stream => gumball_tool::Event::Stream(tick.record),
                    GesturePhase::Commit if !self.runner.at_rest() => gumball_tool::Event::Finish(tick.record),
                    _ => gumball_tool::Event::Once(tick.record),
                }
            }
        };
        self.runner.send(event, semio_framework_tool_machine::authoring_clock(0))
    }

    fn persist(self) -> Option<GumballGesture> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(GumballGesture {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().to_vec(),
            ids: self.ids,
        })
    }
}

/// 📨️ One dispatch of a gumball verb: the verb, the selection (shape instance ids or mesh component ids) it transforms,
/// the tick's motion, its phase, and the admission it runs under.
pub struct GumballDispatch<'a> {
    pub verb: &'static str,
    pub window: &'a str,
    pub ids: Vec<String>,
    pub motion: GumballMotion,
    pub phase: GesturePhase,
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

/// 🧲️ The selection a gumball gesture carries onto the operators it transforms: the `graph` nodes and, for a
/// mesh-component selection, the components re-addressed onto the component operator (`granularity`, ids). The commit
/// lands it as replacing interaction writes; an open gesture's previews paint it without touching the interaction store.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GumballSelection {
    pub nodes: Vec<String>,
    pub components: Option<(String, Vec<String>)>,
}

impl GumballSelection {
    /// ✍️ The replacing interaction writes the commit lands.
    pub fn writes(&self) -> Vec<InteractionWrite> {
        let mut writes = vec![InteractionWrite::replace("graph", "node", self.nodes.iter().cloned())];
        if let Some((granularity, components)) = &self.components {
            writes.push(InteractionWrite::replace(DOMAIN, granularity, components.iter().cloned()));
        }
        writes
    }
}

/// 🔭️ What every open gesture shows on one base: the rows a derived view folds in (each gesture's splice, then its ONE
/// net relative leaf) and the selections those rows carry the gestures' marks onto — a preview, never history.
#[derive(Default)]
pub struct GumballPreview {
    pub rows: Vec<Generation3dMutation>,
    pub selections: Vec<GumballSelection>,
}

/// 🎛️ The channels a component gesture sets on the component operator: its granularity, its component set and — for a
/// turn or a scaling — the selection pivot.
pub fn component_gesture_inputs(mode: &str, components: &[u32], operation: &str) -> Vec<(&'static str, WidgetInputValue)> {
    let mut inputs = vec![("mode", WidgetInputValue::Text(mode.into())), ("selection", WidgetInputValue::Text(serde_json::to_string(components).unwrap_or_default()))];
    if operation != "translate" {
        inputs.push(("pivot", WidgetInputValue::Text("selection".into())));
    }
    inputs
}

/// 🧩️ What one gumball selection needs on `host_snapshot`: the absolute splice rows that insert every missing transform
/// operator with its DEFAULT params, then one `change-widget-input` per channel the gesture sets on an inserted operator
/// (design §19.4: a component operator's mode, components and pivot, every transform's identity), the operator ids the
/// motion composes into, and the selection that follows the gesture onto them. A mesh-component selection splices ONE
/// component operator for the whole component set; an operator that exists already gets no input row.
pub fn gumball_splice(host_snapshot: &FlowHostSnapshot, ids: &[String], operation: &str) -> Result<(Vec<Generation3dMutation>, Vec<String>, GumballSelection), Fault> {
    with_host(host_snapshot, |host| {
        let (targets, selection, mut wanted) = if ids.iter().any(|id| ComponentTarget::parse(id).is_some()) {
            let (id, mode, components) = ensure_component_node(host, ids, operation)?;
            let wanted = component_gesture_inputs(&mode, &components, operation);
            let addressed = components.iter().map(|component| format!("{id}@meshOut#0.{mode}.{component}")).collect();
            (vec![id.clone()], GumballSelection { nodes: vec![id], components: Some((mode, addressed)) }, wanted)
        } else {
            let mut targets: Vec<String> = Vec::new();
            for id in ids {
                let next = ensure_gumball_node(host, id, operation)?;
                if !targets.contains(&next) {
                    targets.push(next);
                }
            }
            (targets.clone(), GumballSelection { nodes: targets, components: None }, Vec::new())
        };
        wanted.extend(gumball_identity(operation));
        let mut rows = commit_host_snapshot(host_snapshot, &host.host_snapshot);
        for target in &targets {
            if host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == target.as_str()) {
                continue;
            }
            if let Some(record) = host.host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == target.as_str()) {
                rows.extend(record_input_leaves(record, &wanted));
            }
        }
        Ok((rows, targets, selection))
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
    pub fn abort(&mut self, window: &str, _reason: ToolAbortReason) -> bool {
        let dropped = self.open.remove(window).is_some();
        if dropped {
            self.revision = self.revision.wrapping_add(1);
        }
        dropped
    }

    /// 👁️ What a derived view folds onto `host_snapshot` to show every open gesture: each gesture's splice re-derived on
    /// that base, then its ONE net relative leaf, and the selection the gesture carries onto its operators — a preview,
    /// never history and never the interaction store.
    pub fn provisional(&self, host_snapshot: &FlowHostSnapshot) -> GumballPreview {
        let mut preview = GumballPreview::default();
        for gesture in self.open.values() {
            let Some(motion) = gesture.leaf().and_then(GumballMotion::of_leaf) else { continue };
            let Ok((splice, targets, selection)) = gumball_splice(host_snapshot, &gesture.ids, motion.operation()) else { continue };
            preview.rows.extend(splice);
            preview.rows.push(motion.leaf(targets));
            preview.selections.push(selection);
        }
        preview
    }

    /// 🛠️ Drives `request.window`'s gumball tool through ONE dispatch on the shared streamed-gesture runner
    /// ([`drive_gesture`]): `Once` commits the motion as one transaction; `Stream` upserts it into the window's open
    /// transaction, which the app instance retains and every preview folds in; `Commit` folds the tail in and commits the
    /// whole gesture as ONE edit (the splice rows re-derived on the committed base, then the net relative leaf, every row
    /// stamped with the transaction); `Abort` drops the open gesture with zero trace. An open gesture another verb or a
    /// one-shot interrupts is aborted `captureLost`; one whose document moved under it is aborted `baseMoved`; a selection
    /// the base no longer splices drops the gesture and refuses with the gumball's named code.
    pub fn dispatch(&mut self, request: GumballDispatch<'_>, host_snapshot: &FlowHostSnapshot) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
        if let GesturePhase::Abort(reason) = request.phase {
            self.abort(request.window, reason);
            return Ok(Emit::default());
        }
        let base_revision: String = request.base_revision.iter().map(|byte| format!("{byte:02x}")).collect();
        let ids = match self.open.get(request.window).filter(|gesture| gesture.continues(request.verb, request.phase, &base_revision)) {
            Some(gesture) => gesture.ids.clone(),
            None => request.ids,
        };
        let (splice, targets, selection) = match gumball_splice(host_snapshot, &ids, request.motion.operation()) {
            Ok(parts) => parts,
            Err(fault) => {
                self.abort(request.window, ToolAbortReason::Tool);
                return Err(fault);
            }
        };
        let tick = GumballTick { ids, record: GumballRecord { targets, motion: request.motion } };
        let drive = drive_gesture::<Generation3dGumballTool>(self.open.get(request.window), request.verb, request.phase, Some(tick), request.authoring_seed, &base_revision);
        if let Some(next) = drive.next {
            match next {
                Some(gesture) => self.open.insert(request.window.to_string(), gesture),
                None => self.open.remove(request.window),
            };
            self.revision = self.revision.wrapping_add(1);
        }
        Ok(match drive.committed {
            Some((transaction, leaves)) => {
                let rows: Vec<Generation3dMutation> = splice.into_iter().chain(leaves).collect();
                let emit = if request.authoring_seed.is_empty() { Emit::mutations(rows) } else { Emit::commit_transaction(transaction, rows) };
                Emit { interaction_writes: selection.writes(), ..emit }
            }
            None => {
                retire_rows(splice);
                Emit::default()
            }
        })
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
    GumballGestures::default().dispatch(GumballDispatch { verb, window: "", ids, motion, phase: GesturePhase::Once, authoring_seed: &authoring_seed, base_revision }, &doc.snapshot.host_snapshot)
}
//#endregion 🛠️GumballTool

/// 🪡️ Splices an adjustable component transform with its DEFAULT params (the gesture's channels follow as
/// `change-widget-input` leaves, [`gumball_splice`]) and reuses it only for its own component set.
pub fn ensure_component_node(host: &mut FlowHost, ids: &[String], operation: &str) -> Result<(String, String, Vec<u32>), GumballRefusal> {
    let (target, components) = component_group(ids).map_err(GumballRefusal::ComponentSelection)?;
    if !matches!(operation, "translate" | "rotate" | "scale") { return Err(GumballRefusal::UnknownOperation); }
    if target.index != 0 { return Err(GumballRefusal::ListOutput); }
    let kind = host.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::Neuron { id, neuron_kind, .. } if id == target.widget => Some(neuron_kind),
        _ => None,
    }).ok_or(GumballRefusal::MeshMissing)?;
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let source = infos.get(kind).ok_or_else(|| GumballRefusal::KindUnavailable(kind.clone()))?;
    if !source.outputs.iter().any(|port| port.name == target.channel && !port.cardinality.is_collection() && port.value_types.iter().any(|kind| kind == "mesh")) {
        return Err(GumballRefusal::NotIndexedMesh);
    }
    let next_kind = format!("brep.mesh.{operation}Components");
    let selection = serde_json::to_string(&components).map_err(|error| GumballRefusal::ComponentSelection(error.to_string()))?;
    let current = crate::standards::v1::subsets::any::schema::gumball_widget_json(host, target.widget);
    let params = current.as_ref().and_then(|value| value.get("params"));
    let parameter = |name| params.and_then(|value| value.get(name)).and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_str);
    let inputs = host.host_snapshot.synapses.iter().filter(|wire| wire.to == target.widget).collect::<Vec<_>>();
    if kind == &next_kind && parameter("mode") == Some(target.granularity) && parameter("selection") == Some(selection.as_str()) && (operation == "translate" || parameter("pivot") == Some("selection")) && inputs.len() == 1 && inputs[0].to_port == "mesh" {
        return Ok((target.widget.into(), target.granularity.into(), components));
    }
    let output = infos.get(&next_kind).and_then(|info| info.outputs.first()).ok_or_else(|| GumballRefusal::TransformUnavailable(next_kind.clone()))?;
    let base = format!("{}__{operation}Components", target.widget);
    let mut id = base.clone();
    let mut suffix = 2;
    while host.host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id) { id = format!("{base}_{suffix}"); suffix += 1; }
    let (x, y) = host.host_snapshot.layout.get(target.widget).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    host.add_widget(&serde_json::json!({"kind":"neuron","id":id,"neuronKind":next_kind}).to_string(), x + 220.0, y).map_err(|error| GumballRefusal::HostEdit(error.to_string()))?;
    host.insert_between(target.widget, target.channel, &id, "mesh", &output.name).map_err(|error| GumballRefusal::HostEdit(error.to_string()))?;
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
