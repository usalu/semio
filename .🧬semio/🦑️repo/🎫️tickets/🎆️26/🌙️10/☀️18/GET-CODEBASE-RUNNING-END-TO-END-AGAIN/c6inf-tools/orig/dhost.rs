//! 🧭️ Dag board host: the composed `DagHost` owner with pointer, paint, minimap and note orchestration.
//!
//! Crate `semio-framework-os-infinite-board-dag-host`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{BoardPaletteOverlay, channel, DagChannelView, DagInputFacts, DagNodeEvaluationStatus, DagNodeStatuses, DagOutputCandidates, DagRefusalView, DagScreenHit, DagScreenHitJson, DagSelectionDomain, DagSelectionSource, decode_board_palette_overlay_json, decode_dag_options_json, field, LayoutControl, LayoutProgress, Selection};
use semio_framework_os_infinite_board_engine::{Camera, CanvasPalette, GRID_FACTOR_DEFAULT, IconPaintCache, InteractionMode};
use semio_framework_os_infinite_board_normal_model::handle_port_shape;
use semio_framework_os_infinite_board_dag_model::{action_control_bounds, canvas_color_with_alpha, dag_bit_contains, dag_bit_set, DAG_BOUNDED_DRAG_HIT_PAD_PX, DAG_CHROME_STROKE_SCREEN_PX, dag_edge_body_stroke, dag_handle_body_fill, dag_handle_body_stroke, DAG_HANDLE_WORLD_RADIUS, DAG_INTERACTION_NODE_CAPACITY, DAG_INTERACTION_WORD_CAPACITY, DAG_LABEL_COMPACT_SCREEN_PX, dag_label_layout_px, dag_label_paint_px, dag_lod_index, dag_lod_scale_json, dag_node_body_stroke, dag_node_internal_chrome_stroke, dag_node_label_fill, dag_node_paint_fill, dag_node_stroke_screen_px, DAG_PREVIEW_PAD, DAG_PREVIEW_ROW_HEIGHT, DAG_PREVIEW_TOGGLE_WIDTH, DAG_PREVIEW_TREE_INDENT, DAG_VARIADIC_PLUS_ZOOM_THRESHOLD, dag_world_stroke, DagBoardEngine, DagDragOffset, DagError, DagGraphEdit, DagInputDisplaced, DagInteractionPlanFault, DagInteractionProjection, DagNodeEvalStatusKind, DagNodeMove, DagNodePaintChrome, DagPointerIntent, DagPointerPhase, DagPointerPlan, DagPortSide, DagProjectionGesture, handle_cap_fill_path, handle_cap_stroke_path, hit_byte_in_note_line, Identity, IdentityStep, label_overlay_rows_for_node, minimap_widget_point_in_rect, minimap_widget_screen_to_world, MinimapWidgetLayout, node_caption_delegated_to_js_overlay, node_label_text, note_text_origin_x, NoteEditState, paint_cluster_affordances, paint_computation_column_divider, paint_computation_node_name, paint_computing_border_arc, paint_eval_status_border, paint_io_widget_channel_borders, paint_io_widget_name, paint_node_name_horizontal, paint_note_caret_bar, paint_slider_name, paint_variadic_plus_controls, pointer_event_now_ms, preview_content_bounds, preview_tree_rows, select_control_bounds, set_slider_value_from_x, should_paint_node_lod_icon, toggle_preview_tree_path, truncate_label_to_fit_width, WidgetPointerKind};
use semio_framework_os_infinite_board_dag_graph::{connection_hit_world, connection_hit_world_while_wiring, dag_draw_lod_label, dag_node_overlays_json, dag_screen_to_world, dag_set_wheel_zoom_active, DagGraphState, DagViewportState, draw_lod_for_frame, entity_screen_json, entity_screen_rect, IdentityCursor, lod_uses_bounded_drag, port_pointer_handle_hit, process_engine_events, screen_to_world_point, sync_channel_row_pointer_hover, sync_connection_hit_picking_for_lod, sync_minimap_pointer_hover, sync_node_positions_from_engine, world_hits_handle, world_to_screen_point};
use semio_framework_canvas::{Affine, Circle, FillRule, Rect, Scene, Stroke};
use std::cell::{Cell, RefCell};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use graph::{HandleId, NodeId};
use semio_framework_os_kernel::os_spr::causal::transition::{HistoryFoldIndex, HistoryFoldSet};
use web_sys::HtmlCanvasElement;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use semio_framework_value::{NativeDecodeControl, ValueError};
use semio_framework_value::numeric_scratch::{NumericIndex, NumericInsertCursor, NumericSet};
use geometry::Point;
use std::rc::Rc;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use ::{graph, semio_framework_canvas as canvas};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
use semio_framework_graph_drawing::routing::handle_outward_at_node_rim;
use semio_framework_os_infinite_directed_presentation::*;
use wasm_bindgen::prelude::*;
use semio_framework_value::retirement::*;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct DagSessionInner {
    pub host: DagHost,
    gpu: canvas::gpu_session::CanvasGpuSession,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct DagSession {
    pub state: Rc<RefCell<DagSessionInner>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl DagSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadHostSnapshotJson)]
    pub fn load_host_snapshot_json(&self, json: &str) -> Result<(), JsValue> {
        let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host = host;
        Ok(())
    }

    #[wasm_bindgen(js_name = hostSnapshotJson)]
    pub fn host_snapshot_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.graph.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = nodeOverlaysJson)]
    pub fn node_overlays_json(&self) -> Result<String, JsValue> {
        match &self.state.borrow().host { __host => dag_node_overlays_json(&__host.viewport, &__host.graph) }.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
    pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
            let admission = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            g.width = lw;
            g.height = lh;
            g.dpr = dpr;
            g.host.viewport.set_viewport(lw, lh, dpr);
            g.gpu.finish_attach(admission);
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
        inner.host.viewport.set_viewport(w, h, d);
        let pw = ((w as f64 * d).round() as u32).max(1);
        let ph = ((h as f64 * d).round() as u32).max(1);
        inner.gpu.resize_surface(pw, ph);
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.graph.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active(&self, active: bool) {
        match &mut self.state.borrow_mut().host { __host => dag_set_wheel_zoom_active(&mut __host.viewport, &mut __host.graph, active) };
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json(&self) -> String {
        dag_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod(&self, enabled: bool) {
        self.state.borrow_mut().host.viewport.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label(&self, label: &str) {
        self.state.borrow_mut().host.viewport.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = drawLodLabel)]
    pub fn draw_lod_label(&self) -> String {
        match &self.state.borrow().host { __host => dag_draw_lod_label(&__host.viewport, &__host.graph) }.to_string()
    }

    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
        self.state.borrow_mut().host.pointer_down(x, y, extend);
    }

    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_move(x, y);
    }

    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_up(x, y);
    }

    #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
    pub fn take_pending_open_instance_id(&self) -> Option<String> {
        dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host.pointer)
    }

    #[wasm_bindgen(js_name = screenToWorld)]
    pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
        let (wx, wy) = dag_screen_to_world(&self.state.borrow().host.viewport, &self.state.borrow().host.graph, x, y);
        let out = js_sys::Array::new();
        out.push(&JsValue::from_f64(wx));
        out.push(&JsValue::from_f64(wy));
        out
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self,options_json:&str,maximum_bytes:u32,maximum_work:u32,progress:js_sys::Function)->Result<(),JsValue>{
        let report=|phase:&str,completed:u64|progress.call2(&JsValue::NULL,&JsValue::from_str(phase),&JsValue::from_f64(completed as f64)).map(|v|v.as_bool().unwrap_or(false)).unwrap_or(false);
        let mut decoding=|p:semio_framework_value::native_decoding::NativeDecodeProgress|report("decode",p.completed as u64);let mut decode=semio_framework_value::NativeDecodeControl::new(maximum_bytes as usize,&mut decoding);
        let opts=decode_dag_options_json(options_json,&mut decode).map_err(|e|JsValue::from_str(&e.to_string()))?;
        let mut working=|p:LayoutProgress|report("layout",p.completed);let mut work=LayoutControl::new(maximum_work as u64,&mut working);
        self.state.borrow_mut().host.graph.reorganize(&opts,&mut work).map_err(|e|JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let mut accepted=|_|true;
        let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
        if let Ok(overlay)=decode_board_palette_overlay_json(json,&mut control){match &mut self.state.borrow_mut().host { __host => dag_set_canvas_palette(&mut __host.viewport, &mut __host.paint, &overlay) };}
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&self) -> Result<(), JsValue> {
        let mut inner = self.state.borrow_mut();
        let mut scene = canvas::Scene::new();
        let clear = inner.host.viewport.canvas_theme.raster_clear;
        inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
        let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
        inner.gpu.render_frame(&scene, clear)
    }
}
// #endregion 🔖️WasmSession

pub struct DagInputApplication{pub facts:Option<DagInputFacts>,pub selection:Selection,pub active:Option<NodeId>,pub stale:NumericSet<NodeId>,statuses:NumericIndex<NodeId,DagNodeEvalStatusKind>,pub ports:NumericSet<(NodeId,usize)>,pub field:usize,pub row:usize,pub port:usize,pub identity:IdentityCursor,pub node:Option<NodeId>,pub phase:u8,pub channels:bool,id_insert:Option<NumericInsertCursor<u64,()>>,status_insert:Option<NumericInsertCursor<u64,DagNodeEvalStatusKind>>,pub port_insert:Option<NumericInsertCursor<(u64,usize),()>>,pub complete:bool,pub progress:semio_framework_value::retained_clone::RetainedCloneProgress}

