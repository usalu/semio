//! 🖥️ Sequence play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum, the
//! manifest stitch, and this app's own typed media I/O surface + plugin registration + editing host
//! (below — constitutional: general, an artifact must never depend on an app, so all three live here
//! rather than under `🗿️artifacts`).
//!
//! Command bodies live in `🎮️commands/*`, window renders in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`. `handle` →
//! `SequenceCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node. `SequenceHost` (below) is the UI-editing engine shared by more than one
//! taxonomy node (commands, windows, panels, the wasm bridge) — a helper with exactly one consumer
//! lives in that consumer's own component file instead.

use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::op::sequence_snapshot_mutations;
use crate::artifacts::sequence::{default_snapshot, SequenceCamera, SequenceEdge, SequenceFixture, SequenceSnapshot, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use crate::editor::sequence::commands::connection::{connect_steps, disconnect_steps};
use crate::editor::sequence::commands::layout::{reorganize, set_orientation};
use crate::editor::sequence::commands::locale::set_locale;
use crate::editor::sequence::commands::node_graph::{node_graph_edit, set_viewport};
use crate::editor::sequence::commands::playback::{run_command, stop_command};
use crate::editor::sequence::commands::step::{add_step, add_step_dropped, add_step_to_slot, delete_selection, move_step, remove_step, set_step_collapsed, set_step_params};
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::modes::edit;
use crate::editor::sequence::modes::edit::windows::{compiled, main, script};
use crate::editor::sequence::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::sequence::presence::{SequencePresence, SequencePresenceMutation};
use crate::editor::sequence::terminology::sequence_play_labels;
use dag::{dag_fixture_to_wire_literal, would_create_cycle, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape};
use graph::manifest::PropertyBag;
use imperative_engine::{
    compile_to_text as imperative_compile_to_text, contributions_json_from_entries, imperative_catalogue_json, imperative_module_registry, register_native_imperative_module, sync_imperative_module_contributions, Executor, Path, RunResult, Step,
};
use infinite_board_port_directed_dag as dag;
use neural_engine::{ChannelSpec, Dictionary, Registry, Value as NeuralValue};
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppDefinition, AppIo, ArtifactEditor, ArtifactView, ConfigFieldShape, ConfigFieldSpec, ConfigSpec, ConfigView,
    ContextMenuItemSpec, ContextMenuRequest, Dialect, DomainTopology, DraftView, DslValue, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel,
    Media, MediaError, MediaPayload, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::Write;
use store::EngineHandles;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_APP_ID: &str = "sequence-play";
pub use catalogue_panel::SEQUENCE_PLAY_BODY_CATALOGUE;
pub use compiled::SEQUENCE_PLAY_BODY_COMPILED;
pub use document_panel::SEQUENCE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::SEQUENCE_PLAY_BODY_INSPECTOR;
pub use main::SEQUENCE_PLAY_BODY_MAIN;
pub use script::SEQUENCE_PLAY_BODY_SCRIPT;
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "steps" interaction domain — one
/// granularity ("step"), `HierarchyProvider::Topology` from each step's own `SlotRef.owner`
/// control-flow nesting (see `SequencePlayApp::interaction_topology`). Ids are the steps' own raw
/// document ids — the SAME ids the main node-graph canvas's `NodeGraphNodeRecord.id` and the document
/// panel tree's row ids both use, so a selection made through either surface resolves identically.
pub const SEQUENCE_INTERACTION_STEPS: &str = "steps";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn sequence_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SEQUENCE_PLAY_APP_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_sequence_app`'s
/// `.artifact_kind(...)` literal (schema/media type copied verbatim) plus the extra `steps:in` input
/// port (Wave-2 port recipe): incoming computation results from an upstream workflow node insert as
/// new steps in the sequence document (see `SequencePlayApp::import_media` below).
pub async fn sequence_io() -> AppIo {
    AppIo {
        document_schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Computation, form: semio_framework::MediaForm::Sequence },
        ports: vec![semio_framework::MediaPortSpec {
            id: "steps:in".into(),
            label: "Steps".into(),
            direction: semio_framework::MediaPortDirection::In,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Computation, form: semio_framework::MediaForm::Any },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "computation.sequence".into(), name: "Sequence".into(), dimension: "graph".into(), component_kind: "sequence".into() },
    }
}

/// 🎯️ Pure next-available step id for `import_media("steps:in", ...)` — mirrors `SequenceHost::next_step_id`
/// but never mutates a host's serial counter (there is no live `SequenceHost` in a pure
/// `ArtifactApp::import_media` call): derives the next id purely from the fixture's own existing
/// `step-N`/`edge-N` ids, exactly like `SequenceHost::from_snapshot`'s own initial-serial derivation.
pub async fn next_available_step_id(fixture: &SequenceSnapshot) -> String {
    format!("step-{}", max_serial_in_snapshot(&fixture.to_fixture()).max(100) + 1)
}
//#endregion 🔖️Io

//#region 🔖️Camera
/// 🎥️ `SequenceCamera` <-> `DagCamera` conversions — plain functions rather than `From`/`Into` trait
/// impls, because `SequenceCamera` is defined in the artifact's own `🦀️.rs` and `DagCamera`
/// is foreign (from the DAG layout kernel): neither type nor trait would be local to THIS file, so a
/// trait impl here would violate the orphan rule. Only `SequenceHost` (which already depends on the DAG
/// kernel for `DagHost`) needs the conversion, so plain functions here are both legal and sufficient.
pub async fn sequence_camera_from_dag(value: &DagCamera) -> SequenceCamera {
    SequenceCamera { x: value.x, y: value.y, zoom: value.zoom }
}

pub async fn dag_camera_from_sequence(value: &SequenceCamera) -> DagCamera {
    DagCamera { x: value.x, y: value.y, zoom: value.zoom }
}
//#endregion 🔖️Camera

//#region ⚠️ Errors
/// 🚨️ `SequenceHost`'s fallible operations.
#[derive(Debug)]
pub enum SequenceCoreError {
    Json(serde_json::Error),
    UnsupportedSchema(String),
    SelfConnect,
    StepNotFound(String),
    MismatchedSlotScope,
    CycleDetected,
    OutgoingFlowExists(String),
    UnknownStep(String),
    Dag(String),
}

impl std::fmt::Display for SequenceCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::UnsupportedSchema(schema) => write!(formatter, "unsupported schema: {schema}"),
            Self::SelfConnect => formatter.write_str("cannot connect step to itself"),
            Self::StepNotFound(step) => write!(formatter, "{step} not found"),
            Self::MismatchedSlotScope => formatter.write_str("steps must share the same slot scope"),
            Self::CycleDetected => formatter.write_str("connection would create cycle"),
            Self::OutgoingFlowExists(step) => write!(formatter, "{step} already has outgoing flow"),
            Self::UnknownStep(step) => write!(formatter, "unknown step: {step}"),
            Self::Dag(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SequenceCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for SequenceCoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️Host
const SEQUENCE_DAG_COMPONENT_WIDTH: f64 = 200.0;
const SEQUENCE_DAG_CHANNEL_ROW_HEIGHT: f64 = 24.0;

async fn sequence_computation_node_width(_name: &str, _inputs: &[IoPortSpec], _outputs: &[IoPortSpec]) -> f64 {
    SEQUENCE_DAG_COMPONENT_WIDTH
}

async fn sequence_computation_node_height(input_count: usize, output_count: usize, _variadic_inputs: bool, _variadic_outputs: bool) -> f64 {
    let rows = input_count.max(output_count).max(1);
    rows as f64 * SEQUENCE_DAG_CHANNEL_ROW_HEIGHT
}
const FLOW_INPUT_PORT: &str = "prev";
const FLOW_OUTPUT_PORT: &str = "next";

async fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    serde_json::from_value(serde_json::to_value(dict).unwrap_or(Value::Null)).unwrap_or_default()
}

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
pub async fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

async fn is_function_kind(kind: &str) -> bool {
    kind.starts_with("math.") || kind.starts_with("logic.") || kind.starts_with("text.")
}

async fn parse_serial_suffix(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

async fn max_serial_in_snapshot(fixture: &SequenceFixture) -> u64 {
    let mut max = 0u64;
    for step in &fixture.steps {
        if let Some(serial) = parse_serial_suffix("step-", &step.id) {
            max = max.max(serial);
        }
    }
    for edge in &fixture.edges {
        if let Some(serial) = parse_serial_suffix("edge-", &edge.id) {
            max = max.max(serial);
        }
    }
    max
}

async fn default_control_slot(kind: &str) -> &'static str {
    if kind == "control.if" {
        "then"
    } else {
        "body"
    }
}

async fn neural_value_to_dsl_value(value: &NeuralValue) -> DslValue {
    dsl::to_dsl_value(value).unwrap_or(DslValue::Null)
}

// 🧯️ `unnecessary_wraps` — mirrors `IoPortSpec::value_type`'s `Option<String>` field shape; every
// branch here happens to be populated today, but the field itself is genuinely optional.
#[allow(clippy::unnecessary_wraps)]
async fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.operators.is_empty() {
        Some("value".into())
    } else {
        Some(spec.operators.join(","))
    }
}

async fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

async fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_dsl_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.value = value;
    port.connected = Some(false);
    port.cardinality = spec.cardinality.symbol();
    port
}

async fn hidden_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

async fn hidden_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

async fn visible_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "Previous");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

async fn visible_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "Next");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
pub async fn control_slots(kind: &str) -> &'static [&'static str] {
    match kind {
        "control.if" => &["then", "else"],
        "control.while" | "control.repeat" => &["body"],
        _ => &[],
    }
}

async fn slot_key(slot: Option<&SlotRef>) -> Option<(String, String)> {
    slot.map(|entry| (entry.owner.clone(), entry.name.clone()))
}

#[cfg(test)]
async fn ensure_imperative_modules_for_tests() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        register_native_imperative_module("imperative-extension-math", semio_s_plugin_imperative_math::register);
        register_native_imperative_module("imperative-extension-text", semio_s_plugin_imperative_text::register);
        register_native_imperative_module("imperative-extension-core", semio_s_plugin_imperative_effect::register);
        let json = contributions_json_from_entries(&[
            semio_s_plugin_imperative_math::imperative_module_contribution(),
            semio_s_plugin_imperative_text::imperative_module_contribution(),
            semio_s_plugin_imperative_effect::imperative_module_contribution(),
            semio_s_plugin_imperative_control::imperative_module_contribution(),
        ]);
        sync_imperative_module_contributions(&json);
    });
}

pub struct SequenceHost {
    /// 🌊️ The plain pre-migration document shape (`{schema, steps, edges}`) — this plugin's own
    /// working representation, matching `SequenceFixture`'s doc comment. `SequenceHost` edits this
    /// in place exactly as it edited `SequenceSnapshot.steps`/`.edges` directly before the
    /// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` migration (`sequence→C:flow`) — only the
    /// boundary conversions (`from_snapshot`/`replace_snapshot`/`to_json`/`load_json`) changed.
    pub snapshot: SequenceFixture,
    /// 🎥️ The canvas camera — session-only host state (never a `SequenceSnapshot` document field; see
    /// `crate::editor::sequence::config::SequenceConfig::camera`). Persists across `rebuild_dag()` calls
    /// within this `SequenceHost` instance (each document mutation rebuilds `dag` from scratch, so
    /// this is what the rebuilt `dag`'s camera gets reseeded from).
    pub camera: SequenceCamera,
    pub dag: DagHost,
    registry: Registry,
    next_serial: u64,
}

impl Default for SequenceHost {
    fn default() -> Self {
        Self::from_snapshot(default_snapshot())
    }
}

