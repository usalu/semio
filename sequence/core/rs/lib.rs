//! 📜 Sequence core: execution-flow canvas host wrapping DagHost.

pub use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, EffectLogEntry, Executor, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry};
pub use infinite_board_port_directed_dag as dag;

use dag::{dag_fixture_to_wire_literal, would_create_cycle, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape};
use imperative_engine::compile_to_text as imperative_compile_to_text;
use mathematical_graph_manifest::PropertyBag;
use neural_engine::{Atom, ChannelSpec, Dictionary, Registry, Value};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

const FLOW_INPUT_PORT: &str = "prev";
const FLOW_OUTPUT_PORT: &str = "next";

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    serde_json::from_value(serde_json::to_value(dict).unwrap_or(serde_json::Value::Null)).unwrap_or_default()
}

fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn is_function_kind(kind: &str) -> bool {
    kind.starts_with("math.") || kind.starts_with("logic.") || kind.starts_with("text.")
}

fn parse_serial_suffix(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

fn max_serial_in_fixture(fixture: &SequenceFixture) -> u64 {
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

fn neural_value_to_json_value(value: &Value) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

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
    port.default = spec.default.as_ref().map(neural_value_to_json_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_json_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_json_value);
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

fn control_slots(kind: &str) -> &'static [&'static str] {
    match kind {
        "control.if" => &["then", "else"],
        "control.while" | "control.repeat" => &["body"],
        _ => &[],
    }
}

fn slot_key(slot: Option<&SlotRef>) -> Option<(String, String)> {
    slot.map(|entry| (entry.owner.clone(), entry.name.clone()))
}

// #region 🔖Fixture
/// 📦 Local newtype around {@link neural_engine::Dictionary} — dynamic/schema-less step params
/// can't be shape-derived field-by-field (arbitrary keys, recursive `Value`), and `Dictionary`
/// itself can't gain a `dsl::DslField` impl directly (foreign trait, foreign type, no local anchor
/// for the orphan rule). Wrapping it as one opaque JSON-text field reuses the exact `serde_json`
/// round trip {@link SequenceHost::to_json}/{@link SequenceHost::load_json} already depend on for
/// fidelity — unlike a schema-less `dsl::Shape::Value`, this never collapses `Atom::Integer` and
/// `Atom::Decimal` into the same wire number.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StepParams(pub Dictionary);

impl StepParams {
    pub fn new() -> Self {
        Self(Dictionary::new())
    }

    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        Self(self.0.insert(key, value))
    }
}

impl std::ops::Deref for StepParams {
    type Target = Dictionary;
    fn deref(&self) -> &Dictionary {
        &self.0
    }
}

impl dsl::DslField for StepParams {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(serde_json::to_string(&self.0).unwrap_or_else(|_| "{}".into()))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(text) => serde_json::from_str(text).map(Self).map_err(|err| err.to_string()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🎥 Local DSL-derivable mirror of {@link dag::DagCamera} — the foreign type can't itself gain a
/// `dsl::DslField` impl (crate-external, no local type to anchor the orphan-rule-legal impl on), so
/// this newtype carries the same three fields through the sequence grammar and converts losslessly
/// at the `DagHost`/`DagFixture` boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl From<DagCamera> for SequenceCamera {
    fn from(value: DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}

impl From<SequenceCamera> for DagCamera {
    fn from(value: SequenceCamera) -> Self {
        DagCamera { x: value.x, y: value.y, zoom: value.zoom }
    }
}

/// 🎯 Only ever embedded `#[dsl(block)]`-wrapped (on `SequenceStep::slot`), so it carries no
/// `#[dsl(keyword = "...")]` of its own — the embedding field already supplies the bare `slot`
/// leading keyword.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SlotRef {
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStep {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: StepParams,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    #[dsl(block)]
    pub slot: Option<SlotRef>,
    #[serde(default)]
    pub collapsed: bool,
}

/// 🔌 Runtime edge shape (id/from/to step ids) — kept plain `Serialize`/`Deserialize` only; the
/// `.sequence` DSL text and op-log representations go through the `SequenceEdgeDsl` mirror (see
/// `🔖Dsl`/`🔖OpText`) instead of deriving `dsl::DslRecord` here directly, so this struct (and
/// every consumer matching on `.from`/`.to` — `connect_steps`, `sync_edges_from_dag`, ...) stays
/// untouched by the unified `dsl::Wire` connection syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdge {
    pub id: String,
    pub from: String,
    pub to: String,
}

/// 🧾 Runtime fixture shape — kept plain `Serialize`/`Deserialize` only; see `SequenceFixtureDsl`
/// (`🔖Dsl` region) for the `.sequence` DSL text mirror (SoA `steps`/`edges` tables, `edges` as
/// `dsl::Wire` links) and the hand-written `impl store::DocumentDsl for SequenceFixture` that
/// converts through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFixture {
    pub schema: String,
    pub camera: SequenceCamera,
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

impl Default for SequenceFixture {
    fn default() -> Self {
        default_fixture()
    }
}

pub fn default_fixture() -> SequenceFixture {
    SequenceFixture {
        schema: "sequence.fixture".into(),
        camera: SequenceCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: StepParams::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Decimal(0.0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            SequenceStep { id: "step-2".into(), kind: "log.print".into(), params: StepParams::new().insert("message", Value::Atom(Atom::String("hello sequence".into()))), x: 280.0, y: 0.0, slot: None, collapsed: false },
        ],
        edges: vec![SequenceEdge { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    }
}
// #endregion 🔖Fixture

// #region ⚠️ Errors
/// 🚨 Sequence core's fallible operations.
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
// #endregion ⚠️ Errors

// #region 🔖Host
pub struct SequenceHost {
    pub fixture: SequenceFixture,
    pub dag: DagHost,
    registry: Registry,
    next_serial: u64,
}

impl Default for SequenceHost {
    fn default() -> Self {
        Self::from_fixture(default_fixture())
    }
}

impl SequenceHost {
    pub fn from_fixture(fixture: SequenceFixture) -> Self {
        let next_serial = max_serial_in_fixture(&fixture).max(100);
        let mut host =
            Self { fixture, dag: DagHost::from_fixture_without_layout(DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }), registry: imperative_module_registry(), next_serial };
        host.rebuild_dag();
        host
    }

    pub fn replace_fixture(&mut self, fixture: SequenceFixture) -> Result<(), SequenceCoreError> {
        if fixture.schema != "sequence.fixture" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        self.next_serial = self.next_serial.max(max_serial_in_fixture(&fixture));
        self.fixture = fixture;
        self.rebuild_dag();
        Ok(())
    }

    pub fn load_json(json: &str) -> Result<Self, SequenceCoreError> {
        let fixture: SequenceFixture = serde_json::from_str(json)?;
        if fixture.schema != "sequence.fixture" {
            return Err(SequenceCoreError::UnsupportedSchema(fixture.schema));
        }
        Ok(Self::from_fixture(fixture))
    }

    pub fn to_json(&self) -> Result<String, SequenceCoreError> {
        Ok(serde_json::to_string(&self.fixture)?)
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
        use infinite_cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use infinite_cavas::Point;
        let viewport = Viewport { width: width.max(1), height: height.max(1), dpr: dpr.max(1.0) };
        let camera = CavasCamera { x: self.dag.fixture.camera.x, y: self.dag.fixture.camera.y, zoom: self.dag.fixture.camera.zoom };
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
            if let Some(owner) = self.fixture.steps.iter().find(|step| step.id == owner_id) {
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
            if !self.fixture.steps.iter().any(|step| step.id == id) {
                return id;
            }
        }
    }

    fn next_edge_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("edge-{}", self.next_serial);
            if !self.fixture.edges.iter().any(|edge| edge.id == id) {
                return id;
            }
        }
    }