impl DagInputApplication{
 pub fn new(facts:DagInputFacts)->Self{Self{facts:Some(facts),selection:Selection::default(),active:None,stale:NumericSet::new(),statuses:NumericIndex::new(),ports:NumericSet::new(),field:0,row:0,port:0,identity:IdentityCursor::default(),node:None,phase:0,channels:false,id_insert:None,status_insert:None,port_insert:None,complete:false,progress:Default::default()}}
 pub fn normal_step_progress(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.progress}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_capacity_byte_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_capacity_byte_demand(&self.statuses),6=>self.ports.insert_capacity_byte_demand(self.port_insert.as_ref().unwrap()),_=>Ok(0)}}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_depth_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_depth_demand(&self.statuses),6=>self.ports.insert_depth_demand(self.port_insert.as_ref().unwrap()),_=>Ok(1)}}
 pub fn step(&mut self,viewport: &DagViewportState, graph: &DagGraphState,units:usize,control:&mut NativeDecodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<bool,ValueError>{
  self.progress=Default::default();if units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}control.checkpoint()?;if self.complete{return Ok(true)}if self.next_capacity_byte_demand()?>grant.maximum_capacity_bytes||self.next_depth_demand()?>grant.maximum_depth{return Ok(false)}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;let before=control.owned_bytes();let result=self.advance(viewport, graph,control);self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;result?;control.step()?;Ok(self.complete)
 }
 fn next_row(&mut self){self.row+=1;self.phase=0;self.identity=IdentityCursor::default();self.node=None;self.port=0;}
 pub fn advance(&mut self,viewport: &DagViewportState, graph: &DagGraphState,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  let facts=self.facts.as_ref().unwrap();
  if self.phase==0{
   let present=match facts{
    DagInputFacts::Progress(facts)=>if self.field==0{if facts.active.is_none(){self.field=1;return Ok(())}true}else{self.row<facts.stale.len()},
    DagInputFacts::Statuses(facts)=>self.row<facts.len(),
    DagInputFacts::Selection(facts)=>{let rows=match self.field{0=>&facts.nodes,1=>&facts.edges,_=>&facts.handles};if self.row==rows.len(){self.field+=1;self.row=0;if self.field==3{self.complete=true;}return Ok(())}true},
    DagInputFacts::Channels(facts)=>{self.channels=draw_lod_for_frame(viewport, graph).uses_channel_row_pick();self.row<facts.len()},
   };if !present{self.complete=true;return Ok(())}self.phase=1;return Ok(())
  }
  if self.phase==1{
   let(target,domain)=match facts{
    DagInputFacts::Progress(facts)=>(Identity{first:if self.field==0{facts.active.as_deref().unwrap()}else{facts.stale[self.row].as_str()},second:None},0),
    DagInputFacts::Statuses(facts)=>(Identity{first:facts.entry_at_rank(self.row).unwrap().0,second:None},0),
    DagInputFacts::Selection(facts)=>(Identity{first:match self.field{0=>&facts.nodes[self.row],1=>&facts.edges[self.row],_=>&facts.handles[self.row]},second:None},self.field),
    DagInputFacts::Channels(facts)=>(Identity{first:&facts[self.row].widget_id,second:self.channels.then_some(facts[self.row].port.as_str())},if self.channels{2}else{0}),
   };
   match self.identity.step(graph,target,domain,None){IdentityStep::Progress=>{},IdentityStep::Missing=>{if matches!(facts,DagInputFacts::Progress(_))&&self.field==0{self.field=1;self.identity=IdentityCursor::default();self.phase=0;}else{self.next_row()}},IdentityStep::Resolved(id)=>{
    self.node=Some(id);self.identity=IdentityCursor::default();self.phase=2;
   }}return Ok(())
  }
  if self.phase==2{
   let id=self.node.unwrap();match facts{
    DagInputFacts::Progress(_)=>{if self.field==0{self.active=Some(id);self.field=1;self.phase=0;self.identity=IdentityCursor::default();return Ok(())}if self.active==Some(id){self.next_row();return Ok(())}self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
    DagInputFacts::Statuses(facts)=>{let status=facts.entry_at_rank(self.row).unwrap().1;let kind=match status{DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,DagNodeEvaluationStatus::Queued{}=>DagNodeEvalStatusKind::Queued,DagNodeEvaluationStatus::Computing{}=>{self.active=Some(id);DagNodeEvalStatusKind::Computing},DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,DagNodeEvaluationStatus::Blocked{..}=>DagNodeEvalStatusKind::Blocked};self.status_insert=Some(NumericInsertCursor::new(id,kind));self.phase=4;},
    DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>{self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
   }return Ok(())
  }
  if self.phase==3{
   let output=match facts{DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&mut self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&mut self.selection.node_ids,1=>&mut self.selection.edge_ids,_=>&mut self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&mut self.selection.handle_ids}else{&mut self.selection.node_ids}};
   if output.insert_step(self.id_insert.as_mut().unwrap(),1,control)?{self.id_insert.take();self.next_row();}return Ok(())
  }
  if self.phase==4{
   if self.status_insert.as_mut().unwrap().step(&mut self.statuses,1,control)?{self.status_insert.take();let DagInputFacts::Statuses(facts)=facts else{unreachable!()};match facts.entry_at_rank(self.row).unwrap().1{DagNodeEvaluationStatus::Queued{}=>{self.id_insert=Some(NumericInsertCursor::new(self.node.unwrap(),()));self.phase=3},DagNodeEvaluationStatus::Blocked{..}=>self.phase=5,_=>self.next_row()}}return Ok(())
  }
  if self.phase==5{
   let DagInputFacts::Statuses(facts)=facts else{unreachable!()};let DagNodeEvaluationStatus::Blocked{ports}=facts.entry_at_rank(self.row).unwrap().1 else{unreachable!()};if self.port==ports.len(){self.next_row();return Ok(())}
   match self.identity.step(graph,Identity{first:&ports[self.port],second:None},3,self.node){IdentityStep::Progress=>{},IdentityStep::Missing=>{self.port+=1;self.identity=IdentityCursor::default()},IdentityStep::Resolved(port)=>{self.port_insert=Some(NumericInsertCursor::new((self.node.unwrap(),port as usize),()));self.phase=6;}}return Ok(())
  }
  if self.ports.insert_step(self.port_insert.as_mut().unwrap(),1,control)?{self.port_insert.take();self.identity.candidate+=1;self.identity.byte=0;self.phase=5;}Ok(())
 }
 pub fn commit(&mut self,graph: &mut DagGraphState, paint: &mut DagPaintState)->DagInputDisplaced{
  assert!(self.complete);let mut displaced=DagInputDisplaced{selection:None,chrome:None,progress:None};
  match self.facts.as_ref().unwrap(){
   DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>displaced.selection=Some((std::mem::replace(&mut graph.engine.selection,std::mem::take(&mut self.selection)),std::mem::take(&mut graph.engine.preselect),std::mem::take(&mut graph.engine.preselect_removed))),
   DagInputFacts::Progress(_)=>displaced.progress=Some((std::mem::replace(&mut paint.computing_active,self.active),std::mem::replace(&mut paint.computing_stale,std::mem::take(&mut self.stale)))),
   DagInputFacts::Statuses(_)=>displaced.chrome=Some((std::mem::replace(&mut paint.computing_active,self.active),std::mem::replace(&mut paint.computing_stale,std::mem::take(&mut self.stale)),std::mem::replace(&mut paint.node_eval_status,std::mem::take(&mut self.statuses)),std::mem::replace(&mut paint.unresolved_input_ports,std::mem::take(&mut self.ports)))),
  }displaced
 }
}

semio_framework_value::artifact_retire_struct!(DagInputApplication{facts,selection,active,stale,statuses,ports,field,row,port,identity,node,phase,channels,id_insert,status_insert,port_insert,complete,progress});