impl SequenceHost {
    /// 🌊️ Builds a live host from a persisted composed-child snapshot — reads the real steps/edges
    /// off the working-scene cache via `to_fixture()` (see `SequenceFixture`'s doc comment).
    pub async fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
        Self::from_fixture(snapshot.to_fixture())
    }

    /// 🌊️ Builds a live host directly from a plain fixture (the WASM bridge's `loadFixtureJson`/
    /// `SequenceHost::load_json` entry point).
    pub async fn from_fixture(fixture: SequenceFixture) -> Self {
        #[cfg(test)]
        ensure_imperative_modules_for_tests();
        let next_serial = max_serial_in_snapshot(&fixture).max(100);
        let mut host = Self {
            snapshot: fixture,
            camera: SequenceCamera::default(),
            dag: DagHost::from_fixture_without_layout(DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            registry: imperative_module_registry(),
            next_serial,
        };
        host.rebuild_dag();
        host
    }

    pub async fn replace_snapshot(&mut self, fixture: SequenceFixture) -> Result<(), SequenceCoreError> {
        if fixture.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        self.next_serial = self.next_serial.max(max_serial_in_snapshot(&fixture));
        self.snapshot = fixture;
        self.rebuild_dag();
        Ok(())
    }

    pub async fn load_json(json: &str) -> Result<Self, SequenceCoreError> {
        let fixture: SequenceFixture = serde_json::from_str(json)?;
        if fixture.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        Ok(Self::from_fixture(fixture))
    }

    pub async fn to_json(&self) -> Result<String, SequenceCoreError> {
        Ok(serde_json::to_string(&self.snapshot)?)
    }

    pub async fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub async fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
        use infinite_canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use infinite_canvas::Point;
        let viewport = Viewport { width: width.max(1), height: height.max(1), dpr: dpr.max(1.0) };
        let camera = CanvasCamera { x: self.dag.fixture.camera.x, y: self.dag.fixture.camera.y, zoom: self.dag.fixture.camera.zoom };
        let world = screen_to_world(&camera, &viewport, Point::new(sx, sy));
        for node in self.dag.fixture.nodes.iter().rev() {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world.x >= node.x - hw && world.x <= node.x + hw && world.y >= node.y - hh && world.y <= node.y + hh {
                return Some(node.id.clone());
            }
        }
        None
    }

    pub async fn add_step(&mut self, kind: &str, x: f64, y: f64) -> String {
        self.add_step_in_slot(kind, x, y, None)
    }

    pub async fn add_step_dropped(&mut self, kind: &str, x: f64, y: f64, picked_step_id: Option<&str>) -> String {
        if let Some(owner_id) = picked_step_id {
            if let Some(owner) = self.snapshot.steps.iter().find(|step| step.id == owner_id) {
                if is_control_kind(&owner.kind) && !owner.collapsed {
                    return self.add_step_in_slot(kind, x, y, Some(SlotRef { owner: owner_id.into(), name: default_control_slot(&owner.kind).into() }));
                }
            }
        }
        self.add_step(kind, x, y)
    }

    async fn next_step_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("step-{}", self.next_serial);
            if !self.snapshot.steps.iter().any(|step| step.id == id) {
                return id;
            }
        }
    }

    async fn next_edge_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("edge-{}", self.next_serial);
            if !self.snapshot.edges.iter().any(|edge| edge.id == id) {
                return id;
            }
        }
    }

    pub async fn add_step_in_slot(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        self.clear_ghost_step();
        let id = self.next_step_id();
        self.snapshot.steps.push(SequenceStep { id: id.clone(), kind: kind.into(), params: StepParams::new(), x, y, slot, collapsed: false });
        self.rebuild_dag();
        id
    }

    pub async fn set_step_collapsed(&mut self, id: &str, collapsed: bool) -> bool {
        let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == id) else {
            return false;
        };
        if !is_control_kind(&step.kind) {
            return false;
        }
        step.collapsed = collapsed;
        self.rebuild_dag();
        true
    }

    pub async fn remove_step(&mut self, id: &str) -> bool {
        let before = self.snapshot.steps.len();
        let mut remove_ids = vec![id.to_string()];
        if self.snapshot.steps.iter().any(|step| step.id == id && is_control_kind(&step.kind)) {
            for step in &self.snapshot.steps {
                if step.slot.as_ref().is_some_and(|slot| slot.owner == id) {
                    remove_ids.push(step.id.clone());
                }
            }
        }
        self.snapshot.steps.retain(|step| !remove_ids.iter().any(|remove_id| remove_id == &step.id));
        self.snapshot.edges.retain(|edge| !remove_ids.iter().any(|remove_id| remove_id == &edge.from || remove_id == &edge.to));
        if self.snapshot.steps.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub async fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), SequenceCoreError> {
        let params: StepParams = serde_json::from_str(json)?;
        let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == id) else {
            return Err(SequenceCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        self.rebuild_dag();
        Ok(())
    }

    pub async fn connect_steps(&mut self, from_id: &str, to_id: &str) -> Result<String, SequenceCoreError> {
        if from_id == to_id {
            return Err(SequenceCoreError::SelfConnect);
        }
        let from_step = self.snapshot.steps.iter().find(|step| step.id == from_id).ok_or_else(|| SequenceCoreError::StepNotFound(from_id.into()))?;
        let to_step = self.snapshot.steps.iter().find(|step| step.id == to_id).ok_or_else(|| SequenceCoreError::StepNotFound(to_id.into()))?;
        if slot_key(from_step.slot.as_ref()) != slot_key(to_step.slot.as_ref()) {
            return Err(SequenceCoreError::MismatchedSlotScope);
        }
        let existing: Vec<(String, String)> = self.snapshot.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(SequenceCoreError::CycleDetected);
        }
        if self.snapshot.edges.iter().any(|edge| edge.from == from_id) {
            return Err(SequenceCoreError::OutgoingFlowExists(from_id.into()));
        }
        if self.snapshot.edges.iter().any(|edge| edge.to == to_id) {
            self.snapshot.edges.retain(|edge| edge.to != to_id);
        }
        let id = self.next_edge_id();
        self.snapshot.edges.push(SequenceEdge { id: id.clone(), from: from_id.into(), to: to_id.into() });
        self.rebuild_dag();
        Ok(id)
    }

    pub async fn disconnect_steps(&mut self, from_id: &str, to_id: &str) -> bool {
        let before = self.snapshot.edges.len();
        self.snapshot.edges.retain(|edge| !(edge.from == from_id && edge.to == to_id));
        if self.snapshot.edges.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub async fn sync_edges_from_dag(&mut self) {
        let dag_pairs: Vec<(String, String)> = self
            .dag
            .fixture
            .edges
            .iter()
            .filter_map(|dag_edge| {
                let from = dag_edge.source.split('@').next()?;
                let to = dag_edge.target.split('@').next()?;
                if from == to {
                    return None;
                }
                Some((from.into(), to.into()))
            })
            .collect();
        let mut edges = Vec::new();
        for (from, to) in dag_pairs {
            // 🧯️ `map_unwrap_or` — `map_or_else` would need to build the `&mut self`-capturing
            // fallback closure alongside the `self.snapshot.edges`-borrowing lookup in one call,
            // which the borrow checker rejects; the two-step form sequences the borrows correctly.
            #[allow(clippy::map_unwrap_or)]
            let id = self.snapshot.edges.iter().find(|edge| edge.from == from && edge.to == to).map(|edge| edge.id.clone()).unwrap_or_else(|| self.next_edge_id());
            edges.push(SequenceEdge { id, from, to });
        }
        self.snapshot.edges = edges;
    }

    pub async fn sync_from_dag(&mut self) {
        self.camera = sequence_camera_from_dag(&self.dag.fixture.camera);
        self.sync_edges_from_dag();
        for step in &mut self.snapshot.steps {
            let Some(node) = self.dag.fixture.nodes.iter().find(|node| node.id == step.id) else {
                continue;
            };
            step.x = node.x;
            step.y = node.y;
        }
    }

    pub async fn build_path(&self) -> Path {
        self.build_path_for_slot(None)
    }

    pub async fn build_path_json(&self) -> Result<String, SequenceCoreError> {
        Ok(serde_json::to_string(&self.build_path())?)
    }

    async fn build_path_for_slot(&self, slot: Option<&SlotRef>) -> Path {
        let slot_filter = slot_key(slot);
        let scoped_steps: Vec<&SequenceStep> = self.snapshot.steps.iter().filter(|step| slot_key(step.slot.as_ref()) == slot_filter).collect();
        let incoming: HashMap<&str, &str> = self.snapshot.edges.iter().map(|edge| (edge.to.as_str(), edge.from.as_str())).collect();
        let outgoing: HashMap<&str, &str> = self.snapshot.edges.iter().map(|edge| (edge.from.as_str(), edge.to.as_str())).collect();
        let heads: Vec<&SequenceStep> = scoped_steps.iter().copied().filter(|step| !incoming.contains_key(step.id.as_str())).collect();
        let start = if heads.len() == 1 {
            heads[0].id.as_str()
        } else if scoped_steps.len() == 1 {
            scoped_steps[0].id.as_str()
        } else {
            return Path { steps: scoped_steps.iter().map(|step| self.step_to_imperative_step(step)).collect() };
        };
        let mut ordered = Vec::new();
        let mut by_id: BTreeMap<&str, &SequenceStep> = scoped_steps.into_iter().map(|step| (step.id.as_str(), step)).collect();
        let mut current = Some(start);
        let mut visited = std::collections::HashSet::new();
        while let Some(id) = current {
            if !visited.insert(id) {
                break;
            }
            if let Some(step) = by_id.remove(id) {
                ordered.push(self.step_to_imperative_step(step));
            }
            current = outgoing.get(id).copied();
        }
        for step in by_id.values() {
            ordered.push(self.step_to_imperative_step(step));
        }
        Path { steps: ordered }
    }

    async fn step_to_imperative_step(&self, step: &SequenceStep) -> Step {
        let mut bodies = BTreeMap::new();
        if is_control_kind(&step.kind) {
            for slot_name in control_slots(&step.kind) {
                let slot_ref = SlotRef { owner: step.id.clone(), name: slot_name.to_string() };
                bodies.insert(slot_name.to_string(), self.build_path_for_slot(Some(&slot_ref)));
            }
        }
        Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies }
    }

    async fn is_step_visible(&self, step: &SequenceStep) -> bool {
        let Some(slot) = &step.slot else {
            return true;
        };
        let Some(owner) = self.snapshot.steps.iter().find(|entry| entry.id == slot.owner) else {
            return false;
        };
        !owner.collapsed
    }

    async fn slot_member_count(&self, owner_id: &str) -> usize {
        self.snapshot.steps.iter().filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == owner_id)).count()
    }

    pub async fn layout_expanded_slots(&mut self) {
        let control_steps: Vec<(String, String, bool)> = self.snapshot.steps.iter().filter(|step| is_control_kind(&step.kind)).map(|step| (step.id.clone(), step.kind.clone(), step.collapsed)).collect();
        for (owner_id, kind, collapsed) in control_steps {
            if collapsed {
                continue;
            }
            let owner = self.snapshot.steps.iter().find(|step| step.id == owner_id);
            let Some(owner) = owner else { continue };
            let base_x = owner.x;
            let base_y = owner.y + 160.0;
            for (index, slot_name) in control_slots(&kind).iter().enumerate() {
                let slot_ref = SlotRef { owner: owner_id.clone(), name: (*slot_name).into() };
                let members: Vec<String> = self.snapshot.steps.iter().filter(|step| step.slot.as_ref() == Some(&slot_ref)).map(|step| step.id.clone()).collect();
                let offset_x = base_x + (index as f64 - (control_slots(&kind).len() as f64 - 1.0) * 0.5) * 320.0;
                for (member_index, member_id) in members.iter().enumerate() {
                    if let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == *member_id) {
                        step.x = offset_x + member_index as f64 * 280.0;
                        step.y = base_y;
                    }
                }
            }
        }
        self.rebuild_dag();
    }

    /// 🌳️ Recomputes visible step positions using the shared layered DAG tree layout, then rebuilds the DAG view.
    pub async fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), SequenceCoreError> {
        self.dag.reorganize(opts).map_err(|e| SequenceCoreError::Dag(e.to_string()))?;
        let positions: HashMap<String, (f64, f64)> = self.dag.fixture.nodes.iter().map(|node| (node.id.clone(), (node.x, node.y))).collect();
        for step in self.snapshot.steps.iter_mut() {
            if let Some(&(x, y)) = positions.get(&step.id) {
                step.x = x;
                step.y = y;
            }
        }
        self.rebuild_dag();
        Ok(())
    }

    pub async fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.build_path(), &Dictionary::new())
    }

    pub async fn compile_text(&self) -> String {
        imperative_compile_to_text(&self.build_path())
    }

    /// 📝️ Renders the compiled DAG fixture as wire-literal text.
    pub async fn compiled_wire_literal(&self) -> String {
        dag_fixture_to_wire_literal(&self.build_dag_fixture())
    }

    async fn rebuild_dag(&mut self) {
        let selected = self.dag.selected_node_ids()?;
        let dag_fixture = self.build_dag_fixture();
        self.dag = DagHost::from_fixture_without_layout(dag_fixture);
        self.dag.set_camera(self.camera.x, self.camera.y, self.camera.zoom);
        if !selected.is_empty() {
            self.dag.set_selection(&selected);
        }
    }

    async fn build_dag_fixture(&self) -> DagFixture {
        let nodes: Vec<DagNodeSpec> = self.snapshot.steps.iter().filter(|step| self.is_step_visible(step)).map(|step| self.step_to_dag_node(step)).collect();
        let visible_ids: std::collections::HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let existing: Vec<(String, String)> = self.snapshot.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        let edges: Vec<DagFixtureEdge> = self
            .snapshot
            .edges
            .iter()
            .filter(|edge| visible_ids.contains(&edge.from) && visible_ids.contains(&edge.to))
            .filter(|edge| !would_create_cycle(&existing, &edge.from, &edge.to))
            .map(|edge| DagFixtureEdge { id: edge.id.clone(), source: format!("{}@{}", edge.from, FLOW_OUTPUT_PORT), target: format!("{}@{}", edge.to, FLOW_INPUT_PORT), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::new() })
            .collect();
        DagFixture { schema: "dag.fixture".into(), camera: dag_camera_from_sequence(&self.camera), nodes, edges }
    }

    async fn step_to_dag_node(&self, step: &SequenceStep) -> DagNodeSpec {
        let info = self.registry.operator_info(&step.kind);
        let (name, mut abbreviation, icon) = info.as_ref().map_or_else(|| (step.kind.clone(), step.kind.clone(), "emoji:⚡️".into()), |entry| (entry.name.clone(), entry.abbreviation.clone(), entry.icon.clone()));
        if is_control_kind(&step.kind) {
            let count = self.slot_member_count(&step.id);
            abbreviation = if step.collapsed { format!("▸️ {count}") } else { format!("▾️ {count}") };
        }
        // 🛡️ falls back to execution-only ports for a function-kind step whose kind isn't (yet) registered,
        // rather than assuming the registry always resolves it — matches the non-function-kind fallback below.
        let (inputs, outputs) = match info.filter(|_| is_function_kind(&step.kind)) {
            Some(info) => {
                let mut inputs: Vec<IoPortSpec> = info.inputs.iter().map(|spec| input_spec_to_port(spec, &step.params)).collect();
                let mut outputs: Vec<IoPortSpec> = info.outputs.iter().map(channel_spec_to_output_port).collect();
                if outputs.is_empty() {
                    outputs.push(channel_spec_to_output_port(&ChannelSpec::wildcard()));
                }
                inputs.push(hidden_flow_input_port());
                outputs.push(hidden_flow_output_port());
                (inputs, outputs)
            }
            None => (vec![visible_flow_input_port()], vec![visible_flow_output_port()]),
        };
        let width = sequence_computation_node_width(&name, &inputs, &outputs);
        let height = sequence_computation_node_height(inputs.len(), outputs.len(), false, false);
        let mut node = DagNodeSpec::computation(step.id.clone(), name, abbreviation, icon, inputs, outputs, false, false, step.x, step.y, width, height);
        node.operator_kind = Some(step.kind.clone());
        node.properties = property_bag_from_dictionary(&step.params);
        node
    }

    pub async fn set_ghost_step(&mut self, kind: &str, x: f64, y: f64) {
        let ghost = SequenceStep { id: "__ghost__".into(), kind: kind.into(), params: StepParams::new(), x, y, slot: None, collapsed: false };
        let node = self.step_to_dag_node(&ghost);
        self.dag.set_ghost_node(Some(node));
    }

    pub async fn clear_ghost_step(&mut self) {
        self.dag.set_ghost_node(None);
    }
}
//#endregion 🔖️Host

//#region 🔖️HostHelpers
/// 🧰️ Builds a {@link SequenceHost} seeded from a projection so a command can mutate it (with all the
/// host's cycle/slot/layout logic) and then diff the result into typed operations. More than one
/// consumer across the taxonomy tree (commands, windows), so it lives here rather than in a single
/// caller's file.
pub async fn host_from_snapshot(fixture: &SequenceSnapshot) -> SequenceHost {
    SequenceHost::from_snapshot(fixture.clone())
}

/// 🔀️ Runs a host mutation seeded from `fixture` and diffs the result into typed operations — a free
/// function (not a method) since `SequencePlayApp` is a unit struct with nothing to borrow.
pub async fn ops_from_host_mutation(fixture: &SequenceSnapshot, mutate: impl FnOnce(&mut SequenceHost)) -> Vec<SequenceMutation> {
    let mut host = host_from_snapshot(fixture);
    mutate(&mut host);
    sequence_snapshot_mutations(&fixture.to_fixture(), &host.snapshot)
}
//#endregion 🔖️HostHelpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SequencePlayApp::Command` — the SOLE dispatch surface for sequence's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword
    /// (the kebab-case `#[dsl(key = ..)]` the codec uses) — every row's wire keyword happens to be the
    /// plain kebab-case of its id (no `flow`-style divergence here), but the two are still copied
    /// independently from the pre-migration `sequence_protocol` enum's `command_id()` match arm and
    /// `#[dsl(key = ..)]` attribute respectively, never derived one from the other. **Row order is the
    /// binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SequenceCommand for SequenceSnapshot, SequenceMutation, SequenceConfig, SequenceConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepToSlot" as "add-step-to-slot" => add_step_to_slot::AddStepToSlot,
        "addStepDropped" as "add-step-dropped" => add_step_dropped::AddStepDropped,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "moveStep" as "move-step" => move_step::MoveStep,
        "connectSteps" as "connect-steps" => connect_steps::ConnectSteps,
        "disconnectSteps" as "disconnect-steps" => disconnect_steps::DisconnectSteps,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepCollapsed" as "set-step-collapsed" => set_step_collapsed::SetStepCollapsed,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "setOrientation" as "set-orientation" => set_orientation::SetOrientation,
        "run" as "run" => run_command::Run,
        "stop" as "stop" => stop_command::Stop,
        "setViewport" as "set-viewport" => set_viewport::SetViewport,
        "setLocale" as "set-locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 📬️ArtifactStorePreparation
const SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS: usize = 256;
const SEQUENCE_STORE_MAXIMUM_BYTES: usize = 65_536;

struct SequenceArtifactStorePreparationFactory;

struct SequenceArtifactStorePreparation {
    base: Option<store::SnapshotRead<SequenceSnapshot>>,
    mutation: Option<SequenceMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<SequenceSnapshot, SequenceMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

struct SequenceBoundedByteCounter {
    written: usize,
    maximum_bytes: usize,
}

impl Write for SequenceBoundedByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self.written.checked_add(bytes.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Sequence retained byte count overflow"))?;
        if next > self.maximum_bytes {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Sequence retained value exceeds its byte cap"));
        }
        self.written = next;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn sequence_bounded_serialized_bytes<T: serde::Serialize>(value: &T, maximum_bytes: usize) -> Result<usize, String> {
    let mut counter = SequenceBoundedByteCounter { written: 0, maximum_bytes };
    serde_json::to_writer(&mut counter, value).map_err(|error| error.to_string())?;
    Ok(counter.written)
}

fn admit_sequence_artifact_mutation(mutation: &SequenceMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    if matches!(mutation, SequenceMutation::DuplicateStep(_)) {
        return Err("Sequence retained Store authority does not admit the unregistered duplicate-step mutation".into());
    }
    let retained_bytes = sequence_bounded_serialized_bytes(mutation, SEQUENCE_RETAINED_RAW_BYTES)?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn sequence_delete_inverse(scene: &SequenceWorkingScene) -> Vec<SequenceMutation> {
    let mut inverse = Vec::with_capacity(scene.steps.len().saturating_mul(2).saturating_add(scene.edges.len()));
    for step in &scene.steps {
        inverse.push(SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id: step.id.clone() }));
    }
    for step in &scene.steps {
        inverse.push(SequenceMutation::CreateStep(crate::artifacts::sequence::mutations::CreateStep { step: step.clone() }));
    }
    for edge in &scene.edges {
        inverse.push(SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }));
    }
    inverse.reverse();
    inverse
}

fn prepare_sequence_artifact(base: &SequenceSnapshot, mutation: SequenceMutation) -> Result<(SequenceSnapshot, Vec<SequenceMutation>, SequenceMutation), String> {
    admit_sequence_artifact_mutation(&mutation)?;
    let owner = base.content.local_owner::<SequenceWorkingScene>().ok_or_else(|| "Sequence artifact base has no exact child-owned scene".to_string())?;
    if owner.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || owner.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err("Sequence artifact base exceeds its fixed scene-item cap".into());
    }
    sequence_bounded_serialized_bytes(&(&owner.steps, &owner.edges), SEQUENCE_STORE_MAXIMUM_BYTES)?;
    let mut scene = owner.as_ref().clone();
    let inverse = match &mutation {
        SequenceMutation::CreateStep(payload) => {
            if scene.steps.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.steps.iter().any(|step| step.id == payload.step.id) {
                return Err("Sequence create-step rejected duplicate or capped identity".into());
            }
            scene.steps.push(payload.step.clone());
            vec![SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id: payload.step.id.clone() })]
        }
        SequenceMutation::DeleteStep(payload) => {
            if !scene.steps.iter().any(|step| step.id == payload.id) {
                return Err(format!("Sequence delete-step target {:?} is missing", payload.id));
            }
            let inverse = sequence_delete_inverse(&scene);
            scene.steps.retain(|step| step.id != payload.id);
            scene.edges.retain(|edge| edge.from != payload.id && edge.to != payload.id);
            inverse
        }
        SequenceMutation::MoveStep(payload) => {
            if !payload.x.is_finite() || !payload.y.is_finite() {
                return Err("Sequence move-step position must be finite".into());
            }
            let step = scene.steps.iter_mut().find(|step| step.id == payload.id).ok_or_else(|| format!("Sequence move-step target {:?} is missing", payload.id))?;
            if step.x == payload.x && step.y == payload.y {
                return Err("Sequence move-step is a no-op".into());
            }
            let inverse = SequenceMutation::MoveStep(crate::artifacts::sequence::mutations::MoveStep { id: payload.id.clone(), x: step.x, y: step.y });
            step.x = payload.x;
            step.y = payload.y;
            vec![inverse]
        }
        SequenceMutation::EditStepParams(payload) => {
            let step = scene.steps.iter_mut().find(|step| step.id == payload.id).ok_or_else(|| format!("Sequence edit-step-params target {:?} is missing", payload.id))?;
            if step.params == payload.params {
                return Err("Sequence edit-step-params is a no-op".into());
            }
            let inverse = SequenceMutation::EditStepParams(crate::artifacts::sequence::mutations::EditStepParams { id: payload.id.clone(), params: step.params.clone() });
            step.params = payload.params.clone();
            vec![inverse]
        }
        SequenceMutation::ChangeStepCollapsed(payload) => {
            let step = scene.steps.iter_mut().find(|step| step.id == payload.id).ok_or_else(|| format!("Sequence change-step-collapsed target {:?} is missing", payload.id))?;
            if step.collapsed == payload.collapsed {
                return Err("Sequence change-step-collapsed is a no-op".into());
            }
            let inverse = SequenceMutation::ChangeStepCollapsed(crate::artifacts::sequence::mutations::ChangeStepCollapsed { id: payload.id.clone(), collapsed: step.collapsed });
            step.collapsed = payload.collapsed;
            vec![inverse]
        }
        SequenceMutation::ConnectSteps(payload) => {
            if payload.from == payload.to
                || !scene.steps.iter().any(|step| step.id == payload.from)
                || !scene.steps.iter().any(|step| step.id == payload.to)
                || scene.edges.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
                || scene.edges.iter().any(|edge| edge.id == payload.id || edge.from == payload.from && edge.to == payload.to)
            {
                return Err("Sequence connect-steps rejected invalid endpoints, duplicate, or capped edge".into());
            }
            scene.edges.push(SequenceEdge { id: payload.id.clone(), from: payload.from.clone(), to: payload.to.clone() });
            vec![SequenceMutation::DisconnectSteps(crate::artifacts::sequence::mutations::DisconnectSteps { id: payload.id.clone() })]
        }
        SequenceMutation::DisconnectSteps(payload) => {
            let edge = scene.edges.iter().find(|edge| edge.id == payload.id).cloned().ok_or_else(|| format!("Sequence disconnect-steps target {:?} is missing", payload.id))?;
            scene.edges.retain(|entry| entry.id != payload.id);
            vec![SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: edge.id, from: edge.from, to: edge.to })]
        }
        SequenceMutation::DuplicateStep(_) => return Err("Sequence duplicate-step has no retained route authority".into()),
    };
    sequence_bounded_serialized_bytes(&inverse, SEQUENCE_STORE_MAXIMUM_BYTES)?;
    sequence_bounded_serialized_bytes(&(&scene.steps, &scene.edges), SEQUENCE_STORE_MAXIMUM_BYTES)?;
    let content = crate::artifacts::sequence::sequence_content_child_with_owner(scene.steps, scene.edges);
    let post = SequenceSnapshot { schema: base.schema.clone(), content };
    Ok((post, inverse, mutation))
}

fn sequence_artifact_store_edit(forward: SequenceMutation, inverse: Vec<SequenceMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<SequenceMutation> {
    let id = format!("sequence-artifact-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<SequenceSnapshot, SequenceMutation> for SequenceArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &SequenceMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Sequence artifact preparation rejected its lane or description envelope".into());
        }
        admit_sequence_artifact_mutation(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<SequenceSnapshot, SequenceMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SequenceSnapshot, SequenceMutation>>, store::ArtifactStoreOneItemPreparationRequest<SequenceSnapshot, SequenceMutation>> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(SequenceArtifactStorePreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<SequenceSnapshot, SequenceMutation> for SequenceArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        let base = self.base.as_ref().ok_or_else(|| "Sequence artifact preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Sequence artifact preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_sequence_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Sequence artifact preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(sequence_artifact_store_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<SequenceSnapshot, SequenceMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<SequenceSnapshot, SequenceMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Sequence artifact preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ArtifactStorePreparation

//#region 📬️ConfigStorePreparation
const SEQUENCE_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;

struct SequenceConfigStorePreparationFactory;

struct SequenceConfigStorePreparation {
    base: Option<store::SnapshotRead<SequenceConfig>>,
    mutation: Option<SequenceConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<SequenceConfig, SequenceConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn sequence_config_retained_bytes(config: &SequenceConfig) -> usize {
    config.last_run_json.len() + config.orientation.len() + config.locale.len()
}

fn sequence_config_mutation_retained_bytes(mutation: &SequenceConfigMutation) -> usize {
    match mutation {
        SequenceConfigMutation::Snapshot { config } => sequence_config_retained_bytes(config),
        SequenceConfigMutation::SetLastRun { json } => json.len(),
        SequenceConfigMutation::SetOrientation { value } | SequenceConfigMutation::SetLocale { value } => value.len(),
        SequenceConfigMutation::SetCamera { .. } => 0,
    }
}

fn admit_sequence_config_mutation(mutation: &SequenceConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = sequence_config_mutation_retained_bytes(mutation);
    if retained_bytes > SEQUENCE_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sequence config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_sequence_config(base: &SequenceConfig, mutation: SequenceConfigMutation) -> Result<(SequenceConfig, Vec<SequenceConfigMutation>, SequenceConfigMutation), String> {
    admit_sequence_config_mutation(&mutation)?;
    if sequence_config_retained_bytes(base) > SEQUENCE_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sequence config base exceeds its fixed retained preparation envelope".into());
    }
    let mut post = base.clone();
    match &mutation {
        SequenceConfigMutation::Snapshot { config } => post = config.clone(),
        SequenceConfigMutation::SetLastRun { json } => post.last_run_json = json.clone(),
        SequenceConfigMutation::SetOrientation { value } => post.orientation = value.clone(),
        SequenceConfigMutation::SetCamera { camera } => post.camera = camera.clone(),
        SequenceConfigMutation::SetLocale { value } => post.locale = value.clone(),
    }
    Ok((post, vec![SequenceConfigMutation::Snapshot { config: base.clone() }], mutation))
}

fn sequence_config_store_edit(forward: SequenceConfigMutation, inverse: Vec<SequenceConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<SequenceConfigMutation> {
    let id = format!("sequence-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<SequenceConfig, SequenceConfigMutation> for SequenceConfigStorePreparationFactory {
    fn preflight(&self, mutation: &SequenceConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Sequence config preparation rejected its lane or description envelope".into());
        }
        admit_sequence_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<SequenceConfig, SequenceConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SequenceConfig, SequenceConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<SequenceConfig, SequenceConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SequenceConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<SequenceConfig, SequenceConfigMutation> for SequenceConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Sequence config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Sequence config preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_sequence_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Sequence config preparation lost its Store authority".to_string())?;
        let edit = sequence_config_store_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<SequenceConfig, SequenceConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<SequenceConfig, SequenceConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Sequence config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 🧵️RetainedArtifactRoutes
const SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA: &str = "sequence.play/retained-artifact-command.v1";
const SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &[
    "addStep", "addStepToSlot", "addStepDropped", "removeStep", "deleteSelection", "moveStep", "connectSteps", "disconnectSteps", "setStepParams", "setStepCollapsed",
];
const SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepToSlot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepDropped", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepParams", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepCollapsed", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
];

fn sequence_retained_id_admitted(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128
}

fn sequence_retained_artifact_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::AddStep(payload) => sequence_retained_id_admitted(&payload.kind) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepToSlot(payload) => sequence_retained_id_admitted(&payload.kind) && sequence_retained_id_admitted(&payload.owner) && sequence_retained_id_admitted(&payload.slot_name) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepDropped(payload) => sequence_retained_id_admitted(&payload.kind) && payload.picked_step_id.as_deref().is_none_or(sequence_retained_id_admitted) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::RemoveStep(payload) => sequence_retained_id_admitted(&payload.id),
        SequenceCommand::DeleteSelection(_) => true,
        SequenceCommand::MoveStep(payload) => sequence_retained_id_admitted(&payload.node_id) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::ConnectSteps(payload) => sequence_retained_id_admitted(&payload.source_node_id) && sequence_retained_id_admitted(&payload.target_node_id),
        SequenceCommand::DisconnectSteps(payload) => sequence_retained_id_admitted(&payload.from_id) && sequence_retained_id_admitted(&payload.to_id),
        SequenceCommand::SetStepParams(payload) => sequence_retained_id_admitted(&payload.id) && payload.params_json.len() <= SEQUENCE_RETAINED_RAW_BYTES,
        SequenceCommand::SetStepCollapsed(payload) => sequence_retained_id_admitted(&payload.id),
        _ => false,
    }
}

fn sequence_retained_serial(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

fn sequence_retained_next_id(scene: &SequenceWorkingScene, prefix: &str) -> String {
    let step_max = scene.steps.iter().filter_map(|step| sequence_retained_serial("step-", &step.id)).max().unwrap_or(0);
    let edge_max = scene.edges.iter().filter_map(|edge| sequence_retained_serial("edge-", &edge.id)).max().unwrap_or(0);
    format!("{prefix}-{}", step_max.max(edge_max).max(100).saturating_add(1))
}

fn sequence_retained_is_control(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn sequence_retained_default_slot(kind: &str) -> &'static str {
    if kind == "control.if" { "then" } else { "body" }
}

fn sequence_retained_create_step(scene: &SequenceWorkingScene, kind: String, x: f64, y: f64, slot: Option<SlotRef>) -> SequenceMutation {
    SequenceMutation::CreateStep(crate::artifacts::sequence::mutations::CreateStep {
        step: SequenceStep { id: sequence_retained_next_id(scene, "step"), kind, params: StepParams::new(), x, y, slot, collapsed: false },
    })
}

fn sequence_retained_delete_ids(scene: &SequenceWorkingScene, roots: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut selected = Vec::new();
    for root in roots {
        if !selected.iter().any(|id| id == &root) && scene.steps.iter().any(|step| step.id == root) {
            selected.push(root.clone());
        }
        if scene.steps.iter().any(|step| step.id == root && sequence_retained_is_control(&step.kind)) {
            for step in &scene.steps {
                if step.slot.as_ref().is_some_and(|slot| slot.owner == root) && !selected.iter().any(|id| id == &step.id) {
                    selected.push(step.id.clone());
                }
            }
        }
    }
    scene.steps.iter().filter(|step| selected.iter().any(|id| id == &step.id)).map(|step| step.id.clone()).collect()
}

fn sequence_retained_artifact_emit(command: &SequenceCommand, snapshot: &SequenceSnapshot, interaction: &protocol::InteractionState) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
    let owner = snapshot.content.local_owner::<SequenceWorkingScene>().ok_or_else(|| Fault::from("sequence-retained-scene-owner-missing"))?;
    if owner.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || owner.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err(Fault::from("sequence-retained-scene-capacity"));
    }
    let scene = owner.as_ref();
    let mutations = match command {
        SequenceCommand::AddStep(payload) => vec![sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, None)],
        SequenceCommand::AddStepToSlot(payload) => vec![sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }))],
        SequenceCommand::AddStepDropped(payload) => {
            let slot = payload.picked_step_id.as_ref().and_then(|owner_id| {
                scene.steps.iter().find(|step| step.id == *owner_id && sequence_retained_is_control(&step.kind) && !step.collapsed).map(|owner| SlotRef { owner: owner_id.clone(), name: sequence_retained_default_slot(&owner.kind).into() })
            });
            vec![sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, slot)]
        }
        SequenceCommand::RemoveStep(payload) => sequence_retained_delete_ids(scene, [payload.id.clone()]).into_iter().map(|id| SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id })).collect(),
        SequenceCommand::DeleteSelection(_) => {
            let selected = interaction.selection(SEQUENCE_INTERACTION_STEPS).ids.clone();
            if selected.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-retained-selection-capacity")); }
            sequence_retained_delete_ids(scene, selected).into_iter().map(|id| SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id })).collect()
        }
        SequenceCommand::MoveStep(payload) => scene.steps.iter().find(|step| step.id == payload.node_id).filter(|step| step.x != payload.x || step.y != payload.y).map(|_| vec![SequenceMutation::MoveStep(crate::artifacts::sequence::mutations::MoveStep { id: payload.node_id.clone(), x: payload.x, y: payload.y })]).unwrap_or_default(),
        SequenceCommand::SetStepParams(payload) => {
            let params = serde_json::from_str::<StepParams>(&payload.params_json).ok();
            match (scene.steps.iter().find(|step| step.id == payload.id), params) {
                (Some(step), Some(params)) if step.params != params => vec![SequenceMutation::EditStepParams(crate::artifacts::sequence::mutations::EditStepParams { id: payload.id.clone(), params })],
                _ => Vec::new(),
            }
        }
        SequenceCommand::SetStepCollapsed(payload) => scene.steps.iter().find(|step| step.id == payload.id && sequence_retained_is_control(&step.kind)).map(|step| vec![SequenceMutation::ChangeStepCollapsed(crate::artifacts::sequence::mutations::ChangeStepCollapsed { id: payload.id.clone(), collapsed: !step.collapsed })]).unwrap_or_default(),
        SequenceCommand::DisconnectSteps(payload) => scene.edges.iter().filter(|edge| edge.from == payload.from_id && edge.to == payload.to_id).map(|edge| SequenceMutation::DisconnectSteps(crate::artifacts::sequence::mutations::DisconnectSteps { id: edge.id.clone() })).collect(),
        SequenceCommand::ConnectSteps(payload) => {
            let from = scene.steps.iter().find(|step| step.id == payload.source_node_id);
            let to = scene.steps.iter().find(|step| step.id == payload.target_node_id);
            let same_slot = from.zip(to).is_some_and(|(from, to)| from.slot.as_ref().map(|slot| (&slot.owner, &slot.name)) == to.slot.as_ref().map(|slot| (&slot.owner, &slot.name)));
            let existing: Vec<(String, String)> = scene.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
            if payload.source_node_id == payload.target_node_id || !same_slot || would_create_cycle(&existing, &payload.source_node_id, &payload.target_node_id) || scene.edges.iter().any(|edge| edge.from == payload.source_node_id) {
                Vec::new()
            } else {
                let mut result: Vec<SequenceMutation> = scene.edges.iter().filter(|edge| edge.to == payload.target_node_id).map(|edge| SequenceMutation::DisconnectSteps(crate::artifacts::sequence::mutations::DisconnectSteps { id: edge.id.clone() })).collect();
                result.push(SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: sequence_retained_next_id(scene, "edge"), from: payload.source_node_id.clone(), to: payload.target_node_id.clone() }));
                result
            }
        }
        _ => return Err(Fault::from("sequence-retained-artifact-route-mismatch")),
    };
    if mutations.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-retained-artifact-output-capacity")); }
    Ok(Emit::mutations(mutations))
}

