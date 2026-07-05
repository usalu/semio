//! 🕸️ Generic node-graph engine for framework renderers.

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed_dag as dag;

use dag::{
    dag_screen_to_world, dag_take_pending_open_instance_id, fit_node_size, DagCamera, DagFixture,
    DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, IoPortSpec,
};
use serde::Deserialize;
use serde_json::Value;

//#region 🔖ScenePayload
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphPortRecord {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    abbreviation: Option<String>,
    #[serde(rename = "fullName", default)]
    full_name: Option<String>,
    #[serde(rename = "resourceKind", default)]
    resource_kind: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphNodeRecord {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    instance_id: Option<String>,
    #[serde(default)]
    program_id: Option<String>,
    #[serde(default)]
    app_id: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    x: Option<f64>,
    #[serde(default)]
    y: Option<f64>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
    #[serde(default)]
    inputs: Option<Vec<GraphPortRecord>>,
    #[serde(default)]
    outputs: Option<Vec<GraphPortRecord>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphViewport {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_zoom")]
    zoom: f64,
}

fn default_zoom() -> f64 {
    1.0
}

fn port_label(port: &GraphPortRecord) -> String {
    port.label.clone().unwrap_or_else(|| {
        let segments: Vec<_> = port.id.split(':').collect();
        segments.last().map(|s| (*s).to_string()).unwrap_or_else(|| port.id.clone())
    })
}

fn port_to_io(port: &GraphPortRecord) -> IoPortSpec {
    let label = port_label(port);
    let mut spec = IoPortSpec::simple(port.id.clone(), label);
    if let Some(code) = &port.code {
        spec.code = code.clone();
    }
    if let Some(abbrev) = &port.abbreviation {
        spec.abbreviation = abbrev.clone();
    }
    if let Some(full) = &port.full_name {
        spec.full_name = full.clone();
    }
    if let Some(kind) = &port.resource_kind {
        spec.resource_kind = Some(kind.clone());
    }
    spec
}

fn node_record_to_spec(record: &GraphNodeRecord) -> DagNodeSpec {
    let name = record.label.clone().unwrap_or_else(|| record.id.clone());
    let abbreviation = name.chars().take(3).collect::<String>();
    let icon = record.icon.clone().unwrap_or_else(|| "emoji:🔷".into());
    let x = record.x.unwrap_or(0.0);
    let y = record.y.unwrap_or(0.0);
    let width = record.width.unwrap_or(180.0);
    let height = record.height.unwrap_or(72.0);
    let inputs: Vec<IoPortSpec> = record.inputs.as_deref().unwrap_or(&[]).iter().map(port_to_io).collect();
    let outputs: Vec<IoPortSpec> = record.outputs.as_deref().unwrap_or(&[]).iter().map(port_to_io).collect();
    if let Some(instance_id) = &record.instance_id {
        let mut node = DagNodeSpec {
            id: record.id.clone(),
            name: name.clone(),
            abbreviation,
            icon: icon.clone(),
            x,
            y,
            width,
            height,
            kind: DagNodeKind::AppInstance {
                instance_id: instance_id.clone(),
                program_id: record.program_id.clone().unwrap_or_else(|| "app".into()),
                app_id: record.app_id.clone().unwrap_or_else(|| record.id.clone()),
                icon,
                inputs,
                outputs,
            },
            ..Default::default()
        };
        fit_node_size(&mut node);
        return node;
    }
    let mut node = DagNodeSpec::computation(record.id.clone(), name, abbreviation, icon, inputs, outputs, false, false, x, y, width, height);
    fit_node_size(&mut node);
    node
}

pub fn fixture_from_node_graph_json(nodes_json: &str, edges_json: &str, viewport_json: &str) -> Result<DagFixture, String> {
    let nodes: Vec<GraphNodeRecord> = if nodes_json.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(nodes_json).map_err(|e| e.to_string())?
    };
    let edges: Vec<GraphEdgeRecord> = if edges_json.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(edges_json).map_err(|e| e.to_string())?
    };
    let viewport: GraphViewport = if viewport_json.trim().is_empty() {
        GraphViewport::default()
    } else {
        serde_json::from_str(viewport_json).map_err(|e| e.to_string())?
    };
    Ok(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: viewport.x, y: viewport.y, zoom: viewport.zoom },
        nodes: nodes.iter().map(node_record_to_spec).collect(),
        edges: edges
            .iter()
            .map(|edge| DagFixtureEdge {
                id: edge.id.clone(),
                source: format!("{}:{}", edge.source_node_id, edge.source_port_id),
                target: format!("{}:{}", edge.target_node_id, edge.target_port_id),
                ..Default::default()
            })
            .collect(),
    })
}