// #region 🔖️DagHost
impl DagHost {
    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        if let Some((ox, oy)) = self.minimap.minimap_widget_drag {
            if let Some(layout) = minimap_widget_layout(&self.graph, &self.minimap, self.viewport.width, self.viewport.height) {
                let (wx, wy) = minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.graph.host_snapshot.camera.zoom;
                self.graph.set_camera(wx + ox, wy + oy, zoom);
                self.viewport.last_screen_x = sx;
                self.viewport.last_screen_y = sy;
                return;
            }
        }
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.viewport.pan_anchor {
            let zoom = self.graph.host_snapshot.camera.zoom.max(1e-9);
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.graph.set_camera(cam_x - dx, cam_y - dy, zoom);
            self.viewport.last_screen_x = sx;
            self.viewport.last_screen_y = sy;
            return;
        }
        let minimap_hovered = minimap_widget_pointer_hit(&self.viewport, &self.graph, &self.minimap, sx, sy).is_some();
        self.minimap.minimap_widget_hovered = minimap_hovered;
        if minimap_hovered {
            self.viewport.last_screen_x = sx;
            self.viewport.last_screen_y = sy;
            return;
        }
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        if let Some(idx) = self.pointer.widget_drag {
            set_slider_value_from_x(&mut self.graph.host_snapshot.nodes[idx], world.x);
            return;
        }
        let (hit_x, hit_y) = connection_hit_world_while_wiring(&self.viewport, &self.graph, world.x, world.y);
        self.graph.engine.pointer_move_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        sync_channel_row_pointer_hover(&mut self.viewport, &mut self.graph, world.x, world.y);
        sync_minimap_pointer_hover(&mut self.viewport, &mut self.graph, world.x, world.y);
        if matches!(self.graph.engine.interaction, InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }) {
            sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        }
        process_engine_events(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.viewport.pan_anchor = None;
        self.minimap.minimap_widget_drag = None;
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        if let Some(index) = self.pointer.widget_drag.take() {
            if let Some(DagNodeSpec { id, kind: DagNodeKind::Slider { value, .. }, .. }) = self.graph.host_snapshot.nodes.get(index) {
                let edit = DagGraphEdit::SetSlider { node_id: id.clone(), value: *value };
                self.graph.push_graph_edit(edit);
            }
            return;
        }
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        let (hit_x, hit_y) = connection_hit_world_while_wiring(&self.viewport, &self.graph, world.x, world.y);
        self.graph.engine.pointer_up_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        process_engine_events(&mut self.viewport, &mut self.graph);
        sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        self.viewport.pan_anchor = None;
        self.minimap.minimap_widget_drag = None;
        self.pointer.widget_drag = None;
        self.pointer.pending_port_insert = None;
        let cancelled = self.graph.engine.pointer_cancel_screen();
        self.graph.pending_graph_edits.clear();
        sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
        cancelled
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
        let lod = draw_lod_for_frame(&self.viewport, &self.graph);
        let cam = &self.graph.host_snapshot.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
            let node = self.graph.node_spec_for_paint(idx, snapshot_node);
            let engine_nid = self.graph.engine_node_id_for_index(idx);
            labels.extend(label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false, engine_nid, &self.paint.unresolved_input_ports));
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            labels.extend(label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true, None, &self.paint.unresolved_input_ports));
        }
        let minimap_widget = minimap_widget_json(&self.viewport, &self.graph, &self.minimap);
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("lod".to_string(), Value::from(lod.label())),
            ("width".to_string(), Value::from(self.viewport.width)),
            ("height".to_string(), Value::from(self.viewport.height)),
            ("labels".to_string(), Value::Array(labels)),
            ("minimapWidget".to_string(), Value::from(minimap_widget)),
        ])))
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use canvas::camera::{camera_content_affine, Camera as CanvasCamera, Viewport};
        use canvas::FillRule;
        use canvas::{Circle, Rect, Stroke};

        let theme = &self.viewport.canvas_theme;
        self.paint.tick_computing_animation();
        let cam = CanvasCamera { x: self.graph.host_snapshot.camera.x, y: self.graph.host_snapshot.camera.y, zoom: self.graph.host_snapshot.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = draw_lod_for_frame(&self.viewport, &self.graph);
        let lod_index = dag_lod_index(cam.zoom);
        self.viewport.paint_lod_grid(scene, &cam, &viewport, lod);
        let snap = self.graph.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for &eid in self.graph.engine.edges.keys() {
            let (is_selected, is_highlighted, is_hovered) = self.graph.edge_interaction_chrome(eid);
            let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
            let style = self.graph.edge_route_style.get(&eid).copied().unwrap_or_default();
            match style {
                EdgeRouteStyle::Bezier => {
                    if let Some(curve) = self.graph.engine.edge_curve(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
                    }
                }
                EdgeRouteStyle::SharpSz => {
                    if let Some(path) = self.graph.edge_sharp_path(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &path);
                    }
                }
            }
        }
        if let Some(preview) = snap.pending_edge {
            scene.stroke(&Stroke::new(edge_stroke), aff, dag_edge_body_stroke(theme, false, true, false, false), None, &preview);
        }
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let paint_snap_handle = |scene: &mut canvas::Scene, hid: &HandleId, center: &canvas::Point, shape_filter: Option<PortShape>| {
            if self.graph.handle_port_visible.get(hid) == Some(&false) {
                return;
            }
            let shape = self.graph.handle_port_shape.get(hid).copied().unwrap_or_default();
            if let Some(filter) = shape_filter {
                if shape != filter {
                    return;
                }
            }
            if !lod.shows_handles() && shape != PortShape::Triangle {
                return;
            }
            let node_id = self.graph.engine.handles.get(hid).map(|handle| handle.node_id);
            let is_dimmed = node_id.is_some_and(|nid| self.paint.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = self.graph.handle_interaction_chrome(*hid);
            let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
            let outward = node_id.and_then(|nid| self.graph.engine.nodes.get(&nid).and_then(|node| handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)));
            if let Some(out) = outward {
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &handle_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &handle_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
            } else {
                let circle = Circle::new(*center, DAG_HANDLE_WORLD_RADIUS);
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &circle);
            }
        };
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Semicircle));
        }
        let paint_minimap_node = |scene: &mut canvas::Scene, idx: usize, snapshot_node: &DagNodeSpec| {
            let node = self.graph.node_spec_for_paint(idx, snapshot_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.graph.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.paint.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.graph.node_interaction_chrome(nid));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(FillRule::NonZero, aff, fill, None, &rect);
            }
        };
        if lod == DagDrawLod::Minimap {
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.graph.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if !chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.graph.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
        } else {
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let node = self.graph.node_spec_for_paint(idx, snapshot_node);
                let node = node.as_ref();
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let is_dimmed = engine_nid.is_some_and(|nid| self.paint.dimmed.contains(&nid));
                let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.graph.node_interaction_chrome(nid));
                let eval_status = engine_nid.and_then(|nid| self.paint.node_eval_status.get(&nid).copied()).unwrap_or(DagNodeEvalStatusKind::Ok);
                paint_node_visual(&self.viewport, &self.graph, &self.paint, &self.notes, scene, &aff, &cam, &viewport, lod, lod_index, node, DagNodePaintChrome { is_dimmed, is_selected, is_highlighted, is_hovered, eval_status, body_fill_alpha: 255, ghost_tint: false });
            }
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            paint_node_visual(&self.viewport, &self.graph, &self.paint, &self.notes, scene, &aff, &cam, &viewport, lod, lod_index, ghost, DagNodePaintChrome::ghost_preview());
        }
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Triangle));
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            let paint_ghost_handles = lod.shows_handles() || ghost.inputs().iter().any(|port| port.shape == PortShape::Triangle) || ghost.outputs().iter().any(|port| port.shape == PortShape::Triangle);
            if paint_ghost_handles {
                self.viewport.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
            }
        }
        paint_minimap_widget(&self.viewport, &self.graph, &self.paint, &self.minimap, scene, viewport_w, viewport_h);
    }
}

impl DagPaintState {
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_image_content(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, src: &str, label_fill: canvas::Color, bg: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        if src.is_empty() {
            let (x0, y0, x1, y1) = preview_content_bounds(node);
            let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
            append_label(scene, "Image…", pos, DAG_LABEL_COMPACT_SCREEN_PX, label_fill, bg);
            return;
        }
        let (x0, y0, x1, y1) = preview_content_bounds(node);
        let center = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        let w = (x1 - x0).max(1.0);
        let h = (y1 - y0).max(1.0);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, src, graph::IconScreenRect { center, width: w, height: h }, label_fill, bg, true);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_content(
        &self,
        scene: &mut canvas::Scene,
        cam: &canvas::camera::Camera,
        viewport: &canvas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &DagExpandedPaths,
        paint_px: f64,
        label_fill: canvas::Color,
        label_halo: canvas::Color,
        bg: canvas::Color,
    ) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        match content {
            DagPreviewContent::Empty => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, "—", pos, paint_px, label_fill, label_halo);
            }
            DagPreviewContent::Scalar { text } => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let display = if text.is_empty() { "—" } else { text.as_str() };
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
            }
            DagPreviewContent::Image { src } => {
                self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, bg);
            }
            DagPreviewContent::Tree { json } => {
                let (x0, y0, x1, _) = preview_content_bounds(node);
                let rows = preview_tree_rows(json, expanded, "", 0);
                let row_font_px = paint_px * 0.9;
                for (index, row) in rows.iter().enumerate() {
                    let row_y = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT + DAG_PREVIEW_ROW_HEIGHT * 0.5;
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    if row.has_children {
                        let glyph = if row.expanded { "▾️" } else { "▸️" };
                        let toggle_pos = world_to_screen(cam, viewport, Point::new(x0 + indent + DAG_PREVIEW_TOGGLE_WIDTH * 0.5, row_y));
                        append_label(scene, glyph, toggle_pos, row_font_px, label_fill, label_halo);
                    }
                    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    let max_w = (x1 - text_x).max(1.0);
                    let shown = truncate_label_to_fit_width(&line, max_w, row_font_px);
                    let text_pos = world_to_screen(cam, viewport, Point::new(text_x, row_y));
                    append_label(scene, &shown, text_pos, row_font_px, label_fill, label_halo);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_lod_icon(&self, scene: &mut canvas::Scene, lod: DagDrawLod, center_screen: canvas::Point, node: &DagNodeSpec, zoom: f64, fg: canvas::Color, bg: canvas::Color) {
        if !should_paint_node_lod_icon(node, lod) {
            return;
        }
        let icon = node.icon.trim();
        if icon.is_empty() {
            return;
        }
        let screen_w = node.width * zoom.max(0.05);
        let screen_h = node.height * zoom.max(0.05);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, icon, graph::IconScreenRect { center: center_screen, width: screen_w, height: screen_h }, fg, bg, false);
    }

    fn paint_computing_active_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_active_anim_phase.get(), false);
    }

    fn paint_computing_stale_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_stale_anim_phase.get(), true);
    }
}

pub fn dag_set_canvas_palette(viewport: &mut DagViewportState, paint: &mut DagPaintState, overlay:&BoardPaletteOverlay) {
    viewport.canvas_theme.apply_overlay(overlay);
    paint.icon_paint_cache.clear();
}

pub fn label_overlay_rows_for_node_spec(viewport: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {
    let lod = draw_lod_for_frame(viewport, graph);
    let zoom = graph.host_snapshot.camera.zoom;
    let lod_index = dag_lod_index(zoom);
    let engine_nid = graph.node_id_for_widget_id(&node.id);
    label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost, engine_nid, &paint.unresolved_input_ports)
}

