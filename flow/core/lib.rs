//! 🌊 Flow core: widgets, neural evaluation, and DAG canvas host.

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed_dag as dag;
pub use neural_engine as neural;

use std::collections::{BTreeMap, HashMap};

use dag::{would_create_cycle, DagFixtureEdgeV1, DagFixtureV1, DagHost, DagLayoutOptions, DagNodeSpec, IoPortSpec};
use neural::{Atom, Dictionary, EvalError, Evaluator, Neuron, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};

// #region 🔖Widget
/// 🎛️ Flow widget discriminant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Widget {
    Neuron { id: String, neuronKind: String, #[serde(default)] params: Dictionary },
    InputSlider { id: String, #[serde(default = "default_slider_value")] value: f64 },
    InputNote { id: String, #[serde(default)] text: String },
    OutputPreview { id: String, #[serde(default)] preview: Dictionary },
    OutputAction { id: String, #[serde(default)] action: String },
}

fn default_slider_value() -> f64 {
    3.0
}

/// 📍 Persisted widget position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 🧩 Serializable flow document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFixtureV1 {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    #[serde(default)]
    pub layout: BTreeMap<String, WidgetLayout>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseSpec {
    pub id: String,
    pub from: String,
    pub to: String,
}

impl Default for FlowFixtureV1 {
    fn default() -> Self {
        Self {
            schema: "flow.fixture/v1".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "slider".into(), value: 3.0 },
                Widget::Neuron { id: "add".into(), neuronKind: "math.add".into(), params: Dictionary::new() },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "slider".into(), to: "add".into() },
                SynapseSpec { id: "s2".into(), from: "add".into(), to: "preview".into() },
            ],
            layout: BTreeMap::new(),
        }
    }
}

fn widget_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { neuronKind, .. } => neuronKind.clone(),
        Widget::InputSlider { .. } => "Slider".into(),
        Widget::InputNote { .. } => "Note".into(),
        Widget::OutputPreview { .. } => "Preview".into(),
        Widget::OutputAction { action, .. } => action.clone(),
    }
}

fn widget_io_ports(widget: &Widget) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    match widget {
        Widget::Neuron { .. } => (
            vec![IoPortSpec { id: "in".into(), label: "in".into() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into() }],
        ),
        Widget::InputSlider { .. } | Widget::InputNote { .. } => (vec![], vec![IoPortSpec { id: "out".into(), label: "out".into() }]),
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } => (vec![IoPortSpec { id: "in".into(), label: "in".into() }], vec![]),
    }
}

fn widget_node_size(widget: &Widget) -> (f64, f64) {
    match widget {
        Widget::InputSlider { .. } => (176.0, 72.0),
        Widget::InputNote { .. } | Widget::OutputPreview { .. } => (176.0, 64.0),
        _ => (160.0, 56.0),
    }
}

fn widget_to_dag_node(widget: &Widget, index: usize, layout: &BTreeMap<String, WidgetLayout>) -> DagNodeSpec {
    let (inputs, outputs) = widget_io_ports(widget);
    let id = match widget {
        Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::OutputPreview { id, .. } | Widget::OutputAction { id, .. } => id.clone(),
    };
    let (width, height) = widget_node_size(widget);
    let (x, y) = layout.get(&id).map(|p| (p.x, p.y)).unwrap_or(((index as f64) * 200.0, 0.0));
    DagNodeSpec::computation(id, widget_label(widget), inputs, outputs, x, y, width, height)
}

const FLOW_SLIDER_MIN: f64 = 0.0;
const FLOW_SLIDER_MAX: f64 = 10.0;
const FLOW_SLIDER_STEP: f64 = 0.1;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum WidgetDescriptor {
    Neuron { neuronKind: String },
    InputSlider,
    InputNote,
    OutputPreview,
    OutputAction { #[serde(default)] action: String },
}
// #endregion 🔖Widget

// #region 🔖Catalogue
/// 📚 Catalogue section for drag-and-drop palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueSection {
    pub id: String,
    pub title: String,
    pub items: Vec<CatalogueItem>,
}

/// 🧷 Draggable catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueItem {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neuronKind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub name: String,
    pub summary: String,
}

