//! 🕸️ Layout play app — the WASM bridge: a stateful `LayoutSession` object the JS shell drives directly
//! for GPU canvas rendering, pointer/camera interaction and export, independent of the `ArtifactApp`
//! dispatch surface (this is host-canvas plumbing, not a document command).

#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::Arc;

    use infinite_canvas::camera::{self, Camera, Viewport};
    use infinite_canvas::Point;
    use js_sys::Promise;
    use semio_framework_async::browser::future_to_promise;
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    use crate::artifacts::layout::schema::parse_layout_document;
    use crate::artifacts::layout::LayoutSnapshot;
    use crate::editor::layout::engine::export::{output_name, LayoutExportJob, LayoutExportKind, LayoutExportRequest, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES};
    use crate::editor::layout::engine::scene::{build_scene_from_document_json, hit_test_document_json, screen_to_world_json, LayoutDropPreview, LayoutEngine, SceneQuery};
    use semio_framework_job::{BatchDriveConfig, BatchJobParams, Generation, InteractiveStage, Operation, RevisionId, StepOutcome, WorkerJobCloseStep, WorkerJobPoll, WorkerJobSession, WorkerJobSessionAdmissionRejected, JOB_PAYLOAD_PAGE_BYTES};
    use semio_framework_plugin::app::ArtifactOutputChunks;

    #[derive(Clone, Debug)]
    enum LayoutInteraction {
        None,
        Pan { origin: Camera, start_screen: Point },
    }

    struct LayoutSessionInner {
        document_json: String,
        snapshot: Arc<LayoutSnapshot>,
        generation: u64,
        page_id: String,
        selected_ids: Vec<String>,
        hovered_id: Option<String>,
        chrome_blueprint: bool,
        camera: Camera,
        viewport: Viewport,
        interaction: LayoutInteraction,
        drop_preview: Option<LayoutDropPreview>,
        layout_engine: LayoutEngine,
        gpu: infinite_canvas::gpu_session::CanvasGpuSession,
    }

    const LAYOUT_EXPORT_REJECTION_SLOTS: usize = 8;

    enum LayoutExportRejectionSlot {
        Empty,
        Reserved,
        ClosingRejected { rejected: WorkerJobSessionAdmissionRejected<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> },
        ClosingSession { session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> },
    }

    fn pump_layout_export_rejections(registry: &RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>) {
        for slot in registry.borrow_mut().iter_mut() {
            match slot {
                LayoutExportRejectionSlot::ClosingRejected { rejected, .. } => {
                    rejected.begin_close();
                    let _ = rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
                    if rejected.terminal_is_empty() {
                        *slot = LayoutExportRejectionSlot::Empty;
                    }
                }
                LayoutExportRejectionSlot::ClosingSession { session, .. } => {
                    let _ = session.begin_close();
                    let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
                    if session.terminal_is_empty() {
                        *slot = LayoutExportRejectionSlot::Empty;
                    }
                }
                LayoutExportRejectionSlot::Empty | LayoutExportRejectionSlot::Reserved => continue,
            }
            break;
        }
    }

    fn reserve_layout_export_rejection(registry: &RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>) -> Option<usize> {
        pump_layout_export_rejections(registry);
        let mut registry = registry.borrow_mut();
        let index = registry.iter().position(|slot| matches!(slot, LayoutExportRejectionSlot::Empty))?;
        registry[index] = LayoutExportRejectionSlot::Reserved;
        Some(index)
    }

    fn release_layout_export_rejection(registry: &RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>, index: usize) {
        registry.borrow_mut()[index] = LayoutExportRejectionSlot::Empty;
    }

    fn retain_layout_export_rejection(registry: &RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>, index: usize, rejected: WorkerJobSessionAdmissionRejected<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot>) {
        let mut registry = registry.borrow_mut();
        debug_assert!(matches!(registry[index], LayoutExportRejectionSlot::Reserved));
        registry[index] = LayoutExportRejectionSlot::ClosingRejected { rejected, snapshot_owner };
    }

    fn retain_layout_export_session(registry: &RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>, index: usize, session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot>) {
        let mut registry = registry.borrow_mut();
        debug_assert!(matches!(registry[index], LayoutExportRejectionSlot::Reserved));
        registry[index] = LayoutExportRejectionSlot::ClosingSession { session, snapshot_owner };
    }

    #[wasm_bindgen]
    pub struct LayoutSession {
        state: Rc<RefCell<LayoutSessionInner>>,
        export_rejections: Rc<RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>>,
    }

    #[wasm_bindgen]
    impl LayoutSession {
        #[wasm_bindgen(constructor)]
        pub async fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(LayoutSessionInner {
                    document_json: String::new(),
                    snapshot: Arc::new(crate::artifacts::layout::schema::default_document()),
                    generation: 0,
                    page_id: "page-1".into(),
                    selected_ids: Vec::new(),
                    hovered_id: None,
                    chrome_blueprint: true,
                    camera: Camera::default(),
                    viewport: Viewport::default(),
                    interaction: LayoutInteraction::None,
                    drop_preview: None,
                    layout_engine: LayoutEngine::new(),
                    gpu: infinite_canvas::gpu_session::CanvasGpuSession::default(),
                })),
                export_rejections: Rc::new(RefCell::new(std::array::from_fn(|_| LayoutExportRejectionSlot::Empty))),
            }
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub async fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub async fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
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
                let (render_ctx, renderer, surface) = infinite_canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                if g.gpu.gpu_ready() {
                    return Err(JsValue::from_str("canvas surface already attached"));
                }
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                g.viewport.set_size(lw, lh, dpr);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = setSize)]
        pub async fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let lw = width.max(1);
            let lh = height.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            let mut inner = self.state.borrow_mut();
            inner.viewport.set_size(lw, lh, dpr);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCamera)]
        pub async fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
            let mut inner = self.state.borrow_mut();
            inner.camera.x = x;
            inner.camera.y = y;
            inner.camera.zoom = camera::clamp_zoom(zoom);
        }

        #[wasm_bindgen(js_name = setDocumentJson)]
        pub async fn set_artifact_json(&mut self, json: &str) -> Result<(), JsValue> {
            let snapshot = parse_layout_document(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut inner = self.state.borrow_mut();
            inner.document_json = json.to_string();
            inner.snapshot = Arc::new(snapshot);
            inner.generation = inner.generation.saturating_add(1);
            Ok(())
        }

        #[wasm_bindgen(js_name = setPageId)]
        pub async fn set_page_id(&mut self, page_id: &str) {
            self.state.borrow_mut().page_id = page_id.to_string();
        }

        #[wasm_bindgen(js_name = setSelectedIdsJson)]
        pub async fn set_selected_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
            let ids: Vec<String> = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().selected_ids = ids;
            Ok(())
        }

        #[wasm_bindgen(js_name = setHoveredId)]
        pub async fn set_hovered_id(&mut self, hovered_id: Option<String>) {
            self.state.borrow_mut().hovered_id = hovered_id;
        }

        #[wasm_bindgen(js_name = setChromeMode)]
        pub async fn set_chrome_mode(&mut self, blueprint: bool) {
            self.state.borrow_mut().chrome_blueprint = blueprint;
        }

        #[wasm_bindgen(js_name = setDropPreview)]
        pub async fn set_drop_preview(&mut self, kind: &str, x: f64, y: f64) {
            self.state.borrow_mut().drop_preview = Some(LayoutDropPreview { kind: kind.to_string(), x, y });
        }

        #[wasm_bindgen(js_name = clearDropPreview)]
        pub async fn clear_drop_preview(&mut self) {
            self.state.borrow_mut().drop_preview = None;
        }

        #[wasm_bindgen(js_name = pointerDownScreen)]
        pub async fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
            if button != 1 {
                return;
            }
            let mut inner = self.state.borrow_mut();
            inner.interaction = LayoutInteraction::Pan { origin: inner.camera.clone(), start_screen: Point::new(sx, sy) };
        }

        #[wasm_bindgen(js_name = pointerMoveScreen)]
        pub async fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
            let mut inner = self.state.borrow_mut();
            let LayoutInteraction::Pan { origin, start_screen } = inner.interaction.clone() else {
                return;
            };
            let delta = Point::new(sx, sy) - start_screen;
            inner.camera.x = origin.x - delta.x / origin.zoom;
            inner.camera.y = origin.y - delta.y / origin.zoom;
            inner.interaction = LayoutInteraction::Pan { origin, start_screen };
        }

        #[wasm_bindgen(js_name = pointerUpScreen)]
        pub async fn pointer_up_screen(&mut self, _sx: f64, _sy: f64) {
            self.state.borrow_mut().interaction = LayoutInteraction::None;
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub async fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
            let mut inner = self.state.borrow_mut();
            let viewport = inner.viewport.clone();
            camera::wheel_screen(&mut inner.camera, &viewport, sx, sy, delta_y);
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub async fn screen_to_world(&self, sx: f64, sy: f64) -> String {
            let inner = self.state.borrow();
            screen_to_world_json(&inner.camera, &inner.viewport, sx, sy)
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub async fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            // 🩹️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining Part A: `SceneQuery`
            // borrows several `inner` fields alongside a `&mut inner.layout_engine`/`&inner.document_json`
            // pair in the same call -- pre-existing E0502 (rustc cannot split a borrow through
            // `RefMut::deref_mut` this way even though the fields are disjoint). Cloning the query
            // inputs into owned locals first removes the aliasing entirely.
            let page_id = inner.page_id.clone();
            let selected_ids = inner.selected_ids.clone();
            let hovered_id = inner.hovered_id.clone();
            let chrome_blueprint = inner.chrome_blueprint;
            let camera = inner.camera.clone();
            let viewport = inner.viewport.clone();
            let drop_preview = inner.drop_preview.clone();
            let document_json = inner.document_json.clone();
            let query = SceneQuery { page_id: &page_id, selected_ids: &selected_ids, hovered_id: hovered_id.as_deref(), chrome_blueprint, camera: &camera, viewport: &viewport };
            let scene = build_scene_from_document_json(&mut inner.layout_engine, &document_json, &query, drop_preview.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let clear = infinite_canvas::theme::default_raster_clear();
            inner.gpu.render_frame(&scene, clear).map_err(|e| e)
        }

        #[wasm_bindgen(js_name = hitTest)]
        pub async fn hit_test(&self, sx: f32, sy: f32) -> Result<JsValue, JsValue> {
            let mut inner = self.state.borrow_mut();
            // 🩹️ same pre-existing E0502 class as `render_frame` above -- owned clones, not a
            // behavior change.
            let page_id = inner.page_id.clone();
            let selected_ids = inner.selected_ids.clone();
            let hovered_id = inner.hovered_id.clone();
            let camera = inner.camera.clone();
            let viewport = inner.viewport.clone();
            let document_json = inner.document_json.clone();
            let query = SceneQuery { page_id: &page_id, selected_ids: &selected_ids, hovered_id: hovered_id.as_deref(), chrome_blueprint: true, camera: &camera, viewport: &viewport };
            let hit = hit_test_document_json(&mut inner.layout_engine, &document_json, sx as f64, sy as f64, &query).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(hit.map(|id| JsValue::from_str(&id)).unwrap_or(JsValue::NULL))
        }

        #[wasm_bindgen(js_name = exportPng)]
        pub async fn export_png(&self, page_id: String) -> Result<LayoutExportOperation, JsValue> {
            self.submit_export(LayoutExportKind::Png, Some(page_id), None)
        }

        #[wasm_bindgen(js_name = exportSvg)]
        pub async fn export_svg(&self, page_id: String) -> Result<LayoutExportOperation, JsValue> {
            self.submit_export(LayoutExportKind::Svg, Some(page_id), None)
        }

        #[wasm_bindgen(js_name = exportPdf)]
        pub async fn export_pdf(&self, page_id: String) -> Result<LayoutExportOperation, JsValue> {
            self.submit_export(LayoutExportKind::Pdf, Some(page_id), None)
        }

        #[wasm_bindgen(js_name = exportPackage)]
        pub async fn export_package(&self, preflight_json: String) -> Result<LayoutExportOperation, JsValue> {
            self.submit_export(LayoutExportKind::Package, None, Some(preflight_json))
        }

        fn submit_export(&self, kind: LayoutExportKind, page_id: Option<String>, preflight_json: Option<String>) -> Result<LayoutExportOperation, JsValue> {
            if preflight_json.as_ref().is_some_and(|value| value.len() > MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES) {
                return Err(JsValue::from_str("layout-export-preflight-byte-limit"));
            }
            let inner = self.state.borrow();
            let generation = inner.generation;
            let snapshot_owner = Arc::clone(&inner.snapshot);
            drop(inner);
            let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(generation), Generation(generation), 0);
            let request = LayoutExportRequest { kind, page_id, snapshot: Arc::clone(&snapshot_owner), preflight_json, parent_document_id: "layout.wasm.session".into(), canonical_base_revision_hex: format!("{:064x}", generation) };
            let name = output_name(&request);
            let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
            let job = LayoutExportJob::new(operation, request).map_err(|error| JsValue::from_str(&error))?.with_output_chunks(output_chunks.clone());
            let cancel = semio_framework_job::root_cancel_token();
            let params = BatchJobParams {
                operation: operation.operation,
                generation: operation.generation,
                cancel: cancel.clone(),
                config: BatchDriveConfig { site: "layout.export.wasm", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_ms: 1 },
                now_ms: semio_framework_job::default_now_ms,
            };
            let retirement_slot = reserve_layout_export_rejection(&self.export_rejections).ok_or_else(|| JsValue::from_str("layout-export-rejection-registry-full"))?;
            let admission = match WorkerJobSession::try_new(job, params) {
                Ok(session) => LayoutExportAdmission::Session(session),
                Err(rejected) => LayoutExportAdmission::Rejected(rejected),
            };
            Ok(LayoutExportOperation {
                admission,
                export_rejections: Rc::clone(&self.export_rejections),
                retirement_slot: Some(retirement_slot),
                snapshot_owner: Some(snapshot_owner),
                cancel,
                authority: self.state.clone(),
                submitted_generation: generation,
                kind,
                name,
                status: RefCell::new("submitted".into()),
                output_chunks,
                completed: Cell::new(false),
            })
        }
    }

    enum LayoutExportAdmission {
        Session(WorkerJobSession<LayoutExportJob>),
        Rejected(WorkerJobSessionAdmissionRejected<LayoutExportJob>),
        Empty,
    }

    #[wasm_bindgen]
    pub struct LayoutExportOperation {
        admission: LayoutExportAdmission,
        export_rejections: Rc<RefCell<[LayoutExportRejectionSlot; LAYOUT_EXPORT_REJECTION_SLOTS]>>,
        retirement_slot: Option<usize>,
        snapshot_owner: Option<Arc<LayoutSnapshot>>,
        cancel: semio_framework_job::CancelToken,
        authority: Rc<RefCell<LayoutSessionInner>>,
        submitted_generation: u64,
        kind: LayoutExportKind,
        name: String,
        status: RefCell<String>,
        output_chunks: ArtifactOutputChunks,
        completed: Cell<bool>,
    }

    impl Drop for LayoutExportOperation {
        fn drop(&mut self) {
            if !self.completed.get() {
                self.cancel.cancel_now();
            }
            match std::mem::replace(&mut self.admission, LayoutExportAdmission::Empty) {
                LayoutExportAdmission::Rejected(rejected) => {
                    retain_layout_export_rejection(
                        &self.export_rejections,
                        self.retirement_slot.take().expect("rejected layout operation owns mounted close slot"),
                        rejected,
                        self.snapshot_owner.take().expect("rejected layout operation owns snapshot handback"),
                    );
                }
                LayoutExportAdmission::Session(session) => {
                    retain_layout_export_session(
                        &self.export_rejections,
                        self.retirement_slot.take().expect("live layout operation owns mounted close slot"),
                        session,
                        self.snapshot_owner.take().expect("live layout operation owns snapshot handback"),
                    );
                }
                LayoutExportAdmission::Empty => {}
            }
        }
    }

    #[wasm_bindgen]
    impl LayoutExportOperation {
        #[wasm_bindgen(js_name = step)]
        pub async fn step(&mut self) -> Result<String, JsValue> {
            pump_layout_export_rejections(&self.export_rejections);
            if self.completed.get() {
                return Ok("completed".into());
            }
            if self.authority.borrow().generation != self.submitted_generation {
                self.cancel.cancel_now();
                *self.status.borrow_mut() = "stale".into();
                return Err(JsValue::from_str("layout-export-stale-generation"));
            }
            if let LayoutExportAdmission::Rejected(rejected) = &mut self.admission {
                rejected.begin_close();
                let step = rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
                if rejected.terminal_is_empty() {
                    self.admission = LayoutExportAdmission::Empty;
                    release_layout_export_rejection(&self.export_rejections, self.retirement_slot.take().expect("closed rejection releases mounted slot"));
                    self.snapshot_owner = None;
                }
                *self.status.borrow_mut() = "admission-rejected-closing".into();
                return match step {
                    semio_framework_job::InteractiveJobCloseStep::Complete => Err(JsValue::from_str("layout-export-admission-rejected")),
                    _ => Ok("admission-rejected-closing".into()),
                };
            }
            let LayoutExportAdmission::Session(session) = &self.admission else {
                return Err(JsValue::from_str("layout-export-terminal-empty"));
            };
            if matches!(session.poll(), WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty) {
                let close = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
                if matches!(close, WorkerJobCloseStep::Complete) {
                    self.admission = LayoutExportAdmission::Empty;
                    release_layout_export_rejection(&self.export_rejections, self.retirement_slot.take().expect("closed layout session releases mounted slot"));
                    self.snapshot_owner = None;
                }
                return Ok(self.status.borrow().clone());
            }
            let (ticket, poll) = session.try_step_on_caller().map_err(|_| JsValue::from_str("layout-export-session-contended"))?;
            let mut owner = match poll {
                WorkerJobPoll::Outcome => session.take_outcome(ticket),
                WorkerJobPoll::Terminal => session.take_terminal(),
                _ => return Err(JsValue::from_str("layout-export-invalid-poll")),
            }
            .map_err(|_| JsValue::from_str("layout-export-outcome-unavailable"))?;
            let mut outcome = owner.take_outcome();
            let status = match &outcome {
                StepOutcome::Yield => "running".into(),
                StepOutcome::PreviewReady(preview) => preview.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("preview").to_owned(),
                StepOutcome::CheckpointReady(checkpoint) => format!("checkpoint:{}", checkpoint.applied_progress),
                StepOutcome::Complete(_) => {
                    self.completed.set(true);
                    "completed".into()
                }
                StepOutcome::Cancelled => "cancelled".into(),
                StepOutcome::Fault(fault) => fault.detail.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("layout-export-fault").to_owned(),
            };
            let terminal = outcome.is_terminal();
            let _ = outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            if !outcome.terminal_is_empty() {
                owner.begin_close();
                return Err(JsValue::from_str("layout-export-outcome-close-pending"));
            }
            if terminal {
                owner.begin_close();
            } else {
                owner.resume().map_err(|owner| {
                    owner.begin_close();
                    JsValue::from_str("layout-export-resume-rejected")
                })?;
            }
            *self.status.borrow_mut() = status.clone();
            if matches!(outcome, StepOutcome::Fault(_)) {
                Err(JsValue::from_str(&status))
            } else {
                Ok(status)
            }
        }

        #[wasm_bindgen(js_name = progress)]
        pub fn progress(&self) -> String {
            self.status.borrow().clone()
        }

        #[wasm_bindgen(js_name = cancel)]
        pub fn cancel(&self) {
            self.cancel.cancel_now();
        }

        #[wasm_bindgen(js_name = resultFilename)]
        pub fn result_filename(&self) -> Option<String> {
            self.completed.get().then(|| format!("{}.{}", self.name, self.kind.extension()))
        }

        #[wasm_bindgen(js_name = resultMimeType)]
        pub fn result_mime_type(&self) -> Option<String> {
            self.completed.get().then(|| self.kind.mime_type().to_string())
        }

        #[wasm_bindgen(js_name = resultEncoding)]
        pub fn result_encoding(&self) -> Option<String> {
            (self.completed.get() && self.kind.binary()).then(|| "base64".into())
        }

        #[wasm_bindgen(js_name = takeResultChunk)]
        pub fn take_result_chunk(&self) -> Result<Option<Vec<u8>>, JsValue> {
            if !self.completed.get() {
                return Ok(None);
            }
            self.output_chunks.take_chunk().map_err(|error| JsValue::from_str(&error.to_string()))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::{LayoutExportOperation, LayoutSession};