    pub fn add_step_in_slot(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        self.clear_ghost_step();
        let id = self.next_step_id();
        self.fixture.steps.push(SequenceStep { id: id.clone(), kind: kind.into(), params: StepParams::new(), x, y, slot, collapsed: false });
        self.rebuild_dag();
        id
    }

    pub fn set_step_collapsed(&mut self, id: &str, collapsed: bool) -> bool {
        let Some(step) = self.fixture.steps.iter_mut().find(|step| step.id == id) else {
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
        let before = self.fixture.steps.len();
        let mut remove_ids = vec![id.to_string()];
        if self.fixture.steps.iter().any(|step| step.id == id && is_control_kind(&step.kind)) {
            for step in &self.fixture.steps {
                if step.slot.as_ref().is_some_and(|slot| slot.owner == id) {
                    remove_ids.push(step.id.clone());
                }
            }
        }
        self.fixture.steps.retain(|step| !remove_ids.iter().any(|remove_id| remove_id == &step.id));
        self.fixture.edges.retain(|edge| !remove_ids.iter().any(|remove_id| remove_id == &edge.from || remove_id == &edge.to));
        if self.fixture.steps.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), SequenceCoreError> {
        let params: StepParams = serde_json::from_str(json)?;
        let Some(step) = self.fixture.steps.iter_mut().find(|step| step.id == id) else {
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
        let from_step = self.fixture.steps.iter().find(|step| step.id == from_id).ok_or_else(|| SequenceCoreError::StepNotFound(from_id.into()))?;
        let to_step = self.fixture.steps.iter().find(|step| step.id == to_id).ok_or_else(|| SequenceCoreError::StepNotFound(to_id.into()))?;
        if slot_key(from_step.slot.as_ref()) != slot_key(to_step.slot.as_ref()) {
            return Err(SequenceCoreError::MismatchedSlotScope);
        }
        let existing: Vec<(String, String)> = self.fixture.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(SequenceCoreError::CycleDetected);
        }
        if self.fixture.edges.iter().any(|edge| edge.from == from_id) {
            return Err(SequenceCoreError::OutgoingFlowExists(from_id.into()));
        }
        if self.fixture.edges.iter().any(|edge| edge.to == to_id) {
            self.fixture.edges.retain(|edge| edge.to != to_id);
        }
        let id = self.next_edge_id();
        self.fixture.edges.push(SequenceEdge { id: id.clone(), from: from_id.into(), to: to_id.into() });
        self.rebuild_dag();
        Ok(id)
    }

    pub fn disconnect_steps(&mut self, from_id: &str, to_id: &str) -> bool {
        let before = self.fixture.edges.len();
        self.fixture.edges.retain(|edge| !(edge.from == from_id && edge.to == to_id));
        if self.fixture.edges.len() == before {
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
            let id = self.fixture.edges.iter().find(|edge| edge.from == from && edge.to == to).map(|edge| edge.id.clone()).unwrap_or_else(|| self.next_edge_id());
            edges.push(SequenceEdge { id, from, to });
        }
        self.fixture.edges = edges;
    }

    pub fn sync_from_dag(&mut self) {
        self.fixture.camera = self.dag.fixture.camera.clone().into();
        self.sync_edges_from_dag();
        for step in &mut self.fixture.steps {
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
        let scoped_steps: Vec<&SequenceStep> = self.fixture.steps.iter().filter(|step| slot_key(step.slot.as_ref()) == slot_filter).collect();
        let incoming: HashMap<&str, &str> = self.fixture.edges.iter().map(|edge| (edge.to.as_str(), edge.from.as_str())).collect();
        let outgoing: HashMap<&str, &str> = self.fixture.edges.iter().map(|edge| (edge.from.as_str(), edge.to.as_str())).collect();
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
        let Some(owner) = self.fixture.steps.iter().find(|entry| entry.id == slot.owner) else {
            return false;
        };
        !owner.collapsed
    }

    fn slot_member_count(&self, owner_id: &str) -> usize {
        self.fixture.steps.iter().filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == owner_id)).count()
    }

    pub fn layout_expanded_slots(&mut self) {
        let control_steps: Vec<(String, String, bool)> = self.fixture.steps.iter().filter(|step| is_control_kind(&step.kind)).map(|step| (step.id.clone(), step.kind.clone(), step.collapsed)).collect();
        for (owner_id, kind, collapsed) in control_steps {
            if collapsed {
                continue;
            }
            let owner = self.fixture.steps.iter().find(|step| step.id == owner_id);
            let Some(owner) = owner else { continue };
            let base_x = owner.x;
            let base_y = owner.y + 160.0;
            for (index, slot_name) in control_slots(&kind).iter().enumerate() {
                let slot_ref = SlotRef { owner: owner_id.clone(), name: (*slot_name).into() };
                let members: Vec<String> = self.fixture.steps.iter().filter(|step| step.slot.as_ref() == Some(&slot_ref)).map(|step| step.id.clone()).collect();
                let offset_x = base_x + (index as f64 - (control_slots(&kind).len() as f64 - 1.0) * 0.5) * 320.0;
                for (member_index, member_id) in members.iter().enumerate() {
                    if let Some(step) = self.fixture.steps.iter_mut().find(|step| step.id == *member_id) {
                        step.x = offset_x + member_index as f64 * 280.0;
                        step.y = base_y;
                    }
                }
            }
        }
        self.rebuild_dag();
    }

    /// 🌳 Recomputes visible step positions using the shared layered DAG tree layout, then rebuilds the DAG view.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), SequenceCoreError> {
        self.dag.reorganize(opts).map_err(|e| SequenceCoreError::Dag(e.to_string()))?;
        let positions: HashMap<String, (f64, f64)> = self.dag.fixture.nodes.iter().map(|node| (node.id.clone(), (node.x, node.y))).collect();
        for step in self.fixture.steps.iter_mut() {
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

    /// 📝 Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_fixture_to_wire_literal(&self.build_dag_fixture())
    }

    fn rebuild_dag(&mut self) {
        let selected = self.dag.selected_node_ids();
        let dag_fixture = self.build_dag_fixture();
        self.dag = DagHost::from_fixture_without_layout(dag_fixture);
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        if !selected.is_empty() {
            self.dag.set_selection(&selected);
        }
    }