/// 🎚️ Track, knob AND value readout live on the HTML `GraphSliderOverlays` control,
/// which renders all three off ONE published row (`slider_overlay_state_json`). A
/// readout painted here was a SECOND producer of the same number: it could only
/// change when a GPU frame was drawn, while `aria-valuenow` moved with the DOM
/// control, so a released knob read `10` to a screen reader and `0.0` on the canvas
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️slider-reevaluation-correctness-2026-09-15.md`).
#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
fn paint_node_visual(viewport_state: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, notes: &DagNotesState, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod, lod_index: usize, node: &DagNodeSpec, chrome: DagNodePaintChrome) {
    use canvas::camera::world_to_screen;
    use canvas::text::append_label;
    use canvas::FillRule;
    use canvas::{Point, Rect, Stroke};

    let theme = &viewport_state.canvas_theme;
    let label_halo = theme.label_halo;
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
    let tint = chrome.tint_highlighted();
    let fill = dag_node_paint_fill(lod, theme, chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered).map(|color| canvas_color_with_alpha(color, chrome.body_fill_alpha));
    let stroke = dag_node_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
    let label_fill = dag_node_label_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
    let internal_chrome_stroke = dag_node_internal_chrome_stroke(stroke, label_fill, chrome.is_hovered || chrome.is_selected || chrome.is_highlighted);
    let stroke_screen_px = dag_node_stroke_screen_px(chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered);
    if let Some(fill) = fill {
        scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
    }
    if !chrome.is_selected {
        scene.stroke(&Stroke::new(dag_world_stroke(stroke_screen_px, cam.zoom)), *aff, stroke, None, &rect);
    }
    match chrome.eval_status {
        DagNodeEvalStatusKind::Computing => paint.paint_computing_active_border(scene, aff, &rect, cam.zoom, theme.node_stroke_computing),
        DagNodeEvalStatusKind::Queued => paint.paint_computing_stale_border(scene, aff, &rect, cam.zoom, theme.node_stroke_stale),
        DagNodeEvalStatusKind::Error => paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_error, false),
        DagNodeEvalStatusKind::Blocked => paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_blocked, true),
        DagNodeEvalStatusKind::Ok => {}
    }
    let layout_px = dag_label_layout_px();
    let paint_px = dag_label_paint_px(cam.zoom, lod_index);
    let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
    let center_screen = world_to_screen(cam, viewport, Point::new(node.x, node.y));
    let node_fill = fill.unwrap_or(theme.node_fill);
    if !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
        paint.paint_node_lod_icon(scene, lod, center_screen, node, cam.zoom, label_fill, node_fill);
    }
    let label_text = node_label_text(node, lod);
    let caption_on_overlay = node_caption_delegated_to_js_overlay(node, lod);
    if lod.node_label_is_horizontal() {
        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
            paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
        }
    } else {
        match &node.kind {
            DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. } => {
                if lod.shows_computation_layout() {
                    let channel_row_pick = lod.uses_channel_row_pick();
                    if channel_row_pick {
                        graph.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
                    }
                    paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_chrome_stroke);
                    graph.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_chrome_stroke, stroke, label_fill, channel_row_pick);
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                    if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                        paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                    }
                    if let DagNodeKind::Computation { variadic_inputs, variadic_outputs, .. } = &node.kind {
                        if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD && (*variadic_inputs || *variadic_outputs) {
                            paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                    }
                }
            }
            DagNodeKind::Slider { .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                    paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                }
            }
            DagNodeKind::Select { options, selected, .. } => {
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (cx0, cy0, cx1, cy1) = select_control_bounds(node);
                    let control = Rect::new(cx0, cy0, cx1, cy1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let option = usize::try_from(*selected).ok().and_then(|index| options.get(index)).map_or("—", String::as_str);
                        let option_pos = world_to_screen(cam, viewport, Point::new((cx0 + cx1) * 0.5, (cy0 + cy1) * 0.5));
                        append_label(scene, option, option_pos, paint_px * 0.95, label_fill, label_halo);
                        let chevron = world_to_screen(cam, viewport, Point::new(cx1 - 6.0 / cam.zoom.max(0.05), (cy0 + cy1) * 0.5));
                        append_label(scene, "▾️", chevron, paint_px, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::Screen { media, .. } => {
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let inset = 8.0 / cam.zoom.max(0.05);
                    let frame = Rect::new(node.x - hw + inset, node.y - hh + hh * 0.35, node.x + hw - inset, node.y + hh - inset);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke_selection_exit, None, &frame);
                }
                if lod.shows_detail_text() {
                    if let Some(media) = media {
                        let kind_label = match media.kind {
                            DagMediaKind::Image => "image",
                            DagMediaKind::Svg => "svg",
                            DagMediaKind::Pdf => "pdf",
                            DagMediaKind::Video => "video",
                        };
                        let hint = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.1));
                        append_label(scene, kind_label, hint, paint_px * 0.85, canvas_color_with_alpha(label_fill, ui_styling::opacities::KIND_HINT_ALPHA), label_halo);
                    }
                }
            }
            DagNodeKind::Note { text, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_detail_text() || lod.shows_controls() {
                    let (x0, y0, x1, y1) = preview_content_bounds(node);
                    let font_px = paint_px * 1.05;
                    let text_x = x0 + DAG_PREVIEW_PAD;
                    let max_w = (x1 - text_x).max(1.0);
                    let display = if text.is_empty() { "…" } else { text.as_str() };
                    let shown = truncate_label_to_fit_width(display, max_w, font_px);
                    let pos = world_to_screen(cam, viewport, Point::new(text_x, (y0 + y1) * 0.5));
                    append_label(scene, &shown, pos, font_px, label_fill, label_halo);
                    if let Some(edit) = &notes.editing_note {
                        if edit.node_id == node.id && notes.caret_visible {
                            paint_note_caret_bar(scene, aff, node, edit.caret, text, font_px, label_fill, cam.zoom);
                        }
                    }
                }
            }
            DagNodeKind::Image { src, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = preview_content_bounds(node);
                    let frame = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &frame);
                }
                if lod.shows_detail_text() || lod.shows_controls() {
                    paint.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, theme.raster_clear);
                }
            }
            DagNodeKind::Preview { content, expanded, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_detail_text() || lod.shows_controls() {
                    paint.paint_preview_content(scene, cam, viewport, node, content, expanded, paint_px, label_fill, label_halo, theme.raster_clear);
                }
            }
            DagNodeKind::Action { label, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    let control = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                        append_label(scene, label, pos, paint_px * 0.95, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::Export { format, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    let control = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let export_label = format.to_uppercase();
                        let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                        append_label(scene, &export_label, pos, paint_px * 0.95, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::AppInstance { plugin_id, app_id, .. } => {
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_detail_text() {
                    let subtitle = format!("{plugin_id}/{app_id}");
                    let subtitle_pos = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.22));
                    append_label(scene, &subtitle, subtitle_pos, paint_px * 0.85, label_fill, label_halo);
                }
            }
        }
    }
}

// #region 🔖️DagHost
impl DagHost {
    pub fn load_host_snapshot_json(json: &str) -> Result<Self, DagError> {
        let host_snapshot: DagHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        if host_snapshot.schema != "dag.hostDocument" {
            return Err(DagError::SchemaMismatch);
        }
        Ok(Self::from_host_snapshot(host_snapshot))
    }

    /// 🖱️ Whether a screen-space interaction only `pointer_*_screen` implements is already in flight
    /// — drawing/reconnecting a wire, dragging the minimap viewport, panning, or dragging an inline
    /// widget. While one is, the bounded `derive_pointer_plan` path (whose gesture vocabulary is
    /// `Idle | Pan | Drag | Select`) must not claim the follow-up move/up and cancel it.
    pub fn screen_pointer_gesture_active(&self) -> bool {
        !matches!(self.graph.engine.interaction, InteractionMode::Idle) || self.minimap.minimap_widget_drag.is_some() || self.viewport.pan_anchor.is_some() || self.pointer.widget_drag.is_some() || self.pointer.pending_port_insert.is_some()
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo (see infinite/board/rs, framework/editor/rs, layout/rs, etc.)")]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if button == 0 && !shift && !ctrl_or_meta && !alt && !pan {
            if let Some((layout, on_viewport)) = minimap_widget_pointer_hit(&self.viewport, &self.graph, &self.minimap, sx, sy) {
                let (wx, wy) = minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.graph.host_snapshot.camera.zoom;
                if on_viewport {
                    let cam = &self.graph.host_snapshot.camera;
                    self.minimap.minimap_widget_drag = Some((cam.x - wx, cam.y - wy));
                } else {
                    self.graph.set_camera(wx, wy, zoom);
                    self.minimap.minimap_widget_drag = Some((0.0, 0.0));
                }
                return;
            }
        }
        if pan {
            self.viewport.pan_anchor = Some((sx, sy, self.graph.host_snapshot.camera.x, self.graph.host_snapshot.camera.y));
            return;
        }
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        if button == 0 && !shift && !ctrl_or_meta && !alt {
            if let Some(node_id) = self.graph.snapshot_draggable_node_hit(world.x, world.y) {
                if !world_hits_handle(&self.viewport, &self.graph, world.x, world.y) {
                    if let Some(widget_id) = self.graph.widget_id_for_node_id(node_id) {
                        if let Some(node) = self.graph.host_snapshot.nodes.iter().find(|entry| entry.id == widget_id) {
                            if let DagNodeKind::AppInstance { instance_id, .. } = &node.kind {
                                let now = pointer_event_now_ms();
                                let dist = ((world.x - self.pointer.last_pointer_down_world.0).powi(2) + (world.y - self.pointer.last_pointer_down_world.1).powi(2)).sqrt();
                                let zoom = self.graph.host_snapshot.camera.zoom.max(1e-9);
                                if self.pointer.last_pointer_down_node_id.as_deref() == Some(widget_id.as_str()) && now - self.pointer.last_pointer_down_at_ms < 350.0 && dist < 8.0 / zoom {
                                    self.pointer.pending_open_instance_id = Some(instance_id.clone());
                                    return;
                                }
                                self.pointer.last_pointer_down_at_ms = now;
                                self.pointer.last_pointer_down_world = (world.x, world.y);
                                self.pointer.last_pointer_down_node_id = Some(widget_id);
                            }
                        }
                    }
                }
            }
        }
        if let Some(hit) = self.graph.port_insert_hit(world.x, world.y, self.graph.host_snapshot.camera.zoom) {
            self.pointer.pending_port_insert = Some(hit);
            return;
        }
        let (hit_x, hit_y) = connection_hit_world(&self.viewport, &self.graph, world.x, world.y);
        if world_hits_handle(&self.viewport, &self.graph, hit_x, hit_y) {
            self.graph.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
            process_engine_events(&mut self.viewport, &mut self.graph);
            self.graph.sync_camera_from_engine();
            return;
        }
        if try_widget_pointer_down(&mut self.viewport, &mut self.graph, &mut self.pointer, world.x, world.y) {
            return;
        }
        if self.graph.try_node_rectangle_pointer_down(world.x, world.y, button, shift, ctrl_or_meta, alt) {
            process_engine_events(&mut self.viewport, &mut self.graph);
            self.graph.sync_camera_from_engine();
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        if button == 0 && !merge_from_modifiers && lod_uses_bounded_drag(&self.viewport, &self.graph) {
            let pad = DAG_BOUNDED_DRAG_HIT_PAD_PX / self.graph.host_snapshot.camera.zoom.max(1e-9);
            if self.graph.engine.try_begin_selection_union_drag_at(world, pad) {
                process_engine_events(&mut self.viewport, &mut self.graph);
                self.graph.sync_camera_from_engine();
                return;
            }
        }
        self.graph.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
        process_engine_events(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }
}

impl DagPaintState {
    /// ✅️ Clears evaluating chrome from all nodes.
    pub fn clear_computing(&mut self) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.reset();
        self.unresolved_input_ports.clear();
    }

    pub(crate) fn tick_computing_animation(&self) {
        if self.computing_active.is_some() {
            let next = (self.computing_active_anim_phase.get() + 0.02) % 1.0;
            self.computing_active_anim_phase.set(next);
        }
        if !self.computing_stale.is_empty() {
            let next = (self.computing_stale_anim_phase.get() + 0.008) % 1.0;
            self.computing_stale_anim_phase.set(next);
        }
    }
}

impl DagPointerState {
    /// ➕️ Returns and clears a pending variadic insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(DagPortSide, String, usize)> {
        self.pending_port_insert.take()
    }

    /// 👻️ Sets or clears the placement preview node painted with the normal LOD path.
    pub fn set_ghost_node(&mut self, node: Option<DagNodeSpec>) {
        self.ghost_node = node;
    }

    pub fn ghost_node(&self) -> Option<&DagNodeSpec> {
        self.ghost_node.as_ref()
    }

    /// 💥️ Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    /// 📤️ Takes a pending export control click from the last widget hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.pending_export_click.take()
    }

    pub fn widget_drag_active(&self) -> bool {
        self.widget_drag.is_some()
    }
}

impl DagNotesState {
    /// ✏️ Ends inline note editing.
    pub fn note_commit_edit(&mut self) {
        self.editing_note = None;
    }

    /// ✏️ Toggles native caret visibility for the active note editor.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    /// ✏️ Returns the widget id currently being edited inline, if any.
    pub fn editing_note_id(&self) -> Option<&str> {
        self.editing_note.as_ref().map(|edit| edit.node_id.as_str())
    }
}

/// 🌫️ Marks preview-off nodes as dimmed on the canvas.
pub fn set_dimmed(graph: &mut DagGraphState, paint: &mut DagPaintState, widget_ids: &[String]) {
    paint.dimmed.clear();
    for widget_id in widget_ids {
        if let Some(nid) = graph.node_id_for_widget_id(widget_id) {
            paint.dimmed.insert(nid);
        }
    }
}

/// ⚙️ Marks one actively computing widget and downstream widgets as stale.
pub fn set_computing_progress(graph: &mut DagGraphState, paint: &mut DagPaintState, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
    paint.computing_active = active_widget_id.and_then(|widget_id| graph.node_id_for_widget_id(widget_id));
    paint.computing_stale.clear();
    for widget_id in stale_widget_ids {
        if let Some(nid) = graph.node_id_for_widget_id(widget_id) {
            if paint.computing_active != Some(nid) {
                paint.computing_stale.insert(nid);
            }
        }
    }
}

/// 🚦️ Replaces chrome from complete admitted per-widget evaluation facts.
pub fn set_node_statuses(graph: &mut DagGraphState, paint: &mut DagPaintState,statuses:&DagNodeStatuses) {
    paint.computing_active=None;
    paint.computing_stale.clear();
    paint.node_eval_status.reset();
    paint.unresolved_input_ports.clear();
    for (widget_id,status)in statuses {
        let Some(nid)=graph.node_id_for_widget_id(widget_id)else{continue};
        let kind=match status {
            DagNodeEvaluationStatus::Computing{}=>{paint.computing_active=Some(nid);DagNodeEvalStatusKind::Computing},
            DagNodeEvaluationStatus::Queued{}=>{paint.computing_stale.insert(nid);DagNodeEvalStatusKind::Queued},
            DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,
            DagNodeEvaluationStatus::Blocked{ports}=>{for port in ports{if let Some(node)=graph.host_snapshot.nodes.get((nid-1)as usize){for(index,input)in node.inputs().iter().enumerate(){if input.id==*port{paint.unresolved_input_ports.insert((nid,index));}}}}DagNodeEvalStatusKind::Blocked},
            DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,
        };
        paint.node_eval_status.insert(nid,kind);
    }
}

/// 📋️ Preview-off snapshot node ids currently dimmed on the canvas.
pub fn dimmed_node_ids(graph: &DagGraphState, paint: &DagPaintState) -> Vec<String> {
    paint.dimmed.iter().filter_map(|&nid| graph.widget_id_for_node_id(nid)).collect()
}

/// 🔌️ Whether React Flow's centered connection cursor applies at this point: either the
/// live hit is a port handle or a wire draw/reconnect gesture owns the pointer beyond its source.
pub fn connection_cursor_active_at_screen(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> bool {
    matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) || screen_hit(viewport, graph, minimap, sx, sy).handle
}

/// 🖱️ Whether this SCREEN point belongs to the screen pointer path at all: the minimap widget, a
/// wire handle, a port insertion target, or an inline widget. Exactly the four hits
/// `bounded_node_hit_index` answers `Unsupported` for, asked as a question instead of as a fault.
///
/// 🖐️ It answers for every PHASE, not only a press: a hover or a release over one of these hits
/// is just as undescribable to the bounded plan path, and routing only presses left a plain move
/// across a port faulting the whole dispatch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn screen_pointer_gesture_begins_at(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> bool {
    screen_hit(viewport, graph, minimap, sx, sy).is_screen_path()
}

/// 🩺️ The ONE classification of a screen point both the path discriminator and a renderer
/// diagnostic read, so a gesture can never be explained by a different hit test than the one that
/// routed it. `node_id` is the draggable snapshot node under the point; the four booleans are the
/// hits `bounded_node_hit_index` answers `Unsupported` for.
pub fn screen_hit(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState, sx: f64, sy: f64) -> DagScreenHit {
    let minimap = minimap_widget_pointer_hit(viewport, graph, minimap_state, sx, sy);
    let world = screen_to_world_point(viewport, graph, sx, sy);
    let handle = port_pointer_handle_hit(viewport, graph, world.x, world.y);
    DagScreenHit {
        node_id: graph.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| graph.widget_id_for_node_id(node_id)),
        channel: handle.and_then(|hid| graph.decode_channel_ref(hid)),
        minimap: minimap.is_some(),
        minimap_viewport: minimap.is_some_and(|(_, on_viewport)| on_viewport),
        port_insert: graph.port_insert_hit(world.x, world.y, graph.host_snapshot.camera.zoom).is_some(),
        handle: handle.is_some(),
        widget: graph.widget_hit_at(world.x, world.y).is_some(),
    }
}

/// 🩺️ [`Self::screen_hit`] as JSON, for a renderer that has to report why a press went where it
/// went across a wasm boundary that carries no Rust types.
pub fn screen_hit_json(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState, sx: f64, sy: f64) -> String {
    let hit = screen_hit(viewport, graph, minimap_state, sx, sy);
    semio_framework_pack_json::to_json_string(&DagScreenHitJson {
        node: hit.node_id.clone(),
        draggable: hit.is_draggable_body(),
        handle: hit.channel.as_ref().map(|channel| format!("{}@{}", channel.widget_id, channel.port)),
        direction: hit.channel.as_ref().map(|channel| channel.direction.clone()),
        widget: hit.widget,
        minimap: hit.minimap,
        minimap_viewport: hit.minimap_viewport,
        port_insert: hit.port_insert,
    })
}

/// ✏️ Begins inline note text editing at a world-space click position.
pub fn begin_note_edit(graph: &mut DagGraphState, notes: &mut DagNotesState, node_id: &str, world_x: f64, _world_y: f64) -> bool {
    let Some(node) = graph.host_snapshot.nodes.iter().find(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &node.kind else {
        return false;
    };
    let lod_index = dag_lod_index(graph.host_snapshot.camera.zoom);
    let font_px = dag_label_paint_px(graph.host_snapshot.camera.zoom, lod_index) * 1.05;
    let origin_x = note_text_origin_x(node);
    let offset = hit_byte_in_note_line(text, world_x, origin_x, font_px);
    notes.editing_note = Some(NoteEditState { node_id: node_id.to_string(), caret: offset, anchor: offset });
    notes.caret_visible = true;
    true
}

/// ✏️ Inserts text at the active note caret, replacing any selection.
pub fn note_insert_text(graph: &mut DagGraphState, notes: &mut DagNotesState, chunk: &str) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    text.replace_range(start..end, chunk);
    let pos = start + chunk.len();
    edit.caret = pos;
    edit.anchor = pos;
    true
}

/// ✏️ Deletes the selection or the character before the note caret.
pub fn note_backspace(graph: &mut DagGraphState, notes: &mut DagNotesState) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    if start < end {
        text.replace_range(start..end, "");
        edit.caret = start;
        edit.anchor = start;
        return true;
    }
    if start == 0 {
        return false;
    }
    let prev = text[..start].char_indices().last().map_or(0, |(i, _)| i);
    text.replace_range(prev..start, "");
    edit.caret = prev;
    edit.anchor = prev;
    true
}

/// ✏️ Deletes the selection or the character after the note caret.
pub fn note_delete_forward(graph: &mut DagGraphState, notes: &mut DagNotesState) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    if start < end {
        text.replace_range(start..end, "");
        edit.caret = start;
        edit.anchor = start;
        return true;
    }
    if start >= text.len() {
        return false;
    }
    let next = text[start..].char_indices().nth(1).map_or(text.len(), |(i, _)| start + i);
    text.replace_range(start..next, "");
    edit.caret = start;
    edit.anchor = start;
    true
}

/// ✏️ Moves the note caret (`left` | `right` | `home` | `end`).
pub fn note_move_caret(graph: &mut DagGraphState, notes: &mut DagNotesState, direction: &str, extend: bool) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let pos = match direction {
        "left" => {
            if edit.caret == 0 {
                0
            } else {
                text[..edit.caret].char_indices().last().map_or(0, |(i, _)| i)
            }
        }
        "right" => text[edit.caret..].char_indices().nth(1).map_or(text.len(), |(i, _)| edit.caret + i),
        "home" => 0,
        "end" => text.len(),
        _ => return false,
    };
    if extend {
        edit.caret = pos;
    } else {
        edit.caret = pos;
        edit.anchor = pos;
    }
    true
}

fn try_widget_pointer_down(viewport: &mut DagViewportState, graph: &mut DagGraphState, pointer: &mut DagPointerState, world_x: f64, world_y: f64) -> bool {
    if !draw_lod_for_frame(viewport, graph).shows_controls() {
        return false;
    }
    let Some((idx, kind)) = graph.widget_hit_at(world_x, world_y) else {
        return false;
    };
    match kind {
        WidgetPointerKind::SliderDrag => {
            pointer.widget_drag = Some(idx);
            set_slider_value_from_x(&mut graph.host_snapshot.nodes[idx], world_x);
        }
        WidgetPointerKind::SelectClick => {
            advance_select_option(&mut graph.host_snapshot.nodes[idx]);
        }
        WidgetPointerKind::PreviewToggle(path) => {
            toggle_preview_tree_path(&mut graph.host_snapshot.nodes[idx], &path);
            graph.sync_snapshot_node_size_to_engine(idx);
        }
        WidgetPointerKind::ClusterExplode => {
            pointer.pending_cluster_explode = Some(graph.host_snapshot.nodes[idx].id.clone());
        }
        WidgetPointerKind::ExportClick => {
            pointer.pending_export_click = Some(graph.host_snapshot.nodes[idx].id.clone());
        }
    }
    true
}

impl DagSelectionSource for DagHost {
    fn selection_candidate_count(&self,domain:DagSelectionDomain)->usize {
        match domain {DagSelectionDomain::Nodes=>self.graph.host_snapshot.nodes.len(),DagSelectionDomain::Edges=>self.graph.host_snapshot.edges.len()}
    }
    fn selection_candidate_id(&self,domain:DagSelectionDomain,index:usize)->Option<&str>{
        match domain {
            DagSelectionDomain::Nodes=>{let id=u64::try_from(index.checked_add(1)?).ok()?;(self.graph.node_id_map.get(&id)==Some(&index)&&self.graph.engine.selection.node_ids.contains(&id)).then(||self.graph.host_snapshot.nodes.get(index).map(|node|node.id.as_str())).flatten()},
            DagSelectionDomain::Edges=>self.graph.edge_engine_ids.get(index).and_then(|id|*id).filter(|id|self.graph.engine.selection.edge_ids.contains(id)).and_then(|_|self.graph.host_snapshot.edges.get(index).map(|edge|edge.id.as_str())),
        }
    }
}

/// 🌳️ Retained DAG host: typed nodes, edges, engine, camera.
pub struct DagHost {
    /// Size, device ratio, LOD and grid policy, theme and pan anchor of the canvas.
    pub viewport: DagViewportState,
    /// The host snapshot, the board engine and the id maps between them, with pending wire edits.
    pub graph: DagGraphState,
    /// Widget drag, ghost node, pending click intents and the last pointer down.
    pub pointer: DagPointerState,
    /// Icon paint cache, computing progress, evaluation status and dimming.
    pub paint: DagPaintState,
    /// The minimap widget state.
    pub minimap: DagMinimapState,
    /// The inline note editing state.
    pub notes: DagNotesState,
}

/// 🧩️ DagHost pointer component: widget drag, ghost node, pending click intents and the last pointer down.
pub struct DagPointerState {
    pub widget_drag: Option<usize>,
    pub pending_port_insert: Option<(DagPortSide, String, usize)>,
    pub ghost_node: Option<DagNodeSpec>,
    pub pending_cluster_explode: Option<String>,
    pub pending_export_click: Option<String>,
    pub pending_open_instance_id: Option<String>,
    pub last_pointer_down_at_ms: f64,
    pub last_pointer_down_world: (f64, f64),
    pub last_pointer_down_node_id: Option<String>,
}

/// 🧩️ DagHost paint component: icon paint cache, computing progress, evaluation status and dimming.
pub struct DagPaintState {
    pub dimmed: HistoryFoldSet<NodeId>,
    pub icon_paint_cache: graph::IconPaintCache,
    pub computing_active: Option<NodeId>,
    pub computing_stale: NumericSet<NodeId>,
    pub computing_active_anim_phase: Cell<f64>,
    pub computing_stale_anim_phase: Cell<f64>,
    pub node_eval_status: NumericIndex<NodeId, DagNodeEvalStatusKind>,
    pub unresolved_input_ports: NumericSet<(NodeId,usize)>,
}

/// 🧩️ DagHost minimap component: the minimap widget state.
pub struct DagMinimapState {
    pub minimap_widget_visible: bool,
    pub minimap_widget_hovered: bool,
    pub minimap_widget_drag: Option<(f64, f64)>,
}

/// 🧩️ DagHost notes component: the inline note editing state.
pub struct DagNotesState {
    pub editing_note: Option<NoteEditState>,
    pub caret_visible: bool,
}

/// 🪟️ Takes a pending double-click open request for an app instance node.
pub fn dag_take_pending_open_instance_id(pointer: &mut DagPointerState) -> Option<String> {
    pointer.pending_open_instance_id.take()
}

impl DagOutputCandidates for DagHost{
    fn node_candidate_count(&self)->usize{self.graph.engine.selection.node_ids.len()}
    fn node_candidate_id(&self,index:usize)->Option<&str>{self.graph.widget_id_for_node_id_ref(*self.graph.engine.selection.node_ids.key_at_rank(index)?)}
    fn edge_candidate_count(&self)->usize{self.graph.engine.selection.edge_ids.len()}
    fn edge_candidate_id(&self,index:usize)->Option<&str>{self.graph.edge_id_map.get(self.graph.engine.selection.edge_ids.key_at_rank(index)?).map(String::as_str)}
    fn channel_candidate_count(&self)->usize{self.graph.engine.selection.handle_ids.len()}
    fn channel_candidate(&self,index:usize)->Option<DagChannelView<'_>>{self.graph.channel_view(*self.graph.engine.selection.handle_ids.key_at_rank(index)?)}
    fn handle_candidate_identity(&self,index:usize)->Option<&str>{let id=self.graph.engine.selection.handle_ids.key_at_rank(index)?;self.graph.channel_view(*id)?;self.graph.handle_key_map.get(id).map(String::as_str)}
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{self.graph.engine.hover.and_then(|id|self.graph.channel_view(id))}
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{
        let(source,target)=self.graph.engine.wire_drag_type_refusal()?;
        Some(DagRefusalView{source:self.graph.handle_key_map.get(&source)?,source_types:self.graph.engine.handles.get(&source).map_or(&[],|handle|handle.value_types.as_slice()),target:self.graph.handle_key_map.get(&target)?,target_types:self.graph.engine.handles.get(&target).map_or(&[],|handle|handle.value_types.as_slice())})
    }
}

// #region 🔖️DagHost
impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_host_snapshot(DagHostSnapshot::default())
    }

    pub fn from_host_snapshot(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot_with_layout(snapshot, false)
    }

    /// 🌳️ Builds a host without running auto-layout (preserves node positions).
    pub fn from_host_snapshot_without_layout(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot(snapshot)
    }

    /// ♻️ Rebuilds transient DAG state while retaining the exact owner of admitted icon paints.
    pub fn replace_host_snapshot_without_layout(&mut self, snapshot: DagHostSnapshot) {
        let mut next = Self::from_host_snapshot_without_layout(snapshot);
        std::mem::swap(&mut self.paint.icon_paint_cache, &mut next.paint.icon_paint_cache);
        *self = next;
    }

    fn from_host_snapshot_with_layout(snapshot: DagHostSnapshot, apply_layout: bool) -> Self {
        let mut host = Self {
            viewport: DagViewportState {
                canvas_theme: CanvasPalette::default(),
                width: 1,
                height: 1,
                dpr: 1.0,
                last_screen_x: 0.0,
                last_screen_y: 0.0,
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                automatic_lod: true,
                forced_draw_lod: None,
                grid_visible: true,
                grid_snap_enabled: false,
                grid_factor: GRID_FACTOR_DEFAULT,
                pan_anchor: None,
            },
            graph: DagGraphState {
                host_snapshot: snapshot,
                engine: DagBoardEngine::new(),
                node_id_map: HistoryFoldIndex::new(),
                handle_key_map: HistoryFoldIndex::new(),
                handle_port_shape: HistoryFoldIndex::new(),
                handle_port_visible: HistoryFoldIndex::new(),
                edge_id_map: HistoryFoldIndex::new(),
                edge_engine_ids: Vec::new(),
                edge_route_style: HistoryFoldIndex::new(),
                pending_graph_edits: Vec::new(),
                journal_refusal: None,
            },
            pointer: DagPointerState {
                widget_drag: None,
                pending_port_insert: None,
                ghost_node: None,
                pending_cluster_explode: None,
                pending_export_click: None,
                pending_open_instance_id: None,
                last_pointer_down_at_ms: 0.0,
                last_pointer_down_world: (0.0, 0.0),
                last_pointer_down_node_id: None,
            },
            paint: DagPaintState {
                dimmed: HistoryFoldSet::new(),
                icon_paint_cache: graph::IconPaintCache::new(),
                computing_active: None,
                computing_stale: NumericSet::new(),
                computing_active_anim_phase: Cell::new(0.0),
                computing_stale_anim_phase: Cell::new(0.0),
                node_eval_status: NumericIndex::new(),
                unresolved_input_ports: NumericSet::new(),
            },
            minimap: DagMinimapState {
                minimap_widget_visible: false,
                minimap_widget_hovered: false,
                minimap_widget_drag: None,
            },
            notes: DagNotesState {
                editing_note: None,
                caret_visible: true,
            },
        };
        host.graph.rebuild_engine_with_layout(apply_layout);
        host
    }
}

pub fn derive_pointer_plan(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, projection: DagInteractionProjection, intent: DagPointerIntent) -> Result<DagPointerPlan, DagInteractionPlanFault> {
    if projection.revision == u64::MAX || graph.host_snapshot.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
        return Err(DagInteractionPlanFault::NodeCredits);
    }
    let mut next = projection;
    next.revision = projection.revision + 1;
    let mut moves = [None; DAG_INTERACTION_NODE_CAPACITY];
    let mut move_len = 0u16;
    let mut drag = None;
    match intent.phase {
        DagPointerPhase::Down => derive_projection_down(viewport, graph, minimap, &mut next, intent)?,
        DagPointerPhase::Move => derive_projection_move(viewport, graph, minimap, &mut next, &mut moves, &mut move_len, &mut drag, intent)?,
        DagPointerPhase::Up => {
            derive_projection_move(viewport, graph, minimap, &mut next, &mut moves, &mut move_len, &mut drag, intent)?;
            next.gesture = DagProjectionGesture::Idle;
        }
        DagPointerPhase::Leave => {
            next.gesture = DagProjectionGesture::Idle;
            next.hover = None;
        }
    }
    Ok(DagPointerPlan { expected_revision: projection.revision, previous_active: !matches!(projection.gesture, DagProjectionGesture::Idle), next, moves, move_len, drag })
}

fn bounded_node_hit_index(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> Result<Option<usize>, DagInteractionPlanFault> {
    if minimap_widget_pointer_hit(viewport, graph, minimap, sx, sy).is_some() {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    let world = screen_to_world_point(viewport, graph, sx, sy);
    if graph.port_insert_hit(world.x, world.y, graph.host_snapshot.camera.zoom).is_some() || world_hits_handle(viewport, graph, world.x, world.y) || graph.widget_hit_at(world.x, world.y).is_some() {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    Ok(graph.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| graph.node_id_map.get(&node_id).copied()))
}

fn derive_projection_down(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, next: &mut DagInteractionProjection, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
    if intent.pan || intent.button == 1 {
        next.gesture = DagProjectionGesture::Pan { start_x: intent.x, start_y: intent.y, camera: next.camera };
        return Ok(());
    }
    if intent.button != 0 || intent.alt {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    let Some(index) = bounded_node_hit_index(viewport, graph, minimap, intent.x, intent.y)? else {
        next.gesture = DagProjectionGesture::Select { start_x: intent.x, start_y: intent.y, initial: next.selected };
        return Ok(());
    };
    if intent.ctrl_or_meta {
        let selected = !dag_bit_contains(&next.selected, index);
        dag_bit_set(&mut next.selected, index, selected);
        if !selected {
            next.gesture = DagProjectionGesture::Idle;
            return Ok(());
        }
    } else if intent.shift {
        dag_bit_set(&mut next.selected, index, true);
    } else if !dag_bit_contains(&next.selected, index) {
        next.selected = [0; DAG_INTERACTION_WORD_CAPACITY];
        dag_bit_set(&mut next.selected, index, true);
    }
    next.hover = u16::try_from(index).ok();
    let mut starts = [None; DAG_INTERACTION_NODE_CAPACITY];
    let mut len = 0usize;
    for node_index in 0..graph.host_snapshot.nodes.len() {
        if !dag_bit_contains(&next.selected, node_index) {
            continue;
        }
        let node = &graph.host_snapshot.nodes[node_index];
        starts[len] = Some(DagNodeMove { index: node_index as u16, x: node.x, y: node.y });
        len += 1;
    }
    next.gesture = DagProjectionGesture::Drag { press: next.revision, start_x: intent.x, start_y: intent.y, starts, len: len as u16 };
    Ok(())
}

fn derive_projection_move(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, next: &mut DagInteractionProjection, moves: &mut [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], move_len: &mut u16, drag: &mut Option<DagDragOffset>, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
    match next.gesture {
        DagProjectionGesture::Pan { start_x, start_y, camera } => {
            let zoom = camera[2].max(1e-9);
            next.camera = [camera[0] - (intent.x - start_x) / zoom, camera[1] - (intent.y - start_y) / zoom, camera[2]];
        }
        DagProjectionGesture::Drag { press, start_x, start_y, starts, len } => {
            let zoom = next.camera[2].max(1e-9);
            let dx = (intent.x - start_x) / zoom;
            let dy = (intent.y - start_y) / zoom;
            *drag = (dx != 0.0 || dy != 0.0).then_some(DagDragOffset { press, dx, dy });
            for index in 0..usize::from(len) {
                let Some(start) = starts[index] else {
                    continue;
                };
                moves[index] = Some(DagNodeMove { index: start.index, x: start.x + dx, y: start.y + dy });
            }
            *move_len = len;
        }
        DagProjectionGesture::Select { start_x, start_y, initial } => {
            let min_x = start_x.min(intent.x);
            let max_x = start_x.max(intent.x);
            let min_y = start_y.min(intent.y);
            let max_y = start_y.max(intent.y);
            let mut selected = if intent.shift || intent.ctrl_or_meta { initial } else { [0; DAG_INTERACTION_WORD_CAPACITY] };
            for index in 0..graph.host_snapshot.nodes.len() {
                let node = &graph.host_snapshot.nodes[index];
                let screen = world_to_screen_point(viewport, graph, node.x, node.y);
                if screen.0 >= min_x && screen.0 <= max_x && screen.1 >= min_y && screen.1 <= max_y {
                    dag_bit_set(&mut selected, index, true);
                }
            }
            next.selected = selected;
        }
        DagProjectionGesture::Idle => {
            next.hover = bounded_node_hit_index(viewport, graph, minimap, intent.x, intent.y)?.and_then(|index| u16::try_from(index).ok());
        }
    }
    Ok(())
}

/// 🗺️ Every node and port this graph paints, each with the SCREEN geometry
/// [`Self::entity_screen_json`] publishes for it — one census of where everything IS, for a
/// renderer diagnostic and for a browser probe that would otherwise have to FIND the graph by
/// sweeping the pointer across it. A sweep is not a cheap way to ask: measured on the wgpu serve,
/// a 224-point hover sweep spends the host's whole attention budget and leaves nothing for the
/// gesture it was sweeping to find (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Rects are SURFACE-local, in the same coordinates `pointer_*_screen` takes.
pub fn screen_geometry_census_json(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState) -> String {
    let mut rows: Vec<String> = Vec::new();
    for node in &graph.host_snapshot.nodes {
        let body = match entity_screen_rect(viewport, graph, "node", &node.id) {
            Some(rect) => first_screen_point_in(viewport, graph, minimap_state, rect, |hit| hit.is_draggable_body() && hit.node_id.as_deref() == Some(node.id.as_str())),
            None => None,
        };
        let body_json = body.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"));
        rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), entity_screen_json(viewport, graph, "node", &node.id)));
        for (port, direction) in node.inputs().iter().map(|port| (port, "in")).chain(node.outputs().iter().map(|port| (port, "out"))) {
            let channel = format!("{}@{}", node.id, port.id);
            rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), entity_screen_json(viewport, graph, "handle", &channel)));
        }
    }
    let panel = [0.0, 0.0, viewport.width as f64, viewport.height as f64];
    let navigate = first_screen_point_in(viewport, graph, minimap_state, panel, |hit| hit.minimap && !hit.minimap_viewport);
    rows.push(format!("{{\"kind\":\"minimap\",\"navigate\":{}}}", navigate.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"))));
    format!("[{}]", rows.join(","))
}

/// 🔎️ The first point of a screen rect whose classification satisfies `wanted`, scanned on a fixed
/// grid inset from the rim. Which part of a node is draggable body and which is inline widget is
/// the ENGINE's answer, and this is how it is asked without a pointer ever moving.
fn first_screen_point_in(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, rect: [f64; 4], wanted: impl Fn(&DagScreenHit) -> bool) -> Option<(f64, f64)> {
    const STEPS: usize = 12;
    for row in 0..STEPS {
        for column in 0..STEPS {
            let x = rect[0] + rect[2] * (0.06 + 0.88 * column as f64 / (STEPS - 1) as f64);
            let y = rect[1] + rect[3] * (0.06 + 0.88 * row as f64 / (STEPS - 1) as f64);
            if x < 0.0 || y < 0.0 || x > viewport.width as f64 || y > viewport.height as f64 {
                continue;
            }
            if wanted(&screen_hit(viewport, graph, minimap, x, y)) {
                return Some((x, y));
            }
        }
    }
    None
}

impl DagMinimapState {
    // #region 🔖️MinimapWidget
    /// 🗺️ Toggles the bottom-right flow minimap navigator.
    pub fn set_minimap_widget_visible(&mut self, visible: bool) {
        self.minimap_widget_visible = visible;
    }
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::layout` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_layout(graph: &DagGraphState, minimap_state: &DagMinimapState, viewport_w: u32, viewport_h: u32) -> Option<MinimapWidgetLayout> {
    if !minimap_state.minimap_widget_visible {
        return None;
    }
    let content = graph.minimap_widget_content_bounds()?;
    if graph.minimap_camera_fully_shows_content(&content, viewport_w, viewport_h) {
        return None;
    }
    let w = ui_styling::metrics::dag::MINIMAP_WIDGET_WIDTH;
    let h = ui_styling::metrics::dag::MINIMAP_WIDGET_HEIGHT;
    let margin = ui_styling::metrics::dag::MINIMAP_WIDGET_MARGIN;
    let ratio = ui_styling::metrics::dag::MINIMAP_WIDGET_MAX_CONTENT_RATIO;
    let cam = &graph.host_snapshot.camera;
    let layout =
        ui_wgpu::wgpu::minimap::layout(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, w, h, margin, ratio);
    Some(MinimapWidgetLayout { panel: layout.panel, world_min_x: layout.world_min_x, world_min_y: layout.world_min_y, scale: layout.scale, map_origin_x: layout.map_origin_x, map_origin_y: layout.map_origin_y, viewport: layout.viewport })
}