struct SequenceRetainedArtifactWork {
    tool_id: &'static str,
    workspace_identity: u64,
    cursor: usize,
    replay_target: Option<usize>,
    completed: bool,
    closing: bool,
}

impl SequenceRetainedArtifactWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedArtifactWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn workspace_identity(&self) -> u64 { self.workspace_identity }
    fn extent(&self, _command: &SequenceCommand, snapshot: &SequenceSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>) -> Option<usize> {
        let scene = snapshot.content.local_owner::<SequenceWorkingScene>()?;
        (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && interaction.selection(SEQUENCE_INTERACTION_STEPS).ids.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS).then_some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(&mut self, command: &SequenceCommand, snapshot: &SequenceSnapshot, _config: &SequenceConfig, _history: &semio_framework_plugin::HistoryView, interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>, _operation: &semio_framework_plugin::AppOperationContext) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_artifact_command_admitted(command) { return Err(Fault::from("sequence-retained-artifact-envelope")); }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target { self.replay_target = None; }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-artifact-replay", preview: b"{\"en\":\"Restoring Sequence edit\",\"de\":\"Sequenzbearbeitung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 { return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-artifact-prepare", preview: b"{\"en\":\"Preparing Sequence edit\",\"de\":\"Sequenzbearbeitung wird vorbereitet\"}" }); }
        let emit = sequence_retained_artifact_emit(command, snapshot, interaction)?;
        if !emit.config_mutations.is_empty() || !emit.draft_mutations.is_empty() || !emit.child_emits.is_empty() || emit.artifact_mutations.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-retained-artifact-publication-lane")); }
        sequence_bounded_serialized_bytes(&emit.artifact_mutations, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| Fault::from("sequence-retained-artifact-output-bytes"))?;
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 { return Err(Fault::from("sequence-retained-artifact-checkpoint-capacity")); }
        target[..24].fill(0); target[..4].copy_from_slice(b"SRA1"); target[4] = u8::from(self.completed); target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes()); target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes()); Ok(24)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRA1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] { return Err(Fault::from("sequence-retained-artifact-checkpoint-invalid")); }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("sequence-retained-artifact-checkpoint-cursor"))?)).map_err(|_| Fault::from("sequence-retained-artifact-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("sequence-retained-artifact-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS { return Err(Fault::from("sequence-retained-artifact-checkpoint-owner-mismatch")); }
        self.cursor = 0; self.replay_target = (cursor != 0).then_some(cursor); self.completed = false; Ok(())
    }

    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        if maximum_items == 0 { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
        self.replay_target = None; semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.replay_target.is_none() }
}

struct SequenceRetainedArtifactJobFactory { keys: Vec<semio_framework::ToolFactoryKey> }
impl SequenceRetainedArtifactJobFactory { fn new(controller_id: &str) -> Self { Self { keys: SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl semio_framework::ToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA }
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 2_000, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) { return Err((semio_framework::ToolJobFactoryError::new("Sequence retained artifact command rejects oversized wire or checkpoint owner"), input, checkpoint)); }
        Ok(match checkpoint { Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint), None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input) })
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedArtifactRoutes

//#region 🧵️PersistentRemainingRoutes
const SEQUENCE_PERSISTENT_MAXIMUM_UNITS: usize = 66_049;
const SEQUENCE_PERSISTENT_TOOL_IDS: &[&str] = &["reorganize", "nodeGraphEdit", "run"];
const SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "run", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
];

enum SequencePersistentAdvance {
    Progress(&'static str, &'static [u8]),
    Complete(Emit<SequenceMutation, SequenceConfigMutation>),
}

#[derive(Default)]
struct SequenceReorganizeState {
    initialized: usize,
    pass: usize,
    edge: usize,
    emit: usize,
    depths: Vec<usize>,
    mutations: Vec<SequenceMutation>,
}

impl SequenceReorganizeState {
    fn advance(&mut self, snapshot: &SequenceSnapshot, config: &SequenceConfig) -> Result<SequencePersistentAdvance, Fault> {
        let scene = snapshot.content.local_owner::<SequenceWorkingScene>().ok_or_else(|| Fault::from("sequence-reorganize-scene-owner"))?;
        let node_count = scene.steps.len();
        let edge_count = scene.edges.len();
        if node_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || edge_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-reorganize-capacity")); }
        if self.initialized < node_count {
            self.depths.push(0);
            self.initialized += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-nodes", b"{\"en\":\"Indexing layout node\",\"de\":\"Layoutknoten wird indiziert\"}"));
        }
        if self.pass < node_count && edge_count != 0 {
            let edge = &scene.edges[self.edge];
            let from = scene.steps.iter().position(|step| step.id == edge.from);
            let to = scene.steps.iter().position(|step| step.id == edge.to);
            if let (Some(from), Some(to)) = (from, to) { self.depths[to] = self.depths[to].max(self.depths[from].saturating_add(1).min(node_count)); }
            self.edge += 1;
            if self.edge == edge_count { self.edge = 0; self.pass += 1; }
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-edges", b"{\"en\":\"Relaxing layout edge\",\"de\":\"Layoutkante wird verarbeitet\"}"));
        }
        if self.emit < node_count {
            let index = self.emit;
            let step = &scene.steps[index];
            let primary = self.depths[index] as f64 * 280.0;
            let secondary = self.depths[..index].iter().filter(|depth| **depth == self.depths[index]).count() as f64 * 160.0;
            let (x, y) = if config.orientation == "topBottom" { (secondary, primary) } else { (primary, secondary) };
            if step.x != x || step.y != y { self.mutations.push(SequenceMutation::MoveStep(crate::artifacts::sequence::mutations::MoveStep { id: step.id.clone(), x, y })); }
            self.emit += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-publish-plan", b"{\"en\":\"Planning node position\",\"de\":\"Knotenposition wird geplant\"}"));
        }
        let mutations = std::mem::take(&mut self.mutations);
        Ok(SequencePersistentAdvance::Complete(Emit::mutations(mutations)))
    }

    fn release_one(&mut self) -> bool {
        self.depths.pop().is_some() || self.mutations.pop().is_some()
    }

    fn empty(&self) -> bool { self.depths.is_empty() && self.mutations.is_empty() }
}