    fn build_dag_fixture(&self) -> DagFixture {
        let nodes: Vec<DagNodeSpec> = self.fixture.steps.iter().filter(|step| self.is_step_visible(step)).map(|step| self.step_to_dag_node(step)).collect();
        let visible_ids: std::collections::HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let existing: Vec<(String, String)> = self.fixture.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        let edges: Vec<DagFixtureEdge> = self
            .fixture
            .edges
            .iter()
            .filter(|edge| visible_ids.contains(&edge.from) && visible_ids.contains(&edge.to))
            .filter(|edge| !would_create_cycle(&existing, &edge.from, &edge.to))
            .map(|edge| DagFixtureEdge { id: edge.id.clone(), source: format!("{}@{}", edge.from, FLOW_OUTPUT_PORT), target: format!("{}@{}", edge.to, FLOW_INPUT_PORT), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::new() })
            .collect();
        DagFixture { schema: "dag.fixture".into(), camera: self.fixture.camera.clone().into(), nodes, edges }
    }

    fn step_to_dag_node(&self, step: &SequenceStep) -> DagNodeSpec {
        let info = self.registry.operator_info(&step.kind);
        let (name, mut abbreviation, icon) = info.as_ref().map(|entry| (entry.name.clone(), entry.abbreviation.clone(), entry.icon.clone())).unwrap_or_else(|| (step.kind.clone(), step.kind.clone(), "emoji:⚡".into()));
        if is_control_kind(&step.kind) {
            let count = self.slot_member_count(&step.id);
            abbreviation = if step.collapsed { format!("▸ {count}") } else { format!("▾ {count}") };
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
        let width = dag::computation_node_width(&name, &inputs, &outputs);
        let height = dag::computation_node_height(inputs.len(), outputs.len(), false, false);
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
// #endregion 🔖Host

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct SequenceSessionInner {
        host: SequenceHost,
        gpu: infinite_cavas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
        pointer_down_sx: f64,
        pointer_down_sy: f64,
        pointer_down_button: u8,
    }

    #[wasm_bindgen]
    pub struct SequenceSession {
        state: Rc<RefCell<SequenceSessionInner>>,
    }

    #[wasm_bindgen]
    impl SequenceSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(SequenceSessionInner {
                    host: SequenceHost::default(),
                    gpu: infinite_cavas::gpu_session::CanvasGpuSession::default(),
                    width: 1,
                    height: 1,
                    dpr: 1.0,
                    pointer_down_sx: 0.0,
                    pointer_down_sy: 0.0,
                    pointer_down_button: 255,
                })),
            }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let fixture: SequenceFixture = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.replace_fixture(fixture).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow_mut().host.sync_from_dag();
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, x: f64, y: f64) -> String {
            self.state.borrow_mut().host.add_step(kind, x, y)
        }

        #[wasm_bindgen(js_name = addStepDropped)]
        pub fn add_step_dropped(&self, kind: &str, x: f64, y: f64, picked_step_id: Option<String>) -> String {
            self.state.borrow_mut().host.add_step_dropped(kind, x, y, picked_step_id.as_deref())
        }

        #[wasm_bindgen(js_name = addStepToSlot)]
        pub fn add_step_to_slot(&self, kind: &str, x: f64, y: f64, owner: &str, slot_name: &str) -> String {
            self.state.borrow_mut().host.add_step_in_slot(kind, x, y, Some(SlotRef { owner: owner.into(), name: slot_name.into() }))
        }

        #[wasm_bindgen(js_name = setStepCollapsed)]
        pub fn set_step_collapsed(&self, id: &str, collapsed: bool) -> bool {
            self.state.borrow_mut().host.set_step_collapsed(id, collapsed)
        }

        #[wasm_bindgen(js_name = pickStepIdAtScreen)]
        pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64) -> Option<String> {
            let inner = self.state.borrow();
            inner.host.pick_step_id_at_screen(sx, sy, inner.width, inner.height, inner.dpr)
        }

        #[wasm_bindgen(js_name = buildPathJson)]
        pub fn build_path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.build_path_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = connectSteps)]
        pub fn connect_steps(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.connect_steps(from_id, to_id).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = disconnectSteps)]
        pub fn disconnect_steps(&self, from_id: &str, to_id: &str) -> bool {
            self.state.borrow_mut().host.disconnect_steps(from_id, to_id)
        }

        #[wasm_bindgen(js_name = compileText)]
        pub fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
        }

        #[wasm_bindgen(js_name = compiledWireLiteral)]
        pub fn compiled_wire_literal(&self) -> String {
            self.state.borrow().host.compiled_wire_literal()
        }

        #[wasm_bindgen]
        pub fn run(&self) -> Result<String, JsValue> {
            let result = self.state.borrow().host.run();
            serde_json::to_string(&result).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
            let inner = self.state.clone();
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = infinite_cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.dag.set_viewport(lw, lh, dpr);
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = setSize)]
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let mut inner = self.state.borrow_mut();
            inner.width = width.max(1);
            inner.height = height.max(1);
            inner.dpr = dpr.max(1.0);
            let (w, h, d) = (inner.width, inner.height, inner.dpr);
            inner.host.dag.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            inner.host.fixture.camera = inner.host.dag.fixture.camera.clone().into();
            let mut scene = infinite_cavas::Scene::new();
            let clear = inner.host.dag.canvas_theme.raster_clear;
            inner.host.dag.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = infinite_cavas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = worldFromScreen)]
        pub fn world_from_screen(&self, sx: f64, sy: f64) -> Result<String, JsValue> {
            use infinite_cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
            use infinite_cavas::Point;
            let inner = self.state.borrow();
            let viewport = Viewport { width: inner.width.max(1), height: inner.height.max(1), dpr: inner.dpr.max(1.0) };
            let camera = CavasCamera { x: inner.host.dag.fixture.camera.x, y: inner.host.dag.fixture.camera.y, zoom: inner.host.dag.fixture.camera.zoom };
            let world = screen_to_world(&camera, &viewport, Point::new(sx, sy));
            Ok(format!("{{\"x\":{},\"y\":{}}}", world.x, world.y))
        }

        #[wasm_bindgen(js_name = pointerDownScreen)]
        pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl: bool, alt: bool) {
            {
                let mut inner = self.state.borrow_mut();
                inner.pointer_down_sx = sx;
                inner.pointer_down_sy = sy;
                inner.pointer_down_button = button;
            }
            self.state.borrow_mut().host.dag.pointer_down_screen(sx, sy, button, shift, ctrl, alt, false);
        }

        #[wasm_bindgen(js_name = pointerMoveScreen)]
        pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_move_screen(sx, sy, shift, ctrl, alt);
        }

        #[wasm_bindgen(js_name = pointerUpScreen)]
        pub fn pointer_up_screen(&self, sx: f64, sy: f64, shift: bool, ctrl: bool, alt: bool) {
            let (down_sx, down_sy, button, width, height, dpr) = {
                let inner = self.state.borrow();
                (inner.pointer_down_sx, inner.pointer_down_sy, inner.pointer_down_button, inner.width, inner.height, inner.dpr)
            };
            self.state.borrow_mut().host.dag.pointer_up_screen(sx, sy, shift, ctrl, alt);
            self.state.borrow_mut().host.sync_from_dag();
            if button == 0 && !shift && !ctrl && !alt {
                let dx = sx - down_sx;
                let dy = sy - down_sy;
                if dx * dx + dy * dy <= 64.0 {
                    let selected = self.state.borrow().host.dag.selected_node_ids();
                    if selected.is_empty() {
                        if let Some(id) = self.state.borrow().host.pick_step_id_at_screen(sx, sy, width, height, dpr) {
                            self.state.borrow_mut().host.dag.set_selection(&[id]);
                        }
                    } else if selected.len() == 1 {
                        if let Some(id) = self.state.borrow().host.pick_step_id_at_screen(sx, sy, width, height, dpr) {
                            if !selected.iter().any(|selected_id| selected_id == &id) {
                                self.state.borrow_mut().host.dag.set_selection(&[id]);
                            }
                        }
                    }
                }
            }
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub fn wheel_screen(&self, sx: f64, sy: f64, delta_y: f64) {
            use infinite_cavas::camera::{wheel_screen, Camera as CavasCamera, Viewport};
            let mut inner = self.state.borrow_mut();
            inner.host.dag.set_wheel_zoom_active(true);
            let viewport = Viewport { width: inner.width.max(1), height: inner.height.max(1), dpr: inner.dpr.max(1.0) };
            let mut camera = CavasCamera { x: inner.host.dag.fixture.camera.x, y: inner.host.dag.fixture.camera.y, zoom: inner.host.dag.fixture.camera.zoom };
            wheel_screen(&mut camera, &viewport, sx, sy, delta_y);
            inner.host.dag.set_camera(camera.x, camera.y, camera.zoom);
            inner.host.dag.set_wheel_zoom_active(false);
            inner.host.sync_from_dag();
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, opts_json: &str) -> Result<(), JsValue> {
            let opts: DagLayoutOptions = serde_json::from_str(opts_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.dag.reorganize(&opts).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.sync_from_dag();
            self.state.borrow_mut().host.layout_expanded_slots();
            Ok(())
        }

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            dag::dag_lod_scale_json()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.dag.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.dag.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.dag.draw_lod_label().to_string()
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.dag.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = selectedNodeIds)]
        pub fn selected_node_ids(&self) -> js_sys::Array {
            let ids = self.state.borrow().host.dag.selected_node_ids();
            ids.into_iter().map(|id| JsValue::from_str(&id)).collect()
        }

        #[wasm_bindgen(js_name = setSelection)]
        pub fn set_selection(&self, ids: js_sys::Array) {
            let selected: Vec<String> = ids.iter().filter_map(|value| value.as_string()).collect();
            self.state.borrow_mut().host.dag.set_selection(&selected);
        }

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = hoveredNodeId)]
        pub fn hovered_node_id(&self) -> Option<String> {
            self.state.borrow().host.dag.hovered_node_id()
        }

        #[wasm_bindgen(js_name = preselectNodeIdsJson)]
        pub fn preselect_node_ids_json(&self) -> String {
            let host = self.state.borrow();
            serde_json::to_string(&serde_json::json!({
                "ids": host.host.dag.preselect_widget_ids(),
                "removedIds": host.host.dag.preselect_removed_widget_ids(),
            }))
            .unwrap_or_else(|_| "{\"ids\":[],\"removedIds\":[]}".into())
        }

        #[wasm_bindgen(js_name = selectionPreviewPointsJson)]
        pub fn selection_preview_points_json(&self) -> String {
            self.state.borrow().host.dag.selection_preview_points_json()
        }

        #[wasm_bindgen(js_name = selectionPreviewCrossing)]
        pub fn selection_preview_crossing(&self) -> bool {
            self.state.borrow().host.dag.selection_preview_crossing()
        }

        #[wasm_bindgen(js_name = selectionPreviewMethod)]
        pub fn selection_preview_method(&self) -> String {
            self.state.borrow().host.dag.selection_preview_method().to_string()
        }

        #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
        pub fn selection_union_bounds_screen_json(&self) -> String {
            self.state.borrow().host.dag.selection_union_bounds_screen_json()
        }

        #[wasm_bindgen(js_name = setSelectionOptions)]
        pub fn set_selection_options(&self, method: &str, mode: &str) {
            self.state.borrow_mut().host.dag.set_selection_options(method, mode, true, false, false);
        }

        #[wasm_bindgen(js_name = setGhostStep)]
        pub fn set_ghost_step(&self, kind: &str, x: f64, y: f64) {
            self.state.borrow_mut().host.set_ghost_step(kind, x, y);
        }

        #[wasm_bindgen(js_name = clearGhostStep)]
        pub fn clear_ghost_step(&self) {
            self.state.borrow_mut().host.clear_ghost_step();
        }
    }
}
// #endregion 🔖WasmSession

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disconnect_steps_removes_edge() {
        let mut host = SequenceHost::default();
        assert!(host.disconnect_steps("step-1", "step-2"));
        assert!(host.fixture.edges.is_empty());
    }

    #[test]
    fn sync_from_dag_copies_node_positions() {
        let mut host = SequenceHost::default();
        if let Some(node) = host.dag.fixture.nodes.iter_mut().find(|node| node.id == "step-1") {
            node.x = 120.0;
            node.y = 80.0;
        }
        host.sync_from_dag();
        let step = host.fixture.steps.iter().find(|step| step.id == "step-1").expect("step-1");
        assert_eq!(step.x, 120.0);
        assert_eq!(step.y, 80.0);
    }

    #[test]
    fn sync_edges_from_dag_preserves_existing_edge_ids() {
        let mut host = SequenceHost::default();
        let first_id = host.fixture.edges[0].id.clone();
        host.sync_edges_from_dag();
        assert_eq!(host.fixture.edges[0].id, first_id);
        host.sync_edges_from_dag();
        assert_eq!(host.fixture.edges[0].id, first_id);
    }

    #[test]
    fn connect_steps_rejects_fan_out() {
        let mut host = SequenceHost::default();
        host.fixture.edges.clear();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.connect_steps("step-1", "step-2").is_ok());
        assert!(host.connect_steps("step-1", "step-3").is_err());
    }

    #[test]
    fn build_path_includes_control_bodies() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("key", Value::Atom(Atom::String("flag".into()))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.fixture.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", Value::Atom(Atom::String("yes".into()))),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        host.fixture.edges.push(SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() });
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
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.rebuild_dag();
        assert!(host.dag.selected_node_ids().contains(&"step-1".to_string()));
    }

    #[test]
    fn execution_ports_use_triangle_shape() {
        let host = SequenceHost::default();
        let node = host.step_to_dag_node(&host.fixture.steps[1]);
        assert_eq!(node.inputs()[0].shape, dag::PortShape::Triangle);
        assert_eq!(node.outputs()[0].shape, dag::PortShape::Triangle);
    }

    #[test]
    fn function_steps_use_data_ports_without_visible_execution_pins() {
        let host = SequenceHost::default();
        let step = SequenceStep { id: "step-fn".into(), kind: "math.add".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
        let node = host.step_to_dag_node(&step);
        assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
        assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
        assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
        assert!(!node.inputs().iter().any(|port| port.shape == dag::PortShape::Triangle && port.visible));
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
        assert!(!node.inputs().iter().any(|port| port.shape == dag::PortShape::Triangle && port.visible));
    }

    #[test]
    fn replace_fixture_preserves_next_serial_and_selection() {
        let mut host = SequenceHost::default();
        let first = host.add_step("math.add", 40.0, 40.0);
        host.dag.set_selection(std::slice::from_ref(&first));
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceFixture = serde_json::from_str(&json).expect("parse");
        host.replace_fixture(round_trip).expect("replace");
        let second = host.add_step("math.add", 80.0, 80.0);
        assert_ne!(first, second);
        assert!(host.fixture.steps.iter().any(|step| step.id == first));
        assert!(host.fixture.steps.iter().any(|step| step.id == second));
        assert!(host.dag.selected_node_ids().contains(&first));
    }

    #[test]
    fn repeated_drops_after_replace_fixture_use_distinct_ids() {
        let mut host = SequenceHost::default();
        let first = host.add_step_dropped("math.add", 10.0, 10.0, None);
        let json = host.to_json().expect("fixture json");
        let round_trip: SequenceFixture = serde_json::from_str(&json).expect("parse");
        host.replace_fixture(round_trip).expect("replace");
        let second = host.add_step_dropped("math.add", 20.0, 20.0, None);
        assert_ne!(first, second);
        assert_eq!(host.fixture.steps.iter().filter(|step| step.kind == "math.add").count(), 2);
    }

    #[test]
    fn add_step_dropped_targets_expanded_control_slot() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.fixture.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert_eq!(step.slot.as_ref().map(|slot| slot.name.as_str()), Some("then"));
    }

    #[test]
    fn execution_edges_use_sharp_sz_routing() {
        let host = SequenceHost::default();
        let fixture = host.build_dag_fixture();
        assert!(fixture.edges.iter().all(|edge| edge.route_style == dag::EdgeRouteStyle::SharpSz));
    }

    #[test]
    fn set_step_collapsed_toggles_control_step() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        assert!(host.set_step_collapsed("step-3", true));
        assert!(host.fixture.steps.iter().find(|step| step.id == "step-3").unwrap().collapsed);
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
        assert!(!host.fixture.steps.iter().find(|step| step.id == "step-1").unwrap().collapsed);
    }

    #[test]
    fn remove_step_also_removes_slot_children() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.fixture.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(host.remove_step("step-3"));
        assert!(!host.fixture.steps.iter().any(|step| step.id == "step-3" || step.id == "step-4"));
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
        let step = host.fixture.steps.iter().find(|step| step.id == "step-1").unwrap();
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
        host.fixture.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        assert!(matches!(host.connect_steps("step-2", "step-4").unwrap_err(), SequenceCoreError::MismatchedSlotScope));
    }

    #[test]
    fn connect_steps_rejects_cycle() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-2", "step-3").expect("connect step-2 to step-3");
        assert!(matches!(host.connect_steps("step-3", "step-1").unwrap_err(), SequenceCoreError::CycleDetected));
    }

    #[test]
    fn connect_steps_rewires_existing_incoming_edge() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", Value::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.connect_steps("step-3", "step-2").expect("rewire onto step-2");
        assert_eq!(host.fixture.edges.len(), 1);
        assert_eq!(host.fixture.edges[0].from, "step-3");
        assert_eq!(host.fixture.edges[0].to, "step-2");
    }

    #[test]
    fn disconnect_steps_returns_false_when_no_matching_edge() {
        let mut host = SequenceHost::default();
        assert!(!host.disconnect_steps("step-2", "step-1"));
        assert_eq!(host.fixture.edges.len(), 1);
    }

    #[test]
    fn load_json_parses_valid_fixture() {
        let json = SequenceHost::default().to_json().expect("fixture json");
        let host = SequenceHost::load_json(&json).expect("load json");
        assert_eq!(host.fixture.steps.len(), 2);
    }

    #[test]
    fn load_json_rejects_unsupported_schema() {
        let result = SequenceHost::load_json(r#"{"schema":"other","camera":{"x":0.0,"y":0.0,"zoom":1.0},"steps":[],"edges":[]}"#);
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
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.fixture.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        host.layout_expanded_slots();
        let child = host.fixture.steps.iter().find(|step| step.id == "step-4").unwrap();
        assert_eq!(child.x, 400.0);
        assert_eq!(child.y, 160.0);
    }

    #[test]
    fn reorganize_syncs_step_positions_from_dag_layout() {
        let mut host = SequenceHost::default();
        host.reorganize(&DagLayoutOptions::default()).expect("reorganize");
        for step in &host.fixture.steps {
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
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: true });
        let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
        let step = host.fixture.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn add_step_dropped_falls_back_for_non_control_owner() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("step-2"));
        let step = host.fixture.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn add_step_dropped_falls_back_for_unknown_owner_id() {
        let mut host = SequenceHost::default();
        let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("nope"));
        let step = host.fixture.steps.iter().find(|entry| entry.id == id).expect("added step");
        assert!(step.slot.is_none());
    }

    #[test]
    fn build_path_returns_unordered_slot_body_when_multiple_heads() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        host.fixture.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
        host.fixture.steps.push(SequenceStep { id: "step-5".into(), kind: "log.print".into(), params: StepParams::new(), x: 280.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
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
        host.fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
        let expanded = host.step_to_dag_node(&host.fixture.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(expanded.abbreviation, "▾0");
        host.set_step_collapsed("step-3", true);
        let collapsed = host.step_to_dag_node(&host.fixture.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
        assert_eq!(collapsed.abbreviation, "▸0");
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
    fn run_executes_default_fixture_and_records_scope() {
        let host = SequenceHost::default();
        let result = host.run();
        assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
        assert!(!result.effects.is_empty());
    }

    #[test]
    fn compile_text_renders_default_fixture_steps() {
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
}

// #region 🔖DocumentVcs
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Identified, Operation, OperationDiff, Patchable};

pub const SEQUENCE_FIXTURE_SCHEMA: &str = "sequence.fixture";

pub type SequenceEnvelope = store::DocumentEnvelope<SequenceFixture, SequenceOperation>;
pub type SequenceStore = store::DocumentStore<SequenceFixture, SequenceOperation>;

// #region 🔖Collections
impl Identified<String> for SequenceStep {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for SequenceEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹 Sparse patch for a step — only the fields user actions ever mutate after creation (kind/slot
/// are fixed for a step's lifetime, so add/remove carries those instead). Only ever embedded
/// `#[dsl(block)]`-wrapped (on `SequenceOperation::StepsPatch`), so it carries no `#[dsl(keyword =
/// "...")]` of its own.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStepPatch {
    pub params: Option<StepParams>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub collapsed: Option<bool>,
}

impl Patchable<SequenceStepPatch> for SequenceStep {
    fn apply_patch(&mut self, patch: &SequenceStepPatch) {
        if let Some(params) = &patch.params {
            self.params = params.clone();
        }
        if let Some(x) = patch.x {
            self.x = x;
        }
        if let Some(y) = patch.y {
            self.y = y;
        }
        if let Some(collapsed) = patch.collapsed {
            self.collapsed = collapsed;
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<SequenceStepPatch> {
        let patch = SequenceStepPatch {
            params: (self.params != other.params).then(|| other.params.clone()),
            x: (self.x != other.x).then_some(other.x),
            y: (self.y != other.y).then_some(other.y),
            collapsed: (self.collapsed != other.collapsed).then_some(other.collapsed),
        };
        (patch != SequenceStepPatch::default()).then_some(patch)
    }
}

/// 🩹 Sparse patch for an edge endpoint rewire. Only ever embedded `#[dsl(block)]`-wrapped (on
/// `SequenceOperation::EdgesPatch`), so it carries no `#[dsl(keyword = "...")]` of its own.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdgePatch {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl Patchable<SequenceEdgePatch> for SequenceEdge {
    fn apply_patch(&mut self, patch: &SequenceEdgePatch) {
        if let Some(from) = &patch.from {
            self.from = from.clone();
        }
        if let Some(to) = &patch.to {
            self.to = to.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<SequenceEdgePatch> {
        let patch = SequenceEdgePatch { from: (self.from != other.from).then(|| other.from.clone()), to: (self.to != other.to).then(|| other.to.clone()) };
        (patch != SequenceEdgePatch::default()).then_some(patch)
    }
}

fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🧮 Typed sequence operation: id-keyed step/edge collection edits plus the scalar canvas camera.
/// Flattened into one keyword-tagged variant per {@link protocol::CollectionOperation} case rather
/// than wrapping that generic type directly — `CollectionOperation` is foreign (defined in
/// `protocol`) and generic, so it can never itself implement `dsl::DslField`/`dsl::DslVariants` from
/// this crate (the orphan rule requires a local type to anchor the impl on, and its OWN outer type
/// isn't local). {@link Operation for SequenceOperation} below reconstructs a `CollectionOperation`
/// ad hoc per match arm to keep reusing `protocol`'s generic collection diff/invert helpers.
/// 🧮 Typed sequence operation — kept plain `Serialize`/`Deserialize` only; see `SequenceOperationDsl`
/// (`🔖OpText` region) for the op-log DSL text mirror (`EdgesAdd`/`EdgesPatch` items as
/// `SequenceEdgeDsl`, a `dsl::Wire`-backed connection) and the hand-written `impl protocol::OpText for
/// SequenceOperation` that converts through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SequenceOperation {
    StepsAdd { index: usize, item: SequenceStep },
    StepsRemove { id: String },
    StepsMove { id: String, to_index: usize },
    StepsPatch { id: String, patch: SequenceStepPatch },
    EdgesAdd { index: usize, item: SequenceEdge },
    EdgesRemove { id: String },
    EdgesMove { id: String, to_index: usize },
    EdgesPatch { id: String, patch: SequenceEdgePatch },
    SetCamera { camera: SequenceCamera },
}

/// 🔁 Converts a generic step `CollectionOperation` (as produced by `protocol::invert_collection_operation`)
/// back into its flat `SequenceOperation` variant.
fn steps_operation_from_collection(operation: CollectionOperation<String, SequenceStep, SequenceStepPatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { id: _id, item, at } => SequenceOperation::StepsAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::StepsRemove { id },
        CollectionOperation::Move { id, to } => SequenceOperation::StepsMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::StepsPatch { id, patch },
    }
}

/// 🔁 Edge counterpart of {@link steps_operation_from_collection}.
fn edges_operation_from_collection(operation: CollectionOperation<String, SequenceEdge, SequenceEdgePatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { id: _id, item, at } => SequenceOperation::EdgesAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::EdgesRemove { id },
        CollectionOperation::Move { id, to } => SequenceOperation::EdgesMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::EdgesPatch { id, patch },
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceDiff {
    pub steps: Option<CollectionDiff<String, SequenceStepPatch, SequenceStep>>,
    pub edges: Option<CollectionDiff<String, SequenceEdgePatch, SequenceEdge>>,
    pub camera: Option<SequenceCamera>,
}

impl OperationDiff<SequenceFixture> for SequenceDiff {
    fn apply(&self, projection: &SequenceFixture) -> SequenceFixture {
        let mut next = projection.clone();
        if let Some(diff) = &self.steps {
            apply_collection_diff(&mut next.steps, diff);
        }
        if let Some(diff) = &self.edges {
            apply_collection_diff(&mut next.edges, diff);
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        absorb_collection_diff(&mut self.steps, other.steps);
        absorb_collection_diff(&mut self.edges, other.edges);
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl Operation<SequenceFixture> for SequenceOperation {
    type Diff = SequenceDiff;

    fn diff(&self, projection: &SequenceFixture) -> SequenceDiff {
        match self {
            SequenceOperation::StepsAdd { index, item } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index })), ..Default::default() }
            }
            SequenceOperation::StepsRemove { id } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::StepsMove { id, to_index } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to: *to_index })), ..Default::default() }
            }
            SequenceOperation::StepsPatch { id, patch } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() }
            }
            SequenceOperation::EdgesAdd { index, item } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index })), ..Default::default() }
            }
            SequenceOperation::EdgesRemove { id } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::EdgesMove { id, to_index } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to: *to_index })), ..Default::default() }
            }
            SequenceOperation::EdgesPatch { id, patch } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() }
            }
            SequenceOperation::SetCamera { camera } => SequenceDiff { camera: Some(camera.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &SequenceFixture) -> Vec<Self> {
        match self {
            SequenceOperation::StepsAdd { index, item } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index }))],
            SequenceOperation::StepsRemove { id } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::StepsMove { id, to_index } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to: *to_index }))]
            }
            SequenceOperation::StepsPatch { id, patch } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
            SequenceOperation::EdgesAdd { index, item } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index }))],
            SequenceOperation::EdgesRemove { id } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::EdgesMove { id, to_index } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to: *to_index }))]
            }
            SequenceOperation::EdgesPatch { id, patch } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
            SequenceOperation::SetCamera { .. } => vec![SequenceOperation::SetCamera { camera: projection.camera.clone() }],
        }
    }
}