pub(crate) fn minimap_widget_pointer_hit(viewport_state: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> Option<(MinimapWidgetLayout, bool)> {
    let layout = minimap_widget_layout(graph, minimap, viewport_state.width, viewport_state.height)?;
    if !minimap_widget_point_in_rect(layout.panel, sx, sy) {
        return None;
    }
    let on_viewport = minimap_widget_point_in_rect(layout.viewport, sx, sy);
    Some((layout, on_viewport))
}

fn minimap_widget_cursor_hint(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState) -> Option<&'static str> {
    if minimap.minimap_widget_drag.is_some() {
        return Some("grabbing");
    }
    if let Some((layout, on_viewport)) = minimap_widget_pointer_hit(viewport, graph, minimap, viewport.last_screen_x, viewport.last_screen_y) {
        let _ = layout;
        return Some(if on_viewport { "grab" } else { "pointer" });
    }
    None
}

pub(crate) fn minimap_widget_json(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState) -> Option<Value> {
    let layout = minimap_widget_layout(graph, minimap, viewport.width, viewport.height)?;
    let (x0, y0, x1, y1) = layout.panel;
    Some(semio_framework_pack_json::object([
        ("x".to_string(), Value::from(x0)),
        ("y".to_string(), Value::from(y0)),
        ("width".to_string(), Value::from(x1 - x0)),
        ("height".to_string(), Value::from(y1 - y0)),
        ("cursor".to_string(), Value::from(minimap_widget_cursor_hint(viewport, graph, minimap))),
    ]))
}

