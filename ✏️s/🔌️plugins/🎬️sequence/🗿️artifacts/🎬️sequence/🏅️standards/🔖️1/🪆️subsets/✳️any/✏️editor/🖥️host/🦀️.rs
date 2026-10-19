//! 🖥️ Sequence editing host: the UI-editing engine shared by commands, windows, panels and the wasm bridge.

use neural_engine::Value as NeuralValue;
use imperative_engine::compile_to_text as imperative_compile_to_text;
use std::collections::{BTreeMap, HashMap};
use neural_engine::{ChannelSpec, Dictionary, Registry};
use semio_framework_os_infinite_directed_presentation::{DagCamera, DagHostSnapshot, DagHostSnapshotEdge, DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape, dag_host_snapshot_to_wire_literal};
use infinite_board_port_directed_dag::{DagHost, would_create_cycle};
use semio_framework_os_infinite::board::schema::layout::DagLayoutOptions;
use semio_framework_plugin::DslValue;
use imperative_engine::{Executor, Path, RunResult, Step, imperative_catalogue_json, imperative_module_registry};
use semio_framework_graph_manifest::PropertyBag;
use crate::{SequenceCamera, SequenceEdge, SequenceHostSnapshot, SequenceStep, SlotRef, StepParams};
use crate::editor::sequence::editor_vocabulary::{SequenceCoreError, dag_camera_from_sequence, sequence_camera_from_dag};

//#region 🔖️Host
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
    semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(dict)).unwrap_or_default()
}

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
pub fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn is_function_kind(kind: &str) -> bool {
    kind.starts_with("math.") || kind.starts_with("logic.") || kind.starts_with("text.")
}

