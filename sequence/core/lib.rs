//! 📜 Sequence core: execution-flow canvas host wrapping DagHost.

pub use imperative_engine::{compile_to_text, EffectLogEntry, Executor, imperative_catalogue_json, imperative_module_registry, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry};
pub use mathematical_graph_port_directed_dag as dag;

use dag::{
    would_create_cycle, DagCameraV1, DagFixtureEdgeV1, DagFixtureV1, DagHost, DagLayoutOptions, DagNodeSpec, IoPortSpec,
};
use imperative_engine::compile_to_text as imperative_compile_to_text;
use neural_engine::{Atom, Dictionary, Registry, Value};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

const FLOW_INPUT_PORT: &str = "prev";
const FLOW_OUTPUT_PORT: &str = "next";

fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotRef {
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepWidgetV1 {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub slot: Option<SlotRef>,
    #[serde(default)]
    pub collapsed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdgeV1 {
    pub id: String,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFixtureV1 {
    pub schema: String,
    pub camera: DagCameraV1,
    pub steps: Vec<StepWidgetV1>,
    pub edges: Vec<SequenceEdgeV1>,
}

impl Default for SequenceFixtureV1 {
    fn default() -> Self {
        default_fixture()
    }
}

pub fn default_fixture() -> SequenceFixtureV1 {
    SequenceFixtureV1 {
        schema: "sequence.fixture/v1".into(),
        camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
        steps: vec![
            StepWidgetV1 {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: Dictionary::new()
                    .insert("key", Value::Atom(Atom::String("counter".into())))
                    .insert("value", Value::Atom(Atom::Decimal(0.0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            StepWidgetV1 {
                id: "step-2".into(),
                kind: "log.print".into(),
                params: Dictionary::new().insert("message", Value::Atom(Atom::String("hello sequence".into()))),
                x: 280.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
        ],
        edges: vec![SequenceEdgeV1 { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    }
}
// #endregion 🔖Fixture

// #region 🔖Host
pub struct SequenceHost {
    pub fixture: SequenceFixtureV1,
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
    pub fn from_fixture(fixture: SequenceFixtureV1) -> Self {
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture_without_layout(DagFixtureV1 {
                schema: "dag.fixture/v1".into(),
                camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
                nodes: vec![],
                edges: vec![],
            }),
            registry: imperative_module_registry(),
            next_serial: 100,
        };
        host.rebuild_dag();
        host
    }

    pub fn load_json(json: &str) -> Result<Self, String> {
        let fixture: SequenceFixtureV1 = serde_json::from_str(json).map_err(|err| err.to_string())?;
        if fixture.schema != "sequence.fixture/v1" {
            return Err(format!("unsupported schema: {}", fixture.schema));
        }
        Ok(Self::from_fixture(fixture))
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|err| err.to_string())
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
        use infinite_cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use infinite_cavas::vello::kurbo::Point;
        let viewport = Viewport {
            width: width.max(1),
            height: height.max(1),
            dpr: dpr.max(1.0),
        };
        let camera = CavasCamera {
            x: self.dag.fixture.camera.x,
            y: self.dag.fixture.camera.y,
            zoom: self.dag.fixture.camera.zoom,
        };
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

    pub fn add_step_in_slot(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        self.fixture.steps.push(StepWidgetV1 {
            id: id.clone(),
            kind: kind.into(),
            params: Dictionary::new(),
            x,
            y,
            slot,
            collapsed: false,
        });
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
        self.fixture
            .steps
            .retain(|step| !remove_ids.iter().any(|remove_id| remove_id == &step.id));
        self.fixture
            .edges
            .retain(|edge| !remove_ids.iter().any(|remove_id| remove_id == &edge.from || remove_id == &edge.to));
        if self.fixture.steps.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), String> {
        let params: Dictionary = serde_json::from_str(json).map_err(|err| err.to_string())?;
        let Some(step) = self.fixture.steps.iter_mut().find(|step| step.id == id) else {
            return Err(format!("unknown step: {id}"));
        };
        step.params = params;
        self.rebuild_dag();
        Ok(())
    }

    pub fn connect_steps(&mut self, from_id: &str, to_id: &str) -> Result<String, String> {
        if from_id == to_id {
            return Err("cannot connect step to itself".into());
        }
        let from_step = self
            .fixture
            .steps
            .iter()
            .find(|step| step.id == from_id)
            .ok_or_else(|| format!("{from_id} not found"))?;
        let to_step = self
            .fixture
            .steps
            .iter()
            .find(|step| step.id == to_id)
            .ok_or_else(|| format!("{to_id} not found"))?;
        if slot_key(from_step.slot.as_ref()) != slot_key(to_step.slot.as_ref()) {
            return Err("steps must share the same slot scope".into());
        }
        let existing: Vec<(String, String)> = self.fixture.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err("connection would create cycle".into());
        }
        if self.fixture.edges.iter().any(|edge| edge.from == from_id) {
            return Err(format!("{from_id} already has outgoing flow"));
        }
        if self.fixture.edges.iter().any(|edge| edge.to == to_id) {
            self.fixture.edges.retain(|edge| edge.to != to_id);
        }
        self.next_serial += 1;
        let id = format!("edge-{}", self.next_serial);
        self.fixture.edges.push(SequenceEdgeV1 { id: id.clone(), from: from_id.into(), to: to_id.into() });
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
        let mut edges = Vec::new();
        for dag_edge in &self.dag.fixture.edges {
            let Some(from) = dag_edge.source.split(':').next() else { continue };
            let Some(to) = dag_edge.target.split(':').next() else { continue };
            if from == to {
                continue;
            }
            let id = self
                .fixture
                .edges
                .iter()
                .find(|edge| edge.from == from && edge.to == to)
                .map(|edge| edge.id.clone())
                .unwrap_or_else(|| {
                    self.next_serial += 1;
                    format!("edge-{}", self.next_serial)
                });
            edges.push(SequenceEdgeV1 { id, from: from.into(), to: to.into() });
        }
        self.fixture.edges = edges;
    }

    pub fn sync_from_dag(&mut self) {
        self.fixture.camera = self.dag.fixture.camera.clone();
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

    pub fn build_path_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.build_path()).map_err(|err| err.to_string())
    }

    fn build_path_for_slot(&self, slot: Option<&SlotRef>) -> Path {
        let slot_filter = slot_key(slot);
        let scoped_steps: Vec<&StepWidgetV1> = self
            .fixture
            .steps
            .iter()
            .filter(|step| slot_key(step.slot.as_ref()) == slot_filter)
            .collect();
        let incoming: HashMap<&str, &str> = self.fixture.edges.iter().map(|edge| (edge.to.as_str(), edge.from.as_str())).collect();
        let outgoing: HashMap<&str, &str> = self.fixture.edges.iter().map(|edge| (edge.from.as_str(), edge.to.as_str())).collect();
        let heads: Vec<&StepWidgetV1> = scoped_steps
            .iter()
            .copied()
            .filter(|step| !incoming.contains_key(step.id.as_str()))
            .collect();
        let start = if heads.len() == 1 {
            heads[0].id.as_str()
        } else if scoped_steps.len() == 1 {
            scoped_steps[0].id.as_str()
        } else {
            return Path {
                steps: scoped_steps
                    .iter()
                    .map(|step| self.step_to_imperative_step(step))
                    .collect(),
            };
        };
        let mut ordered = Vec::new();
        let mut by_id: BTreeMap<&str, &StepWidgetV1> = scoped_steps.into_iter().map(|step| (step.id.as_str(), step)).collect();
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

    fn step_to_imperative_step(&self, step: &StepWidgetV1) -> Step {
        let mut bodies = BTreeMap::new();
        if is_control_kind(&step.kind) {
            for slot_name in control_slots(&step.kind) {
                let slot_ref = SlotRef {
                    owner: step.id.clone(),
                    name: slot_name.to_string(),
                };
                bodies.insert(slot_name.to_string(), self.build_path_for_slot(Some(&slot_ref)));
            }
        }
        Step {
            id: step.id.clone(),
            kind: step.kind.clone(),
            params: step.params.clone(),
            bodies,
        }
    }

    fn is_step_visible(&self, step: &StepWidgetV1) -> bool {
        let Some(slot) = &step.slot else {
            return true;
        };
        let Some(owner) = self.fixture.steps.iter().find(|entry| entry.id == slot.owner) else {
            return false;
        };
        !owner.collapsed
    }

    fn slot_member_count(&self, owner_id: &str) -> usize {
        self.fixture
            .steps
            .iter()
            .filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == owner_id))
            .count()
    }

    pub fn layout_expanded_slots(&mut self) {
        let control_steps: Vec<(String, String, bool)> = self
            .fixture
            .steps
            .iter()
            .filter(|step| is_control_kind(&step.kind))
            .map(|step| (step.id.clone(), step.kind.clone(), step.collapsed))
            .collect();
        for (owner_id, kind, collapsed) in control_steps {
            if collapsed {
                continue;
            }
            let owner = self.fixture.steps.iter().find(|step| step.id == owner_id);
            let Some(owner) = owner else { continue };
            let base_x = owner.x;
            let base_y = owner.y + 160.0;
            for (index, slot_name) in control_slots(&kind).iter().enumerate() {
                let slot_ref = SlotRef {
                    owner: owner_id.clone(),
                    name: (*slot_name).into(),
                };
                let members: Vec<String> = self
                    .fixture
                    .steps
                    .iter()
                    .filter(|step| step.slot.as_ref() == Some(&slot_ref))
                    .map(|step| step.id.clone())
                    .collect();
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

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.build_path(), &Dictionary::new())
    }

    pub fn compile_text(&self) -> String {
        imperative_compile_to_text(&self.build_path())
    }

    fn rebuild_dag(&mut self) {
        let dag_fixture = self.build_dag_fixture();
        self.dag = DagHost::from_fixture_without_layout(dag_fixture);
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
    }

    fn build_dag_fixture(&self) -> DagFixtureV1 {
        let nodes: Vec<DagNodeSpec> = self
            .fixture
            .steps
            .iter()
            .filter(|step| self.is_step_visible(step))
            .map(|step| self.step_to_dag_node(step))
            .collect();
        let visible_ids: std::collections::HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let existing: Vec<(String, String)> = self.fixture.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        let edges: Vec<DagFixtureEdgeV1> = self
            .fixture
            .edges
            .iter()
            .filter(|edge| visible_ids.contains(&edge.from) && visible_ids.contains(&edge.to))
            .filter(|edge| !would_create_cycle(&existing, &edge.from, &edge.to))
            .map(|edge| DagFixtureEdgeV1 {
                id: edge.id.clone(),
                source: format!("{}:{}", edge.from, FLOW_OUTPUT_PORT),
                target: format!("{}:{}", edge.to, FLOW_INPUT_PORT),
            })
            .collect();
        DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: self.fixture.camera.clone(),
            nodes,
            edges,
        }
    }

    fn step_to_dag_node(&self, step: &StepWidgetV1) -> DagNodeSpec {
        let info = self.registry.operator_info(&step.kind);
        let (name, mut abbreviation, icon) = info
            .map(|entry| (entry.name.clone(), entry.abbreviation.clone(), entry.icon.clone()))
            .unwrap_or_else(|| (step.kind.clone(), step.kind.clone(), "emoji:⚡".into()));
        if is_control_kind(&step.kind) {
            let count = self.slot_member_count(&step.id);
            abbreviation = if step.collapsed {
                format!("▸ {count}")
            } else {
                format!("▾ {count}")
            };
        }
        let inputs = vec![IoPortSpec::named("P", "Prv", FLOW_INPUT_PORT, "Previous")];
        let outputs = vec![IoPortSpec::named("N", "Nxt", FLOW_OUTPUT_PORT, "Next")];
        let width = dag::computation_node_width(&name, &inputs, &outputs);
        let height = dag::computation_node_height(inputs.len(), outputs.len(), false, false);
        DagNodeSpec::computation(step.id.clone(), name, abbreviation, icon, inputs, outputs, false, false, step.x, step.y, width, height)
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
            let host = SequenceHost::load_json(json).map_err(|err| JsValue::from_str(&err))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow_mut().host.sync_from_dag();
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, x: f64, y: f64) -> String {
            self.state.borrow_mut().host.add_step(kind, x, y)
        }

        #[wasm_bindgen(js_name = addStepToSlot)]
        pub fn add_step_to_slot(&self, kind: &str, x: f64, y: f64, owner: &str, slot_name: &str) -> String {
            self.state
                .borrow_mut()
                .host
                .add_step_in_slot(kind, x, y, Some(SlotRef { owner: owner.into(), name: slot_name.into() }))
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
            self.state.borrow().host.build_path_json().map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen(js_name = connectSteps)]
        pub fn connect_steps(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.connect_steps(from_id, to_id).map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen(js_name = disconnectSteps)]
        pub fn disconnect_steps(&self, from_id: &str, to_id: &str) -> bool {
            self.state.borrow_mut().host.disconnect_steps(from_id, to_id)
        }

        #[wasm_bindgen(js_name = compileText)]
        pub fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
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
                let (render_ctx, renderer, surface) = infinite_cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                    .await
                    .map_err(|err| JsValue::from_str(&err))?;
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
            inner.host.fixture.camera = inner.host.dag.fixture.camera.clone();
            let mut scene = infinite_cavas::vello::Scene::new();
            let clear = inner.host.dag.vello_theme.raster_clear;
            inner.host.dag.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = infinite_cavas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = worldFromScreen)]
        pub fn world_from_screen(&self, sx: f64, sy: f64) -> Result<String, JsValue> {
            use infinite_cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
            use infinite_cavas::vello::kurbo::Point;
            let inner = self.state.borrow();
            let viewport = Viewport {
                width: inner.width.max(1),
                height: inner.height.max(1),
                dpr: inner.dpr.max(1.0),
            };
            let camera = CavasCamera {
                x: inner.host.dag.fixture.camera.x,
                y: inner.host.dag.fixture.camera.y,
                zoom: inner.host.dag.fixture.camera.zoom,
            };
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
            self.state.borrow_mut().host.dag.pointer_down_screen(sx, sy, button, shift, ctrl, alt);
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
            let viewport = Viewport {
                width: inner.width.max(1),
                height: inner.height.max(1),
                dpr: inner.dpr.max(1.0),
            };
            let mut camera = CavasCamera {
                x: inner.host.dag.fixture.camera.x,
                y: inner.host.dag.fixture.camera.y,
                zoom: inner.host.dag.fixture.camera.zoom,
            };
            wheel_screen(&mut camera, &viewport, sx, sy, delta_y);
            inner.host.dag.set_camera(camera.x, camera.y, camera.zoom);
            inner.host.dag.set_wheel_zoom_active(false);
            inner.host.sync_from_dag();
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, opts_json: &str) -> Result<(), JsValue> {
            let opts: DagLayoutOptions = serde_json::from_str(opts_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.dag.reorganize(&opts).map_err(|err| JsValue::from_str(&err))?;
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

        #[wasm_bindgen(js_name = setVelloThemeJson)]
        pub fn set_vello_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.dag.set_vello_theme_from_json(json);
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
        host.fixture.steps.push(StepWidgetV1 {
            id: "step-3".into(),
            kind: "wait.delay".into(),
            params: Dictionary::new().insert("ms", Value::Atom(Atom::Decimal(10.0))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: false,
        });
        assert!(host.connect_steps("step-1", "step-2").is_ok());
        assert!(host.connect_steps("step-1", "step-3").is_err());
    }

    #[test]
    fn build_path_includes_control_bodies() {
        let mut host = SequenceHost::default();
        host.fixture.steps.push(StepWidgetV1 {
            id: "step-3".into(),
            kind: "control.if".into(),
            params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: false,
        });
        host.fixture.steps.push(StepWidgetV1 {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: Dictionary::new().insert("message", Value::Atom(Atom::String("yes".into()))),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        host.fixture.edges.push(SequenceEdgeV1 { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() });
        let path = host.build_path();
        assert_eq!(path.steps.len(), 3);
        let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
        assert!(control.bodies.contains_key("then"));
        assert_eq!(control.bodies.get("then").map(|body| body.steps.len()), Some(1));
    }
}