/// 🔀 Diffs two fixtures into a minimal typed operation set: removed/added/patched steps and edges plus a
/// camera change. Lets action handlers keep computing the target fixture via {@link SequenceHost}
/// (with all its cycle/slot/layout logic) while emitting granular, mergeable operations.
pub fn sequence_fixture_operations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceOperation> {
    let mut operations = Vec::new();
    for step in &before.steps {
        if !after.steps.iter().any(|entry| entry.id == step.id) {
            operations.push(SequenceOperation::StepsRemove { id: step.id.clone() });
        }
    }
    for (index, step) in after.steps.iter().enumerate() {
        match before.steps.iter().find(|entry| entry.id == step.id) {
            None => operations.push(SequenceOperation::StepsAdd { index, item: step.clone() }),
            Some(prior) => {
                let patch = SequenceStepPatch {
                    params: (prior.params != step.params).then(|| step.params.clone()),
                    x: (prior.x != step.x).then_some(step.x),
                    y: (prior.y != step.y).then_some(step.y),
                    collapsed: (prior.collapsed != step.collapsed).then_some(step.collapsed),
                };
                if patch != SequenceStepPatch::default() {
                    operations.push(SequenceOperation::StepsPatch { id: step.id.clone(), patch });
                }
            }
        }
    }
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            operations.push(SequenceOperation::EdgesRemove { id: edge.id.clone() });
        }
    }
    for (index, edge) in after.edges.iter().enumerate() {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => operations.push(SequenceOperation::EdgesAdd { index, item: edge.clone() }),
            Some(prior) => {
                let patch = SequenceEdgePatch { from: (prior.from != edge.from).then(|| edge.from.clone()), to: (prior.to != edge.to).then(|| edge.to.clone()) };
                if patch != SequenceEdgePatch::default() {
                    operations.push(SequenceOperation::EdgesPatch { id: edge.id.clone(), patch });
                }
            }
        }
    }
    if before.camera != after.camera {
        operations.push(SequenceOperation::SetCamera { camera: after.camera.clone() });
    }
    operations
}
// #endregion 🔖Operations

