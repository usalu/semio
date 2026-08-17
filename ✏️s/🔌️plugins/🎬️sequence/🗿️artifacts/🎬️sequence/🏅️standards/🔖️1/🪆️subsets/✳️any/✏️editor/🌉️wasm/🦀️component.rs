//! 🌉️ Sequence play app — direct-canvas WASM bridge: a bespoke JS API (`SequenceSession`) for
//! driving the sequence DAG canvas outside the generic `ArtifactApp`/`PluginApp` render pipeline
//! (raw GPU frame painting, pointer/wheel routing). Only compiled for `target_arch = "wasm32"` (see
//! the `#[cfg(target_arch = "wasm32")]` on this file's `mod wasm;` declaration in `📦️glue.rs`).

use crate::editor::sequence::{sequence_camera_from_dag, SequenceHost};
use crate::artifacts::sequence::{SequenceFixture, SlotRef};
use infinite_board_port_directed_dag::DagLayoutOptions;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

//#region 🔖️SequenceSession
struct SequenceSessionInner {
    host: SequenceHost,
    gpu: infinite_canvas::gpu_session::CanvasGpuSession,
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
                gpu: infinite_canvas::gpu_session::CanvasGpuSession::default(),
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
        self.state.borrow_mut().host.replace_snapshot(fixture).map_err(|err| JsValue::from_str(&err.to_string()))
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
    pub fn attach_canvas(&mut self, canvas: web_sys::HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = infinite_canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
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
        inner.host.camera = sequence_camera_from_dag(&inner.host.dag.fixture.camera);
        let mut scene = infinite_canvas::Scene::new();
        let clear = inner.host.dag.canvas_theme.raster_clear;
        inner.host.dag.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
        let scene = infinite_canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
        inner.gpu.render_frame(&scene, clear)
    }

    #[wasm_bindgen(js_name = worldFromScreen)]
    pub fn world_from_screen(&self, sx: f64, sy: f64) -> Result<String, JsValue> {
        use infinite_canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use infinite_canvas::Point;
        let inner = self.state.borrow();
        let viewport = Viewport { width: inner.width.max(1), height: inner.height.max(1), dpr: inner.dpr.max(1.0) };
        let camera = CanvasCamera { x: inner.host.dag.fixture.camera.x, y: inner.host.dag.fixture.camera.y, zoom: inner.host.dag.fixture.camera.zoom };
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
        use infinite_canvas::camera::{wheel_screen, Camera as CanvasCamera, Viewport};
        let mut inner = self.state.borrow_mut();
        inner.host.dag.set_wheel_zoom_active(true);
        let viewport = Viewport { width: inner.width.max(1), height: inner.height.max(1), dpr: inner.dpr.max(1.0) };
        let mut camera = CanvasCamera { x: inner.host.dag.fixture.camera.x, y: inner.host.dag.fixture.camera.y, zoom: inner.host.dag.fixture.camera.zoom };
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
        infinite_board_port_directed_dag::dag_lod_scale_json()
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
//#endregion 🔖️SequenceSession