fn static_catalogue_sections() -> Vec<CatalogueSection> {
    vec![
        CatalogueSection {
            id: "inputs".into(),
            title: "Inputs".into(),
            items: vec![
                CatalogueItem { kind: "inputSlider".into(), neuronKind: None, action: None, name: "Slider".into(), summary: "Number input".into() },
                CatalogueItem { kind: "inputNote".into(), neuronKind: None, action: None, name: "Note".into(), summary: "Text input".into() },
            ],
        },
        CatalogueSection {
            id: "outputs".into(),
            title: "Outputs".into(),
            items: vec![
                CatalogueItem { kind: "outputPreview".into(), neuronKind: None, action: None, name: "Preview".into(), summary: "Preview dictionary".into() },
                CatalogueItem { kind: "outputAction".into(), neuronKind: None, action: Some("log".into()), name: "Action".into(), summary: "Side-effect action".into() },
            ],
        },
    ]
}

fn merge_catalogue_sections(host_json: &str) -> Result<Vec<CatalogueSection>, String> {
    let mut sections: Vec<CatalogueSection> = if host_json.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(host_json).map_err(|e| e.to_string())?
    };
    sections.extend(static_catalogue_sections());
    Ok(sections)
}

fn titleize_module(module: &str) -> String {
    let mut chars = module.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
// #endregion 🔖Catalogue

// #region 🔖EvalBridge
fn parse_bridge_dictionary_json(result_json: &str) -> Result<Dictionary, EvalError> {
    if let Ok(dict) = serde_json::from_str::<Dictionary>(result_json) {
        return Ok(dict);
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(result_json) {
        if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
            return Err(EvalError::InvalidInput(err.into()));
        }
    }
    Err(EvalError::InvalidInput("invalid bridge response".into()))
}

#[cfg(target_arch = "wasm32")]
struct EvalBridge {
    cb: js_sys::Function,
}

#[cfg(target_arch = "wasm32")]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        use wasm_bindgen::JsValue;
        let input_json = serde_json::to_string(input).map_err(|e| EvalError::InvalidInput(e.to_string()))?;
        let result = self
            .cb
            .call2(&JsValue::NULL, &JsValue::from_str(kind_id), &JsValue::from_str(&input_json))
            .map_err(|_| EvalError::InvalidInput("bridge call failed".into()))?;
        let result_json = result.as_string().ok_or_else(|| EvalError::InvalidInput("bridge did not return string".into()))?;
        parse_bridge_dictionary_json(&result_json)
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct EvalBridge {
    cb: Box<dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        (self.cb)(kind_id, input)
    }
}
// #endregion 🔖EvalBridge

// #region 🔖FlowHost
/// 🏠 Retained flow host: fixture, dag scene, evaluation cache.
pub struct FlowHost {
    pub fixture: FlowFixtureV1,
    pub dag: DagHost,
    pub outputs: HashMap<String, Dictionary>,
    pub last_eval_json: String,
    eval_bridge: Option<EvalBridge>,
    host_catalogue_json: String,
    next_widget_serial: u64,
    next_synapse_serial: u64,
    viewport_w: u32,
    viewport_h: u32,
    viewport_dpr: f64,
    pan_anchor: Option<(f64, f64, f64, f64)>,
    slider_adjust_id: Option<String>,
}

impl Default for FlowHost {
    fn default() -> Self {
        Self::from_fixture(FlowFixtureV1::default())
    }
}

impl FlowHost {
    pub fn from_fixture(fixture: FlowFixtureV1) -> Self {
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture(DagFixtureV1 { schema: "dag.fixture/v1".into(), camera: dag::DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            outputs: HashMap::new(),
            last_eval_json: String::new(),
            eval_bridge: None,
            host_catalogue_json: String::new(),
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            slider_adjust_id: None,
        };
        host.rebuild_dag();
        host.evaluate_internal();
        host
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixtureV1, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|e| e.to_string())
    }

