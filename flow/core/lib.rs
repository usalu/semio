//! 🌊 Flow core: widgets, neural evaluation, and DAG canvas host.

pub use flow_module_math;
pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed_dag as dag;
pub use neural_engine as neural;

use std::collections::HashMap;

use dag::{apply_dag_layout_to_fixture_v1_value, would_create_cycle, DagBoardEngine, DagLayoutOptions, IoNodeSpec, IoPortSpec};
use neural::{Atom, Dictionary, Evaluator, Neuron, Registry, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

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

/// 🧩 Serializable flow document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFixtureV1 {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
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

fn widget_to_io_node(widget: &Widget, index: usize) -> IoNodeSpec {
    let (inputs, outputs) = widget_io_ports(widget);
    let id = match widget {
        Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::OutputPreview { id, .. } | Widget::OutputAction { id, .. } => id.clone(),
    };
    IoNodeSpec {
        id: id.clone(),
        name: widget_label(widget),
        inputs,
        outputs,
        x: (index as f64) * 200.0,
        y: 0.0,
        width: 160.0,
        height: 56.0,
    }
}
// #endregion 🔖Widget

// #region 🔖FlowHost
/// 🏠 Retained flow host: fixture, engine, evaluation cache.
pub struct FlowHost {
    pub fixture: FlowFixtureV1,
    pub engine: DagBoardEngine,
    pub outputs: HashMap<String, Dictionary>,
    pub last_eval_json: String,
    next_node_id: u64,
    next_handle_id: u64,
    next_edge_id: u64,
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
            engine: DagBoardEngine::new(),
            outputs: HashMap::new(),
            last_eval_json: String::new(),
            next_node_id: 1,
            next_handle_id: 10,
            next_edge_id: 100,
        };
        host.rebuild_engine_from_fixture();
        host.evaluate_internal();
        host
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixtureV1, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|e| e.to_string())
    }

    pub fn dag_fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.build_dag_fixture_value()).map_err(|e| e.to_string())
    }

    pub fn evaluate(&mut self) -> Result<String, String> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    fn evaluate_internal(&mut self) {
        let mut registry = Registry::new();
        flow_module_math::register(&mut registry);
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        match Evaluator::new(&registry).evaluate(&tree, &seeds) {
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
        let synapses = self
            .fixture
            .synapses
            .iter()
            .map(|s| Synapse { id: s.id.clone(), from: s.from.clone(), to: s.to.clone() })
            .collect();
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

    fn rebuild_engine_from_fixture(&mut self) {
        self.engine = DagBoardEngine::new();
        self.engine.set_camera(self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        let mut dag_fixture = self.build_dag_fixture_value();
        let _ = apply_dag_layout_to_fixture_v1_value(&mut dag_fixture, &DagLayoutOptions::default());
        self.sync_engine_from_dag_fixture(&dag_fixture);
    }

    fn build_dag_fixture_value(&self) -> JsonValue {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for (i, widget) in self.fixture.widgets.iter().enumerate() {
            let io = widget_to_io_node(widget, i);
            let mut handles = Vec::new();
            for (idx, port) in io.inputs.iter().enumerate() {
                let (_in_a, _out_a) = dag::io_node_handle_angles(idx, io.inputs.len(), 0, io.outputs.len().max(1));
                handles.push(serde_json::json!({
                    "id": format!("{}:{}", io.id, port.id),
                    "angle": _in_a,
                    "handleKind": port.label
                }));
            }
            for (idx, port) in io.outputs.iter().enumerate() {
                let (_in_a, out_a) = dag::io_node_handle_angles(0, io.inputs.len().max(1), idx, io.outputs.len());
                handles.push(serde_json::json!({
                    "id": format!("{}:{}", io.id, port.id),
                    "angle": out_a,
                    "handleKind": port.label
                }));
            }
            nodes.push(serde_json::json!({
                "id": io.id,
                "x": io.x,
                "y": io.y,
                "width": io.width,
                "height": io.height,
                "shape": "rectangle",
                "text": io.name,
                "handles": handles
            }));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        for syn in &self.fixture.synapses {
            if would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to) {
                continue;
            }
            edges.push(serde_json::json!({
                "id": syn.id,
                "source": format!("{}:out", syn.from),
                "target": format!("{}:in", syn.to)
            }));
        }
        serde_json::json!({
            "schema": "dag.fixture/v1",
            "camera": self.fixture.camera,
            "nodes": nodes,
            "edges": edges
        })
    }

    fn sync_engine_from_dag_fixture(&mut self, fixture: &JsonValue) {
        let nodes = fixture.get("nodes").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        for node in nodes {
            let Some(obj) = node.as_object() else { continue };
            let id = self.next_node_id;
            self.next_node_id += 1;
            let x = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let y = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(80.0);
            self.engine.create_node(id, x, y, (w * 0.5).max(28.0), true);
            if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                for h in handles {
                    let Some(ho) = h.as_object() else { continue };
                    let hid = self.next_handle_id;
                    self.next_handle_id += 1;
                    let ang = ho.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    self.engine.create_handle(hid, id, ang);
                }
            }
        }
        let edges = fixture.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let mut handle_ids: Vec<u64> = self.engine.handles.keys().copied().collect();
        handle_ids.sort();
        for (i, edge) in edges.iter().enumerate() {
            let Some(_eo) = edge.as_object() else { continue };
            let eid = self.next_edge_id + i as u64;
            let src = handle_ids.get(i * 2).copied().or_else(|| handle_ids.first().copied());
            let tgt = handle_ids.get(i * 2 + 1).copied().or_else(|| handle_ids.get(1).copied());
            if let (Some(s), Some(t)) = (src, tgt) {
                self.engine.create_edge(eid, s, t);
            }
        }
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        for widget in &mut self.fixture.widgets {
            if let Widget::InputSlider { id, value: v } = widget {
                if id == widget_id {
                    *v = value;
                }
            }
        }
        let _ = self.evaluate();
    }

    pub fn preview_text(&self) -> String {
        self.fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => preview.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).map(|n| format!("{n}")),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }
}

fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::OutputPreview { id, .. } | Widget::OutputAction { id, .. } => id,
    }
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
}

#[cfg(target_arch = "wasm32")]
impl FlowSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        let cam = &self.host.fixture.camera;
        self.host.engine.set_camera(cam.x, cam.y, cam.zoom);
        self.gpu.resize_surface(pw, ph);
        let _ = (lw, lh, dpr);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let snap = self.host.engine.render_snapshot();
        let mut scene = cavas::vello::Scene::new();
        use cavas::vello::kurbo::{Circle, Point};
        use cavas::vello::peniko::{Color, Fill};
        for (_nid, center, radius) in &snap.nodes {
            scene.fill(Fill::NonZero, cavas::vello::kurbo::Affine::IDENTITY, Color::from_rgb8(90, 110, 140), None, &Circle::new(Point::new(center.x, center.y), *radius));
        }
        for (_hid, center, radius) in &snap.handles {
            scene.fill(Fill::NonZero, cavas::vello::kurbo::Affine::IDENTITY, Color::from_rgb8(180, 200, 230), None, &Circle::new(Point::new(center.x, center.y), *radius));
        }
        for curve in &snap.edges {
            scene.stroke(&cavas::vello::kurbo::Stroke::new(2.0), cavas::vello::kurbo::Affine::IDENTITY, Color::from_rgb8(200, 210, 230), None, curve);
        }
        self.gpu.render_frame(&scene, Color::from_rgba8(20, 22, 28, 255))
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
        Self { state: Rc::new(RefCell::new(FlowSessionInner { host: FlowHost::default(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
        self.state.borrow_mut().host = FlowHost::from_fixture(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
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
            let cam = &g.host.fixture.camera;
            g.host.engine.set_camera(cam.x, cam.y, cam.zoom);
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

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }

    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
        self.state.borrow_mut().host.engine.pointer_down(x, y, extend);
    }

    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.engine.pointer_move(x, y);
    }

    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(&self) {
        self.state.borrow_mut().host.engine.pointer_up();
    }
}
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_fixture_evaluates_add_preview() {
        let mut host = FlowHost::default();
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "3");
    }

    #[test]
    fn slider_updates_preview() {
        let mut host = FlowHost::default();
        host.set_slider_value("slider", 5.0);
        assert_eq!(host.preview_text(), "5");
    }

    #[test]
    fn fixture_json_round_trip() {
        let host = FlowHost::default();
        let json = host.fixture_json().unwrap();
        let parsed = FlowHost::parse_fixture_json(&json).unwrap();
        assert_eq!(parsed.schema, "flow.fixture/v1");
    }
}
// #endregion 🔖Tests