fn parse_serial_suffix(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

pub fn max_serial_in_snapshot(snapshot: &SequenceHostSnapshot) -> u64 {
    let mut max = 0u64;
    for step in &snapshot.steps {
        if let Some(serial) = parse_serial_suffix("step-", &step.id) {
            max = max.max(serial);
        }
    }
    for edge in &snapshot.edges {
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

fn neural_value_to_dsl_value(value: &NeuralValue) -> DslValue {
    semio_framework_value::ToValue::to_value(value)
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

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
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


pub struct SequenceHost {
    /// 🌊️ The plain pre-migration document shape (`{schema, steps, edges}`) — this plugin's own
    /// working representation, matching `SequenceHostSnapshot`'s doc comment. `SequenceHost` edits this
    /// in place exactly as it edited `SequenceSnapshot.steps`/`.edges` directly before the
    /// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` migration (`sequence→C:flow`) — only the
    /// boundary conversions (`from_snapshot`/`to_json`/`load_json`) changed.
    pub snapshot: SequenceHostSnapshot,
    /// 🎥️ The canvas camera — session-only host state (never a `SequenceSnapshot` document field; see
    /// the exact main-window camera). Persists across `rebuild_dag()` calls
    /// within this `SequenceHost` instance (each document mutation rebuilds `dag` from scratch, so
    /// this is what the rebuilt `dag`'s camera gets reseeded from).
    pub camera: SequenceCamera,
    pub dag: DagHost,
    pub registry: Registry,
    next_serial: u64,
}

impl neural_engine::ColdRetire for SequenceHost {
    fn retire_cold(self) {
        let Self { snapshot, registry, .. } = self;
        neural_engine::ColdRetire::retire_cold(snapshot);
        neural_engine::ColdRetire::retire_cold(registry);
    }
}

/// 🧊️ The canonical genesis content, moved into the host (its `StepParams` dictionaries are then owned by the host's own
/// cold boundary, [`SequenceHost`]'s `ColdRetire`).
impl Default for SequenceHost {
    fn default() -> Self {
        Self::from_host_snapshot(crate::snapshot::schema::default_host_snapshot())
    }
}

impl SequenceHost {
    /// 🌊️ Builds a live host directly from a plain snapshot (the WASM bridge's `loadSnapshotJson`/
    /// `SequenceHost::load_json` entry point).
    pub fn from_host_snapshot(host_snapshot: SequenceHostSnapshot) -> Self {
        let next_serial = max_serial_in_snapshot(&host_snapshot).max(100);
        let mut host = Self {
            snapshot: host_snapshot,
            camera: SequenceCamera::default(),
            dag: DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            registry: imperative_module_registry(),
            next_serial,
        };
        host.rebuild_dag();
        host
    }

    pub fn load_json(json: &str) -> Result<Self, SequenceCoreError> {
        let snapshot: SequenceHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SequenceCoreError::Json(error.to_string()))?;
        if snapshot.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(snapshot.schema));
        }
        Ok(Self::from_host_snapshot(snapshot))
    }

    pub fn to_json(&self) -> Result<String, SequenceCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.snapshot))
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
        use semio_framework_canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use semio_framework_canvas::Point;
        let viewport = Viewport { width: width.max(1), height: height.max(1), dpr: dpr.max(1.0) };
        let camera = CanvasCamera { x: self.dag.graph.host_snapshot.camera.x, y: self.dag.graph.host_snapshot.camera.y, zoom: self.dag.graph.host_snapshot.camera.zoom };
        let world = screen_to_world(&camera, &viewport, Point::new(sx, sy));
        for node in self.dag.graph.host_snapshot.nodes.iter().rev() {
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
        let params: StepParams = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SequenceCoreError::Json(error.to_string()))?;
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
            .dag.graph
            .host_snapshot
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
        self.camera = sequence_camera_from_dag(&self.dag.graph.host_snapshot.camera);
        self.sync_edges_from_dag();
        for step in &mut self.snapshot.steps {
            let Some(node) = self.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == step.id) else {
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
        Ok(semio_framework_pack_json::to_json_string(&self.build_path()))
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
        let mut progress=|_|true;let mut control=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000,&mut progress);self.dag.graph.reorganize(opts,&mut control).map_err(|e| SequenceCoreError::Dag(e.to_string()))?;
        let positions: HashMap<String, (f64, f64)> = self.dag.graph.host_snapshot.nodes.iter().map(|node| (node.id.clone(), (node.x, node.y))).collect();
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

    /// 📝️ Renders the compiled DAG snapshot as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_host_snapshot_to_wire_literal(&self.build_dag_host_snapshot())
    }

    pub fn rebuild_dag(&mut self) {
        let selected = self.dag.graph.selected_node_ids();
        let dag_fixture = self.build_dag_host_snapshot();
        self.dag = DagHost::from_host_snapshot_without_layout(dag_fixture);
        self.dag.graph.set_camera(self.camera.x, self.camera.y, self.camera.zoom);
        if !selected.is_empty() {
            self.dag.graph.set_selection(&selected);
        }
    }

    pub fn build_dag_host_snapshot(&self) -> DagHostSnapshot {
        let nodes: Vec<DagNodeSpec> = self.snapshot.steps.iter().filter(|step| self.is_step_visible(step)).map(|step| self.step_to_dag_node(step)).collect();
        let visible_ids: std::collections::HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let existing: Vec<(String, String)> = self.snapshot.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        let edges: Vec<DagHostSnapshotEdge> = self
            .snapshot
            .edges
            .iter()
            .filter(|edge| visible_ids.contains(&edge.from) && visible_ids.contains(&edge.to))
            .filter(|edge| !would_create_cycle(&existing, &edge.from, &edge.to))
            .map(|edge| DagHostSnapshotEdge { id: edge.id.clone(), source: format!("{}@{}", edge.from, FLOW_OUTPUT_PORT), target: format!("{}@{}", edge.to, FLOW_INPUT_PORT), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::new() })
            .collect();
        DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: dag_camera_from_sequence(&self.camera), nodes, edges }
    }

    pub fn step_to_dag_node(&self, step: &SequenceStep) -> DagNodeSpec {
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
        let mut node = DagNodeSpec::computation(step.id.clone(), &name, &abbreviation, icon, inputs, outputs, false, false, step.x, step.y, width, height);
        node.operator_kind = Some(step.kind.clone());
        node.properties = property_bag_from_dictionary(&step.params);
        node
    }

    pub fn set_ghost_step(&mut self, kind: &str, x: f64, y: f64) {
        let ghost = SequenceStep { id: "__ghost__".into(), kind: kind.into(), params: StepParams::new(), x, y, slot: None, collapsed: false };
        let node = self.step_to_dag_node(&ghost);
        self.dag.pointer.set_ghost_node(Some(node));
    }

    pub fn clear_ghost_step(&mut self) {
        self.dag.pointer.set_ghost_node(None);
    }
}
//#endregion 🔖️Host

//#region 🔖️HostHelpers
/// 🧸️ Builds a host from an already resolved typed host snapshot projection.
pub fn host_from_host_snapshot(host_snapshot: &SequenceHostSnapshot) -> SequenceHost {
    SequenceHost::from_host_snapshot(host_snapshot.clone())
}

/// 🧊️ Retires one synchronous execution result through its domain-owned dictionaries.
pub fn retire_run_result_cold(result: RunResult) {
    let RunResult { scope, effects } = result;
    neural_engine::ColdRetire::retire_cold(scope);
    for effect in effects {
        neural_engine::ColdRetire::retire_cold(effect.input);
        if let Some(output) = effect.output {
            neural_engine::ColdRetire::retire_cold(output);
        }
    }
}

//#endregion 🔖️HostHelpers
