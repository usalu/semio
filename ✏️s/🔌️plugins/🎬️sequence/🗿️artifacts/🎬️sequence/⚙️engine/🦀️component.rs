//! ⚙️ Sequence artifact — headless compute over the `SequenceSnapshot` projection (constitutional: engine).
//!
//! The rule for what lands here rather than next to a single caller: a helper with MORE THAN ONE
//! consumer across the taxonomy tree lives here; a helper with exactly one consumer lives in that
//! consumer's component file.

use crate::artifacts::sequence::op::{sequence_snapshot_mutations, SequenceMutation};
use crate::artifacts::sequence::{default_snapshot, SequenceCamera, SequenceEdge, SequenceSnapshot, SequenceStep, SlotRef, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use infinite_board_port_directed_dag as dag;

use dag::{dag_fixture_to_wire_literal, would_create_cycle, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape};
use imperative_engine::{
    compile_to_text as imperative_compile_to_text, contributions_json_from_entries, imperative_catalogue_json, imperative_module_registry,
    register_native_imperative_module, sync_imperative_module_contributions, Executor, Path, RunResult, Step,
};
use math::graph::manifest::PropertyBag;
use neural_engine::{ChannelSpec, Dictionary, Registry, Value};
use std::collections::{BTreeMap, HashMap};
use store::DocumentDsl;

const SEQUENCE_DAG_COMPONENT_WIDTH: f64 = 200.0;
const SEQUENCE_DAG_CHANNEL_ROW_HEIGHT: f64 = 24.0;

fn sequence_computation_node_width(_name: &str, _inputs: &[IoPortSpec], _outputs: &[IoPortSpec]) -> f64 {
    SEQUENCE_DAG_COMPONENT_WIDTH
}

fn sequence_computation_node_height(input_count: usize, output_count: usize, _variadic_inputs: bool, _variadic_outputs: bool) -> f64 {
    let rows = input_count.max(output_count).max(1);
    rows as f64 * SEQUENCE_DAG_CHANNEL_ROW_HEIGHT
}
const FLOW_INPUT_PORT: &str = "prev";
const FLOW_OUTPUT_PORT: &str = "next";

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    serde_json::from_value(serde_json::to_value(dict).unwrap_or(serde_json::Value::Null)).unwrap_or_default()
}

/// 🧭️ `pub` — reused by app panels/commands (control-flow nesting, catalogue slots).
pub fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn is_function_kind(kind: &str) -> bool {
    kind.starts_with("math.") || kind.starts_with("logic.") || kind.starts_with("text.")
}

fn parse_serial_suffix(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

fn max_serial_in_snapshot(fixture: &SequenceSnapshot) -> u64 {
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

fn default_control_slot(kind: &str) -> &'static str {
    if kind == "control.if" {
        "then"
    } else {
        "body"
    }
}

fn neural_value_to_dsl_value(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

// 🧯️ `unnecessary_wraps` — mirrors `IoPortSpec::value_type`'s `Option<String>` field shape; every
// branch here happens to be populated today, but the field itself is genuinely optional.
#[allow(clippy::unnecessary_wraps)]
fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.operators.is_empty() {
        Some("value".into())
    } else {
        Some(spec.operators.join(","))
    }
}

fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary) -> IoPortSpec {
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

fn hidden_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

fn hidden_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

fn visible_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "Previous");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

fn visible_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "Next");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

/// 🧭️ `pub` — reused by app panels/commands (control-flow nesting, catalogue slots).
pub fn control_slots(kind: &str) -> &'static [&'static str] {
    match kind {
        "control.if" => &["then", "else"],
        "control.while" | "control.repeat" => &["body"],
        _ => &[],
    }
}

fn slot_key(slot: Option<&SlotRef>) -> Option<(String, String)> {
    slot.map(|entry| (entry.owner.clone(), entry.name.clone()))
}