// #region 🔖Dsl
/// 🔌 DSL-only mirror of `SequenceEdge` — models the `from`/`to` step-id pair as a single unified
/// `dsl::Wire` literal (`from->to`) instead of two separate string fields, per the unified syntax
/// law for graph edges/connections. Converts at the `store::DocumentDsl`/`store::OpText` boundary only
/// (`sequence_fixture_to_dsl`/`sequence_operation_to_dsl` and their inverses); `SequenceEdge`
/// itself (and every consumer matching on its `from`/`to` fields directly) is completely
/// untouched. `SequenceEdgePatch` stays a plain sparse two-`Option<String>` patch rather than a
/// `Wire` — a `Wire`'s two endpoints are not independently optional, but `EdgesPatch` legitimately
/// needs to rewire only `from` OR only `to` (see `sequence_fixture_operations`).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct SequenceEdgeDsl {
    id: String,
    link: dsl::Wire,
}

fn sequence_edge_to_dsl(edge: &SequenceEdge) -> SequenceEdgeDsl {
    let from = dsl::WireNode { id: edge.from.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.to.clone(), kind: None, port: None };
    SequenceEdgeDsl { id: edge.id.clone(), link: dsl::Wire(dsl::WireValue { from, edge: Some((true, to)), properties: dsl::DslValue::Object(Vec::new()) }) }
}

