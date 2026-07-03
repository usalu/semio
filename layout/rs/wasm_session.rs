use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlCanvasElement;

use crate::document::parse_layout_document;
use crate::engine::{build_scene_from_document_json, hit_test_document_json};
use crate::export::{export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip};

struct LayoutSessionInner {
    document_json: String,
    page_id: String,
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    chrome_blueprint: bool,
    gpu: infinite_cavas::gpu_session::CanvasGpuSession,
}

#[wasm_bindgen]
pub struct LayoutSession {
    state: Rc<RefCell<LayoutSessionInner>>,
}

#[wasm_bindgen]
impl LayoutSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(LayoutSessionInner {
                document_json: String::new(),
                page_id: "page-1".into(),
                selected_ids: Vec::new(),
                hovered_id: None,
                chrome_blueprint: true,
                gpu: infinite_cavas::gpu_session::CanvasGpuSession::default(),
            })),
        }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
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
            let (render_ctx, renderer, surface) = infinite_cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                .await
                .map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let mut inner = self.state.borrow_mut();
        inner.gpu.resize_surface(pw, ph);
    }

    #[wasm_bindgen(js_name = setDocumentJson)]
    pub fn set_document_json(&mut self, json: &str) -> Result<(), JsValue> {
        parse_layout_document(json).map_err(|e| JsValue::from_str(&e))?;
        self.state.borrow_mut().document_json = json.to_string();
        Ok(())
    }

    #[wasm_bindgen(js_name = setPageId)]
    pub fn set_page_id(&mut self, page_id: &str) {
        self.state.borrow_mut().page_id = page_id.to_string();
    }

    #[wasm_bindgen(js_name = setSelectedIdsJson)]
    pub fn set_selected_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().selected_ids = ids;
        Ok(())
    }

    #[wasm_bindgen(js_name = setHoveredId)]
    pub fn set_hovered_id(&mut self, hovered_id: Option<String>) {
        self.state.borrow_mut().hovered_id = hovered_id;
    }

    #[wasm_bindgen(js_name = setChromeMode)]
    pub fn set_chrome_mode(&mut self, blueprint: bool) {
        self.state.borrow_mut().chrome_blueprint = blueprint;
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&self) -> Result<(), JsValue> {
        let mut inner = self.state.borrow_mut();
        let hovered = inner.hovered_id.as_deref();
        let scene = build_scene_from_document_json(&inner.document_json, &inner.page_id, &inner.selected_ids, hovered, inner.chrome_blueprint)
            .map_err(|e| JsValue::from_str(&e))?;
        let clear = vello::peniko::Color::new([0.12, 0.13, 0.15, 1.0]);
        inner.gpu.render_frame(&scene, clear).map_err(|e| e)
    }

    #[wasm_bindgen(js_name = hitTest)]
    pub fn hit_test(&self, x: f32, y: f32) -> Result<JsValue, JsValue> {
        let inner = self.state.borrow();
        let hovered = inner.hovered_id.as_deref();
        let hit = hit_test_document_json(&inner.document_json, &inner.page_id, x, y, &inner.selected_ids, hovered).map_err(|e| JsValue::from_str(&e))?;
        Ok(hit.map(|id| JsValue::from_str(&id)).unwrap_or(JsValue::NULL))
    }

    #[wasm_bindgen(js_name = exportPng)]
    pub fn export_png(&self, page_id: &str) -> Result<Vec<u8>, JsValue> {
        let inner = self.state.borrow();
        let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e))?;
        export_document_png_cpu(&doc, page_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = exportSvg)]
    pub fn export_svg(&self, page_id: &str) -> Result<String, JsValue> {
        let inner = self.state.borrow();
        let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e))?;
        export_document_svg(&doc, page_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = exportPdf)]
    pub fn export_pdf(&self, page_id: &str) -> Result<Vec<u8>, JsValue> {
        let inner = self.state.borrow();
        let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e))?;
        export_document_pdf(&doc, page_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = exportPackage)]
    pub fn export_package(&self, preflight_json: &str) -> Result<Vec<u8>, JsValue> {
        let inner = self.state.borrow();
        export_package_zip(&inner.document_json, preflight_json).map_err(|e| JsValue::from_str(&e))
    }
}