pub(crate) fn paint_minimap_widget(viewport_state: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, minimap: &DagMinimapState, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32) {
    let Some(layout) = minimap_widget_layout(graph, minimap, viewport_w, viewport_h) else {
        return;
    };
    use canvas::{Affine, FillRule, Rect, Stroke};
    use ui_styling::strokes;
    let theme = &viewport_state.canvas_theme;
    let aff = Affine::IDENTITY;
    let (px0, py0, px1, py1) = layout.panel;
    let panel = Rect::new(px0, py0, px1, py1);
    scene.fill(FillRule::NonZero, aff, theme.minimap_widget_panel_fill, None, &panel);
    scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_PANEL), aff, theme.minimap_widget_panel_stroke, None, &panel);
    let node_min = ui_styling::metrics::dag::MINIMAP_WIDGET_NODE_MIN_SIZE;
    let lod = DagDrawLod::Minimap;
    for (idx, snapshot_node) in graph.host_snapshot.nodes.iter().enumerate() {
        let node = graph.node_spec_for_paint(idx, snapshot_node);
        let node = node.as_ref();
        let engine_nid = graph.engine_node_id_for_index(idx);
        let is_dimmed = engine_nid.is_some_and(|nid| paint.dimmed.contains(&nid));
        let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| graph.node_interaction_chrome(nid));
        let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) else {
            continue;
        };
        let hw = (node.width * layout.scale * 0.5).max(node_min * 0.5);
        let hh = (node.height * layout.scale * 0.5).max(node_min * 0.5);
        let cx = layout.map_origin_x + (node.x - layout.world_min_x) * layout.scale;
        let cy = layout.map_origin_y + (node.y - layout.world_min_y) * layout.scale;
        let rect = Rect::new(cx - hw, cy - hh, cx + hw, cy + hh);
        scene.fill(FillRule::NonZero, aff, fill, None, &rect);
    }
    let (vx0, vy0, vx1, vy1) = layout.viewport;
    let view_rect = Rect::new(vx0, vy0, vx1, vy1);
    let active = minimap.minimap_widget_hovered || minimap.minimap_widget_drag.is_some();
    scene.fill(FillRule::NonZero, aff, theme.minimap_widget_viewport_fill, None, &view_rect);
    let stroke_c = if active { theme.minimap_widget_viewport_stroke_hovered } else { theme.minimap_widget_viewport_stroke };
    scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_VIEWPORT), aff, stroke_c, None, &view_rect);
}
// #endregion 🔖️MinimapWidget