fn sequence_edge_from_dsl(edge: SequenceEdgeDsl) -> Result<SequenceEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.link.0;
    let (directed, to) = link.ok_or_else(|| "sequence edge wire literal must have a target".to_string())?;
    if !directed {
        return Err("sequence edge wire literal must be directed".into());
    }
    Ok(SequenceEdge { id: edge.id, from: from.id, to: to.id })
}

/// 📄 DSL-only mirror of `SequenceFixture` — `steps`/`edges` print as SoA `#[dsl(table)]` columns
/// instead of the old array-of-structures form, and `edges` goes through `SequenceEdgeDsl` for the
/// unified wire syntax. See this region's opening doc comment on `SequenceEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "sequence")]
#[dsl(layout = "lines")]
struct SequenceFixtureDsl {
    schema: String,
    #[dsl(block)]
    camera: SequenceCamera,
    #[dsl(table)]
    steps: Vec<SequenceStep>,
    #[dsl(table)]
    edges: Vec<SequenceEdgeDsl>,
}

fn sequence_fixture_to_dsl(fixture: &SequenceFixture) -> SequenceFixtureDsl {
    SequenceFixtureDsl { schema: fixture.schema.clone(), camera: fixture.camera.clone(), steps: fixture.steps.clone(), edges: fixture.edges.iter().map(sequence_edge_to_dsl).collect() }
}