#[derive(Clone, Copy, Default)]
enum SequenceNodeGraphStage { #[default] Parse, Apply, FixtureSteps, FixtureEdges, DeleteSelectionDiscover, DeleteSelectionApply, DeleteSteps, UpsertSteps, DeleteEdges, UpsertEdges, Complete }

#[derive(Default)]
struct SequenceNodeGraphState {
    stage: SequenceNodeGraphStage,
    operations: Vec<Value>,
    operation: usize,
    base: Option<SequenceWorkingScene>,
    target: Option<SequenceWorkingScene>,
    fixture_steps: VecDeque<SequenceStep>,
    fixture_edges: VecDeque<SequenceEdge>,
    delete_frontier: VecDeque<String>,
    delete_current: Option<String>,
    delete_scan: usize,
    selection_deleted: Vec<String>,
    cursor: usize,
    deleted: Vec<String>,
    recreated: Vec<String>,
    mutations: Vec<SequenceMutation>,
}

impl SequenceNodeGraphState {
    fn advance(&mut self, command: &SequenceCommand, snapshot: &SequenceSnapshot, interaction: &protocol::InteractionState) -> Result<SequencePersistentAdvance, Fault> {
        match self.stage {
            SequenceNodeGraphStage::Parse => {
                let SequenceCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("sequence-node-graph-route")); };
                if payload.operations_json.len() > SEQUENCE_RETAINED_RAW_BYTES { return Err(Fault::from("sequence-node-graph-bytes")); }
                self.operations = serde_json::from_str(&payload.operations_json).map_err(|_| Fault::from("sequence-node-graph-json"))?;
                if self.operations.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-node-graph-items")); }
                let scene = snapshot.content.local_owner::<SequenceWorkingScene>().ok_or_else(|| Fault::from("sequence-node-graph-scene-owner"))?;
                self.base = Some(scene.as_ref().clone()); self.target = Some(scene.as_ref().clone()); self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-parse", b"{\"en\":\"Decoded graph edit\",\"de\":\"Graphbearbeitung wurde dekodiert\"}"))
            }
            SequenceNodeGraphStage::Apply if self.operation < self.operations.len() => {
                let operation = &self.operations[self.operation];
                let target = self.target.as_mut().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                match operation.get("operation").and_then(Value::as_str).unwrap_or("") {
                    "setFixture" => if let Some(fixture) = operation.get("fixtureJson").and_then(Value::as_str).and_then(|json| serde_json::from_str::<SequenceFixture>(json).ok()) {
                        if fixture.schema == SEQUENCE_DOCUMENT_SCHEMA && fixture.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && fixture.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                            target.steps.clear(); target.edges.clear(); self.fixture_steps = fixture.steps.into(); self.fixture_edges = fixture.edges.into(); self.stage = SequenceNodeGraphStage::FixtureSteps;
                            return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-fixture", b"{\"en\":\"Preparing bounded fixture replacement\",\"de\":\"Begrenzter Dokumentersatz wird vorbereitet\"}"));
                        }
                    },
                    "deleteSelection" => {
                        let selected = &interaction.selection(SEQUENCE_INTERACTION_STEPS).ids;
                        if selected.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-node-graph-selection-capacity")); }
                        self.delete_frontier = selected.iter().cloned().collect(); self.stage = SequenceNodeGraphStage::DeleteSelectionDiscover;
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection", b"{\"en\":\"Preparing bounded graph selection\",\"de\":\"Begrenzte Graphauswahl wird vorbereitet\"}"));
                    }
                    "connect" => if let (Some(from), Some(to)) = (operation.get("sourceNodeId").and_then(Value::as_str), operation.get("targetNodeId").and_then(Value::as_str)) {
                        let existing: Vec<(String, String)> = target.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
                        if from != to && target.steps.iter().any(|step| step.id == from) && target.steps.iter().any(|step| step.id == to) && !would_create_cycle(&existing, from, to) && !target.edges.iter().any(|edge| edge.from == from) {
                            target.edges.retain(|edge| edge.to != to);
                            target.edges.push(SequenceEdge { id: sequence_retained_next_id(target, "edge"), from: from.into(), to: to.into() });
                        }
                    },
                    _ => {}
                }
                self.operation += 1;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-operation", b"{\"en\":\"Applying graph operation\",\"de\":\"Graphoperation wird angewendet\"}"))
            }
            SequenceNodeGraphStage::FixtureSteps => {
                if let Some(step) = self.fixture_steps.pop_front() { self.target.as_mut().ok_or_else(|| Fault::from("sequence-node-graph-target"))?.steps.push(step); return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-fixture-step", b"{\"en\":\"Replacing one graph step\",\"de\":\"Ein Graphschritt wird ersetzt\"}")); }
                self.stage = SequenceNodeGraphStage::FixtureEdges;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-fixture-edges", b"{\"en\":\"Preparing fixture edges\",\"de\":\"Dokumentkanten werden vorbereitet\"}"))
            }
            SequenceNodeGraphStage::FixtureEdges => {
                if let Some(edge) = self.fixture_edges.pop_front() { self.target.as_mut().ok_or_else(|| Fault::from("sequence-node-graph-target"))?.edges.push(edge); return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-fixture-edge", b"{\"en\":\"Replacing one graph edge\",\"de\":\"Eine Graphkante wird ersetzt\"}")); }
                self.operation += 1; self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-fixture-complete", b"{\"en\":\"Completed bounded fixture replacement\",\"de\":\"Begrenzter Dokumentersatz wurde abgeschlossen\"}"))
            }
            SequenceNodeGraphStage::DeleteSelectionDiscover => {
                if self.delete_current.is_none() {
                    if let Some(id) = self.delete_frontier.pop_front() {
                        if !self.selection_deleted.contains(&id) { self.selection_deleted.push(id.clone()); self.delete_current = Some(id); self.delete_scan = 0; }
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-root", b"{\"en\":\"Traversing one selected graph root\",\"de\":\"Eine ausgewählte Graphwurzel wird durchlaufen\"}"));
                    }
                    self.stage = SequenceNodeGraphStage::DeleteSelectionApply; self.delete_scan = 0;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-apply", b"{\"en\":\"Preparing selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wird vorbereitet\"}"));
                }
                let target = self.target.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                if self.delete_scan < target.steps.len() {
                    let step = &target.steps[self.delete_scan];
                    if step.slot.as_ref().is_some_and(|slot| self.delete_current.as_ref().is_some_and(|id| slot.owner == *id)) && !self.selection_deleted.contains(&step.id) && !self.delete_frontier.contains(&step.id) { self.delete_frontier.push_back(step.id.clone()); }
                    self.delete_scan += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-child", b"{\"en\":\"Traversing one nested graph step\",\"de\":\"Ein verschachtelter Graphschritt wird durchlaufen\"}"));
                }
                self.delete_current = None; self.delete_scan = 0;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-next", b"{\"en\":\"Advancing selected graph traversal\",\"de\":\"Ausgewählter Graphdurchlauf wird fortgesetzt\"}"))
            }
            SequenceNodeGraphStage::DeleteSelectionApply => {
                if let Some(id) = self.selection_deleted.pop() {
                    let target = self.target.as_mut().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                    target.steps.retain(|step| step.id != id); target.edges.retain(|edge| edge.from != id && edge.to != id);
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-delete", b"{\"en\":\"Removing one selected graph step\",\"de\":\"Ein ausgewählter Graphschritt wird entfernt\"}"));
                }
                self.operation += 1; self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-complete", b"{\"en\":\"Completed selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wurde abgeschlossen\"}"))
            }
            SequenceNodeGraphStage::Apply => { self.stage = SequenceNodeGraphStage::DeleteSteps; self.cursor = 0; self.advance(command, snapshot, interaction) }
            SequenceNodeGraphStage::DeleteSteps => {
                let base = self.base.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-base"))?; let target = self.target.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                if self.cursor < base.steps.len() { let step = &base.steps[self.cursor]; if !target.steps.iter().any(|entry| entry.id == step.id) { self.deleted.push(step.id.clone()); self.mutations.push(SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id: step.id.clone() })); } self.cursor += 1; return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-delete-step", b"{\"en\":\"Diffing removed step\",\"de\":\"Entfernter Schritt wird verglichen\"}")); }
                self.stage = SequenceNodeGraphStage::UpsertSteps; self.cursor = 0; self.advance(command, snapshot, interaction)
            }
            SequenceNodeGraphStage::UpsertSteps => {
                let base = self.base.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-base"))?; let target = self.target.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                if self.cursor < target.steps.len() { let step = &target.steps[self.cursor]; match base.steps.iter().find(|entry| entry.id == step.id) { None => self.mutations.push(SequenceMutation::CreateStep(crate::artifacts::sequence::mutations::CreateStep { step: step.clone() })), Some(old) if old.kind != step.kind || old.slot != step.slot => { self.recreated.push(step.id.clone()); self.mutations.push(SequenceMutation::DeleteStep(crate::artifacts::sequence::mutations::DeleteStep { id: step.id.clone() })); self.mutations.push(SequenceMutation::CreateStep(crate::artifacts::sequence::mutations::CreateStep { step: step.clone() })); }, Some(old) => { if old.x != step.x || old.y != step.y { self.mutations.push(SequenceMutation::MoveStep(crate::artifacts::sequence::mutations::MoveStep { id: step.id.clone(), x: step.x, y: step.y })); } if old.params != step.params { self.mutations.push(SequenceMutation::EditStepParams(crate::artifacts::sequence::mutations::EditStepParams { id: step.id.clone(), params: step.params.clone() })); } if old.collapsed != step.collapsed { self.mutations.push(SequenceMutation::ChangeStepCollapsed(crate::artifacts::sequence::mutations::ChangeStepCollapsed { id: step.id.clone(), collapsed: step.collapsed })); } } } self.cursor += 1; return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-upsert-step", b"{\"en\":\"Diffing changed step\",\"de\":\"Geänderter Schritt wird verglichen\"}")); }
                self.stage = SequenceNodeGraphStage::DeleteEdges; self.cursor = 0; self.advance(command, snapshot, interaction)
            }
            SequenceNodeGraphStage::DeleteEdges => {
                let base = self.base.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-base"))?; let target = self.target.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                if self.cursor < base.edges.len() { let edge = &base.edges[self.cursor]; if !self.deleted.iter().any(|id| id == &edge.from || id == &edge.to) && !self.recreated.iter().any(|id| id == &edge.from || id == &edge.to) && !target.edges.iter().any(|entry| entry.id == edge.id) { self.mutations.push(SequenceMutation::DisconnectSteps(crate::artifacts::sequence::mutations::DisconnectSteps { id: edge.id.clone() })); } self.cursor += 1; return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-delete-edge", b"{\"en\":\"Diffing removed edge\",\"de\":\"Entfernte Kante wird verglichen\"}")); }
                self.stage = SequenceNodeGraphStage::UpsertEdges; self.cursor = 0; self.advance(command, snapshot, interaction)
            }
            SequenceNodeGraphStage::UpsertEdges => {
                let base = self.base.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-base"))?; let target = self.target.as_ref().ok_or_else(|| Fault::from("sequence-node-graph-target"))?;
                if self.cursor < target.edges.len() { let edge = &target.edges[self.cursor]; let endpoint_recreated = self.recreated.iter().any(|id| id == &edge.from || id == &edge.to); match base.edges.iter().find(|entry| entry.id == edge.id) { None => self.mutations.push(SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() })), Some(_) if endpoint_recreated => self.mutations.push(SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() })), Some(old) if old.from != edge.from || old.to != edge.to => { self.mutations.push(SequenceMutation::DisconnectSteps(crate::artifacts::sequence::mutations::DisconnectSteps { id: old.id.clone() })); self.mutations.push(SequenceMutation::ConnectSteps(crate::artifacts::sequence::mutations::ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() })); }, Some(_) => {} } self.cursor += 1; return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-upsert-edge", b"{\"en\":\"Diffing changed edge\",\"de\":\"Geänderte Kante wird verglichen\"}")); }
                self.stage = SequenceNodeGraphStage::Complete; self.advance(command, snapshot, interaction)
            }
            SequenceNodeGraphStage::Complete => {
                if self.mutations.len() > SEQUENCE_PERSISTENT_MAXIMUM_UNITS { return Err(Fault::from("sequence-node-graph-output-items")); }
                sequence_bounded_serialized_bytes(&self.mutations, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| Fault::from("sequence-node-graph-output-bytes"))?;
                Ok(SequencePersistentAdvance::Complete(Emit::mutations(std::mem::take(&mut self.mutations))))
            }
        }
    }

    fn release_one(&mut self) -> bool {
        if self.operations.pop().is_some() || self.fixture_steps.pop_front().is_some() || self.fixture_edges.pop_front().is_some() || self.delete_frontier.pop_front().is_some() || self.delete_current.take().is_some() || self.selection_deleted.pop().is_some() || self.deleted.pop().is_some() || self.recreated.pop().is_some() || self.mutations.pop().is_some() { return true; }
        if let Some(base) = self.base.as_mut() { if base.steps.pop().is_some() || base.edges.pop().is_some() { return true; } }
        if self.base.take().is_some() { return true; }
        if let Some(target) = self.target.as_mut() { if target.steps.pop().is_some() || target.edges.pop().is_some() { return true; } }
        self.target.take().is_some()
    }
    fn empty(&self) -> bool { self.operations.is_empty() && self.fixture_steps.is_empty() && self.fixture_edges.is_empty() && self.delete_frontier.is_empty() && self.delete_current.is_none() && self.selection_deleted.is_empty() && self.deleted.is_empty() && self.recreated.is_empty() && self.mutations.is_empty() && self.base.is_none() && self.target.is_none() }
}

#[derive(Clone, Copy, Default)]
enum SequenceRunOrderStage { #[default] Steps, Edges, Heads, Choose, Walk, Remainder, Complete }

#[derive(Default)]
struct SequenceRunOrder {
    owner: Option<String>,
    name: Option<String>,
    stage: SequenceRunOrderStage,
    cursor: usize,
    scoped: Vec<usize>,
    incoming: Vec<(String, String)>,
    outgoing: Vec<(String, String)>,
    heads: Vec<usize>,
    ordered: Vec<usize>,
    current: Option<String>,
}

impl SequenceRunOrder {
    fn new(slot: Option<(&str, &str)>) -> Self { Self { owner: slot.map(|value| value.0.into()), name: slot.map(|value| value.1.into()), ..Self::default() } }

    fn matches(&self, step: &SequenceStep) -> bool {
        match ((self.owner.as_deref(), self.name.as_deref()), step.slot.as_ref()) {
            ((None, None), None) => true,
            ((Some(owner), Some(name)), Some(slot)) => slot.owner == owner && slot.name == name,
            _ => false,
        }
    }

    fn advance(&mut self, scene: &SequenceWorkingScene) -> &'static str {
        match self.stage {
            SequenceRunOrderStage::Steps if self.cursor < scene.steps.len() => {
                if self.matches(&scene.steps[self.cursor]) { self.scoped.push(self.cursor); }
                self.cursor += 1;
                "sequence-run-order-step"
            }
            SequenceRunOrderStage::Steps => { self.stage = SequenceRunOrderStage::Edges; self.cursor = 0; "sequence-run-order-edges" }
            SequenceRunOrderStage::Edges if self.cursor < scene.edges.len() => {
                let edge = &scene.edges[self.cursor];
                if let Some(entry) = self.incoming.iter_mut().find(|entry| entry.0 == edge.to) { entry.1 = edge.from.clone(); } else { self.incoming.push((edge.to.clone(), edge.from.clone())); }
                if let Some(entry) = self.outgoing.iter_mut().find(|entry| entry.0 == edge.from) { entry.1 = edge.to.clone(); } else { self.outgoing.push((edge.from.clone(), edge.to.clone())); }
                self.cursor += 1;
                "sequence-run-order-edge"
            }
            SequenceRunOrderStage::Edges => { self.stage = SequenceRunOrderStage::Heads; self.cursor = 0; "sequence-run-order-heads" }
            SequenceRunOrderStage::Heads if self.cursor < self.scoped.len() => {
                let index = self.scoped[self.cursor];
                if !self.incoming.iter().any(|entry| entry.0 == scene.steps[index].id) { self.heads.push(index); }
                self.cursor += 1;
                "sequence-run-order-head"
            }
            SequenceRunOrderStage::Heads => { self.stage = SequenceRunOrderStage::Choose; "sequence-run-order-choose" }
            SequenceRunOrderStage::Choose => {
                self.current = if self.heads.len() == 1 { Some(scene.steps[self.heads[0]].id.clone()) } else if self.scoped.len() == 1 { Some(scene.steps[self.scoped[0]].id.clone()) } else { None };
                self.stage = if self.current.is_some() { SequenceRunOrderStage::Walk } else { SequenceRunOrderStage::Remainder };
                "sequence-run-order-start"
            }
            SequenceRunOrderStage::Walk => {
                let Some(id) = self.current.take() else { self.stage = SequenceRunOrderStage::Remainder; return "sequence-run-order-remainder"; };
                if let Some(index) = self.scoped.iter().copied().find(|index| scene.steps[*index].id == id && !self.ordered.contains(index)) { self.ordered.push(index); }
                self.current = self.outgoing.iter().find(|entry| entry.0 == id).map(|entry| entry.1.clone());
                if self.current.as_ref().is_some_and(|next| self.ordered.iter().any(|index| scene.steps[*index].id == *next)) { self.current = None; }
                "sequence-run-order-walk"
            }
            SequenceRunOrderStage::Remainder => {
                let next = self.scoped.iter().copied().filter(|index| !self.ordered.contains(index)).min_by(|left, right| scene.steps[*left].id.cmp(&scene.steps[*right].id));
                if let Some(index) = next { self.ordered.push(index); } else { self.stage = SequenceRunOrderStage::Complete; }
                "sequence-run-order-remainder"
            }
            SequenceRunOrderStage::Complete => "sequence-run-order-complete",
        }
    }

    fn complete(&self) -> bool { matches!(self.stage, SequenceRunOrderStage::Complete) }
    fn release_one(&mut self) -> bool { self.scoped.pop().is_some() || self.incoming.pop().is_some() || self.outgoing.pop().is_some() || self.heads.pop().is_some() || self.ordered.pop().is_some() || self.current.take().is_some() || self.owner.take().is_some() || self.name.take().is_some() }
    fn empty(&self) -> bool { self.scoped.is_empty() && self.incoming.is_empty() && self.outgoing.is_empty() && self.heads.is_empty() && self.ordered.is_empty() && self.current.is_none() && self.owner.is_none() && self.name.is_none() }
}

struct SequenceRunFrame { order: SequenceRunOrder, cursor: usize, repeat_remaining: usize, repeat_total: usize, while_key: Option<String>, while_iterations: usize }

#[derive(Default)]
struct SequenceRunState {
    initialized: bool,
    registry: Option<Registry>,
    scope: Dictionary,
    effects: Vec<imperative_engine::EffectLogEntry>,
    frames: Vec<SequenceRunFrame>,
}

fn sequence_run_string(params: &Dictionary, key: &str) -> String {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or_default().to_string()
}

fn sequence_run_number(params: &Dictionary, key: &str) -> usize {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).unwrap_or(0.0).max(0.0) as usize
}

fn sequence_run_scope_bool(scope: &Dictionary, key: &str) -> bool {
    scope.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_bool()).unwrap_or(false)
}