    pub fn catalogue_json(&self) -> Result<String, String> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        serde_json::to_string(&sections).map_err(|e| e.to_string())
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_eval_bridge_fn(&mut self, cb: Box<dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError>>) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_eval_bridge_js(&mut self, cb: js_sys::Function) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    pub fn evaluate(&mut self) -> Result<String, String> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport_w = width.max(1);
        self.viewport_h = height.max(1);
        self.viewport_dpr = dpr.max(1.0);
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
    }

    pub fn world_from_screen(&self, sx: f64, sy: f64) -> (f64, f64) {
        let p = self.screen_to_world_point(sx, sy);
        (p.x, p.y)
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = CameraJson { x, y, zoom: zoom.clamp(0.05, 8.0) };
        self.dag.set_camera(x, y, self.fixture.camera.zoom);
    }

    pub fn wheel(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
        let zoom = (self.fixture.camera.zoom * factor).clamp(0.05, 8.0);
        self.fixture.camera.zoom = zoom;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
        let after = self.screen_to_world_point(sx, sy);
        self.fixture.camera.x += before.x - after.x;
        self.fixture.camera.y += before.y - after.y;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
    }

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, String> {
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json).map_err(|e| e.to_string())?;
        let id = self.next_widget_id(&descriptor);
        let widget = match descriptor {
            WidgetDescriptor::Neuron { neuronKind } => Widget::Neuron { id: id.clone(), neuronKind, params: Dictionary::new() },
            WidgetDescriptor::InputSlider => Widget::InputSlider { id: id.clone(), value: 3.0 },
            WidgetDescriptor::InputNote => Widget::InputNote { id: id.clone(), text: String::new() },
            WidgetDescriptor::OutputPreview => Widget::OutputPreview { id: id.clone(), preview: Dictionary::new() },
            WidgetDescriptor::OutputAction { action } => Widget::OutputAction { id: id.clone(), action: if action.is_empty() { "log".into() } else { action } },
        };
        self.fixture.widgets.push(widget);
        self.fixture.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.rebuild_dag();
        self.evaluate_internal();
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), String> {
        let before = self.fixture.widgets.len();
        self.fixture.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.fixture.widgets.len() == before {
            return Err(format!("unknown widget: {widget_id}"));
        }
        self.fixture.layout.remove(widget_id);
        self.fixture.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        self.rebuild_dag();
        self.evaluate_internal();
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), String> {
        if !self.fixture.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(format!("unknown widget: {widget_id}"));
        }
        self.fixture.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        if let Some(node) = self.dag.fixture.nodes.iter_mut().find(|n| n.id == widget_id) {
            node.x = x;
            node.y = y;
        }
        self.rebuild_dag();
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, String> {
        if from_id == to_id {
            return Err("cannot connect widget to itself".into());
        }
        if !widget_has_output(from_id, &self.fixture.widgets) {
            return Err(format!("{from_id} has no output port"));
        }
        if !widget_has_input(to_id, &self.fixture.widgets) {
            return Err(format!("{to_id} has no input port"));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err("connection would create cycle".into());
        }
        if self.fixture.synapses.iter().any(|s| s.from == from_id && s.to == to_id) {
            return Err("connection already exists".into());
        }
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string() });
        self.rebuild_dag();
        self.evaluate_internal();
        Ok(synapse_id)
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), String> {
        let before = self.fixture.synapses.len();
        self.fixture.synapses.retain(|s| s.id != synapse_id);
        if self.fixture.synapses.len() == before {
            return Err(format!("unknown synapse: {synapse_id}"));
        }
        self.rebuild_dag();
        self.evaluate_internal();
        Ok(())
    }

    /// 🌳 Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts_json: &str) -> Result<(), String> {
        let opts: DagLayoutOptions = if opts_json.trim().is_empty() {
            DagLayoutOptions::default()
        } else {
            serde_json::from_str(opts_json).map_err(|e| e.to_string())?
        };
        self.dag = DagHost::from_fixture_without_layout(self.build_dag_fixture_v1());
        self.dag.reorganize(&opts)?;
        self.sync_from_dag();
        Ok(())
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, extend: bool, pan: bool) {
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        let world = self.screen_to_world_point(sx, sy);
        if let Some(widget_id) = self.hit_slider_widget_at(world.x, world.y) {
            self.slider_adjust_id = Some(widget_id.clone());
            self.adjust_slider_at_world(&widget_id, world.x);
            return;
        }
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_down(sx, sy, extend);
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.fixture.camera.zoom;
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            return;
        }
        if let Some(widget_id) = self.slider_adjust_id.clone() {
            let world = self.screen_to_world_point(sx, sy);
            self.adjust_slider_at_world(&widget_id, world.x);
            return;
        }
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_move(sx, sy);
        self.sync_from_dag();
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        self.pan_anchor = None;
        let was_slider = self.slider_adjust_id.take();
        if was_slider.is_some() {
            self.evaluate_internal();
            return;
        }
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_up(sx, sy);
        self.sync_from_dag();
        self.evaluate_internal();
    }

    fn evaluate_internal(&mut self) {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let Some(bridge) = self.eval_bridge.as_ref() else {
            self.last_eval_json = serde_json::json!({ "error": "evaluation bridge not configured" }).to_string();
            return;
        };
        let registry = neural::Registry::new();
        let evaluator = Evaluator::new(&registry);
        let mut dispatch = |kind: &str, input: &Dictionary| bridge.evaluate(kind, input);
        match evaluator.evaluate_with(&tree, &seeds, &mut dispatch) {
            Ok(outputs) => {
                self.outputs = outputs.clone();
                self.apply_preview_outputs(&outputs);
                self.last_eval_json = serde_json::to_string(&outputs).unwrap_or_else(|_| "{}".into());
            }
            Err(err) => {
                self.last_eval_json = serde_json::json!({ "error": err.to_string() }).to_string();
            }
        }
    }

    fn build_tree(&self) -> Tree {
        let neurons = self
            .fixture
            .widgets
            .iter()
            .filter_map(|w| match w {
                Widget::Neuron { id, neuronKind, params } => Some(Neuron { id: id.clone(), kind: neuronKind.clone(), params: params.clone() }),
                _ => None,
            })
            .collect();
        let synapses = self.fixture.synapses.iter().map(|s| Synapse { id: s.id.clone(), from: s.from.clone(), to: s.to.clone() }).collect();
        Tree { neurons, synapses }
    }

    fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.fixture.widgets {
            match widget {
                Widget::InputSlider { id, value } => {
                    seeds.insert(id.clone(), Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(*value))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), Dictionary::new().insert("text", NeuralValue::Atom(Atom::String(text.clone()))));
                }
                _ => {}
            }
        }
        seeds
    }

    fn apply_preview_outputs(&mut self, outputs: &HashMap<String, Dictionary>) {
        for widget in &mut self.fixture.widgets {
            if let Widget::OutputPreview { id, preview } = widget {
                if let Some(out) = outputs.get(id) {
                    *preview = out.clone();
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        *preview = src.clone();
                    }
                }
            }
        }
    }

    fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_fixture_v1();
        let apply_layout = self.fixture.layout.len() < self.fixture.widgets.len();
        self.dag = if apply_layout {
            DagHost::from_fixture(fixture)
        } else {
            DagHost::from_fixture_without_layout(fixture)
        };
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.sync_from_dag();
    }

    fn sync_from_dag(&mut self) {
        for node in &self.dag.fixture.nodes {
            self.fixture.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        self.fixture.synapses = self
            .dag
            .fixture
            .edges
            .iter()
            .filter_map(|edge| {
                let from = edge.source.split(':').next()?.to_string();
                let to = edge.target.split(':').next()?.to_string();
                Some(SynapseSpec { id: edge.id.clone(), from, to })
            })
            .collect();
        self.fixture.camera = CameraJson {
            x: self.dag.fixture.camera.x,
            y: self.dag.fixture.camera.y,
            zoom: self.dag.fixture.camera.zoom,
        };
    }

    fn build_dag_fixture_v1(&self) -> DagFixtureV1 {
        let nodes: Vec<DagNodeSpec> = self.fixture.widgets.iter().enumerate().map(|(i, w)| widget_to_dag_node(w, i, &self.fixture.layout)).collect();
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagFixtureEdgeV1> = self
            .fixture
            .synapses
            .iter()
            .filter(|syn| !would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to))
            .map(|syn| DagFixtureEdgeV1 { id: syn.id.clone(), source: format!("{}:out", syn.from), target: format!("{}:in", syn.to) })
            .collect();
        DagFixtureV1 { schema: "dag.fixture/v1".into(), camera: dag::DagCameraV1 { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::vello::kurbo::Point {
        use cavas::camera::{screen_to_world, Camera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        self.next_widget_serial += 1;
        let prefix = match descriptor {
            WidgetDescriptor::Neuron { neuronKind } => neuronKind.replace('.', "_"),
            WidgetDescriptor::InputSlider => "slider".into(),
            WidgetDescriptor::InputNote => "note".into(),
            WidgetDescriptor::OutputPreview => "preview".into(),
            WidgetDescriptor::OutputAction { .. } => "action".into(),
        };
        format!("{prefix}_{}", self.next_widget_serial)
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        for widget in &mut self.fixture.widgets {
            if let Widget::InputSlider { id, value: v } = widget {
                if id == widget_id {
                    *v = value.clamp(FLOW_SLIDER_MIN, FLOW_SLIDER_MAX);
                }
            }
        }
        let _ = self.evaluate();
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        for widget in &mut self.fixture.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        let _ = self.evaluate();
    }

    fn hit_slider_widget_at(&self, wx: f64, wy: f64) -> Option<String> {
        for widget in &self.fixture.widgets {
            let Widget::InputSlider { id, .. } = widget else { continue };
            let node = self.dag.fixture.nodes.iter().find(|n| n.id == *id)?;
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let left = node.x - hw + 4.0;
            let right = node.x + hw - 16.0;
            let top = node.y - hh + 4.0;
            let bottom = node.y + hh - 4.0;
            if wx >= left && wx <= right && wy >= top && wy <= bottom {
                return Some(id.clone());
            }
        }
        None
    }

    fn adjust_slider_at_world(&mut self, widget_id: &str, world_x: f64) {
        let Some(node) = self.dag.fixture.nodes.iter().find(|n| n.id == widget_id) else { return };
        let hw = node.width * 0.5;
        let track_left = node.x - hw + 12.0;
        let track_right = node.x + hw - 28.0;
        let span = (track_right - track_left).max(1.0);
        let t = ((world_x - track_left) / span).clamp(0.0, 1.0);
        let raw = FLOW_SLIDER_MIN + t * (FLOW_SLIDER_MAX - FLOW_SLIDER_MIN);
        let stepped = (raw / FLOW_SLIDER_STEP).round() * FLOW_SLIDER_STEP;
        self.set_slider_value(widget_id, stepped);
    }

    fn format_preview_number(n: f64) -> String {
        if (n - n.round()).abs() < 0.05 {
            format!("{}", n.round() as i64)
        } else {
            format!("{n:.1}")
        }
    }

    fn format_dictionary_preview(dict: &Dictionary) -> String {
        dict.get("number")
            .and_then(|v| v.as_atom())
            .and_then(|a| a.as_f64())
            .map(Self::format_preview_number)
            .or_else(|| dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string))
            .unwrap_or_else(|| "—".into())
    }

    fn paint_flow_widget_chrome(&self, scene: &mut cavas::vello::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, world_to_screen, Camera, Viewport};
        use cavas::text::append_label;
        use cavas::vello::kurbo::{Circle, Point, RoundedRect, RoundedRectRadii, Stroke};
        use cavas::vello::peniko::{Color, Fill};

        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let theme = &self.dag.vello_theme;
        let track_fill = theme.node_fill;
        let track_stroke = theme.node_stroke;
        let thumb_fill = theme.handle_stroke_selected;
        let text_fill = theme.node_stroke;
        let text_halo = {
            let rgba = theme.raster_clear.to_rgba8();
            Color::from_rgba8(rgba.r, rgba.g, rgba.b, 210)
        };

        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.fixture.nodes.iter().find(|n| n.id == id) else { continue };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let px = (10.5 * cam.zoom).clamp(8.0, 16.0);
            match widget {
                Widget::InputSlider { value, .. } => {
                    let track_left = node.x - hw + 12.0;
                    let track_right = node.x + hw - 28.0;
                    let track_y = node.y + 8.0;
                    let track_h = 8.0;
                    let track = RoundedRect::new(
                        track_left,
                        track_y - track_h * 0.5,
                        track_right,
                        track_y + track_h * 0.5,
                        RoundedRectRadii::from_single_radius(4.0),
                    );
                    scene.fill(Fill::NonZero, aff, track_fill, None, &track);
                    scene.stroke(&Stroke::new(1.0), aff, track_stroke, None, &track);
                    let t = ((*value - FLOW_SLIDER_MIN) / (FLOW_SLIDER_MAX - FLOW_SLIDER_MIN)).clamp(0.0, 1.0);
                    let thumb_x = track_left + t * (track_right - track_left);
                    scene.fill(Fill::NonZero, aff, thumb_fill, None, &Circle::new(Point::new(thumb_x, track_y), 6.0));
                    let label = format!("{value:.1}");
                    let screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y - hh + 16.0));
                    append_label(scene, &label, screen, px, text_fill, text_halo);
                }
                Widget::InputNote { text, .. } => {
                    let display = if text.is_empty() { "Note…" } else { text.as_str() };
                    let screen = world_to_screen(&cam, &viewport, Point::new(node.x - hw + 14.0, node.y + 6.0));
                    append_label(scene, display, screen, px, text_fill, text_halo);
                }
                Widget::OutputPreview { preview, .. } => {
                    let display = Self::format_dictionary_preview(preview);
                    let screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y + 10.0));
                    append_label(scene, &display, screen, px * 1.05, text_fill, text_halo);
                }
                _ => {}
            }
        }
    }

    pub fn preview_text(&self) -> String {
        self.fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => preview
                    .get("number")
                    .and_then(|v| v.as_atom())
                    .and_then(|a| a.as_f64())
                    .map(Self::format_preview_number)
                    .or_else(|| preview.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string)),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.dag.set_vello_theme_from_json(json)
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
        self.paint_flow_widget_chrome(scene, width, height, dpr);
    }
}