fn sequence_fixture_dsl_to_fixture(fixture: SequenceFixtureDsl) -> Result<SequenceFixture, String> {
    Ok(SequenceFixture { schema: fixture.schema, camera: fixture.camera, steps: fixture.steps, edges: fixture.edges.into_iter().map(sequence_edge_from_dsl).collect::<Result<Vec<_>, _>>()? })
}

impl store::DocumentDsl for SequenceFixture {
    const EXTENSION: &'static str = "sequence";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentDsl>::parse_dsl(text)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <SequenceFixtureDsl as store::DocumentDsl>::print_dsl(&sequence_fixture_to_dsl(self))
    }
}

/// 📦 Hand-written `store::DocumentPack` mirror of the `DocumentDsl` impl above — `SequenceFixture`
/// itself doesn't derive `dsl::DslDocument` (see `SequenceFixtureDsl`'s doc comment), so it doesn't
/// pick up the blanket derive-emitted `DocumentPack` impl either; this converts through the same
/// `SequenceFixtureDsl` mirror, which does derive it.
impl store::DocumentPack for SequenceFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <SequenceFixtureDsl as store::DocumentPack>::encode_pack_with(&sequence_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
// #endregion 🔖Dsl

// #region 🔖OpText
/// ✂️ DSL-only mirror of `SequenceOperation` — identical shape except `EdgesAdd.item` goes through
/// `SequenceEdgeDsl` for the unified wire syntax (see `🔖Dsl`'s doc comment on `SequenceEdgeDsl`
/// for why `EdgesPatch.patch` stays a plain `SequenceEdgePatch`, not a wire).
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum SequenceOperationDsl {
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: SequenceStep,
    },
    StepsRemove { id: String },
    StepsMove { id: String, to_index: usize },
    StepsPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceStepPatch,
    },
    EdgesAdd {
        index: usize,
        #[dsl(block)]
        item: SequenceEdgeDsl,
    },
    EdgesRemove { id: String },
    EdgesMove { id: String, to_index: usize },
    EdgesPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceEdgePatch,
    },
    SetCamera {
        #[dsl(block)]
        camera: SequenceCamera,
    },
}

fn sequence_operation_to_dsl(operation: &SequenceOperation) -> SequenceOperationDsl {
    match operation {
        SequenceOperation::StepsAdd { index, item } => SequenceOperationDsl::StepsAdd { index: *index, item: item.clone() },
        SequenceOperation::StepsRemove { id } => SequenceOperationDsl::StepsRemove { id: id.clone() },
        SequenceOperation::StepsMove { id, to_index } => SequenceOperationDsl::StepsMove { id: id.clone(), to_index: *to_index },
        SequenceOperation::StepsPatch { id, patch } => SequenceOperationDsl::StepsPatch { id: id.clone(), patch: patch.clone() },
        SequenceOperation::EdgesAdd { index, item } => SequenceOperationDsl::EdgesAdd { index: *index, item: sequence_edge_to_dsl(item) },
        SequenceOperation::EdgesRemove { id } => SequenceOperationDsl::EdgesRemove { id: id.clone() },
        SequenceOperation::EdgesMove { id, to_index } => SequenceOperationDsl::EdgesMove { id: id.clone(), to_index: *to_index },
        SequenceOperation::EdgesPatch { id, patch } => SequenceOperationDsl::EdgesPatch { id: id.clone(), patch: patch.clone() },
        SequenceOperation::SetCamera { camera } => SequenceOperationDsl::SetCamera { camera: camera.clone() },
    }
}