impl SequenceRunState {
    fn advance(&mut self, snapshot: &SequenceSnapshot) -> Result<SequencePersistentAdvance, Fault> {
        let scene = snapshot.content.local_owner::<SequenceWorkingScene>().ok_or_else(|| Fault::from("sequence-run-scene-owner"))?;
        if scene.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-scene-capacity")); }
        if !self.initialized {
            self.registry = Some(imperative_module_registry());
            self.scope = Dictionary::new();
            self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(None), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
            self.initialized = true;
            return Ok(SequencePersistentAdvance::Progress("sequence-run-initialize", b"{\"en\":\"Preparing execution\",\"de\":\"Ausführung wird vorbereitet\"}"));
        }
        let Some(frame) = self.frames.last_mut() else {
            let result = RunResult { scope: self.scope.clone(), effects: std::mem::take(&mut self.effects) };
            let json = serde_json::to_string(&result).map_err(|_| Fault::from("sequence-run-result-json"))?;
            if json.len() > SEQUENCE_STORE_MAXIMUM_BYTES { return Err(Fault::from("sequence-run-result-capacity")); }
            return Ok(SequencePersistentAdvance::Complete(Emit::config(vec![SequenceConfigMutation::SetLastRun { json }])));
        };
        if !frame.order.complete() {
            let stage = frame.order.advance(scene.as_ref());
            return Ok(SequencePersistentAdvance::Progress(stage, b"{\"en\":\"Ordering Sequence graph\",\"de\":\"Sequenzgraph wird geordnet\"}"));
        }
        if frame.cursor >= frame.order.ordered.len() {
            if let Some(key) = frame.while_key.as_ref() {
                if sequence_run_scope_bool(&self.scope, key) {
                    if frame.while_iterations == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-while-capacity")); }
                    frame.cursor = 0;
                    frame.while_iterations += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-run-while-cursor", b"{\"en\":\"Continuing bounded while body\",\"de\":\"Begrenzter Solange-Block wird fortgesetzt\"}"));
                }
            } else if frame.repeat_remaining > 1 {
                frame.repeat_remaining -= 1;
                frame.cursor = 0;
                let index = frame.repeat_total - frame.repeat_remaining;
                self.scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
                return Ok(SequencePersistentAdvance::Progress("sequence-run-repeat-cursor", b"{\"en\":\"Continuing bounded repeat body\",\"de\":\"Begrenzter Wiederholungsblock wird fortgesetzt\"}"));
            }
            self.frames.pop();
            return Ok(SequencePersistentAdvance::Progress("sequence-run-retire-frame", b"{\"en\":\"Completed nested execution frame\",\"de\":\"Verschachtelter Ausführungsrahmen wurde abgeschlossen\"}"));
        }
        let index = frame.order.ordered[frame.cursor];
        frame.cursor += 1;
        let step = scene.steps.get(index).ok_or_else(|| Fault::from("sequence-run-step-index"))?;
        let input = self.scope.merge(&step.params.0);
        match step.kind.as_str() {
            "control.if" => {
                let key = sequence_run_string(&step.params.0, "key");
                let slot = if sequence_run_scope_bool(&self.scope, &key) { "then" } else { "else" };
                let depth_fault = self.frames.len() >= 65;
                let required_effects = if depth_fault { 2 } else { 1 };
                if self.effects.len().saturating_add(required_effects) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-effect-capacity")); }
                self.effects.push(imperative_engine::EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input, output: Some(Dictionary::new().insert("branch", NeuralValue::Atom(neural_engine::Atom::String(slot.into())))), error: None });
                if depth_fault {
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else {
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, slot))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
                }
            }
            "control.repeat" => {
                let count = sequence_run_number(&step.params.0, "count");
                if count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-repeat-capacity")); }
                if self.frames.len() >= 65 {
                    if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-effect-capacity")); }
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else if count != 0 {
                    self.scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(0)));
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: count, repeat_total: count, while_key: None, while_iterations: 0 });
                }
            }
            "control.while" => {
                let key = sequence_run_string(&step.params.0, "key");
                if sequence_run_scope_bool(&self.scope, &key) {
                    if self.frames.len() >= 65 {
                        if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-effect-capacity")); }
                        self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                    } else {
                        self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: Some(key), while_iterations: 1 });
                    }
                }
            }
            _ => {
                let registry = self.registry.as_ref().ok_or_else(|| Fault::from("sequence-run-registry"))?;
                let result = Executor::new(registry).run(&Path { steps: vec![Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies: BTreeMap::new() }] }, &self.scope);
                if self.effects.len().saturating_add(result.effects.len()) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS { return Err(Fault::from("sequence-run-effect-capacity")); }
                let halt_frame = result.effects.iter().any(|effect| effect.error.is_some());
                self.scope = result.scope;
                self.effects.extend(result.effects);
                if halt_frame { if let Some(frame) = self.frames.last_mut() { frame.cursor = frame.order.ordered.len(); } }
            }
        }
        Ok(SequencePersistentAdvance::Progress("sequence-run-step", b"{\"en\":\"Executed Sequence step\",\"de\":\"Sequenzschritt wurde ausgeführt\"}"))
    }

    fn release_one(&mut self) -> bool {
        if self.effects.pop().is_some() { return true; }
        if let Some(frame) = self.frames.last_mut() { if frame.order.release_one() { return true; } }
        self.frames.pop().is_some() || self.registry.take().is_some() || if self.initialized { self.scope = Dictionary::new(); self.initialized = false; true } else { false }
    }
    fn empty(&self) -> bool { !self.initialized && self.registry.is_none() && self.effects.is_empty() && self.frames.is_empty() }
}

enum SequencePersistentWorkspace { Reorganize(SequenceReorganizeState), NodeGraph(SequenceNodeGraphState), Run(SequenceRunState) }
impl SequencePersistentWorkspace {
    fn new(tool_id: &str) -> Self { match tool_id { "reorganize" => Self::Reorganize(SequenceReorganizeState::default()), "nodeGraphEdit" => Self::NodeGraph(SequenceNodeGraphState::default()), _ => Self::Run(SequenceRunState::default()) } }
    fn advance(&mut self, command: &SequenceCommand, snapshot: &SequenceSnapshot, config: &SequenceConfig, interaction: &protocol::InteractionState) -> Result<SequencePersistentAdvance, Fault> { match self { Self::Reorganize(state) => state.advance(snapshot, config), Self::NodeGraph(state) => state.advance(command, snapshot, interaction), Self::Run(state) => state.advance(snapshot) } }
    fn release_one(&mut self) -> bool { match self { Self::Reorganize(state) => state.release_one(), Self::NodeGraph(state) => state.release_one(), Self::Run(state) => state.release_one() } }
    fn empty(&self) -> bool { match self { Self::Reorganize(state) => state.empty(), Self::NodeGraph(state) => state.empty(), Self::Run(state) => state.empty() } }
}

struct SequencePersistentWork { tool_id: &'static str, workspace_identity: u64, progress: usize, replay_target: Option<usize>, workspace: SequencePersistentWorkspace, completed: bool, closing: bool }
impl SequencePersistentWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self { let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation); let identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)); Self { tool_id, workspace_identity: identity, progress: 0, replay_target: None, workspace: SequencePersistentWorkspace::new(tool_id), completed: false, closing: false } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequencePersistentWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn workspace_identity(&self) -> u64 { self.workspace_identity }
    fn extent(&self, _command: &SequenceCommand, snapshot: &SequenceSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>) -> Option<usize> { let scene = snapshot.content.local_owner::<SequenceWorkingScene>()?; (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS).then_some(SEQUENCE_PERSISTENT_MAXIMUM_UNITS) }
    fn step(&mut self, command: &SequenceCommand, snapshot: &SequenceSnapshot, config: &SequenceConfig, _history: &semio_framework_plugin::HistoryView, interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>, _operation: &semio_framework_plugin::AppOperationContext) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.progress >= SEQUENCE_PERSISTENT_MAXIMUM_UNITS || command.command_id() != self.tool_id { return Err(Fault::from("sequence-persistent-progress-capacity")); }
        match self.workspace.advance(command, snapshot, config, interaction)? {
            SequencePersistentAdvance::Progress(stage, preview) => { self.progress += 1; if let Some(target) = self.replay_target { if self.progress == target { self.replay_target = None; } Ok(ArtifactCommandWorkStep::Replay { stage, preview }) } else { Ok(ArtifactCommandWorkStep::Progress { stage, preview }) } }
            SequencePersistentAdvance::Complete(emit) => {
                if self.replay_target.is_some() { return Err(Fault::from("sequence-persistent-replay-overrun")); }
                let exact_lane = if self.tool_id == "run" {
                    emit.artifact_mutations.is_empty() && emit.config_mutations.len() == 1 && emit.draft_mutations.is_empty() && emit.child_emits.is_empty()
                } else {
                    emit.config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.child_emits.is_empty()
                };
                if !exact_lane { return Err(Fault::from("sequence-persistent-publication-lane")); }
                sequence_bounded_serialized_bytes(&(&emit.artifact_mutations, &emit.config_mutations), SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| Fault::from("sequence-persistent-output-bytes"))?;
                self.completed = true;
                Ok(ArtifactCommandWorkStep::Complete(emit))
            }
        }
    }
    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> { if target.len() < 24 { return Err(Fault::from("sequence-persistent-checkpoint-capacity")); } target[..24].fill(0); target[..4].copy_from_slice(b"SRP1"); target[8..16].copy_from_slice(&(self.progress as u64).to_le_bytes()); target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes()); Ok(24) }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> { if checkpoint.len() != 24 || &checkpoint[..4] != b"SRP1" || checkpoint[4..8] != [0,0,0,0] { return Err(Fault::from("sequence-persistent-checkpoint-invalid")); } let progress = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("sequence-persistent-checkpoint-cursor"))?)).map_err(|_| Fault::from("sequence-persistent-checkpoint-cursor"))?; let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("sequence-persistent-checkpoint-identity"))?); if identity != self.workspace_identity || progress > SEQUENCE_PERSISTENT_MAXIMUM_UNITS { return Err(Fault::from("sequence-persistent-checkpoint-owner")); } self.progress = 0; self.replay_target = (progress != 0).then_some(progress); self.workspace = SequencePersistentWorkspace::new(self.tool_id); self.completed = false; Ok(()) }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep { if !self.closing { return semio_framework_job::InteractiveJobCloseStep::Blocked; } if maximum_items == 0 || maximum_bytes == 0 { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; } self.replay_target = None; if self.workspace.release_one() { semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 } } else { semio_framework_job::InteractiveJobCloseStep::Complete } }
    fn terminal_is_empty(&self) -> bool { self.closing && self.replay_target.is_none() && self.workspace.empty() }
}

struct SequencePersistentJobFactory { keys: Vec<semio_framework::ToolFactoryKey> }
impl SequencePersistentJobFactory { fn new(controller: &str) -> Self { Self { keys: SEQUENCE_PERSISTENT_TOOL_IDS.iter().map(|id| semio_framework::ToolFactoryKey::new(controller, *id)).collect() } } }
impl semio_framework::ToolJobFactory for SequencePersistentJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>; type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys } fn payload_schema_id(&self) -> &str { "sequence.play/persistent-command.v1" } fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_PERSISTENT_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 7_500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> { if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) { return Err((semio_framework::ToolJobFactoryError::new("Sequence persistent command wire or checkpoint exceeds cap"), input, checkpoint)); } Ok(match checkpoint { Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint), None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input) }) }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequencePersistentJobFactory { type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>; const TOOL_IDS: &'static [&'static str] = SEQUENCE_PERSISTENT_TOOL_IDS; const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA; const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS; }
//#endregion 🧵️PersistentRemainingRoutes

//#region 🧵️RetainedConfigRoutes
const SEQUENCE_RETAINED_PAYLOAD_SCHEMA: &str = "sequence.play/retained-config-command.v1";
const SEQUENCE_RETAINED_RAW_BYTES: usize = 4_096;
const SEQUENCE_RETAINED_MAXIMUM_UNITS: usize = 2;
const SEQUENCE_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["setViewport", "setOrientation", "stop", "setLocale"];
const SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setOrientation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stop", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
];

fn sequence_retained_config_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::SetViewport(_) | SequenceCommand::Stop(_) => true,
        SequenceCommand::SetOrientation(payload) => payload.value.len() <= 32,
        SequenceCommand::SetLocale(payload) => payload.value.len() <= 64,
        _ => false,
    }
}

struct SequenceRetainedConfigWork {
    tool_id: &'static str,
    workspace_identity: u64,
    cursor: usize,
    replay_target: Option<usize>,
    completed: bool,
    closing: bool,
}

impl SequenceRetainedConfigWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedConfigWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    fn extent(
        &self,
        _command: &SequenceCommand,
        _snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        Some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        command: &SequenceCommand,
        snapshot: &SequenceSnapshot,
        config: &SequenceConfig,
        history: &semio_framework_plugin::HistoryView,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
        _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
        operation: &semio_framework_plugin::AppOperationContext,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_config_command_admitted(command) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.retained.config-command"), "Sequence retained config command exceeded its exact route or payload envelope"));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-config-replay", preview: b"{\"en\":\"Restoring Sequence setting\",\"de\":\"Sequenzeinstellung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-config-prepare", preview: b"{\"en\":\"Preparing Sequence setting\",\"de\":\"Sequenzeinstellung wird vorbereitet\"}" });
        }
        let emit = command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })?;
        if !emit.artifact_mutations.is_empty() || emit.config_mutations.len() != 1 || !emit.draft_mutations.is_empty() || !emit.child_emits.is_empty() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.retained.publication-lane"), "Sequence retained config route crossed its exact Config publication lane"));
        }
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(Fault::from("sequence-retained-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRC1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRC1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(Fault::from("sequence-retained-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("sequence-retained-checkpoint-cursor"))?)).map_err(|_| Fault::from("sequence-retained-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("sequence-retained-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS {
            return Err(Fault::from("sequence-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.replay_target = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none()
    }
}

struct SequenceRetainedConfigJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SequenceRetainedConfigJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SequenceRetainedConfigJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, 4_096, 2_000, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES
            || checkpoint
                .as_ref()
                .is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES)
        {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained config command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedConfigJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedConfigRoutes

//#region 🔖️SequencePlayApp
/// 🧪️ B1: unit struct — every former `SequencePlayRuntime` field now lives in
/// `crate::editor::sequence::config::SequenceConfig` (see `ArtifactApp::Config`), written through
/// `SequenceConfigMutation`s.
#[derive(Default)]
pub struct SequencePlayApp;

//#region 🧾️ProofCatalogs
struct SequenceArtifactProofs;
impl SequenceArtifactProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        document_schema: "sequence.sequence",
        factory: "SequenceRetainedArtifactJobFactory",
        factory_type: SequenceRetainedArtifactJobFactory,
        tools: {
            "addStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepToSlot" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepDropped" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "removeStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "deleteSelection" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "moveStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "connectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "disconnectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepParams" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepCollapsed" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
        }
    }
}

struct SequencePersistentProofs;
impl SequencePersistentProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        document_schema: "sequence.sequence",
        factory: "SequencePersistentJobFactory",
        factory_type: SequencePersistentJobFactory,
        tools: {
            "reorganize" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "run" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
        }
    }
}