#[derive(Clone, Debug, Default)]
pub struct NodeGraphScenePayload {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    pub selection_json: Option<String>,
    pub hover_json: Option<String>,
    pub preview_off_json: Option<String>,
    pub lod_json: Option<String>,
    pub catalogue_json: Option<String>,
    pub controls_json: Option<String>,
    pub clusters_json: Option<String>,
    pub computing_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub fixture_json: Option<String>,
}

impl NodeGraphScenePayload {
    pub fn from_json(value: &Value) -> Self {
        Self {
            nodes_json: value.get("nodesJson").and_then(|v| v.as_str()).unwrap_or("[]").to_string(),
            edges_json: value.get("edgesJson").and_then(|v| v.as_str()).unwrap_or("[]").to_string(),
            viewport_json: value.get("viewportJson").and_then(|v| v.as_str()).unwrap_or(r#"{"x":0,"y":0,"zoom":1}"#).to_string(),
            selection_json: value.get("selectionJson").and_then(|v| v.as_str()).map(str::to_string),
            hover_json: value.get("hoverJson").and_then(|v| v.as_str()).map(str::to_string),
            preview_off_json: value.get("previewOffJson").and_then(|v| v.as_str()).map(str::to_string),
            lod_json: value.get("lodJson").and_then(|v| v.as_str()).map(str::to_string),
            catalogue_json: value.get("catalogueJson").and_then(|v| v.as_str()).map(str::to_string),
            controls_json: value.get("controlsJson").and_then(|v| v.as_str()).map(str::to_string),
            clusters_json: value.get("clustersJson").and_then(|v| v.as_str()).map(str::to_string),
            computing_json: value.get("computingJson").and_then(|v| v.as_str()).map(str::to_string),
            capabilities_json: value.get("capabilitiesJson").and_then(|v| v.as_str()).map(str::to_string),
            fixture_json: value.get("fixtureJson").and_then(|v| v.as_str()).map(str::to_string),
        }
    }
}
//#endregion 🔖ScenePayload

//#region 🔖GraphHost
/// 🕸️ Retained generic node-graph host wrapping the DAG canvas engine.
pub struct GraphHost {
    pub dag: DagHost,
    pub catalogue_json: String,
    pub controls_json: String,
    pub capabilities_json: String,
    last_payload_signature: u64,
}

impl Default for GraphHost {
    fn default() -> Self {
        Self::from_fixture(DagFixture::default())
    }
}

impl GraphHost {
    pub fn from_fixture(fixture: DagFixture) -> Self {
        Self {
            dag: DagHost::from_fixture_without_layout(fixture),
            catalogue_json: String::new(),
            controls_json: String::new(),
            capabilities_json: String::new(),
            last_payload_signature: 0,
        }
    }

    fn payload_signature(payload: &NodeGraphScenePayload) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        payload.nodes_json.hash(&mut hasher);
        payload.edges_json.hash(&mut hasher);
        payload.viewport_json.hash(&mut hasher);
        payload.selection_json.hash(&mut hasher);
        payload.hover_json.hash(&mut hasher);
        payload.preview_off_json.hash(&mut hasher);
        payload.lod_json.hash(&mut hasher);
        payload.computing_json.hash(&mut hasher);
        hasher.finish()
    }