fn widget_id_for(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::OutputPreview { id, .. } | Widget::OutputAction { id, .. } => id,
    }
}

fn widget_has_output(widget_id: &str, widgets: &[Widget]) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w).1.is_empty())
}

fn widget_has_input(widget_id: &str, widgets: &[Widget]) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w).0.is_empty())
}
// #endregion 🔖FlowHost

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
struct FlowSessionInner {
    host: FlowHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
    width: u32,
    height: u32,
    dpr: f64,
}

#[cfg(target_arch = "wasm32")]
impl FlowSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.width = lw;
        self.height = lh;
        self.dpr = dpr;
        self.host.set_viewport(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let mut scene = cavas::vello::Scene::new();
        let clear = self.host.dag.vello_theme.raster_clear;
        self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct FlowSession {
    state: Rc<RefCell<FlowSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl FlowSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(FlowSessionInner { host: FlowHost::default(), gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
        let mut inner = self.state.borrow_mut();
        let (w, h, dpr) = (inner.width, inner.height, inner.dpr);
        inner.host = FlowHost::from_fixture(fixture);
        inner.host.set_viewport(w.max(1), h.max(1), dpr.max(1.0));
        Ok(())
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = catalogueJson)]
    pub fn catalogue_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.catalogue_json().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setEvalBridge)]
    pub fn set_eval_bridge(&self, cb: js_sys::Function) {
        self.state.borrow_mut().host.set_eval_bridge_js(cb);
    }

    #[wasm_bindgen(js_name = setCatalogueJson)]
    pub fn set_catalogue_json(&self, json: &str) {
        self.state.borrow_mut().host.set_host_catalogue_json(json);
    }

    #[wasm_bindgen(js_name = evaluate)]
    pub fn evaluate(&self) -> Result<String, JsValue> {
        self.state.borrow_mut().host.evaluate().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = previewText)]
    pub fn preview_text(&self) -> String {
        self.state.borrow().host.preview_text()
    }

    #[wasm_bindgen(js_name = setSliderValue)]
    pub fn set_slider_value(&self, widget_id: &str, value: f64) {
        self.state.borrow_mut().host.set_slider_value(widget_id, value);
    }

    #[wasm_bindgen(js_name = setNoteText)]
    pub fn set_note_text(&self, widget_id: &str, text: &str) {
        self.state.borrow_mut().host.set_note_text(widget_id, text);
    }

    #[wasm_bindgen(js_name = addWidget)]
    pub fn add_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, JsValue> {
        self.state.borrow_mut().host.add_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = removeWidget)]
    pub fn remove_widget(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_widget(widget_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = moveWidget)]
    pub fn move_widget(&self, widget_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.move_widget(widget_id, x, y).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = connect)]
    pub fn connect(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect(from_id, to_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = disconnect)]
    pub fn disconnect(&self, synapse_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.disconnect(synapse_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = worldFromScreen)]
    pub fn world_from_screen(&self, sx: f64, sy: f64) -> String {
        let (x, y) = self.state.borrow().host.world_from_screen(sx, sy);
        serde_json::json!({ "x": x, "y": y }).to_string()
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = wheel)]
    pub fn wheel(&self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                .await
                .map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
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
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let mut inner = self.state.borrow_mut();
        inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = setVelloThemeJson)]
    pub fn set_vello_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .reorganize(options_json)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&self, sx: f64, sy: f64, extend: bool, pan: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, extend, pan);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy);
    }
}
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cavas::camera::{world_to_screen, Camera, Viewport};
    use cavas::vello::kurbo::Point;

    fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "math.add" {
            let a = input
                .get("a")
                .or_else(|| input.get("number"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(a + b))));
        }
        if kind == "math.passThrough" {
            let n = input
                .get("number")
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("number".into()))?;
            return Ok(Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(n))));
        }
        Err(EvalError::UnknownKind(kind.into()))
    }

    fn host_with_test_bridge() -> FlowHost {
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(test_math_bridge));
        host.set_host_catalogue_json(&serde_json::to_string(&[CatalogueSection {
            id: "math".into(),
            title: "Math".into(),
            items: vec![
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some("math.add".into()),
                    action: None,
                    name: "Add".into(),
                    summary: "Sums two numbers".into(),
                },
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some("math.passThrough".into()),
                    action: None,
                    name: "Pass Through".into(),
                    summary: "Forwards a number".into(),
                },
            ],
        }]).unwrap());
        host.evaluate_internal();
        host
    }

    fn widget_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == widget_id).expect("node");
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
        (screen.x, screen.y)
    }

    #[test]
    fn default_fixture_evaluates_add_preview() {
        let host = host_with_test_bridge();
        assert_eq!(host.preview_text(), "3");
    }

    #[test]
    fn slider_updates_preview() {
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 5.0);
        assert_eq!(host.preview_text(), "5");
    }

    #[test]
    fn default_auto_layout_orders_slider_add_preview_left_to_right() {
        let host = host_with_test_bridge();
        let slider = host.fixture.layout.get("slider").expect("slider");
        let add = host.fixture.layout.get("add").expect("add");
        let preview = host.fixture.layout.get("preview").expect("preview");
        assert!(add.x > slider.x);
        assert!(preview.x > add.x);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value_playground_viewport() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1259, 706, 1.0);
        let (sx, sy) = widget_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, false, false);
        host.pointer_move_screen(sx + 90.0, sy);
        host.pointer_up_screen(sx + 90.0, sy);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, false, false);
        host.pointer_move_screen(sx + 80.0, sy);
        host.pointer_up_screen(sx + 80.0, sy);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn reorganize_overwrites_saved_layout_left_to_right() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.rebuild_dag();
        host.reorganize("").unwrap();
        let slider = host.fixture.layout.get("slider").expect("slider layout");
        let add = host.fixture.layout.get("add").expect("add layout");
        let preview = host.fixture.layout.get("preview").expect("preview layout");
        assert!(add.x > slider.x);
        assert!(preview.x > add.x);
    }

    #[test]
    fn fixture_json_round_trip() {
        let host = FlowHost::default();
        let json = host.fixture_json().unwrap();
        let parsed = FlowHost::parse_fixture_json(&json).unwrap();
        assert_eq!(parsed.schema, "flow.fixture/v1");
    }

    #[test]
    fn catalogue_has_module_sections() {
        let host = host_with_test_bridge();
        let json = host.catalogue_json().unwrap();
        assert!(json.contains("math"));
        assert!(json.contains("math.add"));
        assert!(json.contains("Inputs"));
        assert!(json.contains("Outputs"));
    }

    #[test]
    fn add_widget_and_connect() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
        host.connect("slider", &id).unwrap();
        host.connect(&id, "preview").unwrap();
        host.set_slider_value("slider", 4.0);
        assert_eq!(host.preview_text(), "4");
    }
}
// #endregion 🔖Tests