struct SequenceConfigProofs;
impl SequenceConfigProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        document_schema: "sequence.sequence",
        factory: "SequenceRetainedConfigJobFactory",
        factory_type: SequenceRetainedConfigJobFactory,
        tools: {
            "setViewport" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "setOrientation" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "stop" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "setLocale" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs

impl ArtifactEditor for SequencePlayApp {
    type Snapshot = SequenceSnapshot;
    type Mutation = SequenceMutation;
    type Config = SequenceConfig;
    type ConfigMutation = SequenceConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SequencePresence;
    type PresenceMutation = SequencePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SequenceCommand;

    const DIALECT: Dialect = crate::artifacts::sequence::SEQUENCE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(SequenceArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(SequenceConfigStorePreparationFactory))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        SequenceArtifactProofs::bounded_first_step_tool_proofs().into_iter().chain(SequencePersistentProofs::bounded_first_step_tool_proofs()).chain(SequenceConfigProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SequenceRetainedArtifactJobFactory::new(&controller))?;
        registry.register(SequencePersistentJobFactory::new(&controller))?;
        registry.register(SequenceRetainedConfigJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let artifact_route = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.contains(&request.tool_id.as_str());
        let config_route = SEQUENCE_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str());
        let persistent_route = SEQUENCE_PERSISTENT_TOOL_IDS.contains(&request.tool_id.as_str());
        if !artifact_route && !config_route && !persistent_route {
            return Ok(None);
        }
        let persistent_admitted = match request.command.as_ref() { SequenceCommand::NodeGraphEdit(payload) => payload.operations_json.len() <= SEQUENCE_RETAINED_RAW_BYTES, SequenceCommand::Reorganize(_) | SequenceCommand::Run(_) => true, _ => false };
        if request.command.command_id() != request.tool_id || (artifact_route && !sequence_retained_artifact_command_admitted(&request.command)) || (config_route && !sequence_retained_config_command_admitted(&request.command)) || (persistent_route && !persistent_admitted) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.retained.tool-mismatch"), "Sequence command does not match its exact retained route or payload envelope"));
        }
        let tool_id = request.command.command_id();
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if persistent_route {
            Box::new(SequencePersistentWork::new(tool_id, &operation_context))
        } else if artifact_route {
            Box::new(SequenceRetainedArtifactWork::new(tool_id, &operation_context))
        } else {
            Box::new(SequenceRetainedConfigWork::new(tool_id, &operation_context))
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            SequenceCommand::command_id,
            SEQUENCE_RETAINED_RAW_BYTES,
            if persistent_route { SEQUENCE_PERSISTENT_MAXIMUM_UNITS } else if artifact_route { SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS } else { 1 },
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::sequence::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> SequenceSnapshot {
        default_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(sequence_io())
    }

    /// 🎞️ `steps:in` (Wave-2 port recipe): inserts incoming computation results as a new step at the
    /// far right of the flow — an object payload becomes that step's params verbatim, a bare
    /// scalar/array is wrapped under a single `"value"` key. Never mutates anything directly (matches
    /// every other `import_media` override): the caller (a headless runner or the UI) applies the
    /// returned `create-step` mutation through the ordinary, undoable document store.
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, SequenceSnapshot>) -> Result<Emit<SequenceMutation, SequenceConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "steps:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "steps:in importer only accepts a Structured (JSON) payload".into()));
        };
        let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let params_value = if value.is_object() { value } else { json!({ "value": value }) };
        let params: StepParams = serde_json::from_value(params_value).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let fixture = doc.snapshot;
        let id = next_available_step_id(fixture);
        let live = fixture.to_fixture();
        let x = live.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if live.steps.is_empty() { 0.0 } else { 280.0 };
        let step = SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false };
        Ok(Emit::mutations(vec![crate::artifacts::sequence::mutations::create_step(step)]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &SequenceCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit` read the "steps" interaction domain directly (bypassing
    /// the `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    async fn handle(
        command: &SequenceCommand,
        doc: &ArtifactView<'_, SequenceSnapshot>,
        cfg: &ConfigView<'_, SequenceConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SequenceMutation, SequenceConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SequenceCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            SequenceCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `steps`'s `HierarchyProvider::Topology` — every step is registered at the "step"
    /// granularity, parented to its control-flow slot owner (`SlotRef.owner`) when nested inside a
    /// `then`/`else`/`body` slot, or as a root otherwise — mirrors the document panel's own nesting
    /// (`build_step_tree_item`) so a deleted step's id auto-prunes out of the live selection.
    async fn interaction_topology(doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> InteractionTopology {
        let ordered = doc.snapshot.to_fixture().steps.iter().map(|step| TopologyNode { id: step.id.clone(), granularity: "step".into(), parent: step.slot.as_ref().map(|slot| slot.owner.clone()) }).collect();
        let mut domains = BTreeMap::new();
        domains.insert(SEQUENCE_INTERACTION_STEPS.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    /// 🧮️ This app's typed configuration spec — the layout orientation `reorganize` reads.
    async fn config_spec() -> ConfigSpec {
        ConfigSpec {
            fields: vec![ConfigFieldSpec {
                key: "orientation".into(),
                label: "Layout Orientation".into(),
                shape: ConfigFieldShape::Select { options: vec!["leftRight".into(), "topBottom".into()] },
                default: Some(DslValue::String("leftRight".into())),
            }],
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let fixture = doc.snapshot;
        let live = fixture.to_fixture();
        let config = cfg.snapshot;
        let labels = sequence_play_labels(config);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(fixture, config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(fixture, config),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(fixture),
            SEQUENCE_PLAY_BODY_DOCUMENT => document_panel::render(&live, labels),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(&live, labels),
            // 🕹️ `render` carries no `InteractionView` (same gap as `context_menu` below — see ticket
            // 26/08/14's w3b-summary.md), so this always takes the "nothing selected" branch rather
            // than reading a stale/wrong selection.
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(&live, &[], labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` above — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent rows built by
    /// `sequence_context_menu_items` below always take the "nothing selected" branch here rather than
    /// reading a stale/wrong selection.
    async fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let is_de = cfg.snapshot.locale.starts_with("de");
        sequence_context_menu_items(registry, is_de, request.surface.as_ref(), &[])
    }
}

/// 🗂️ Grouped disclosure: `run`/`stop`/`addStep` stay top-level (the most frequent verbs);
/// `reorganize` folds into the `transform` group and a single-node hit's `setStepCollapsed` folds
/// into the `selection` group; `deleteSelection` stays a direct destructive item last —
/// `organize_context_menu` (applied automatically at the `VcsArtifactApp::context_menu` funnel)
/// sorts the groups into `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator
/// itself. Factored out of `ArtifactApp::context_menu` (which carries no `InteractionView` — ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) so a test can exercise the selection-dependent
/// rows directly with a real `selected` slice, matching `space`'s own precedent.
async fn sequence_context_menu_items(registry: &AppActionRegistry, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected: &[String]) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

    let (nodes, edges) = selection_domains_from_surface(surface, selected, &[]);

    let mut menu = Menu::of(registry).action("run").action("stop").action("addStep").group("transform", |m| m.action("reorganize"));

    if nodes.len() == 1 {
        let id = nodes[0].clone();
        menu = menu.group("selection", |m| {
            m.item(ContextMenuItemSpec {
                id: "setStepCollapsed".into(),
                label: Some(if is_de { "Schritt einklappen".into() } else { "Toggle Collapsed".into() }),
                icon: Some("chevrons-up-down".into()),
                action: Some("setStepCollapsed".into()),
                args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "id": id }))),
                ..Default::default()
            })
        });
    }

    if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::Direct) {
        menu = menu.item(spec);
    }
    menu.build()
}
//#endregion 🔖️SequencePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own
/// `definition()`. Only the leaf action/keybinding declarations (which have no dedicated `_def`
/// passthrough) are written out inline.
pub async fn create_sequence_app() -> AppDefinition {
    Editor::builder(crate::artifacts::sequence::SEQUENCE_DIALECT)
            .document(["semio", "sequence"])
            .artifact_kind(crate::artifacts::sequence::artifact_kind())
            .icon_id("sequence")
            .mode_def(edit::definition())
            .default_mode_id(edit::SEQUENCE_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(script::definition())
            .window_kind_def(compiled::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition::bounded_catalog("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("addStepToSlot", LocalizedLabel::native("Add Step To Slot", "Schritt zu Slot hinzufügen"))
            .mutation("addStepDropped", LocalizedLabel::native("Add Step Dropped", "Schritt per Ablegen hinzufügen"))
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("connectSteps", LocalizedLabel::native("Connect Steps", "Schritte verbinden"))
            .mutation("disconnectSteps", LocalizedLabel::native("Disconnect Steps", "Schritte trennen"))
            .mutation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .action_with(ActionDefinition::bounded_catalog("setStepCollapsed", LocalizedLabel::native("Set Step Collapsed", "Schritt einklappen"), ActionKind::Mutation).with_category("selection"))
            .action_with(ActionDefinition::bounded_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("setViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            // 👁️ Ephemeral view state — run output, layout orientation, locale. Selection is no
            // longer declared here: framework-owned, injected via `.interaction(...)` below.
            .view_action("setOrientation", LocalizedLabel::native("Set Orientation", "Ausrichtung festlegen"))
            .action_with(ActionDefinition::bounded_catalog("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View).with_category("actions"))
            .action_with(ActionDefinition::bounded_catalog("stop", LocalizedLabel::native("Stop", "Stopp"), ActionKind::View).with_category("actions"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument forms for the panel-visible create + layout actions.
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print", "Ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value("log.print"),
            ])
            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", LocalizedLabel::native("Orientation", "Ausrichtung"), vec![
                    ActionArgOption::new("leftRight", LocalizedLabel::native("Left to Right", "Links nach rechts")),
                    ActionArgOption::new("topBottom", LocalizedLabel::native("Top to Bottom", "Oben nach unten")),
                ]).required(),
            ])
            .action_interactive_job("addStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepToSlot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepDropped", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("connectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("disconnectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepParams", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepCollapsed", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setOrientation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("run", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("stop", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the step graph, `HierarchyProvider::Topology` (see
            // `SequencePlayApp::interaction_topology` above) from each step's own control-flow slot
            // nesting — `transitive: false` matches the pre-migration behavior exactly (deleting a
            // selected control step never cascaded into its `then`/`else`/`body` children).
            .interaction(InteractionDefinition {
                id: SEQUENCE_INTERACTION_STEPS.into(),
                label: LocalizedLabel::native("Steps", "Schritte"),
                granularities: vec![GranularityDefinition { id: "step".into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(main::SEQUENCE_PLAY_WINDOW_MAIN, vec![InteractionRef::new(SEQUENCE_INTERACTION_STEPS)])
            .config(SequencePlayApp::config_spec())
            .io(sequence_io())
            // 🕳️ SDK gap (contract §2.4, confirmed absent on `EditorBuilder` as of this packet): the
            // pre-migration chain's trailing `.example_source(art_sequence_demo::source())` /
            // `.workflow("sequence", "Sequence", "graph")` calls are dropped here, not silently ported
            // — `Editor::builder(...)` has no such methods, and `PluginBuilder::editor::<E>(def)`
            // wraps `def` in `App { definition: def, examples: Vec::new() }`, discarding `App.examples`
            // even if it were populated. The subset's own `📚️examples/🎬️demo` facet (mounted at the
            // plugin root as `examples::art_sequence_demo`) is the closest surviving carrier of this
            // content today.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must
/// be able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type SequenceApp = VcsArtifactApp<EditorApp<SequencePlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> SequenceApp {
        semio_framework_plugin::testkit::new_app::<EditorApp<SequencePlayApp>>()
    }

    /// 🧩️ `create_sequence_app` now returns `AppDefinition` (contract §2.4), not the runtime-shaped
    /// `App { definition, examples }` `new_app_with_registry` still expects (SDK gap, unchanged by
    /// this ticket — `testkit::assert_declared_actions_bridge_to_commands` carries the identical gap
    /// per `📓️w0-f-report.md` Gap 3) — wraps it with an empty `examples` list rather than porting one.
    async fn sequence_manifest_for_testkit() -> App {
        App { definition: create_sequence_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry_wired() -> SequenceApp {
        new_app_with_registry::<EditorApp<SequencePlayApp>>(sequence_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut SequenceApp, command: SequenceCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut SequenceApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "steps" domain declared on this app —
    /// requires `new_app_with_registry_wired()` (a bare `new_app()` has no declared interaction
    /// domains to select against). `ids` are the steps' own raw document ids — the SAME ids the
    /// "steps" domain's topology/the document panel tree/the main node-graph canvas all use.
    pub async fn select_steps(app: &mut SequenceApp, ids: &[&str]) {
        let target_list: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": "step", "id": id })).collect();
        let targets = serde_json::to_string(&target_list).expect("targets json");
        app.handle_action("interactionSelect", Some(&serde_json::json!({ "domainId": SEQUENCE_INTERACTION_STEPS, "targets": targets, "merge": "replace" })), &meta("test")).expect("interactionSelect");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired};
    use semio_framework_plugin::{testkit::assert_undo_redo_round_trip, Locale, PluginApp, Terminology};

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_steps() {
        assert_eq!(crate::artifacts::sequence::default_snapshot().to_fixture().steps.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        assert_undo_redo_round_trip(&mut app, SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("projection").to_fixture().steps.len(), 2, 3);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a
    /// `MemoryBackbone` converges both sides onto an identical projection.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<SequencePlayApp>, _>(
            "mem://sequence-convergence",
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 111.0, y: 0.0 }),
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-2".into(), x: 222.0, y: 0.0 }),
            |app| app.snapshot().expect("projection"),
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_action_ids_resolve_to_labels_in_native_english_and_german() {
        let definition = create_sequence_app();
        for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
            assert_eq!(action.label.resolve(Terminology::Native, Locale::En), label, "{id} action label");
        }
        for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
            assert_eq!(action.label.resolve(Terminology::Native, Locale::De), label, "{id} action label");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = new_app();
        assert!(testkit::render(&mut app, "sequence.play.nope").contains("Unknown body"));
    }

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_sequence_app()).expect("app definition json");
        for id in [main::SEQUENCE_PLAY_WINDOW_MAIN, script::SEQUENCE_PLAY_WINDOW_SCRIPT, compiled::SEQUENCE_PLAY_WINDOW_COMPILED] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::SEQUENCE_PLAY_MODE_EDIT), "edit mode missing from the manifest");
        for body in [SEQUENCE_PLAY_BODY_DOCUMENT, SEQUENCE_PLAY_BODY_CATALOGUE, SEQUENCE_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("computation.sequence"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️ContextMenuTests
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::context_menu`
    /// carries no `InteractionView` (a documented framework gap — see `sequence_context_menu_items`'s
    /// own doc comment), so this exercises that free function directly with a real `selected` slice
    /// instead of going through the app's live (always-empty) `context_menu` trait method.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_stays_within_nine_rows_and_ends_with_destructive_delete() {
        let registry = AppActionRegistry::from_definition(&create_sequence_app());
        let items = sequence_context_menu_items(&registry, false, None, &["step-1".to_string()]);
        assert!(items.len() <= 9, "expected <= 9 top-level rows, got {} ({items:?})", items.len());
        let last = items.last().expect("at least one row");
        assert_eq!(last.id, "delete-selection");
        assert_eq!(last.destructive, Some(true));
    }
    //#endregion 🔖️ContextMenuTests

    //#region 🔖️PortTests
    #[semio_framework_async_macros::async_test]
    async fn sequence_io_declares_steps_in_and_document_ports() {
        let ports = SequencePlayApp::io().expect("io").all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "steps:in"));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_steps_in_inserts_a_new_step_from_an_object_payload() {
        let mut app = new_app_with_registry_wired();
        let before = app.snapshot().expect("projection").to_fixture().steps.len();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: json!({ "message": "from upstream" }).to_string() },
        };
        app.import_media("steps:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import steps:in");
        let after = app.snapshot().expect("projection").to_fixture();
        assert_eq!(after.steps.len(), before + 1);
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.kind, "computation.import");
        assert_eq!(imported.params.get("message").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("from upstream"));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_steps_in_wraps_a_bare_scalar_payload() {
        let mut app = new_app_with_registry_wired();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: "42".into() },
        };
        app.import_media("steps:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import steps:in");
        let after = app.snapshot().expect("projection").to_fixture();
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(42.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_rejects_unknown_port() {
        let mut app = new_app_with_registry_wired();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: "{}".into() },
        };
        assert!(app.import_media("not-a-port", &media, &semio_framework_plugin::testkit::meta("local")).is_err());
    }
    //#endregion 🔖️PortTests

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 17, "every SequenceCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, for every row (sequence has no `flow`-style id/keyword divergence).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected: String = id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect();
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<SequenceCommand> {
        vec![
            SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 1.0, y: 2.0 }),
            SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot { kind: "log.print".into(), x: 1.0, y: 2.0, owner: "step-1".into(), slot_name: "then".into() }),
            SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) }),
            SequenceCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
            SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 5.0, y: 6.0 }),
            SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() }),
            SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() }),
            SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params_json: "{\"a\":1}".into() }),
            SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: "step-1".into() }),
            SequenceCommand::Reorganize(reorganize::Reorganize {}),
            SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: "topBottom".into() }),
            SequenceCommand::Run(run_command::Run {}),
            SequenceCommand::Stop(stop_command::Stop {}),
            SequenceCommand::SetViewport(set_viewport::SetViewport { camera: crate::artifacts::sequence::SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
            SequenceCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ Pinned to the exact hex captured from the pre-merge `sequence_protocol` crate — a
    /// regression here is a real wire-format break, not a test-fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_row_keeps_its_pre_migration_bytes() {
        let some = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) });
        assert_eq!(protocol::OpText::print_op(&some), "add-step-dropped add-step-dropped kind=log.print x=1 y=2 picked-step-id=step-1");
        assert_eq!(protocol::OpBinary::encode_op(&some).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010202096c6f672e7072696e7406737465702d31040006000105000000000000f03f02050000000000000040030601");
        let none = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: None });
        assert_eq!(protocol::OpText::print_op(&none), "add-step-dropped add-step-dropped kind=log.print x=1 y=2");
        assert_eq!(protocol::OpBinary::encode_op(&none).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010201096c6f672e7072696e74030006000105000000000000f03f02050000000000000040");
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️HostTests
    use neural_engine::Atom;

    #[semio_framework_async_macros::async_test]
    async fn disconnect_steps_removes_edge() {
        let mut host = SequenceHost::default();
        assert!(host.disconnect_steps("step-1", "step-2"));
        assert!(host.snapshot.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn sync_from_dag_copies_node_positions() {
        let mut host = SequenceHost::default();
        if let Some(node) = host.dag.fixture.nodes.iter_mut().find(|node| node.id == "step-1") {
            node.x = 120.0;
            node.y = 80.0;
        }
        host.sync_from_dag();
        let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").expect("step-1");
        assert_eq!(step.x, 120.0);
        assert_eq!(step.y, 80.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn sync_edges_from_dag_preserves_existing_edge_ids() {
        let mut host = SequenceHost::default();
        let first_id = host.snapshot.edges[0].id.clone();
        host.sync_edges_from_dag();
        assert_eq!(host.snapshot.edges[0].id, first_id);
        host.sync_edges_from_dag();
        assert_eq!(host.snapshot.edges[0].id, first_id);
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_fan_out() {
        let mut host = SequenceHost::default();
        host.snapshot.edges.clear();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.connect_steps("step-1", "step-2").is_ok());
        assert!(host.connect_steps("step-1", "step-3").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_path_includes_control_bodies() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("key", NeuralValue::Atom(Atom::String("flag".into()))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", NeuralValue::Atom(Atom::String("yes".into()))),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        host.snapshot.edges.push(SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() });
        let path = host.build_path();
        assert_eq!(path.steps.len(), 3);
        let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
        assert!(control.bodies.contains_key("then"));
        assert_eq!(control.bodies.get("then").map(|body| body.steps.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn rebuild_dag_preserves_selection() {
        let mut host = SequenceHost::default();
        host.dag.set_selection(&["step-1".into()]);
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.rebuild_dag();
        assert!(host.dag.selected_node_ids()?.contains(&"step-1".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn execution_ports_use_triangle_shape() {
        let host = SequenceHost::default();
        let node = host.step_to_dag_node(&host.snapshot.steps[1]);
        assert_eq!(node.inputs()[0].shape, PortShape::Triangle);
        assert_eq!(node.outputs()[0].shape, PortShape::Triangle);
    }

    #[semio_framework_async_macros::async_test]
    async fn function_steps_use_data_ports_without_visible_execution_pins() {
        let host = SequenceHost::default();
        let step = SequenceStep { id: "step-fn".into(), kind: "math.add".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
        let node = host.step_to_dag_node(&step);
        assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
        assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
        assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_steps_use_data_ports_without_visible_execution_pins() {
        let host = SequenceHost::default();
        let step = SequenceStep { id: "step-txt".into(), kind: "text.concat".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
        let node = host.step_to_dag_node(&step);
        assert!(node.inputs().iter().any(|port| port.id == "left" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "into" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
        assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
        assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_snapshot_preserves_next_serial_and_selection() {
        let mut host = SequenceHost::default();
        let first = host.add_step("math.add", 40.0, 40.0);
        host.dag.set_selection(std::slice::from_ref(&first));
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceFixture = serde_json::from_str(&json).expect("parse");
        host.replace_snapshot(round_trip).expect("replace");
        let second = host.add_step("math.add", 80.0, 80.0);
        assert_ne!(first, second);
        assert!(host.snapshot.steps.iter().any(|step| step.id == first));
        assert!(host.snapshot.steps.iter().any(|step| step.id == second));
        assert!(host.dag.selected_node_ids()?.contains(&first));
    }

    #[semio_framework_async_macros::async_test]
    async fn repeated_drops_after_replace_snapshot_use_distinct_ids() {
        let mut host = SequenceHost::default();
        let first = host.add_step_dropped("math.add", 10.0, 10.0, None);
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceFixture = serde_json::from_str(&json).expect("parse");
        host.replace_snapshot(round_trip).expect("replace");
        let second = host.add_step_dropped("math.add", 20.0, 20.0, None);
        assert_ne!(first, second);
        assert_eq!(host.snapshot.steps.iter().filter(|step| step.kind == "math.add").count(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_step_dropped_targets_expanded_control_slot() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert_eq!(step.slot.as_ref().map(|slot| slot.name.as_str()), Some("then"));
    }

    #[semio_framework_async_macros::async_test]
    async fn execution_edges_use_sharp_sz_routing() {
        let host = SequenceHost::default();
        let fixture = host.build_dag_fixture();
        assert!(fixture.edges.iter().all(|edge| edge.route_style == EdgeRouteStyle::SharpSz));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_collapsed_toggles_control_step() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.set_step_collapsed("step-3", true));
        assert!(host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().collapsed);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_collapsed_rejects_unknown_id() {
        let mut host = SequenceHost::default();
        assert!(!host.set_step_collapsed("nope", true));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_collapsed_rejects_non_control_step() {
        let mut host = SequenceHost::default();
        assert!(!host.set_step_collapsed("step-1", true));
        assert!(!host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap().collapsed);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_step_also_removes_slot_children() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(host.remove_step("step-3"));
        assert!(!host.snapshot.steps.iter().any(|step| step.id == "step-3" || step.id == "step-4"));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_step_returns_false_for_unknown_id() {
        let mut host = SequenceHost::default();
        assert!(!host.remove_step("nope"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_params_json_updates_step_params() {
        let mut host = SequenceHost::default();
        host.set_step_params_json("step-1", r#"{"key":"renamed"}"#).expect("set params");
        let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap();
        assert_eq!(step.params.get("key").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("renamed"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_params_json_rejects_unknown_step() {
        let mut host = SequenceHost::default();
        let err = host.set_step_params_json("nope", "{}").unwrap_err();
        assert!(matches!(err, SequenceCoreError::UnknownStep(id) if id == "nope"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_step_params_json_rejects_invalid_json() {
        let mut host = SequenceHost::default();
        let err = host.set_step_params_json("step-1", "not json").unwrap_err();
        assert!(matches!(err, SequenceCoreError::Json(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_self_connect() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("step-1", "step-1").unwrap_err(), SequenceCoreError::SelfConnect));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_unknown_from_step() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("nope", "step-2").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_unknown_to_step() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("step-1", "nope").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_mismatched_slot_scope() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(matches!(host.connect_steps("step-2", "step-4").unwrap_err(), SequenceCoreError::MismatchedSlotScope));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rejects_cycle() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-2", "step-3").expect("connect step-2 to step-3");
        assert!(matches!(host.connect_steps("step-3", "step-1").unwrap_err(), SequenceCoreError::CycleDetected));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_steps_rewires_existing_incoming_edge() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-3", "step-2").expect("rewire onto step-2");
        assert_eq!(host.snapshot.edges.len(), 1);
        assert_eq!(host.snapshot.edges[0].from, "step-3");
        assert_eq!(host.snapshot.edges[0].to, "step-2");
    }

    #[semio_framework_async_macros::async_test]
    async fn disconnect_steps_returns_false_when_no_matching_edge() {
        let mut host = SequenceHost::default();
        assert!(!host.disconnect_steps("step-2", "step-1"));
        assert_eq!(host.snapshot.edges.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn load_json_parses_valid_fixture() {
        let json = SequenceHost::default().to_json().expect("fixture json");
        let host = SequenceHost::load_json(&json).expect("load json");
        assert_eq!(host.snapshot.steps.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn load_json_rejects_unsupported_schema() {
        let result = SequenceHost::load_json(r#"{"schema":"other","steps":[],"edges":[]}"#);
        assert!(matches!(result, Err(SequenceCoreError::UnsupportedSchema(schema)) if schema == "other"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_json_reports_imperative_catalogue_schema() {
        let host = SequenceHost::default();
        assert!(host.catalogue_json().contains("\"imperative.catalogue\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_expanded_slots_positions_slot_members_relative_to_owner() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        host.layout_expanded_slots();
        let child = host.snapshot.steps.iter().find(|step| step.id == "step-4").unwrap();
        assert_eq!(child.x, 400.0);
        assert_eq!(child.y, 160.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn reorganize_syncs_step_positions_from_dag_layout() {
        let mut host = SequenceHost::default();
        host.reorganize(&DagLayoutOptions::default()).expect("reorganize");
        for step in &host.snapshot.steps {
            let node = host.dag.fixture.nodes.iter().find(|node| node.id == step.id).expect("node for step");
            assert_eq!(step.x, node.x);
            assert_eq!(step.y, node.y);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn pick_step_id_at_screen_finds_step_under_cursor() {
        let host = SequenceHost::default();
        let id = host.pick_step_id_at_screen(400.0, 300.0, 800, 600, 1.0);
        assert_eq!(id, Some("step-1".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn pick_step_id_at_screen_returns_none_when_missing_all_nodes() {
        let host = SequenceHost::default();
        let id = host.pick_step_id_at_screen(-9000.0, -9000.0, 800, 600, 1.0);
        assert_eq!(id, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_step_dropped_falls_back_when_owner_collapsed() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: true });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_step_dropped_falls_back_for_non_control_owner() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("step-2"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_step_dropped_falls_back_for_unknown_owner_id() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("nope"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_path_returns_unordered_slot_body_when_multiple_heads() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-5".into(), kind: "log.print".into(), params: StepParams::new(), x: 280.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        let path = host.build_path();
        let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
        let body = control.bodies.get("then").expect("then body");
        assert_eq!(body.steps.len(), 2);
        assert!(body.steps.iter().any(|step| step.id == "step-4"));
        assert!(body.steps.iter().any(|step| step.id == "step-5"));
    }

    #[semio_framework_async_macros::async_test]
    async fn step_to_dag_node_shows_collapsed_indicator_for_collapsed_control_step() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let expanded = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(expanded.abbreviation, "▾️0");
        host.set_step_collapsed("step-3", true);
        let collapsed = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(collapsed.abbreviation, "▸️0");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_ghost_step_and_clear_ghost_step_toggle_dag_ghost_node() {
        let mut host = SequenceHost::default();
        assert!(host.dag.ghost_node().is_none());
        host.set_ghost_step("math.add", 10.0, 20.0);
        assert!(host.dag.ghost_node().is_some());
        host.clear_ghost_step();
        assert!(host.dag.ghost_node().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn run_executes_default_snapshot_and_records_scope() {
        let host = SequenceHost::default();
        let result = host.run();
        assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
        assert!(!result.effects.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn compile_text_renders_default_snapshot_steps() {
        let host = SequenceHost::default();
        let text = host.compile_text();
        assert!(text.contains("state.set"));
        assert!(text.contains("log.print"));
    }

    #[semio_framework_async_macros::async_test]
    async fn compiled_wire_literal_includes_step_ids() {
        let host = SequenceHost::default();
        let literal = host.compiled_wire_literal();
        assert!(literal.contains("step-1"));
        assert!(literal.contains("step-2"));
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_io_declares_the_steps_in_port() {
        let io = sequence_io();
        assert_eq!(io.document_schema, SEQUENCE_DOCUMENT_SCHEMA);
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "steps:in");
        assert_eq!(port.direction, semio_framework::MediaPortDirection::In);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[semio_framework_async_macros::async_test]
    async fn next_available_step_id_is_free_and_deterministic() {
        let fixture = default_snapshot();
        let id = next_available_step_id(&fixture);
        assert!(!fixture.to_fixture().steps.iter().any(|step| step.id == id));
        assert_eq!(id, next_available_step_id(&fixture), "pure function of the fixture, not a mutating counter");
    }
    //#endregion 🔖️HostTests
}
//#endregion 🧪️Tests