    pub fn sync_from_payload(&mut self, payload: &NodeGraphScenePayload) -> Result<(), String> {
        let signature = Self::payload_signature(payload);
        if signature != self.last_payload_signature {
            let fixture = fixture_from_node_graph_json(&payload.nodes_json, &payload.edges_json, &payload.viewport_json)?;
            self.dag = DagHost::from_fixture_without_layout(fixture);
            self.last_payload_signature = signature;
        }
        if let Some(selection_json) = &payload.selection_json {
            if let Ok(ids) = serde_json::from_str::<Vec<String>>(selection_json) {
                self.dag.set_selection(&ids);
            }
        }
        if let Some(hover_json) = &payload.hover_json {
            if let Ok(value) = serde_json::from_str::<Value>(hover_json) {
                if let (Some(widget_id), Some(port_id)) = (value.get("nodeId").and_then(|v| v.as_str()), value.get("portId").and_then(|v| v.as_str())) {
                    self.dag.set_hover_channel(Some(widget_id), Some(port_id));
                } else if let Some(node_id) = value.get("nodeId").and_then(|v| v.as_str()) {
                    self.dag.set_hover(Some(node_id));
                }
            }
        }
        if let Some(preview_off_json) = &payload.preview_off_json {
            if let Ok(ids) = serde_json::from_str::<Vec<String>>(preview_off_json) {
                self.dag.set_dimmed(&ids);
            }
        }
        if let Some(lod_json) = &payload.lod_json {
            if let Ok(value) = serde_json::from_str::<Value>(lod_json) {
                if let Some(automatic) = value.get("automatic").and_then(|v| v.as_bool()) {
                    self.dag.set_automatic_lod(automatic);
                }
                if let Some(label) = value.get("lod").and_then(|v| v.as_str()) {
                    self.dag.set_forced_draw_lod_label(label);
                }
            }
        }
        if let Some(computing_json) = &payload.computing_json {
            if let Ok(value) = serde_json::from_str::<Value>(computing_json) {
                let active = value.get("active").and_then(|v| v.as_str()).map(str::to_string);
                let stale: Vec<String> = value
                    .get("stale")
                    .and_then(|v| v.as_array())
                    .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                self.dag.set_computing_progress(active.as_deref(), &stale);
            }
        }
        self.catalogue_json = payload.catalogue_json.clone().unwrap_or_default();
        self.controls_json = payload.controls_json.clone().unwrap_or_default();
        self.capabilities_json = payload.capabilities_json.clone().unwrap_or_default();
        Ok(())
    }

    pub fn sync_from_scene_json(&mut self, scene_json: &str) -> Result<(), String> {
        let value: Value = serde_json::from_str(scene_json).map_err(|e| e.to_string())?;
        self.sync_from_payload(&NodeGraphScenePayload::from_json(&value))
    }

    pub fn paint_scene(&self, scene: &mut cavas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.dag.set_viewport(width, height, dpr);
    }

    pub fn camera_json(&self) -> String {
        serde_json::to_string(&self.dag.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into())
    }

    pub fn selected_node_ids_json(&self) -> String {
        serde_json::to_string(&self.dag.selected_node_ids()).unwrap_or_else(|_| "[]".into())
    }

    pub fn hovered_node_id(&self) -> Option<String> {
        self.dag.hovered_node_id()
    }