impl RetireOwned for DagPointerState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{widget_drag,pending_port_insert,ghost_node,pending_cluster_explode,pending_export_click,pending_open_instance_id,last_pointer_down_at_ms,last_pointer_down_world,last_pointer_down_node_id}=self;sequence(vec![deferred(widget_drag),deferred(pending_port_insert),deferred(ghost_node),deferred(pending_cluster_explode),deferred(pending_export_click),deferred(pending_open_instance_id),deferred(last_pointer_down_at_ms),deferred(last_pointer_down_world),deferred(last_pointer_down_node_id)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.widget_drag),deferred_birth_bytes_for(&self.pending_port_insert),deferred_birth_bytes_for(&self.ghost_node),deferred_birth_bytes_for(&self.pending_cluster_explode),deferred_birth_bytes_for(&self.pending_export_click),deferred_birth_bytes_for(&self.pending_open_instance_id),deferred_birth_bytes_for(&self.last_pointer_down_at_ms),deferred_birth_bytes_for(&self.last_pointer_down_world),deferred_birth_bytes_for(&self.last_pointer_down_node_id)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagPaintState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{dimmed,icon_paint_cache,computing_active,computing_stale,computing_active_anim_phase,computing_stale_anim_phase,node_eval_status,unresolved_input_ports}=self;sequence(vec![deferred(dimmed),deferred(icon_paint_cache),deferred(computing_active),deferred(computing_stale),deferred(computing_active_anim_phase.into_inner()),deferred(computing_stale_anim_phase.into_inner()),deferred(node_eval_status),deferred(unresolved_input_ports)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.dimmed),deferred_birth_bytes_for(&self.icon_paint_cache),deferred_birth_bytes_for(&self.computing_active),deferred_birth_bytes_for(&self.computing_stale),deferred_birth_bytes_for(&self.computing_active_anim_phase.get()),deferred_birth_bytes_for(&self.computing_stale_anim_phase.get()),deferred_birth_bytes_for(&self.node_eval_status),deferred_birth_bytes_for(&self.unresolved_input_ports)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagMinimapState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{minimap_widget_visible,minimap_widget_hovered,minimap_widget_drag}=self;sequence(vec![deferred(minimap_widget_visible),deferred(minimap_widget_hovered),deferred(minimap_widget_drag)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.minimap_widget_visible),deferred_birth_bytes_for(&self.minimap_widget_hovered),deferred_birth_bytes_for(&self.minimap_widget_drag)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagNotesState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{editing_note,caret_visible}=self;sequence(vec![deferred(editing_note),deferred(caret_visible)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.editing_note),deferred_birth_bytes_for(&self.caret_visible)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagHost {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{viewport,graph,pointer,paint,minimap,notes}=self;sequence(vec![deferred(viewport),deferred(graph),deferred(pointer),deferred(paint),deferred(minimap),deferred(notes)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.viewport),deferred_birth_bytes_for(&self.graph),deferred_birth_bytes_for(&self.pointer),deferred_birth_bytes_for(&self.paint),deferred_birth_bytes_for(&self.minimap),deferred_birth_bytes_for(&self.notes)])}
 fn controlled_retirement_supported()->bool{true}
}

/// 🌳️ Retains the whole original DAG, including engine, scene, pending interaction and journal sources.
#[must_use="the original DAG must reach physical terminal emptiness"]
pub struct DagHostRetirement {pub owner:ControlledRetirement<DagHost>}

impl DagHostRetirement {
 pub fn new(source:DagHost)->Self{Self{owner:ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("original DAG fields declare typed retirement"))}}
 pub fn original(&self)->Option<&DagHost>{self.owner.original()}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
}