fn sequence_operation_from_dsl(operation: SequenceOperationDsl) -> Result<SequenceOperation, String> {
    Ok(match operation {
        SequenceOperationDsl::StepsAdd { index, item } => SequenceOperation::StepsAdd { index, item },
        SequenceOperationDsl::StepsRemove { id } => SequenceOperation::StepsRemove { id },
        SequenceOperationDsl::StepsMove { id, to_index } => SequenceOperation::StepsMove { id, to_index },
        SequenceOperationDsl::StepsPatch { id, patch } => SequenceOperation::StepsPatch { id, patch },
        SequenceOperationDsl::EdgesAdd { index, item } => SequenceOperation::EdgesAdd { index, item: sequence_edge_from_dsl(item)? },
        SequenceOperationDsl::EdgesRemove { id } => SequenceOperation::EdgesRemove { id },
        SequenceOperationDsl::EdgesMove { id, to_index } => SequenceOperation::EdgesMove { id, to_index },
        SequenceOperationDsl::EdgesPatch { id, patch } => SequenceOperation::EdgesPatch { id, patch },
        SequenceOperationDsl::SetCamera { camera } => SequenceOperation::SetCamera { camera },
    })
}

impl protocol::OpText for SequenceOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_operation = <SequenceOperationDsl as protocol::OpText>::parse_op(line)?;
        sequence_operation_from_dsl(dsl_operation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <SequenceOperationDsl as protocol::OpText>::print_op(&sequence_operation_to_dsl(self))
    }
}
// #endregion 🔖OpText

// #region 🧪OpsTests
#[cfg(test)]
mod ops_tests {
    use super::*;
    use vcs::apply_operation;
use store::{create_document_envelope, DocumentCommand};

    fn round_trip(fixture: &SequenceFixture, operation: &SequenceOperation) -> SequenceFixture {
        let forward = apply_operation(fixture, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(fixture) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, fixture, "backwards() must restore the pre-operation fixture");
        forward
    }

    #[test]
    fn add_remove_patch_steps_round_trip() {
        let fixture = default_fixture();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let added = round_trip(&fixture, &SequenceOperation::StepsAdd { index: 2, item: step });
        assert_eq!(added.steps.len(), 3);
        let patched = round_trip(&added, &SequenceOperation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch { x: Some(120.0), ..Default::default() } });
        assert_eq!(patched.steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
        let removed = round_trip(&patched, &SequenceOperation::StepsRemove { id: "step-99".into() });
        assert!(!removed.steps.iter().any(|step| step.id == "step-99"));
    }

    #[test]
    fn set_camera_round_trip() {
        let fixture = default_fixture();
        let next = round_trip(&fixture, &SequenceOperation::SetCamera { camera: SequenceCamera { x: 10.0, y: 20.0, zoom: 2.0 } });
        assert_eq!(next.camera.zoom, 2.0);
    }

    #[test]
    fn fixture_ops_capture_move_and_connect() {
        let mut host = SequenceHost::default();
        let before = host.fixture.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let operations = sequence_fixture_operations(&before, &host.fixture);
        assert!(operations.iter().any(|operation| matches!(operation, SequenceOperation::StepsAdd { item, .. } if item.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_add() {
        let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_FIXTURE_SCHEMA, "sequence", default_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 3);
    }

    // #region 🔖DslAndOpText
    #[test]
    fn dsl_round_trips_default_fixture() {
        store::test_support::assert_dsl_round_trip(&default_fixture());
        store::test_support::assert_dsl_pack_equivalence(&default_fixture());
    }

    /// 📜 `sequence/example/default.sequence` is the handcrafted `.sequence` DSL-text fixture
    /// (regenerated from `default_fixture()`'s canonical print form) — this is the permanent proof
    /// that the checked-in fixture still parses and round trips, not a one-time migration script.
    #[test]
    fn default_sequence_example_dsl_round_trips() {
        let text = include_str!("../../example/default.sequence");
        let fixture = <SequenceFixture as store::DocumentDsl>::parse_dsl(text).expect("default.sequence must parse");
        store::test_support::assert_dsl_round_trip(&fixture);
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn dsl_round_trips_fixture_with_slots_and_nested_params() {
        let mut fixture = default_fixture();
        fixture.steps.push(SequenceStep {
            id: "step-3".into(),
            kind: "control.if".into(),
            params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: true,
        });
        fixture.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", Value::Atom(Atom::String("nested \"quote\" and \\ backslash".into()))).insert(
                "meta",
                Value::Dictionary(Dictionary::new().insert("count", Value::Atom(Atom::Integer(-3))).insert("ratio", Value::Atom(Atom::Decimal(2.5)))),
            ),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        store::test_support::assert_dsl_round_trip(&fixture);
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn op_text_round_trips_steps_add() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsAdd {
            index: 2,
            item: SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new().insert("message", Value::Atom(Atom::String("hi there".into()))), x: 5.0, y: -6.5, slot: None, collapsed: false },
        });
    }

    #[test]
    fn op_text_round_trips_steps_add_with_slot() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsAdd {
            index: 0,
            item: SequenceStep { id: "step-98".into(), kind: "control.while".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "body".into() }), collapsed: true },
        });
    }

    #[test]
    fn op_text_round_trips_steps_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsRemove { id: "step-99".into() });
    }

    #[test]
    fn op_text_round_trips_steps_move() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsMove { id: "step-99".into(), to_index: 3 });
    }

    #[test]
    fn op_text_round_trips_steps_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsPatch {
            id: "step-99".into(),
            patch: SequenceStepPatch {
                params: Some(StepParams::new().insert("value", Value::Atom(Atom::Decimal(120.0))).insert("meta", Value::Dictionary(Dictionary::new().insert("k", Value::Atom(Atom::Null))))),
                x: Some(120.0),
                y: None,
                collapsed: Some(true),
            },
        });
    }

    #[test]
    fn op_text_round_trips_steps_patch_with_no_fields() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch::default() });
    }

    #[test]
    fn op_text_round_trips_edges_add() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesAdd { index: 1, item: SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() } });
    }

    #[test]
    fn op_text_round_trips_edges_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesRemove { id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trips_edges_move() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesMove { id: "edge-1".into(), to_index: 0 });
    }

    #[test]
    fn op_text_round_trips_edges_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesPatch { id: "edge-1".into(), patch: SequenceEdgePatch { from: Some("step-3".into()), to: None } });
    }

    #[test]
    fn op_text_round_trips_set_camera() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::SetCamera { camera: SequenceCamera { x: 10.5, y: -20.25, zoom: 2.0 } });
    }

    #[test]
    fn document_text_round_trips_store_with_applied_operation() {
        let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_FIXTURE_SCHEMA, "sequence-text-test", default_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    // #endregion 🔖DslAndOpText
}
// #endregion 🧪OpsTests
// #endregion 🔖DocumentVcs