    pub fn hovered_channel_json(&self) -> String {
        self.dag.hovered_channel_json()
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64, zoom_gesture: bool) {
        if !zoom_gesture {
            let cam = &self.dag.fixture.camera;
            let zoom = cam.zoom.max(1e-9);
            self.dag.set_camera(cam.x, cam.y - delta_y / zoom, zoom);
            return;
        }
        let (wx, wy) = dag_screen_to_world(&self.dag, sx, sy);
        let factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
        let cam = &self.dag.fixture.camera;
        let new_zoom = (cam.zoom * factor).clamp(0.05, 32.0);
        let nx = wx - (wx - cam.x) * (new_zoom / cam.zoom);
        let ny = wy - (wy - cam.y) * (new_zoom / cam.zoom);
        self.dag.set_camera(nx, ny, new_zoom);
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.dag.pick_targets_at_screen_json(sx, sy)
    }

    pub fn set_hover(&mut self, node_id: Option<&str>) {
        self.dag.set_hover(node_id);
    }

    pub fn set_hover_channel(&mut self, node_id: Option<&str>, port_id: Option<&str>) {
        self.dag.set_hover_channel(node_id, port_id);
    }

    pub fn align_selection(&mut self, mode: &str) -> Result<(), String> {
        self.dag.align_selection(mode)
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        self.dag.fixture_json()
    }
}
//#endregion 🔖GraphHost