#[cfg(test)]
fn ensure_imperative_modules_for_tests() {
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

//#region 🔖️Register
/// 🗂️ Registers `SequenceSnapshot`'s pack↔dsl codec under `SEQUENCE_DOCUMENT_SCHEMA` so
/// `framework/sync`'s folder endpoints and any other schema-string-keyed caller can print/parse
/// sequence documents. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::sequence::io::register();

    register_pilot_languages();
    register_artifact_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::sequence::SequencePlayApp>(SEQUENCE_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "sequence.document",
        extension: Some("sequence"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::sequence::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::sequence::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::sequence::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::sequence::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sequence.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sequence.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::sequence::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::sequence::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::sequence::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::sequence::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sequence.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sequence.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::sequence::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::sequence::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("sequence.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sequence.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::sequence::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::sequence::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sequence.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sequence.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::sequence::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::sequence::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sequence.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Camera
/// 🎥️ `SequenceCamera` <-> `DagCamera` conversions — plain functions rather than `From`/`Into` trait
/// impls, because `SequenceCamera` is defined in this artifact's own `🦀️component.rs` and `DagCamera`
/// is foreign (from the DAG layout kernel): neither type nor trait would be local to THIS file, so a
/// trait impl here would violate the orphan rule. Only this engine (which already depends on the DAG
/// kernel for `SequenceHost`) needs the conversion, so plain functions here are both legal and
/// sufficient.
pub fn sequence_camera_from_dag(value: &DagCamera) -> SequenceCamera {
    SequenceCamera { x: value.x, y: value.y, zoom: value.zoom }
}

pub fn dag_camera_from_sequence(value: &SequenceCamera) -> DagCamera {
    DagCamera { x: value.x, y: value.y, zoom: value.zoom }
}
//#endregion 🔖️Camera

//#region ⚠️ Errors
/// 🚨️ Sequence engine's fallible operations.
#[derive(Debug, thiserror::Error)]
pub enum SequenceCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("cannot connect step to itself")]
    SelfConnect,
    #[error("{0} not found")]
    StepNotFound(String),
    #[error("steps must share the same slot scope")]
    MismatchedSlotScope,
    #[error("connection would create cycle")]
    CycleDetected,
    #[error("{0} already has outgoing flow")]
    OutgoingFlowExists(String),
    #[error("unknown step: {0}")]
    UnknownStep(String),
    #[error("{0}")]
    Dag(String),
}
//#endregion ⚠️ Errors

//#region 🔖️Host
pub struct SequenceHost {
    pub snapshot: SequenceSnapshot,
    /// 🎥️ The canvas camera — session-only host state (never a `SequenceSnapshot` document field; see
    /// `crate::apps::sequence::config::SequenceConfig::camera`). Persists across `rebuild_dag()` calls
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
    pub fn from_snapshot(fixture: SequenceSnapshot) -> Self {
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

    pub fn replace_snapshot(&mut self, fixture: SequenceSnapshot) -> Result<(), SequenceCoreError> {
        if fixture.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        self.next_serial = self.next_serial.max(max_serial_in_snapshot(&fixture));
        self.snapshot = fixture;
        self.rebuild_dag();
        Ok(())
    }

    pub fn load_json(json: &str) -> Result<Self, SequenceCoreError> {
        let fixture: SequenceSnapshot = serde_json::from_str(json)?;
        if fixture.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        Ok(Self::from_snapshot(fixture))
    }

    pub fn to_json(&self) -> Result<String, SequenceCoreError> {
        Ok(serde_json::to_string(&self.snapshot)?)
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
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

    pub fn add_step(&mut self, kind: &str, x: f64, y: f64) -> String {
        self.add_step_in_slot(kind, x, y, None)
    }

    pub fn add_step_dropped(&mut self, kind: &str, x: f64, y: f64, picked_step_id: Option<&str>) -> String {
        if let Some(owner_id) = picked_step_id {
            if let Some(owner) = self.snapshot.steps.iter().find(|step| step.id == owner_id) {
                if is_control_kind(&owner.kind) && !owner.collapsed {
                    return self.add_step_in_slot(kind, x, y, Some(SlotRef { owner: owner_id.into(), name: default_control_slot(&owner.kind).into() }));
                }
            }
        }
        self.add_step(kind, x, y)
    }

    fn next_step_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("step-{}", self.next_serial);
            if !self.snapshot.steps.iter().any(|step| step.id == id) {
                return id;
            }
        }
    }

    fn next_edge_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("edge-{}", self.next_serial);
            if !self.snapshot.edges.iter().any(|edge| edge.id == id) {
                return id;
            }
        }
    }

    pub fn add_step_in_slot(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        self.clear_ghost_step();
        let id = self.next_step_id();
        self.snapshot.steps.push(SequenceStep { id: id.clone(), kind: kind.into(), params: StepParams::new(), x, y, slot, collapsed: false });
        self.rebuild_dag();
        id
    }

    pub fn set_step_collapsed(&mut self, id: &str, collapsed: bool) -> bool {
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

    pub fn remove_step(&mut self, id: &str) -> bool {
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

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), SequenceCoreError> {
        let params: StepParams = serde_json::from_str(json)?;
        let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == id) else {
            return Err(SequenceCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        self.rebuild_dag();
        Ok(())
    }

    pub fn connect_steps(&mut self, from_id: &str, to_id: &str) -> Result<String, SequenceCoreError> {
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

    pub fn disconnect_steps(&mut self, from_id: &str, to_id: &str) -> bool {
        let before = self.snapshot.edges.len();
        self.snapshot.edges.retain(|edge| !(edge.from == from_id && edge.to == to_id));
        if self.snapshot.edges.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub fn sync_edges_from_dag(&mut self) {
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

    pub fn sync_from_dag(&mut self) {
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

    pub fn build_path(&self) -> Path {
        self.build_path_for_slot(None)
    }

    pub fn build_path_json(&self) -> Result<String, SequenceCoreError> {
        Ok(serde_json::to_string(&self.build_path())?)
    }

    fn build_path_for_slot(&self, slot: Option<&SlotRef>) -> Path {
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

    fn step_to_imperative_step(&self, step: &SequenceStep) -> Step {
        let mut bodies = BTreeMap::new();
        if is_control_kind(&step.kind) {
            for slot_name in control_slots(&step.kind) {
                let slot_ref = SlotRef { owner: step.id.clone(), name: slot_name.to_string() };
                bodies.insert(slot_name.to_string(), self.build_path_for_slot(Some(&slot_ref)));
            }
        }
        Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies }
    }

    fn is_step_visible(&self, step: &SequenceStep) -> bool {
        let Some(slot) = &step.slot else {
            return true;
        };
        let Some(owner) = self.snapshot.steps.iter().find(|entry| entry.id == slot.owner) else {
            return false;
        };
        !owner.collapsed
    }

    fn slot_member_count(&self, owner_id: &str) -> usize {
        self.snapshot.steps.iter().filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == owner_id)).count()
    }

    pub fn layout_expanded_slots(&mut self) {
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
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), SequenceCoreError> {
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

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.build_path(), &Dictionary::new())
    }

    pub fn compile_text(&self) -> String {
        imperative_compile_to_text(&self.build_path())
    }

    /// 📝️ Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_fixture_to_wire_literal(&self.build_dag_fixture())
    }

    fn rebuild_dag(&mut self) {
        let selected = self.dag.selected_node_ids();
        let dag_fixture = self.build_dag_fixture();
        self.dag = DagHost::from_fixture_without_layout(dag_fixture);
        self.dag.set_camera(self.camera.x, self.camera.y, self.camera.zoom);
        if !selected.is_empty() {
            self.dag.set_selection(&selected);
        }
    }

    fn build_dag_fixture(&self) -> DagFixture {
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

    fn step_to_dag_node(&self, step: &SequenceStep) -> DagNodeSpec {
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

    pub fn set_ghost_step(&mut self, kind: &str, x: f64, y: f64) {
        let ghost = SequenceStep { id: "__ghost__".into(), kind: kind.into(), params: StepParams::new(), x, y, slot: None, collapsed: false };
        let node = self.step_to_dag_node(&ghost);
        self.dag.set_ghost_node(Some(node));
    }

    pub fn clear_ghost_step(&mut self) {
        self.dag.set_ghost_node(None);
    }
}
//#endregion 🔖️Host

//#region 🔖️HostHelpers
/// 🧰️ Builds a {@link SequenceHost} seeded from a projection so a command can mutate it (with all the
/// host's cycle/slot/layout logic) and then diff the result into typed operations. More than one
/// consumer across the taxonomy tree (commands, windows), so it lives here rather than in a single
/// caller's file.
pub fn host_from_snapshot(fixture: &SequenceSnapshot) -> SequenceHost {
    SequenceHost::from_snapshot(fixture.clone())
}

/// 🔀️ Runs a host mutation seeded from `fixture` and diffs the result into typed operations — a free
/// function (not a method) since `SequencePlayApp` is a unit struct with nothing to borrow.
pub fn ops_from_host_mutation(fixture: &SequenceSnapshot, mutate: impl FnOnce(&mut SequenceHost)) -> Vec<SequenceMutation> {
    let mut host = host_from_snapshot(fixture);
    mutate(&mut host);
    sequence_snapshot_mutations(fixture, &host.snapshot)
}
//#endregion 🔖️HostHelpers

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_sequence_app`'s
/// `.artifact_kind(...)` literal (schema/media type copied verbatim) plus the extra `steps:in` input
/// port (Wave-2 port recipe): incoming computation results from an upstream workflow node insert as
/// new steps in the sequence document (see `crate::apps::sequence::SequencePlayApp::import_media`).
pub fn sequence_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
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
/// `DocumentApp::import_media` call): derives the next id purely from the fixture's own existing
/// `step-N`/`edge-N` ids, exactly like `SequenceHost::from_snapshot`'s own initial-serial derivation.
pub fn next_available_step_id(fixture: &SequenceSnapshot) -> String {
    format!("step-{}", max_serial_in_snapshot(fixture).max(100) + 1)
}
//#endregion 🔖️Io

//#region 🔖️Example
/// 📄️ JSON re-serialization of `default_snapshot()`, round-tripped through its own `.sequence` DSL
/// first (see `crate::artifacts::sequence::dsl`), to prove the fixture is fully expressible in text —
/// for the framework-generic call site that contractually requires JSON (`App::example`'s manifest
/// `document_json` is loaded via `serde_json::from_str` by `DocumentApp::load_document`'s default impl)
/// — out of scope to change, since both are defined in `framework/plugin`.
pub fn sequence_example_json() -> String {
    let fixture = <SequenceSnapshot as DocumentDsl>::parse_dsl(&default_snapshot().print_dsl()).expect("default_snapshot round-trips through its own DSL");
    serde_json::to_string(&fixture).expect("default_snapshot is a static, hand-built value with no non-finite floats or non-UTF8 keys")
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::Atom;

    #[test]
    fn disconnect_steps_removes_edge() {
        let mut host = SequenceHost::default();
        assert!(host.disconnect_steps("step-1", "step-2"));
        assert!(host.snapshot.edges.is_empty());
    }

    #[test]
    fn sync_from_dag_copies_node_positions() {
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

    #[test]
    fn sync_edges_from_dag_preserves_existing_edge_ids() {
        let mut host = SequenceHost::default();
        let first_id = host.snapshot.edges[0].id.clone();
        host.sync_edges_from_dag();
        assert_eq!(host.snapshot.edges[0].id, first_id);
        host.sync_edges_from_dag();
        assert_eq!(host.snapshot.edges[0].id, first_id);
    }

    #[test]
    fn connect_steps_rejects_fan_out() {
        let mut host = SequenceHost::default();
        host.snapshot.edges.clear();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.connect_steps("step-1", "step-2").is_ok());
        assert!(host.connect_steps("step-1", "step-3").is_err());
    }

    #[test]
    fn build_path_includes_control_bodies() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("key", Value::Atom(Atom::String("flag".into()))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", Value::Atom(Atom::String("yes".into()))),
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

    #[test]
    fn rebuild_dag_preserves_selection() {
        let mut host = SequenceHost::default();
        host.dag.set_selection(&["step-1".into()]);
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.rebuild_dag();
        assert!(host.dag.selected_node_ids().contains(&"step-1".to_string()));
    }

    #[test]
    fn execution_ports_use_triangle_shape() {
        let host = SequenceHost::default();
        let node = host.step_to_dag_node(&host.snapshot.steps[1]);
        assert_eq!(node.inputs()[0].shape, PortShape::Triangle);
        assert_eq!(node.outputs()[0].shape, PortShape::Triangle);
    }

    #[test]
    fn function_steps_use_data_ports_without_visible_execution_pins() {
        let host = SequenceHost::default();
        let step = SequenceStep { id: "step-fn".into(), kind: "math.add".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
        let node = host.step_to_dag_node(&step);
        assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
        assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
        assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
    }

    #[test]
    fn text_steps_use_data_ports_without_visible_execution_pins() {
        let host = SequenceHost::default();
        let step = SequenceStep { id: "step-txt".into(), kind: "text.concat".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
        let node = host.step_to_dag_node(&step);
        assert!(node.inputs().iter().any(|port| port.id == "left" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "into" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
        assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
        assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
    }

    #[test]
    fn replace_snapshot_preserves_next_serial_and_selection() {
        let mut host = SequenceHost::default();
        let first = host.add_step("math.add", 40.0, 40.0);
        host.dag.set_selection(std::slice::from_ref(&first));
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceSnapshot = serde_json::from_str(&json).expect("parse");
        host.replace_snapshot(round_trip).expect("replace");
        let second = host.add_step("math.add", 80.0, 80.0);
        assert_ne!(first, second);
        assert!(host.snapshot.steps.iter().any(|step| step.id == first));
        assert!(host.snapshot.steps.iter().any(|step| step.id == second));
        assert!(host.dag.selected_node_ids().contains(&first));
    }

    #[test]
    fn repeated_drops_after_replace_snapshot_use_distinct_ids() {
        let mut host = SequenceHost::default();
        let first = host.add_step_dropped("math.add", 10.0, 10.0, None);
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceSnapshot = serde_json::from_str(&json).expect("parse");
        host.replace_snapshot(round_trip).expect("replace");
        let second = host.add_step_dropped("math.add", 20.0, 20.0, None);
        assert_ne!(first, second);
        assert_eq!(host.snapshot.steps.iter().filter(|step| step.kind == "math.add").count(), 2);
    }

    #[test]
    fn add_step_dropped_targets_expanded_control_slot() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert_eq!(step.slot.as_ref().map(|slot| slot.name.as_str()), Some("then"));
    }

    #[test]
    fn execution_edges_use_sharp_sz_routing() {
        let host = SequenceHost::default();
        let fixture = host.build_dag_fixture();
        assert!(fixture.edges.iter().all(|edge| edge.route_style == EdgeRouteStyle::SharpSz));
    }

    #[test]
    fn set_step_collapsed_toggles_control_step() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.set_step_collapsed("step-3", true));
        assert!(host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().collapsed);
    }

    #[test]
    fn set_step_collapsed_rejects_unknown_id() {
        let mut host = SequenceHost::default();
        assert!(!host.set_step_collapsed("nope", true));
    }

    #[test]
    fn set_step_collapsed_rejects_non_control_step() {
        let mut host = SequenceHost::default();
        assert!(!host.set_step_collapsed("step-1", true));
        assert!(!host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap().collapsed);
    }

    #[test]
    fn remove_step_also_removes_slot_children() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(host.remove_step("step-3"));
        assert!(!host.snapshot.steps.iter().any(|step| step.id == "step-3" || step.id == "step-4"));
    }

    #[test]
    fn remove_step_returns_false_for_unknown_id() {
        let mut host = SequenceHost::default();
        assert!(!host.remove_step("nope"));
    }

    #[test]
    fn set_step_params_json_updates_step_params() {
        let mut host = SequenceHost::default();
        host.set_step_params_json("step-1", r#"{"key":"renamed"}"#).expect("set params");
        let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap();
        assert_eq!(step.params.get("key").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("renamed"));
    }

    #[test]
    fn set_step_params_json_rejects_unknown_step() {
        let mut host = SequenceHost::default();
        let err = host.set_step_params_json("nope", "{}").unwrap_err();
        assert!(matches!(err, SequenceCoreError::UnknownStep(id) if id == "nope"));
    }

    #[test]
    fn set_step_params_json_rejects_invalid_json() {
        let mut host = SequenceHost::default();
        let err = host.set_step_params_json("step-1", "not json").unwrap_err();
        assert!(matches!(err, SequenceCoreError::Json(_)));
    }

    #[test]
    fn connect_steps_rejects_self_connect() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("step-1", "step-1").unwrap_err(), SequenceCoreError::SelfConnect));
    }

    #[test]
    fn connect_steps_rejects_unknown_from_step() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("nope", "step-2").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
    }

    #[test]
    fn connect_steps_rejects_unknown_to_step() {
        let mut host = SequenceHost::default();
        assert!(matches!(host.connect_steps("step-1", "nope").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
    }

    #[test]
    fn connect_steps_rejects_mismatched_slot_scope() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(matches!(host.connect_steps("step-2", "step-4").unwrap_err(), SequenceCoreError::MismatchedSlotScope));
    }

    #[test]
    fn connect_steps_rejects_cycle() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-2", "step-3").expect("connect step-2 to step-3");
        assert!(matches!(host.connect_steps("step-3", "step-1").unwrap_err(), SequenceCoreError::CycleDetected));
    }

    #[test]
    fn connect_steps_rewires_existing_incoming_edge() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-3", "step-2").expect("rewire onto step-2");
        assert_eq!(host.snapshot.edges.len(), 1);
        assert_eq!(host.snapshot.edges[0].from, "step-3");
        assert_eq!(host.snapshot.edges[0].to, "step-2");
    }

    #[test]
    fn disconnect_steps_returns_false_when_no_matching_edge() {
        let mut host = SequenceHost::default();
        assert!(!host.disconnect_steps("step-2", "step-1"));
        assert_eq!(host.snapshot.edges.len(), 1);
    }

    #[test]
    fn load_json_parses_valid_fixture() {
        let json = SequenceHost::default().to_json().expect("fixture json");
        let host = SequenceHost::load_json(&json).expect("load json");
        assert_eq!(host.snapshot.steps.len(), 2);
    }

    #[test]
    fn load_json_rejects_unsupported_schema() {
        let result = SequenceHost::load_json(r#"{"schema":"other","steps":[],"edges":[]}"#);
        assert!(matches!(result, Err(SequenceCoreError::UnsupportedSchema(schema)) if schema == "other"));
    }

    #[test]
    fn catalogue_json_reports_imperative_catalogue_schema() {
        let host = SequenceHost::default();
        assert!(host.catalogue_json().contains("\"imperative.catalogue\""));
    }

    #[test]
    fn layout_expanded_slots_positions_slot_members_relative_to_owner() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        host.layout_expanded_slots();
        let child = host.snapshot.steps.iter().find(|step| step.id == "step-4").unwrap();
        assert_eq!(child.x, 400.0);
        assert_eq!(child.y, 160.0);
    }

    #[test]
    fn reorganize_syncs_step_positions_from_dag_layout() {
        let mut host = SequenceHost::default();
        host.reorganize(&DagLayoutOptions::default()).expect("reorganize");
        for step in &host.snapshot.steps {
            let node = host.dag.fixture.nodes.iter().find(|node| node.id == step.id).expect("node for step");
            assert_eq!(step.x, node.x);
            assert_eq!(step.y, node.y);
        }
    }

    #[test]
    fn pick_step_id_at_screen_finds_step_under_cursor() {
        let host = SequenceHost::default();
        let id = host.pick_step_id_at_screen(400.0, 300.0, 800, 600, 1.0);
        assert_eq!(id, Some("step-1".to_string()));
    }

    #[test]
    fn pick_step_id_at_screen_returns_none_when_missing_all_nodes() {
        let host = SequenceHost::default();
        let id = host.pick_step_id_at_screen(-9000.0, -9000.0, 800, 600, 1.0);
        assert_eq!(id, None);
    }

    #[test]
    fn add_step_dropped_falls_back_when_owner_collapsed() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: true });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn add_step_dropped_falls_back_for_non_control_owner() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("step-2"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn add_step_dropped_falls_back_for_unknown_owner_id() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("nope"));
        let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn build_path_returns_unordered_slot_body_when_multiple_heads() {
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

    #[test]
    fn step_to_dag_node_shows_collapsed_indicator_for_collapsed_control_step() {
        let mut host = SequenceHost::default();
        host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let expanded = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(expanded.abbreviation, "▾️0");
        host.set_step_collapsed("step-3", true);
        let collapsed = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(collapsed.abbreviation, "▸️0");
    }

    #[test]
    fn set_ghost_step_and_clear_ghost_step_toggle_dag_ghost_node() {
        let mut host = SequenceHost::default();
        assert!(host.dag.ghost_node().is_none());
        host.set_ghost_step("math.add", 10.0, 20.0);
        assert!(host.dag.ghost_node().is_some());
        host.clear_ghost_step();
        assert!(host.dag.ghost_node().is_none());
    }

    #[test]
    fn run_executes_default_snapshot_and_records_scope() {
        let host = SequenceHost::default();
        let result = host.run();
        assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
        assert!(!result.effects.is_empty());
    }

    #[test]
    fn compile_text_renders_default_snapshot_steps() {
        let host = SequenceHost::default();
        let text = host.compile_text();
        assert!(text.contains("state.set"));
        assert!(text.contains("log.print"));
    }

    #[test]
    fn compiled_wire_literal_includes_step_ids() {
        let host = SequenceHost::default();
        let literal = host.compiled_wire_literal();
        assert!(literal.contains("step-1"));
        assert!(literal.contains("step-2"));
    }

    #[test]
    fn sequence_io_declares_the_steps_in_port() {
        let io = sequence_io();
        assert_eq!(io.document_schema, SEQUENCE_DOCUMENT_SCHEMA);
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "steps:in");
        assert_eq!(port.direction, semio_framework::MediaPortDirection::In);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[test]
    fn next_available_step_id_is_free_and_deterministic() {
        let fixture = default_snapshot();
        let id = next_available_step_id(&fixture);
        assert!(!fixture.steps.iter().any(|step| step.id == id));
        assert_eq!(id, next_available_step_id(&fixture), "pure function of the fixture, not a mutating counter");
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — every transition is a `SequenceMutation`.
pub struct SequenceEngine {
    artifact: crate::artifacts::sequence::schema::SequenceArtifact,
    snapshot: crate::artifacts::sequence::SequenceSnapshot,
}

impl SequenceEngine {
    pub fn new(snapshot: crate::artifacts::sequence::SequenceSnapshot) -> Self {
        let artifact = crate::artifacts::sequence::schema::SequenceArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::sequence::SequenceSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for SequenceEngine {
    type Artifact = crate::artifacts::sequence::schema::SequenceArtifact;
    type Snapshot = crate::artifacts::sequence::SequenceSnapshot;
    type Mutation = crate::artifacts::sequence::mutations::SequenceMutation;
    type Diff = crate::artifacts::sequence::diff::SequenceDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.sequence.sequence`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::sequence::schema::sequence_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