//#region 🔖Wasm
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct GraphSessionInner {
        host: GraphHost,
        gpu: cavas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct GraphSession {
        state: Rc<RefCell<GraphSessionInner>>,
    }

    #[wasm_bindgen]
    impl GraphSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(GraphSessionInner {
                    host: GraphHost::default(),
                    gpu: cavas::gpu_session::CanvasGpuSession::default(),
                    width: 1,
                    height: 1,
                    dpr: 1.0,
                })),
            }
        }

        #[wasm_bindgen(js_name = syncFromSceneJson)]
        pub fn sync_from_scene_json(&self, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.sync_from_scene_json(json).map_err(|e| JsValue::from_str(&e))
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
                let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                    .await
                    .map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.set_viewport(lw, lh, dpr);
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
            inner.host.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.dag.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = cavas::Scene::new();
            let clear = inner.host.dag.canvas_theme.raster_clear;
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = cavas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = pointerDownScreen)]
        pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt);
        }

        #[wasm_bindgen(js_name = pointerMoveScreen)]
        pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        }

        #[wasm_bindgen(js_name = pointerUpScreen)]
        pub fn pointer_up_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub fn wheel_screen(&self, sx: f64, sy: f64, _delta_x: f64, delta_y: f64, zoom_gesture: bool) {
            self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y, zoom_gesture);
        }

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = paramOverlayPaintStateJson)]
        pub fn param_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.param_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = stepperOverlayStateJson)]
        pub fn stepper_overlay_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.stepper_overlay_state_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
        pub fn selection_union_bounds_screen_json(&self) -> String {
            self.state.borrow().host.dag.selection_union_bounds_screen_json()
        }

        #[wasm_bindgen(js_name = selectionPreviewPointsJson)]
        pub fn selection_preview_points_json(&self) -> String {
            self.state.borrow().host.dag.selection_preview_points_json()
        }

        #[wasm_bindgen(js_name = selectionPreviewCrossing)]
        pub fn selection_preview_crossing(&self) -> bool {
            self.state.borrow().host.dag.selection_preview_crossing()
        }

        #[wasm_bindgen(js_name = selectedNodeIdsJson)]
        pub fn selected_node_ids_json(&self) -> String {
            self.state.borrow().host.selected_node_ids_json()
        }

        #[wasm_bindgen(js_name = hoveredNodeId)]
        pub fn hovered_node_id(&self) -> Option<String> {
            self.state.borrow().host.hovered_node_id()
        }

        #[wasm_bindgen(js_name = hoveredChannelJson)]
        pub fn hovered_channel_json(&self) -> String {
            self.state.borrow().host.hovered_channel_json()
        }

        #[wasm_bindgen(js_name = cameraJson)]
        pub fn camera_json(&self) -> String {
            self.state.borrow().host.camera_json()
        }

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            dag::dag_lod_scale_json()
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.dag.draw_lod_label().to_string()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.dag.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.dag.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = setGhostNodeJson)]
        pub fn set_ghost_node_json(&self, json: &str) {
            if json.trim().is_empty() {
                self.state.borrow_mut().host.dag.set_ghost_node(None);
                return;
            }
            if let Ok(node) = serde_json::from_str::<DagNodeSpec>(json) {
                self.state.borrow_mut().host.dag.set_ghost_node(Some(node));
            }
        }

        #[wasm_bindgen(js_name = clearGhostNode)]
        pub fn clear_ghost_node(&self) {
            self.state.borrow_mut().host.dag.set_ghost_node(None);
        }

        #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
        pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)
        }

        #[wasm_bindgen(js_name = setHover)]
        pub fn set_hover(&self, widget_id: Option<String>) {
            self.state.borrow_mut().host.set_hover(widget_id.as_deref());
        }

        #[wasm_bindgen(js_name = setHoverChannel)]
        pub fn set_hover_channel(&self, widget_id: Option<String>, port: Option<String>) {
            self.state
                .borrow_mut()
                .host
                .set_hover_channel(widget_id.as_deref(), port.as_deref());
        }

        #[wasm_bindgen(js_name = alignSelection)]
        pub fn align_selection(&self, mode: &str) -> Result<(), JsValue> {
            self.state
                .borrow_mut()
                .host
                .align_selection(mode)
                .map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state
                .borrow()
                .host
                .fixture_json()
                .map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
        pub fn take_pending_open_instance_id(&self) -> Option<String> {
            dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host.dag)
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
            let (wx, wy) = dag_screen_to_world(&self.state.borrow().host.dag, x, y);
            let out = js_sys::Array::new();
            out.push(&JsValue::from_f64(wx));
            out.push(&JsValue::from_f64(wy));
            out
        }

        #[wasm_bindgen(js_name = worldFromScreen)]
        pub fn world_from_screen(&self, x: f64, y: f64) -> js_sys::Array {
            self.screen_to_world(x, y)
        }

        #[wasm_bindgen(js_name = selectAll)]
        pub fn select_all(&self) {
            self.state.borrow_mut().host.dag.select_all();
        }

        #[wasm_bindgen(js_name = deleteSelection)]
        pub fn delete_selection(&self) {
            self.state.borrow_mut().host.dag.delete_selected();
        }

        #[wasm_bindgen(js_name = cancelAreaSelect)]
        pub fn cancel_area_select(&self) {
            self.state.borrow_mut().host.dag.cancel_area_select();
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
            let opts = if options_json.trim().is_empty() {
                DagLayoutOptions::default()
            } else {
                serde_json::from_str(options_json).unwrap_or_default()
            };
            self.state.borrow_mut().host.dag.reorganize(&opts).map_err(|e| JsValue::from_str(&e))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::GraphSession;
//#endregion 🔖Wasm

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_from_media_graph_json() {
        let nodes = r#"[{"id":"a","label":"Alpha","x":10,"y":20,"inputs":[],"outputs":[{"id":"out","label":"Out"}]}]"#;
        let edges = r#"[]"#;
        let fixture = fixture_from_node_graph_json(nodes, edges, r#"{"x":0,"y":0,"zoom":1}"#).expect("fixture");
        assert_eq!(fixture.nodes.len(), 1);
        assert_eq!(fixture.nodes[0].id, "a");
    }

    #[test]
    fn graph_host_syncs_selection() {
        let mut host = GraphHost::default();
        let payload = NodeGraphScenePayload {
            nodes_json: r#"[{"id":"a","label":"A","outputs":[{"id":"out"}]}]"#.into(),
            edges_json: "[]".into(),
            viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
            selection_json: Some(r#"["a"]"#.into()),
            ..Default::default()
        };
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.dag.selected_node_ids(), vec!["a"]);
    }
}
//#endregion 🔖Tests
