//! 🕸️ Dag board graph and viewport components with the engine sync, selection and projection functions.
//!
//! Crate `semio-framework-os-infinite-board-dag-graph`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{channel, DagChannelDirection, DagChannelRef, DagChannelView, DagHoverFacts, DagLayoutOptions, DagSelectionDomains, DagWireTypeRefusal, LayoutControl, refusal, Selection};
use semio_framework_os_infinite_board_engine::{BoardEvent, Camera, CanvasPalette, distance, GRID_WORLD_LARGE, GRID_WORLD_MEDIUM, GRID_WORLD_MICRO, GRID_WORLD_SMALL, handle_position, InteractionMode};
use semio_framework_os_infinite_board_normal_model::handle_port_shape;
use semio_framework_os_infinite_board_dag_model::{action_control_bounds, apply_dag_layout, channel_row_bounds, channel_row_divider_y, cluster_explode_hit, computation_channel_row_count, computation_channel_row_divider_x_span, computation_input_column_x_bounds, computation_io_side_row_counts, computation_io_side_row_divider_indices, computation_output_column_x_bounds, ComputationChannelRowSide, dag_bit_contains, dag_bit_set, DAG_CHROME_STROKE_SCREEN_PX, DAG_COMPUTATION_HEADER_ROWS, DAG_DISTRIBUTE_MIN_GAP, dag_drag_gesture_id, dag_draw_lod, DAG_GRAPH_EDIT_CAPACITY, dag_handle_body_fill, dag_handle_body_stroke, DAG_HANDLE_WORLD_RADIUS, DAG_INTERACTION_NODE_CAPACITY, DAG_INTERACTION_WORD_CAPACITY, DAG_LABEL_SCREEN_PX, dag_node_internal_chrome_stroke, dag_node_world_bounds, dag_port_endpoint_parts, dag_port_handle_key, DAG_SLIDER_VALUE_LABEL_RATIO, dag_stroke_world_step_grid, DAG_VARIADIC_PLUS_ZOOM_THRESHOLD, dag_world_stroke, DagBoardEngine, DagError, DagGraphEdit, DagInteractionPlanFault, DagInteractionProjection, DagJournalRefusal, DagNodePaintChrome, DagPointerPlan, DagPortSide, DagProjectionGesture, fit_node_size, handle_cap_fill_path, handle_cap_peak, handle_cap_stroke_path, Identity, IdentityStep, input_port_connector_bounds, input_port_row_hit_bounds, io_node_rect_port_angle_for_node, output_port_connector_bounds, output_port_row_hit_bounds, paint_computation_column_divider, parse_snapshot_edge_numeric_id, point_in_rect, port_value_types, preview_tree_row_layouts, select_control_bounds, slider_track_bounds, WidgetPointerKind, would_create_cycle};
use semio_framework_canvas::{Circle, FillRule, Rect, Stroke};
use graph::{EdgeId, HandleId, HandleRole, NodeId, NodeShape};
use geometry::geom_sel::{world_box_from_points, WorldBox};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
use semio_framework_os_kernel::os_spr::causal::transition::HistoryFoldIndex;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use geometry::Point;
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use ::{graph, semio_framework_canvas as canvas};
use semio_framework_graph_drawing::routing::{compute_edge_sharp_sz_path, handle_outward_at_node_rim, handle_position_on_rectangle};
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_value::retirement::*;

#[derive(Default)]
pub struct IdentityCursor{pub candidate:usize,pub byte:usize}

impl IdentityCursor{
 pub fn step(&mut self,graph: &DagGraphState,target:Identity<'_>,domain:usize,node:Option<u64>)->IdentityStep{
  let count=match domain{0=>graph.host_snapshot.nodes.len(),1=>graph.host_snapshot.edges.len(),2=>graph.handle_key_map.len(),_=>node.and_then(|id|graph.host_snapshot.nodes.get((id-1)as usize)).map_or(0,|node|node.inputs().len())};
  if self.candidate==count{return IdentityStep::Missing}
  let id=match domain{0=>self.candidate as u64+1,1=>graph.edge_engine_ids.get(self.candidate).and_then(|id|*id).unwrap_or(0),2=>self.candidate as u64+10,_=>self.candidate as u64};
  let candidate=match domain{0=>graph.host_snapshot.nodes.get(self.candidate).filter(|_|graph.node_id_map.get(&id)==Some(&self.candidate)).map(|node|node.id.as_str()),1=>graph.host_snapshot.edges.get(self.candidate).filter(|_|id!=0).map(|edge|edge.id.as_str()),2=>graph.handle_key_map.get(&id).filter(|_|graph.engine.handles.contains_key(&id)).map(String::as_str),_=>node.and_then(|id|graph.host_snapshot.nodes.get((id-1)as usize)).and_then(|node|node.inputs().get(self.candidate)).map(|port|port.id.as_str())};
  let Some(candidate)=candidate else{self.candidate+=1;self.byte=0;return IdentityStep::Progress};
  if candidate.len()!=target.len()||candidate.as_bytes().get(self.byte).copied()!=target.byte(self.byte){self.candidate+=1;self.byte=0;return IdentityStep::Progress}
  if self.byte==target.len(){return IdentityStep::Resolved(id)}self.byte+=1;IdentityStep::Progress
 }
}

semio_framework_value::artifact_retire_struct!(IdentityCursor{candidate,byte});

impl DagGraphState {
    pub fn edge_sharp_path(&self, eid: EdgeId) -> Option<canvas::BezPath> {
        let edge = self.engine.edges.get(&eid)?;
        let source_handle = self.engine.handles.get(&edge.source)?;
        let target_handle = self.engine.handles.get(&edge.target)?;
        let source_node = self.engine.nodes.get(&source_handle.node_id)?;
        let target_node = self.engine.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        let source_out = handle_outward_at_node_rim(source_position, source_node.center, source_node.shape, source_node.radius, source_node.width, source_node.height)?;
        let target_out = handle_outward_at_node_rim(target_position, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height)?;
        let source_shape = self.handle_port_shape.get(&edge.source).copied().unwrap_or_default();
        let target_shape = self.handle_port_shape.get(&edge.target).copied().unwrap_or_default();
        let source_wire = handle_cap_peak(source_position, source_out, source_handle.radius, source_shape);
        let target_wire = handle_cap_peak(target_position, target_out, target_handle.radius, target_shape);
        Some(compute_edge_sharp_sz_path(source_wire, target_wire, source_out, target_out))
    }

    pub fn paint_computation_channel_row_highlights(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, theme: &CanvasPalette, is_dimmed: bool) {
        use canvas::FillRule;
        use canvas::Rect;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(FillRule::NonZero, *aff, fill, None, &Rect::new(x0, y0, x1, y1));
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            let Some(bounds) = input_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            let Some(bounds) = output_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
    }

    fn computation_channel_row_divider_stroke(&self, node: &DagNodeSpec, port_id: &str, body_stroke: canvas::Color, label_fill: canvas::Color, default_stroke: canvas::Color) -> canvas::Color {
        let Some(hid) = self.handle_id_for_port(&node.id, port_id) else {
            return default_stroke;
        };
        let (selected, _, hovered) = self.handle_interaction_chrome(hid);
        if selected || hovered {
            dag_node_internal_chrome_stroke(body_stroke, label_fill, true)
        } else {
            default_stroke
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub fn paint_computation_channel_row_dividers(&self, scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color, channel_row_pick: bool) {
        use canvas::{Line, Point, Stroke};
        let grid_rows = computation_channel_row_count(node);
        if grid_rows <= 1 {
            return;
        }
        let (input_rows, output_rows) = computation_io_side_row_counts(node);
        let stroke_style = Stroke::new(chrome_stroke);
        let row_stroke = |port_id: &str| {
            if channel_row_pick {
                self.computation_channel_row_divider_stroke(node, port_id, body_stroke, label_fill, stroke)
            } else {
                stroke
            }
        };
        if computation_input_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input);
            let inputs = node.inputs();
            for row in computation_io_side_row_divider_indices(input_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = inputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
    }

    /// 📐️ Center divider and channel-row chrome for a one-sided or two-sided widget.
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub fn paint_sided_widget_columns(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, lod: DagDrawLod, node: &DagNodeSpec, theme: &CanvasPalette, chrome: &DagNodePaintChrome, chrome_stroke: f64, internal_stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color) {
        if !lod.shows_computation_layout() {
            return;
        }
        let channel_row_pick = lod.uses_channel_row_pick();
        if channel_row_pick {
            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
        }
        paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_stroke);
        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_stroke, body_stroke, label_fill, channel_row_pick);
    }
}

impl DagViewportState {
    pub fn paint_node_handles_for_spec(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, node: &DagNodeSpec, chrome: &DagNodePaintChrome) {
        use canvas::FillRule;
        use canvas::Stroke;
        use canvas::{Circle, Point};
        use graph::NodeShape;
        use semio_framework_graph_drawing::routing::{handle_outward_at_node_rim, handle_position_on_rectangle};

        let theme = &self.canvas_theme;
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let tint = chrome.tint_highlighted();
        let fill = dag_handle_body_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let stroke_c = dag_handle_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let handle_chrome = chrome.has_interaction_chrome();
        let center = Point::new(node.x, node.y);
        let paint_port = |scene: &mut canvas::Scene, port_idx: usize, inputs: bool, port: &IoPortSpec| {
            if !port.visible {
                return;
            }
            let angle = io_node_rect_port_angle_for_node(node, port_idx, inputs);
            let handle_center = handle_position_on_rectangle(center, node.width, node.height, angle);
            let outward = handle_outward_at_node_rim(handle_center, center, NodeShape::Rectangle, 0.0, node.width, node.height);
            if let Some(out) = outward {
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &handle_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &handle_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
            } else {
                let circle = Circle::new(handle_center, DAG_HANDLE_WORLD_RADIUS);
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &circle);
            }
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            paint_port(scene, port_idx, true, port);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            paint_port(scene, port_idx, false, port);
        }
    }
}

/// 🖼️ Screen-node overlay rects in CSS pixel space for DOM media layers.
pub fn dag_node_overlays_json(viewport_state: &DagViewportState, graph: &DagGraphState) -> Result<String, DagError> {
    use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
    let mut overlays = Vec::new();
    for node in &graph.host_snapshot.nodes {
        let DagNodeKind::Screen { media: Some(media), .. } = &node.kind else {
            continue;
        };
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let inset = 8.0 / cam.zoom.max(0.05);
        let top = node.y - hh + hh * 0.35;
        let bottom = node.y + hh - inset;
        let left = node.x - hw + inset;
        let right = node.x + hw - inset;
        let tl = world_to_screen(&cam, &viewport, Point::new(left, top));
        let br = world_to_screen(&cam, &viewport, Point::new(right, bottom));
        let media_kind = match media.kind {
            DagMediaKind::Image => "image",
            DagMediaKind::Svg => "svg",
            DagMediaKind::Pdf => "pdf",
            DagMediaKind::Video => "video",
        };
        overlays.push(semio_framework_pack_json::object([
            ("id".to_string(), Value::from(node.id.clone())),
            ("mediaKind".to_string(), Value::from(media_kind)),
            ("src".to_string(), Value::from(media.src.clone())),
            (
                "rect".to_string(),
                semio_framework_pack_json::object([("x".to_string(), Value::from(tl.x)), ("y".to_string(), Value::from(tl.y)), ("w".to_string(), Value::from((br.x - tl.x).max(1.0))), ("h".to_string(), Value::from((br.y - tl.y).max(1.0)))]),
            ),
        ]));
    }
    Ok(semio_framework_pack_json::to_string(&Value::Array(overlays)))
}

/// 🎚️ Slider track anchors for the HTML slider overlay — the ONE row that carries a knob's value.
///
/// 🔢 `value` is published exactly once and the overlay renders it twice: as the control's own
/// value (hence `aria-valuenow`) and as the readout beside the track. The readout used to be
/// painted on the GPU instead, which made it a second producer that could only move when a frame
/// was drawn — a released knob then read `10` to a screen reader and `0.0` on the canvas. The
/// readout's typography rides along as `fontScreenPx`/`gapScreenPx` so the DOM lands where the
/// paint did without re-deriving the board's own metrics
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️slider-reevaluation-correctness-2026-09-15.md`).
pub fn slider_overlay_state_json(viewport: &DagViewportState, graph: &DagGraphState) -> Result<String, DagError> {
    let cam = &graph.host_snapshot.camera;
    let mut sliders: Vec<Value> = Vec::new();
    for (idx, snapshot_node) in graph.host_snapshot.nodes.iter().enumerate() {
        let node = graph.node_spec_for_paint(idx, snapshot_node);
        let DagNodeKind::Slider { min, max, step, value, .. } = &node.kind else {
            continue;
        };
        let (x0, y0, x1, y1) = slider_track_bounds(&node);
        sliders.push(semio_framework_pack_json::object([
            ("widgetId".to_string(), Value::from(snapshot_node.id.clone())),
            ("label".to_string(), Value::from(node.name.clone())),
            ("value".to_string(), Value::from(*value)),
            ("min".to_string(), Value::from(*min)),
            ("max".to_string(), Value::from(*max)),
            ("step".to_string(), Value::from(*step)),
            ("x".to_string(), Value::from((x0 + x1) * 0.5)),
            ("y".to_string(), Value::from((y0 + y1) * 0.5)),
            ("w".to_string(), Value::from((x1 - x0).max(1.0))),
            ("h".to_string(), Value::from((y1 - y0).max(1.0))),
            ("fontScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * DAG_SLIDER_VALUE_LABEL_RATIO)),
            ("gapScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_RATIO)),
        ]));
    }
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
        ("width".to_string(), Value::from(viewport.width)),
        ("height".to_string(), Value::from(viewport.height)),
        ("sliders".to_string(), Value::Array(sliders)),
    ])))
}

impl DagGraphState {
    /// 📍️ Sets a snapshot widget position in both the snapshot and engine snapshots.
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), DagError> {
        let idx = self.host_snapshot.nodes.iter().position(|node| node.id == widget_id).ok_or_else(|| DagError::UnknownWidget(widget_id.to_string()))?;
        self.host_snapshot.nodes[idx].x = x;
        self.host_snapshot.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center = canvas::Point::new(x, y);
        }
        Ok(())
    }

    /// 🗑️ Deletes the current selection from the snapshot.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.host_snapshot.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.sync_edges_from_engine();
        self.rebuild_engine_with_layout(false);
    }

    /// ⌨️ Selects every snapshot node id.
    pub fn select_all_node_ids(&self) -> Vec<String> {
        self.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect()
    }

    pub fn select_all(&mut self) {
        self.engine.select_all();
    }

    /// 🖱️ Sets hover to a snapshot widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        let next = widget_id.and_then(|id| self.node_id_for_widget_id(id));
        if self.engine.hover != next {
            self.engine.hover = next;
        }
    }

    /// 🎯️ Hit-tests variadic `+` controls; returns side, node id, and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(DagPortSide, String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.host_snapshot.nodes.iter().rev() {
            if node.variadic_inputs() {
                let inputs = node.inputs();
                let hw = node.width * 0.5;
                let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, hit_x1, y1) = {
                    let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                    (x0, y0, node.x - hw * 0.5, y1)
                };
                if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                    return Some((DagPortSide::Input, node.id.clone(), inputs.len()));
                }
            }
            if node.variadic_outputs() {
                let outputs = node.outputs();
                let row = outputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, x1, y1) = channel_row_bounds(node, row);
                let hit_x0 = computation_output_column_x_bounds(node).map_or(x0, |(left, _)| left);
                if point_in_rect(world_x, world_y, hit_x0, y0, x1, y1) {
                    return Some((DagPortSide::Output, node.id.clone(), outputs.len()));
                }
            }
        }
        None
    }

    /// 🔗️ World-space proximity radius for channel auto-connect; `0` disables snapping.
    pub fn set_proximity_distance(&mut self, world: f64) {
        self.engine.proximity_distance_world = world.max(0.0);
    }

    pub fn host_snapshot_json(&self) -> Result<String, DagError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    /// 🌳️ Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions,control:&mut LayoutControl<'_>) -> Result<(), DagError> {
        apply_dag_layout(&mut self.host_snapshot,opts,control)?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    pub fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.handle_port_shape.clear();
        self.handle_port_visible.clear();
        self.edge_id_map.clear();
        self.edge_engine_ids.clear();
        self.edge_engine_ids.resize(self.host_snapshot.edges.len(), None);
        self.edge_route_style.clear();
        for node in &mut self.host_snapshot.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut progress=|_|true;let mut control=LayoutControl::new(100_000_000,&mut progress);
            let _=apply_dag_layout(&mut self.host_snapshot,&DagLayoutOptions::default(),&mut control);
        }
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.host_snapshot.nodes.iter().enumerate() {
            let nid = idx as u64 + 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(dag_port_handle_key(&node.id, &port.id, true), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, in_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Target);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
            for (port_idx, port) in outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle_for_node(node, port_idx, false);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(dag_port_handle_key(&node.id, &port.id, false), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, out_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Source);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
        }
        let existing: Vec<(String, String)> = self
            .host_snapshot
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split('@').next()?.to_string();
                let tgt = e.target.split('@').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for (edge_index, edge) in self.host_snapshot.edges.iter().enumerate() {
            if would_create_cycle(&existing, edge.source.split('@').next().unwrap_or(""), edge.target.split('@').next().unwrap_or("")) {
                continue;
            }
            let (source_node, source_port) = dag_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = dag_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&dag_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&dag_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = parse_snapshot_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
                self.edge_engine_ids[edge_index] = Some(id);
                self.edge_route_style.insert(id, edge.route_style);
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    pub fn node_spec_for_paint<'a>(&'a self, idx: usize, node: &'a DagNodeSpec) -> std::borrow::Cow<'a, DagNodeSpec> {
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return std::borrow::Cow::Borrowed(node);
        };
        let Some(engine_node) = self.engine.nodes.get(&nid) else {
            return std::borrow::Cow::Borrowed(node);
        };
        if (engine_node.center.x - node.x).abs() < 1e-9 && (engine_node.center.y - node.y).abs() < 1e-9 {
            return std::borrow::Cow::Borrowed(node);
        }
        let mut synced = node.clone();
        synced.x = engine_node.center.x;
        synced.y = engine_node.center.y;
        std::borrow::Cow::Owned(synced)
    }

    pub fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        let mut edge_engine_ids = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagHostSnapshotEdge { id, source, target, ..Default::default() });
            edge_engine_ids.push(Some(*eid));
        }
        self.host_snapshot.edges = edges;
        self.edge_engine_ids = edge_engine_ids;
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.host_snapshot.camera = DagCamera { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }
}

impl DagGraphState {
    /// 🔗️ Records one completed connection in the guest's own vocabulary. `handle_key_map` already
    /// keys every handle as `"{nodeId}@{portId}"` — the same grammar the pick-target/hover channel
    /// uses — so this is a lookup, never a second derivation of what a port is.
    fn journal_connect(&mut self, source: HandleId, target: HandleId) {
        let Some((source_node_id, source_port_id)) = self.handle_key_map.get(&source).and_then(|key| key.split_once('@')) else { return };
        let Some((target_node_id, target_port_id)) = self.handle_key_map.get(&target).and_then(|key| key.split_once('@')) else { return };
        let edit = DagGraphEdit::Connect { source_node_id: source_node_id.to_string(), source_port_id: source_port_id.to_string(), target_node_id: target_node_id.to_string(), target_port_id: target_port_id.to_string() };
        self.push_graph_edit(edit);
    }

    /// 🔗️ Records one removed wire by the SYNAPSE id the guest knows it as. An edge the engine
    /// created in this same gesture has no synapse id yet and is simply not journalled — the guest
    /// never learned about it, so it has nothing to remove.
    fn journal_disconnect(&mut self, edge: EdgeId) {
        let Some(synapse_id) = self.edge_id_map.get(&edge).cloned() else { return };
        self.push_graph_edit(DagGraphEdit::Disconnect { synapse_id });
    }

    /// 🔗️ Journals one edit a press performed (a wire, a slider value, a port); an edit already pending is the same edit
    /// and is not journalled twice. A press journals a bounded handful, so these never compete with
    /// [`DAG_GRAPH_EDIT_CAPACITY`]; only moves grow with the graph and are reserved whole ([`Self::journal_moves`]).
    pub fn push_graph_edit(&mut self, edit: DagGraphEdit) {
        if !self.pending_graph_edits.contains(&edit) {
            self.pending_graph_edits.push(edit);
        }
    }

    /// 🔗️ Drains the wire edits this host performed since the last call — the renderer's one read
    /// point after a completed pointer gesture.
    pub fn take_graph_edits(&mut self) -> Vec<DagGraphEdit> {
        std::mem::take(&mut self.pending_graph_edits)
    }

    /// 👁️ The journalled graph edits not yet drained, in gesture order — read without draining so the embedding host can record
    /// the gesture's own concrete edits as history leaves before the renderer drains them ([`Self::take_graph_edits`]).
    pub fn graph_edits(&self) -> &[DagGraphEdit] {
        &self.pending_graph_edits
    }

    /// 🚫️ Drains why the last gesture was refused whole, if it was.
    pub fn take_journal_refusal(&mut self) -> Option<DagJournalRefusal> {
        self.journal_refusal.take()
    }

    /// 🚫️ Keeps a refusal the embedding host carried across rebuilding this host from the gesture's baseline.
    pub fn carry_journal_refusal(&mut self, refusal: DagJournalRefusal) {
        self.journal_refusal = Some(refusal);
    }

    /// ✋️ Journals what a gesture moved — `displacements` of `(node id, dx, dy)` in node order — as node-graph gesture records
    /// of `gesture_id` (design §13.3): one record per distinct offset, the nodes of each in order; a zero offset moved
    /// nothing. All or nothing: when the records do not fit the journal ([`DAG_GRAPH_EDIT_CAPACITY`]) the whole gesture is
    /// refused — the journal is emptied, the refusal kept for the renderer — and the embedding host restores the gesture's
    /// baseline, so neither the document nor the host keeps any of it.
    pub fn journal_moves(&mut self, gesture_id: &str, displacements: impl IntoIterator<Item = (String, f64, f64)>) -> Result<(), DagJournalRefusal> {
        let mut drags: Vec<(f64, f64, Vec<String>)> = Vec::new();
        for (id, dx, dy) in displacements {
            if (dx, dy) == (0.0, 0.0) {
                continue;
            }
            match drags.iter_mut().find(|(known_x, known_y, _)| (*known_x, *known_y) == (dx, dy)) {
                Some((_, _, ids)) => ids.push(id),
                None => drags.push((dx, dy, vec![id])),
            }
        }
        let rows = self.pending_graph_edits.len() + drags.len();
        if rows > DAG_GRAPH_EDIT_CAPACITY {
            let refusal = DagJournalRefusal { rows, limit: DAG_GRAPH_EDIT_CAPACITY };
            self.pending_graph_edits.clear();
            self.journal_refusal = Some(refusal);
            return Err(refusal);
        }
        self.pending_graph_edits.extend(drags.into_iter().map(|(dx, dy, node_ids)| DagGraphEdit::Move { gesture_id: gesture_id.to_string(), node_ids, dx, dy }));
        Ok(())
    }

    /// 📍️ Every node's id and centre, in node order — the baseline an embedding host records at a press so
    /// [`Self::journal_moves_since`] can narrate what the gesture moved.
    pub fn node_positions(&self) -> Vec<(String, f64, f64)> {
        self.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect()
    }

    /// ✋️ Journals every node that moved since `baseline` ([`Self::node_positions`] at the press) — a drag's release or an
    /// align — through [`Self::journal_moves`]; a node the baseline does not know is not narrated.
    pub fn journal_moves_since(&mut self, gesture_id: &str, baseline: &[(String, f64, f64)]) -> Result<(), DagJournalRefusal> {
        let displacements: Vec<(String, f64, f64)> = self.host_snapshot.nodes.iter().filter_map(|node| baseline.iter().find(|(id, _, _)| *id == node.id).map(|(_, x, y)| (node.id.clone(), node.x - x, node.y - y))).collect();
        self.journal_moves(gesture_id, displacements)
    }

    /// ↩️ Puts every node `baseline` names back where it stood — the zero trace of a gesture refused whole.
    pub fn restore_node_positions(&mut self, baseline: &[(String, f64, f64)]) {
        for (id, x, y) in baseline {
            let Some(index) = self.host_snapshot.nodes.iter().position(|node| node.id == *id) else { continue };
            (self.host_snapshot.nodes[index].x, self.host_snapshot.nodes[index].y) = (*x, *y);
            self.sync_snapshot_node_center_to_engine(index);
        }
    }

    /// 🔌️ Journals a variadic port the embedding host inserted on node `node_id` at `index` of `side`.
    pub fn journal_port_insert(&mut self, node_id: String, side: DagPortSide, index: usize) {
        self.push_graph_edit(DagGraphEdit::InsertPort { node_id, side, index });
    }

    pub(crate) fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        self.handle_key_map.iter().filter(|(_, k)| k.as_str() == key).map(|(&hid, _)| hid).min()
    }

    /// 🔌️ The handle on ONE side of a port. `handle_key_map` keys every handle as `"{nodeId}@{portId}"`,
    /// and a node may carry an input AND an output under the same port id (`extrusion-axis` carries
    /// `vector`, `x`, `y`, `z` on both sides): a name-only lookup then answers whichever handle came
    /// first in the map, so a press on an OUTPUT could resolve to the input handle 40 world units away
    /// on the other side of the node, snap there, and start a wire from the wrong end. The engine
    /// already records the side as [`HandleRole`]; this reads it rather than keeping a second map.
    pub(crate) fn handle_id_for_port_side(&self, node_id: &str, port_id: &str, input: bool) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        let wanted = if input { HandleRole::Target } else { HandleRole::Source };
        self.handle_key_map
            .iter()
            .filter(|(_, candidate)| candidate.as_str() == key)
            .map(|(&hid, _)| hid)
            .find(|hid| self.engine.handles.get(hid).is_some_and(|handle| handle.role == wanted))
            .or_else(|| self.handle_id_for_port(node_id, port_id))
    }

    fn port_row_handle_hit(&self, world_x: f64, world_y: f64, inputs: bool, outputs: bool) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            if inputs {
                for (port_idx, port) in node.inputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = input_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            if outputs {
                for (port_idx, port) in node.outputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = output_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    /// 🔌️ The port whose CONNECTOR rect — the published one — contains this world point.
    fn port_connector_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            for (port_idx, port) in node.inputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = input_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            for (port_idx, port) in node.outputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = output_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    pub fn snapshot_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw || world_x > node.x + hw || world_y < node.y - hh || world_y > node.y + hh {
                continue;
            }
            let Some(nid) = self.engine_node_id_for_index(idx) else {
                continue;
            };
            if self.engine.nodes.get(&nid).is_some_and(|n| n.draggable) {
                return Some(nid);
            }
        }
        None
    }

    /// 🔌️ A press on a port ROW never arrives here any more — `port_pointer_handle_hit` claims it
    /// first, and the engine's own `HitObject::Endpoint` arm already selects that channel before it
    /// begins the wire. The channel-row pick this branch used to duplicate is that selection.
    pub fn try_node_rectangle_pointer_down(&mut self, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) -> bool {
        if button != 0 || alt {
            return false;
        }
        let Some(node_id) = self.snapshot_draggable_node_hit(world_x, world_y) else {
            return false;
        };
        use canvas::Point;

        let point = Point::new(world_x, world_y);
        self.engine.pointer_down_on_draggable_node_at(node_id, point, shift, ctrl_or_meta);
        true
    }

    pub fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, WidgetPointerKind)> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SliderDrag));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SelectClick));
                    }
                }
                DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded, .. } => {
                    for row in preview_tree_row_layouts(node, json, expanded) {
                        let (x0, y0, x1, y1) = row.row_rect;
                        if x1 > x0 && point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                            return Some((idx, WidgetPointerKind::PreviewToggle(row.path)));
                        }
                    }
                }
                DagNodeKind::Cluster { .. } => {
                    if cluster_explode_hit(node, world_x, world_y) {
                        return Some((idx, WidgetPointerKind::ClusterExplode));
                    }
                }
                DagNodeKind::Export { .. } => {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::ExportClick));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn sync_snapshot_node_size_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return;
        };
        let Some(engine_node) = self.engine.nodes.get_mut(&nid) else {
            return;
        };
        engine_node.width = node.width;
        engine_node.height = node.height;
    }

    /// 📐️ Recomputes preview and image node sizes after content changes.
    pub fn fit_preview_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            let kind = &self.host_snapshot.nodes[idx].kind;
            if matches!(kind, DagNodeKind::Preview { .. } | DagNodeKind::Image { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }

    /// 📐️ Recomputes note node sizes after text changes.
    pub fn fit_note_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            if matches!(self.host_snapshot.nodes[idx].kind, DagNodeKind::Note { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }
}

impl DagViewportState {
    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    pub fn automatic_lod(&self) -> bool {
        self.automatic_lod
    }

    pub fn forced_draw_lod_label(&self) -> Option<&'static str> {
        self.forced_draw_lod.map(|lod| lod.label())
    }

    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier.
    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    /// 🔲️ Toggles LOD-tiered world grid painting.
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.grid_visible = visible;
    }

    /// 🧲️ Toggles node-drag snap to the finest visible LOD grid step.
    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    /// 📐️ Sets the positive multiplier for LOD world grid steps.
    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), DagError> {
        if !factor.is_finite() || factor <= 0.0 || factor > 1e6 {
            return Err(DagError::GridFactorOutOfRange);
        }
        self.grid_factor = factor;
        Ok(())
    }

    pub fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }

    pub fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }

    pub fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }

    pub fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    pub fn paint_lod_grid(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod) {
        if !self.grid_visible || self.wheel_zoom_active || lod == DagDrawLod::Minimap {
            return;
        }
        let grid_color = self.canvas_theme.grid_minor_stroke;
        dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
        match lod {
            DagDrawLod::Normal | DagDrawLod::Detail | DagDrawLod::Micro => {
                dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
            }
            DagDrawLod::Minimap | DagDrawLod::Overview | DagDrawLod::Compact => {}
        }
        if matches!(lod, DagDrawLod::Detail | DagDrawLod::Micro) {
            dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
        }
        if lod == DagDrawLod::Micro {
            dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
        }
    }

    /// 📶️ Pins WASM draw LOD when {@link DagHost::set_automatic_lod} is false; pass an empty label to follow zoom bands.
    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = DagDrawLod::from_id(trimmed);
    }
}

/// 🔌️ Sets hover to a snapshot channel handle, falling back to node hover below channel LOD.
pub fn set_hover_channel(viewport: &mut DagViewportState, graph: &mut DagGraphState, widget_id: Option<&str>, port_id: Option<&str>) {
    let Some(widget_id) = widget_id else {
        graph.set_hover(None);
        return;
    };
    if let Some(port_id) = port_id {
        if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
            if let Some(hid) = graph.handle_id_for_port(widget_id, port_id) {
                if graph.engine.hover != Some(hid) {
                    graph.engine.hover = Some(hid);
                }
                return;
            }
        }
    }
    graph.set_hover(Some(widget_id));
}

/// 🔌️ Replaces typed channel selection, using node selection below channel LOD.
pub fn set_selected_channels(viewport: &mut DagViewportState, graph: &mut DagGraphState, channels: &[DagChannelRef]) {
    if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        let mut selection = Selection::default();
        for channel in channels {
            if let Some(hid) = graph.handle_id_for_port(&channel.widget_id, &channel.port) {
                selection.handle_ids.insert(hid);
            }
        }
        graph.engine.selection = selection;
        graph.engine.preselect = Selection::default();
        graph.engine.preselect_removed = Selection::default();
        return;
    }
    let widget_ids: Vec<String> = channels.iter().map(|channel| channel.widget_id.clone()).collect();
    graph.set_selection(&widget_ids);
}

/// 🔍️ Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
pub fn dag_set_wheel_zoom_active(viewport: &mut DagViewportState, graph: &mut DagGraphState, active: bool) {
    if active && !viewport.wheel_zoom_active {
        viewport.wheel_zoom_render_lod = Some(resolved_draw_lod(viewport, graph));
    } else if !active {
        viewport.wheel_zoom_render_lod = None;
    }
    viewport.wheel_zoom_active = active;
}

fn resolved_draw_lod(viewport: &DagViewportState, graph: &DagGraphState) -> DagDrawLod {
    if !viewport.automatic_lod {
        if let Some(lod) = viewport.forced_draw_lod {
            return lod;
        }
    }
    dag_draw_lod(graph.host_snapshot.camera.zoom)
}

pub fn draw_lod_for_frame(viewport: &DagViewportState, graph: &DagGraphState) -> DagDrawLod {
    if !viewport.automatic_lod {
        if let Some(lod) = viewport.forced_draw_lod {
            return lod;
        }
    }
    if viewport.wheel_zoom_active {
        if let Some(pinned) = viewport.wheel_zoom_render_lod {
            return pinned;
        }
    }
    dag_draw_lod(graph.host_snapshot.camera.zoom)
}

/// 📶️ Active draw LOD tier label (`minimap`, `overview`, …).
pub fn dag_draw_lod_label(viewport: &DagViewportState, graph: &DagGraphState) -> &'static str {
    draw_lod_for_frame(viewport, graph).label()
}

/// 🔍️ Frames the current node selection in the viewport camera.
pub fn focus_selection_camera(viewport: &DagViewportState, graph: &DagGraphState, pad: f64) -> Option<DagCamera> {
    let selected = graph.selected_snapshot_nodes();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (_, node) in &selected {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        min_x = min_x.min(node.x - hw);
        min_y = min_y.min(node.y - hh);
        max_x = max_x.max(node.x + hw);
        max_y = max_y.max(node.y + hh);
    }
    if !min_x.is_finite() {
        return None;
    }
    let pad = pad.max(1.05);
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let span_w = (max_x - min_x).max(1.0);
    let span_h = (max_y - min_y).max(1.0);
    let vw = viewport.width.max(1) as f64;
    let vh = viewport.height.max(1) as f64;
    let zoom = (vw / (span_w * pad)).min(vh / (span_h * pad)).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
    Some(DagCamera { x: cx, y: cy, zoom })
}

fn lod_visible_grid_snap_step_world(viewport: &DagViewportState, graph: &DagGraphState) -> Option<f64> {
    match draw_lod_for_frame(viewport, graph) {
        DagDrawLod::Minimap => None,
        DagDrawLod::Overview | DagDrawLod::Compact => Some(viewport.grid_step_large_world()),
        DagDrawLod::Normal => Some(viewport.grid_step_medium_world()),
        DagDrawLod::Detail => Some(viewport.grid_step_small_world()),
        DagDrawLod::Micro => Some(viewport.grid_step_micro_world()),
    }
}

fn snap_world_scalar(viewport: &DagViewportState, graph: &DagGraphState, value: f64) -> f64 {
    if !viewport.grid_snap_enabled {
        return value;
    }
    let Some(step) = lod_visible_grid_snap_step_world(viewport, graph) else {
        return value;
    };
    (value / step).round() * step
}

fn snap_world_pair(viewport: &DagViewportState, graph: &DagGraphState, x: f64, y: f64) -> (f64, f64) {
    (snap_world_scalar(viewport, graph, x), snap_world_scalar(viewport, graph, y))
}

pub fn screen_to_world_point(viewport_state: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> canvas::Point {
    use canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width, height: viewport_state.height, dpr: viewport_state.dpr };
    screen_to_world(&cam, &viewport, Point::new(sx, sy))
}

pub fn sync_node_positions_from_engine(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    for (&nid, &idx) in &graph.node_id_map {
        let Some(engine_node) = graph.engine.nodes.get(&nid) else {
            continue;
        };
        let (mut x, mut y) = (engine_node.center.x, engine_node.center.y);
        if viewport.grid_snap_enabled {
            (x, y) = snap_world_pair(viewport, graph, x, y);
        }
        graph.host_snapshot.nodes[idx].x = x;
        graph.host_snapshot.nodes[idx].y = y;
        if viewport.grid_snap_enabled {
            if let Some(engine_node) = graph.engine.nodes.get_mut(&nid) {
                engine_node.center = canvas::Point::new(x, y);
            }
        }
    }
}

pub fn process_engine_events(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    let events = graph.engine.drain_events();
    let mut moved = false;
    let mut wired = false;
    for event in events {
        match event {
            BoardEvent::NodeMoved { .. } => moved = true,
            BoardEvent::EdgeConnected { source, target, .. } => {
                wired = true;
                graph.journal_connect(source, target);
            }
            BoardEvent::EdgeRemoved { id } => {
                wired = true;
                graph.journal_disconnect(id);
            }
            BoardEvent::SelectionChanged { .. } | BoardEvent::PreselectChanged { .. } | BoardEvent::HoverChanged { .. } => {}
        }
    }
    if moved {
        sync_node_positions_from_engine(viewport, graph);
    }
    if wired {
        graph.sync_edges_from_engine();
    }
}

fn channel_row_handle_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        return None;
    }
    graph.port_row_handle_hit(world_x, world_y, true, true)
}

fn handle_anchor_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).allows_connection_hit_picking() {
        return None;
    }
    use canvas::Point;
    let p = Point::new(world_x, world_y);
    for (&hid, handle) in graph.engine.handles.iter().rev() {
        let Some(node) = graph.engine.nodes.get(&handle.node_id) else {
            continue;
        };
        let pos = handle_position(node, handle);
        let dx = p.x - pos.x;
        let dy = p.y - pos.y;
        let tol = handle.radius + 6.0;
        if dx * dx + dy * dy <= tol * tol {
            return Some(hid);
        }
    }
    None
}

/// 🔌️ The ONE port hit — the CONNECTOR rect, which is also the one rect
/// [`Self::entity_screen_json`] publishes for `"handle"`. Grabbing and publishing used to be two
/// different shapes (a 6.5-world-unit rim disc against the whole 20-unit row), so the published
/// centre never wired at any zoom, and a radius-11 anchor disc reached past the row centre and
/// stole the node drag (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️generate-mode-interactions-2026-09-13.md` §4).
pub fn port_pointer_handle_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).allows_connection_hit_picking() {
        return None;
    }
    graph.port_connector_handle_hit(world_x, world_y)
}

/// 🔗️ Wire snapping belongs to a wire: while a node is being DRAGGED, snapping the pointer onto a
/// port anchor teleports the node to that anchor. Only a gesture that can still land on a port —
/// a press, or a wire in flight — is snapped.
pub fn connection_hit_world_while_wiring(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> (f64, f64) {
    if matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) {
        return connection_hit_world(viewport, graph, world_x, world_y);
    }
    (world_x, world_y)
}

pub fn connection_hit_world(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> (f64, f64) {
    let Some(hid) = port_pointer_handle_hit(viewport, graph, world_x, world_y) else {
        return (world_x, world_y);
    };
    let Some(handle) = graph.engine.handles.get(&hid) else {
        return (world_x, world_y);
    };
    let Some(node) = graph.engine.nodes.get(&handle.node_id) else {
        return (world_x, world_y);
    };
    let pos = handle_position(node, handle);
    (pos.x, pos.y)
}

pub fn world_hits_handle(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> bool {
    port_pointer_handle_hit(viewport, graph, world_x, world_y).is_some()
}

pub fn sync_channel_row_pointer_hover(viewport: &mut DagViewportState, graph: &mut DagGraphState, world_x: f64, world_y: f64) {
    if !draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        return;
    }
    if !matches!(graph.engine.interaction, InteractionMode::Idle) {
        return;
    }
    if handle_anchor_hit(viewport, graph, world_x, world_y).is_some() {
        return;
    }
    if let Some(hid) = channel_row_handle_hit(viewport, graph, world_x, world_y) {
        graph.engine.hover = Some(hid);
    }
}

pub fn sync_connection_hit_picking_for_lod(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    let allow = draw_lod_for_frame(viewport, graph).allows_connection_hit_picking();
    graph.engine.handle_pointer_picking = allow;
    if !allow && matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) {
        graph.engine.interaction = InteractionMode::Idle;
    }
}

/// 🧭️ Minimap LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
pub fn lod_uses_bounded_drag(viewport: &DagViewportState, graph: &DagGraphState) -> bool {
    matches!(draw_lod_for_frame(viewport, graph), DagDrawLod::Minimap)
}

/// 🧩️ DagHost viewport component: size, device ratio, LOD and grid policy, theme and pan anchor of the canvas.
pub struct DagViewportState {
    pub canvas_theme: CanvasPalette,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    pub last_screen_x: f64,
    pub last_screen_y: f64,
    pub wheel_zoom_active: bool,
    pub wheel_zoom_render_lod: Option<DagDrawLod>,
    pub automatic_lod: bool,
    pub forced_draw_lod: Option<DagDrawLod>,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub pan_anchor: Option<(f64, f64, f64, f64)>,
}

/// 🧩️ DagHost graph component: the host snapshot, the board engine and the id maps between them, with pending wire edits.
pub struct DagGraphState {
    pub host_snapshot: DagHostSnapshot,
    pub engine: DagBoardEngine,
    pub node_id_map: HistoryFoldIndex<NodeId, usize>,
    pub handle_key_map: HistoryFoldIndex<HandleId, String>,
    pub handle_port_shape: HistoryFoldIndex<HandleId, PortShape>,
    pub handle_port_visible: HistoryFoldIndex<HandleId, bool>,
    pub edge_id_map: HistoryFoldIndex<EdgeId, String>,
    pub edge_engine_ids: Vec<Option<EdgeId>>,
    pub edge_route_style: HistoryFoldIndex<EdgeId, EdgeRouteStyle>,
    /// 🔗️ Wire edits this host performed on its OWN geometry since the last drain, for a renderer to
    /// forward to the guest that owns the graph — see [`DagGraphEdit`].
    pub pending_graph_edits: Vec<DagGraphEdit>,
    /// 🚫️ Why the last gesture was refused whole instead of journalled — its moves outgrew one dispatch — until a renderer
    /// drains it ([`Self::take_journal_refusal`]).
    pub journal_refusal: Option<DagJournalRefusal>,
}

/// 🧭️ Screen coordinates to world coordinates for the current viewport.
pub fn dag_screen_to_world(viewport: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> (f64, f64) {
    let point = screen_to_world_point(viewport, graph, sx, sy);
    (point.x, point.y)
}

/// 👻️ World coordinates to surface-local pixels through the DAG's live camera.
pub fn dag_world_to_screen(viewport: &DagViewportState, graph: &DagGraphState, wx: f64, wy: f64) -> (f64, f64) {
    world_to_screen_point(viewport, graph, wx, wy)
}

impl DagGraphState {
    pub fn engine_node_id_for_index(&self, idx: usize) -> Option<NodeId> {
        self.node_id_map.iter().find(|(_, &i)| i == idx).map(|(&nid, _)| nid)
    }

    pub fn node_id_for_widget_id(&self, widget_id: &str) -> Option<NodeId> {
        let idx = self.host_snapshot.nodes.iter().position(|n| n.id == widget_id)?;
        self.engine_node_id_for_index(idx)
    }

    pub fn widget_id_for_node_id(&self, node_id: NodeId) -> Option<String> {
        self.widget_id_for_node_id_ref(node_id).map(str::to_owned)
    }

    pub fn widget_id_for_node_id_ref(&self, node_id: NodeId) -> Option<&str> {
        let idx = *self.node_id_map.get(&node_id)?;
        self.host_snapshot.nodes.get(idx).map(|node| node.id.as_str())
    }

    pub fn is_node_hovered(&self, node_id: NodeId) -> bool {
        matches!(self.engine.hover, Some(hover) if hover == node_id)
    }

    fn resolve_minimap_hover_node_id(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x >= node.x - hw && world_x <= node.x + hw && world_y >= node.y - hh && world_y <= node.y + hh {
                return self.engine_node_id_for_index(idx);
            }
        }
        None
    }

    fn is_node_selected(&self, node_id: NodeId) -> bool {
        self.engine.selection.node_ids.contains(&node_id)
    }

    fn is_node_preselected(&self, node_id: NodeId) -> bool {
        self.engine.preselect.node_ids.contains(&node_id)
    }

    fn is_node_preselect_removed(&self, node_id: NodeId) -> bool {
        self.engine.preselect_removed.node_ids.contains(&node_id)
    }

    pub fn is_preselect_active(&self) -> bool {
        matches!(self.engine.interaction, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. })
            || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    pub fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.is_node_preselected(node_id), self.is_node_preselect_removed(node_id), false);
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    pub fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.handle_ids.contains(&handle_id), self.engine.preselect_removed.handle_ids.contains(&handle_id), false);
        }
        (self.engine.selection.handle_ids.contains(&handle_id), false, self.engine.hover == Some(handle_id))
    }

    pub fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.edge_ids.contains(&edge_id), self.engine.preselect_removed.edge_ids.contains(&edge_id), false);
        }
        (self.engine.selection.edge_ids.contains(&edge_id), false, self.engine.hover == Some(edge_id))
    }

    pub fn sync_camera_from_engine(&mut self) {
        let cam = self.engine.camera;
        self.host_snapshot.camera = DagCamera { x: cam.x, y: cam.y, zoom: cam.zoom };
    }

    /// 🎯️ Selected snapshot node ids from the engine selection snapshot.
    pub fn selected_node_ids(&self) -> Vec<String> {
        self.selected_node_id_refs().map(str::to_owned).collect()
    }

    pub fn selected_node_id_refs(&self) -> impl Iterator<Item = &str> {
        self.engine.selection.node_ids.iter().filter_map(|&node_id| self.widget_id_for_node_id_ref(node_id))
    }

    pub fn selected_node_count(&self) -> usize {
        self.engine.selection.node_ids.len()
    }

    pub fn bounded_interaction_projection(&self, revision: u64) -> Result<DagInteractionProjection, DagInteractionPlanFault> {
        if self.host_snapshot.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
            return Err(DagInteractionPlanFault::NodeCredits);
        }
        let mut bytes = 0usize;
        for node in &self.host_snapshot.nodes {
            if node.id.len() > 256 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
            bytes = bytes.checked_add(node.id.len()).ok_or(DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
        }
        let mut selected = [0; DAG_INTERACTION_WORD_CAPACITY];
        for node_id in &self.engine.selection.node_ids {
            if let Some(index) = self.node_id_map.get(node_id).copied() {
                dag_bit_set(&mut selected, index, true);
            }
        }
        let hover = self.engine.hover.and_then(|node_id| self.node_id_map.get(&node_id).copied()).and_then(|index| u16::try_from(index).ok());
        let camera = &self.host_snapshot.camera;
        Ok(DagInteractionProjection { revision, camera: [camera.x, camera.y, camera.zoom], selected, hover, gesture: DagProjectionGesture::Idle })
    }

    pub fn apply_pointer_plan(&mut self, plan: &DagPointerPlan) {
        self.set_camera(plan.next.camera[0], plan.next.camera[1], plan.next.camera[2]);
        for index in 0..usize::from(plan.move_len) {
            let Some(delta) = plan.moves[index] else {
                continue;
            };
            let index = usize::from(delta.index);
            if let Some(node) = self.host_snapshot.nodes.get_mut(index) {
                node.x = delta.x;
                node.y = delta.y;
            }
            self.sync_snapshot_node_center_to_engine(index);
        }
        self.engine.selection = Selection::default();
        for index in 0..self.host_snapshot.nodes.len() {
            if dag_bit_contains(&plan.next.selected, index) {
                if let Some(node_id) = self.engine_node_id_for_index(index) {
                    self.engine.selection.node_ids.insert(node_id);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
        self.engine.hover = plan.next.hover.and_then(|index| self.engine_node_id_for_index(usize::from(index)));
    }

    pub fn projection_selected_id_refs<'a>(&'a self, projection: &'a DagInteractionProjection) -> impl Iterator<Item = &'a str> + 'a {
        self.host_snapshot.nodes.iter().enumerate().filter(move |(index, _)| dag_bit_contains(&projection.selected, *index)).map(|(_, node)| node.id.as_str())
    }

    pub fn projection_hovered_id_ref<'a>(&'a self, projection: &DagInteractionProjection) -> Option<&'a str> {
        projection.hover.and_then(|index| self.host_snapshot.nodes.get(usize::from(index))).map(|node| node.id.as_str())
    }

    pub fn pointer_plan_move(&self, plan: &DagPointerPlan, index: usize) -> Option<(&str, f64, f64)> {
        let delta = plan.moves.get(index).and_then(|delta| *delta)?;
        self.host_snapshot.nodes.get(usize::from(delta.index)).map(|node| (node.id.as_str(), delta.x, delta.y))
    }

    /// 🫳️ The node drag a plan COMMITS when it ends a drag: ONE gesture record (design §13.3) — the press, every moved
    /// node and the drag's ONE relative offset — in the guest's own edit vocabulary.
    ///
    /// 🩸️ A bounded drag writes the new positions into `snapshot.layout` and into the engine — and
    /// told the guest NOTHING, so every node the user dragged snapped back on the next snapshot push.
    /// Only the TERMINAL application is journalled (a drag publishes one record, not one per pointer
    /// sample), and a gesture that ended where it started is not an edit at all — a plain click on a
    /// node body derives a zero-delta drag, which must not spend a guest mutation.
    ///
    /// @see `🕸️NodeGraph/🟦️.tsx` — the SSR `Diagram` fallback's `onNodeDragStop`
    pub fn plan_graph_edits(&self, plan: &DagPointerPlan) -> Vec<DagGraphEdit> {
        let Some(drag) = plan.drag.filter(|_| plan.previous_active && matches!(plan.next.gesture, DagProjectionGesture::Idle)) else {
            return Vec::new();
        };
        let node_ids: Vec<String> = plan.moves[..usize::from(plan.move_len)].iter().flatten().filter_map(|delta| self.host_snapshot.nodes.get(usize::from(delta.index))).map(|node| node.id.clone()).collect();
        if node_ids.is_empty() {
            return Vec::new();
        }
        vec![DagGraphEdit::Move { gesture_id: dag_drag_gesture_id(drag.press), node_ids, dx: drag.dx, dy: drag.dy }]
    }

    /// 🔗️ Selected snapshot edge ids (synapse ids) from the engine selection snapshot.
    pub fn selected_edge_ids(&self) -> Vec<String> {
        self.engine.selection.edge_ids.iter().filter_map(|&eid| self.edge_id_map.get(&eid).cloned()).collect()
    }

    /// 🎯️ Projects typed node, edge and handle selection facts.
    pub fn selection_domains(&self)->DagSelectionDomains {
        let handles=self.selected_channels().into_iter().map(|channel|format!("{}@{}",channel.widget_id,channel.port)).collect();
        DagSelectionDomains {nodes:self.selected_node_ids(),edges:self.selected_edge_ids(),handles}
    }

    fn apply_selection_domains(&mut self, nodes: &[String], edges: &[String], handles: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in nodes {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        for edge_id in edges {
            for (&eid, synapse_id) in &self.edge_id_map {
                if synapse_id == edge_id {
                    self.engine.selection.edge_ids.insert(eid);
                    break;
                }
            }
        }
        for handle_key in handles {
            if let Some((node_id, port_id)) = handle_key.split_once('@') {
                if let Some(hid) = self.handle_id_for_port(node_id, port_id) {
                    self.engine.selection.handle_ids.insert(hid);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// ✅️ Replaces selection from fully admitted semantic domains.
    pub fn set_selection_domains(&mut self,domains:&DagSelectionDomains){self.apply_selection_domains(&domains.nodes,&domains.edges,&domains.handles);}

    /// ✅️ Whether the engine has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        !self.engine.selection.node_ids.is_empty() || !self.engine.selection.edge_ids.is_empty() || !self.engine.selection.handle_ids.is_empty()
    }

    pub fn channel_view(&self, target:u64)->Option<DagChannelView<'_>> {
        if self.node_id_map.contains_key(&target) {
            return None;
        }
        let key = self.handle_key_map.get(&target)?;
        let handle=self.engine.handles.get(&target)?;
        let node=self.host_snapshot.nodes.get(*self.node_id_map.get(&handle.node_id)?)?;
        let port=key.get(node.id.len().checked_add(1)?..)?;
        let direction=match handle.role{HandleRole::Target=>DagChannelDirection::In,HandleRole::Source=>DagChannelDirection::Out,_=>return None};
        Some(DagChannelView {widget_id:&node.id,port,direction})
    }

    pub fn decode_channel_ref(&self,target:u64)->Option<DagChannelRef>{self.channel_view(target).map(|value|DagChannelRef{widget_id:value.widget_id.to_owned(),port:value.port.to_owned(),direction:value.direction})}

    /// 🔌️ Hovered snapshot channel when the pointer is over a port row or handle.
    pub fn hovered_channel(&self) -> Option<DagChannelRef> {
        let hover = self.engine.hover?;
        self.decode_channel_ref(hover)
    }

    /// 🔌️ Selected snapshot channels from handle picks in the current selection snapshot.
    pub fn selected_channels(&self) -> Vec<DagChannelRef> {
        self.engine.selection.handle_ids.iter().filter_map(|&handle_id| self.decode_channel_ref(handle_id)).collect()
    }

    /// 🚫️ Projects a live wire refusal with both declared semantic value type lists.
    pub fn wire_type_refusal(&self)->Option<DagWireTypeRefusal>{
        let (source_hid,target_hid)=self.engine.wire_drag_type_refusal()?;
        let source=self.handle_key_map.get(&source_hid)?.clone();let target=self.handle_key_map.get(&target_hid)?.clone();
        let types=|id|self.engine.handles.get(&id).map(|handle|handle.value_types.clone()).unwrap_or_default();
        Some(DagWireTypeRefusal{source,source_types:types(source_hid),target,target_types:types(target_hid)})
    }

    /// 🖱️ Projects channel hover and live wire refusal from one accepted graph state.
    pub fn hover_facts(&self)->DagHoverFacts{DagHoverFacts{channel:self.hovered_channel(),refusal:self.wire_type_refusal()}}

    /// ✅️ Replaces node selection from snapshot widget ids.
    pub fn set_selection(&mut self, widget_ids: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// 🎯️ Configures rectangle/lasso area-select behavior.
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.engine.set_selection_options(method, mode, select_nodes, select_handles, select_edges);
    }

    /// 🧿️ Screen-space marquee overlay points for the shared selection overlay.
    pub fn selection_preview_points_json(&self) -> String {
        let points: Vec<[f64; 2]> = self.engine.selection_preview_points().iter().map(|p| [p.x, p.y]).collect();
        semio_framework_pack_json::to_json_string(&points)
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.engine.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.engine.selection_preview_method()
    }

    /// 👁️ Preselected widget ids during an in-flight marquee gesture.
    pub fn preselect_widget_ids(&self) -> Vec<String> {
        self.engine.preselect.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// 👁️ Widget ids highlighted as marquee-exit candidates.
    pub fn preselect_removed_widget_ids(&self) -> Vec<String> {
        self.engine.preselect_removed.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ↩️ Cancels an in-flight area select and restores the pre-drag selection.
    pub fn cancel_area_select(&mut self) -> bool {
        self.engine.cancel_area_select()
    }
}

pub fn sync_minimap_pointer_hover(viewport: &mut DagViewportState, graph: &mut DagGraphState, world_x: f64, world_y: f64) {
    if !matches!(draw_lod_for_frame(viewport, graph), DagDrawLod::Minimap) {
        return;
    }
    if !matches!(graph.engine.interaction, InteractionMode::Idle) {
        return;
    }
    graph.engine.hover = graph.resolve_minimap_hover_node_id(world_x, world_y);
}

pub fn world_to_screen_point(viewport_state: &DagViewportState, graph: &DagGraphState, x: f64, y: f64) -> (f64, f64) {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    let camera = Camera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width, height: viewport_state.height, dpr: viewport_state.dpr };
    let point = world_to_screen(&camera, &viewport, canvas::Point::new(x, y));
    (point.x, point.y)
}

/// 🖱️ Hovered snapshot widget id for node body hover, or parent widget when a channel handle is hovered at detail LOD.
pub fn hovered_node_id(viewport: &DagViewportState, graph: &DagGraphState) -> Option<String> {
    hovered_node_id_ref(viewport, graph).map(str::to_owned)
}

pub fn hovered_node_id_ref<'a>(viewport: &'a DagViewportState, graph: &'a DagGraphState) -> Option<&'a str> {
    let hover = graph.engine.hover?;
    if graph.node_id_map.contains_key(&hover) {
        return graph.widget_id_for_node_id_ref(hover);
    }
    if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        if let Some(handle) = graph.engine.handles.get(&hover) {
            return graph.widget_id_for_node_id_ref(handle.node_id);
        }
    }
    None
}

/// 🗺️ The screen rect [`Self::entity_screen_json`] reports for one entity, or `None` when it is
/// off screen — the typed read the census needs before it can scan inside it.
pub fn entity_screen_rect(viewport: &DagViewportState, graph: &DagGraphState, domain: &str, id: &str) -> Option<[f64; 4]> {
    let value = semio_framework_pack_json::parse(&entity_screen_json(viewport, graph, domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    if value.get("visible").and_then(Value::as_bool) != Some(true) {
        return None;
    }
    let rect = value.get("rect")?.as_array()?;
    if rect.len() < 4 {
        return None;
    }
    let read = |index: usize| rect.get(index).and_then(Value::as_f64);
    Some([read(0)?, read(1)?, read(2)?, read(3)?])
}

/// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
pub fn dag_pick_targets_at_screen_json(viewport: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> String {
    #[derive(ToValue, FromValue)]
    struct Row {
        domain: String,
        id: String,
        generality: u32,
        #[value(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }
    let world = screen_to_world_point(viewport, graph, sx, sy);
    let targets = graph.engine.hit_test_pick_targets(world);
    let rows: Vec<Row> = targets
        .into_iter()
        .filter_map(|target| {
            let row = match target.domain.as_str() {
                "node" => graph.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                "edge" => graph.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                "handle" => graph.decode_channel_ref(target.id).map(|channel| Row {
                    domain: "handle".into(),
                    id: format!("{}@{}", channel.widget_id, channel.port),
                    generality: target.generality,
                    label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                }),
                _ => None,
            };
            row
        })
        .collect();
    semio_framework_pack_json::to_json_string(&rows)
}

impl DagGraphState {
    // #region 🔖️DagHost
    // #region 🔖️ContentFraming
    /// 🖼️ World bounds of every node this board holds — what a fit frames and what the minimap maps.
    /// `None` for an empty graph, which has nothing to frame.
    pub fn content_world_bounds(&self) -> Option<canvas::camera::ContentBounds> {
        let first = self.host_snapshot.nodes.first()?;
        let mut union = dag_node_world_bounds(first);
        for node in self.host_snapshot.nodes.iter().skip(1) {
            let b = dag_node_world_bounds(node);
            union.min_x = union.min_x.min(b.min_x);
            union.min_y = union.min_y.min(b.min_y);
            union.max_x = union.max_x.max(b.max_x);
            union.max_y = union.max_y.max(b.max_y);
        }
        Some(canvas::camera::ContentBounds { min_x: union.min_x, min_y: union.min_y, max_x: union.max_x, max_y: union.max_y })
    }

    fn camera_state(&self) -> canvas::camera::Camera {
        canvas::camera::Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }
    }

    pub fn minimap_widget_content_bounds(&self) -> Option<WorldBox> {
        let pad = ui_styling::metrics::dag::MINIMAP_WIDGET_CONTENT_PAD;
        let content = self.content_world_bounds()?;
        Some(WorldBox { min_x: content.min_x - pad, min_y: content.min_y - pad, max_x: content.max_x + pad, max_y: content.max_y + pad })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::content_fully_visible` — pure layout math relocated there
    /// (see `.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`).
    pub fn minimap_camera_fully_shows_content(&self, content: &WorldBox, viewport_w: u32, viewport_h: u32) -> bool {
        let cam = &self.host_snapshot.camera;
        ui_wgpu::wgpu::minimap::content_fully_visible(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, 12.0)
    }

    pub fn selected_snapshot_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter().filter_map(|id| self.host_snapshot.nodes.iter().enumerate().find(|(_, node)| node.id == id).map(|(idx, node)| (idx, node.clone()))).collect()
    }

    pub(crate) fn sync_snapshot_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center = canvas::Point::new(node.x, node.y);
        }
    }

    /// 📐️ Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), DagError> {
        use canvas::Point;
        let mut selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return Ok(());
        }
        match mode {
            "alignLeft" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_left = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.x = min_left + node.width * 0.5;
                }
            }
            "alignRight" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_right = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.x = max_right - node.width * 0.5;
                }
            }
            "alignTop" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_top = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.y = min_top + node.height * 0.5;
                }
            }
            "alignBottom" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_bottom = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.y = max_bottom - node.height * 0.5;
                }
            }
            "alignHorizontal" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_x = (union.min_x + union.max_x) * 0.5;
                for (_, node) in &mut selected {
                    node.x = center_x;
                }
            }
            "alignVertical" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_y = (union.min_y + union.max_y) * 0.5;
                for (_, node) in &mut selected {
                    node.y = center_y;
                }
            }
            "distributeHorizontal" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
                let left = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                let right = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                let total_width: f64 = selected.iter().map(|(_, node)| node.width).sum();
                let raw_gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = left;
                for (_, node) in &mut selected {
                    node.x = cursor + node.width * 0.5;
                    cursor += node.width + gap;
                }
            }
            "distributeVertical" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal));
                let top = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                let bottom = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                let total_height: f64 = selected.iter().map(|(_, node)| node.height).sum();
                let raw_gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(DagError::UnknownAlignMode(other.to_string())),
        }
        for (idx, node) in selected {
            self.host_snapshot.nodes[idx].x = node.x;
            self.host_snapshot.nodes[idx].y = node.y;
            self.sync_snapshot_node_center_to_engine(idx);
        }
        Ok(())
    }
    // #endregion 🔖️SelectionAlign
    // #endregion 🔖️DagHost
}

impl DagViewportState {
    fn camera_viewport(&self) -> canvas::camera::Viewport {
        canvas::camera::Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) }
    }

    /// 👁️ Whether the point a caller would AIM at is actually on the surface this geometry is
    /// published in. The centre is that point — `entity_screen_json` publishes a rect whose centre is
    /// where anything trusting the host presses — so the centre is what `visible` has to be true of.
    ///
    /// 🩸️ `visible` used to mean "the entity EXISTS", never "you can aim at it": a port the camera had
    /// scrolled past was published as `visible: true` with its centre at `x = -4.9` in surface space,
    /// so a caller that trusts the published geometry — a demonstration, an assistive caller, a
    /// scripted drag — pressed OUTSIDE the canvas and the press reached nothing at all. Measured on
    /// 6026 as the wire row's redraw press, twice, with the two aims byte-identical
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
    fn screen_point_is_on_surface(&self, point: (f64, f64)) -> bool {
        point.0 >= 0.0 && point.1 >= 0.0 && point.0 <= f64::from(self.width.max(1)) && point.1 <= f64::from(self.height.max(1))
    }
}

/// 🖼️ Fraction of the graph's own area the live camera currently shows — `1.0` when the whole
/// graph is framed, `0.0` when none of it is. An empty graph is framed by definition.
pub fn camera_content_coverage(viewport: &DagViewportState, graph: &DagGraphState) -> f64 {
    graph.content_world_bounds().map_or(1.0, |content| canvas::camera::content_coverage(&content, &graph.camera_state(), &viewport.camera_viewport()))
}

/// 🖼️ Moves the camera so the whole graph is framed with the shared screen padding. `false` when
/// there is no graph to frame.
pub fn fit_camera_to_content(viewport: &mut DagViewportState, graph: &mut DagGraphState) -> bool {
    let Some(content) = graph.content_world_bounds() else { return false };
    let camera = canvas::camera::fit_camera(&content, &viewport.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX);
    graph.set_camera(camera.x, camera.y, camera.zoom);
    true
}

/// 🖼️ The opening camera law: a STORED camera is adopted only when it already frames the graph
/// it was stored for; otherwise the graph is fitted. Returns `true` when the fit won — the
/// caller persists that exactly the way it persists a pan or a zoom gesture.
pub fn adopt_camera_or_fit(viewport: &mut DagViewportState, graph: &mut DagGraphState, x: f64, y: f64, zoom: f64) -> bool {
    let stored = canvas::camera::Camera { x, y, zoom };
    let content = graph.content_world_bounds();
    let (camera, fitted) =
        canvas::camera::startup_camera(Some(&stored), content.as_ref(), &viewport.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX, canvas::camera::CONTENT_FRAMED_MIN_COVERAGE);
    graph.set_camera(camera.x, camera.y, camera.zoom);
    fitted
}

/// 🖼️ Re-fits a graph that CHANGED under a live camera (an example switch) only when the change
/// left essentially nothing on screen. A camera the viewer set themselves is never yanked back
/// for an ordinary edit — see `canvas::camera::CONTENT_REFIT_MAX_COVERAGE`.
pub fn refit_camera_if_content_left_view(viewport: &mut DagViewportState, graph: &mut DagGraphState) -> bool {
    if graph.content_world_bounds().is_none() || camera_content_coverage(viewport, graph) > canvas::camera::CONTENT_REFIT_MAX_COVERAGE {
        return false;
    }
    fit_camera_to_content(viewport, graph)
}
// #endregion 🔖️ContentFraming

/// 📦️ Screen-space union bounds of the current node selection for DOM chrome overlays.
pub fn selection_union_bounds_screen_json(viewport_state: &DagViewportState, graph: &DagGraphState) -> String {
    let selected = graph.selected_snapshot_nodes();
    if selected.is_empty() {
        return "null".into();
    }
    use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let pad_world = 4.0 / graph.host_snapshot.camera.zoom.max(0.05);
    let mut corners = Vec::new();
    for (_, node) in &selected {
        let hw = node.width * 0.5 + pad_world;
        let hh = node.height * 0.5 + pad_world;
        corners.push(Point::new(node.x - hw, node.y - hh));
        corners.push(Point::new(node.x + hw, node.y + hh));
    }
    let Some(bounds) = world_box_from_points(&corners) else {
        return "null".into();
    };
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
    let tl = world_to_screen(&cam, &viewport, Point::new(bounds.min_x, bounds.min_y));
    let br = world_to_screen(&cam, &viewport, Point::new(bounds.max_x, bounds.max_y));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("x".to_string(), Value::from(tl.x)),
        ("y".to_string(), Value::from(tl.y)),
        ("width".to_string(), Value::from((br.x - tl.x).max(1.0))),
        ("height".to_string(), Value::from((br.y - tl.y).max(1.0))),
    ]))
}

/// 🎯️ Screen-space geometry (canvas-local px) for a live entity in the shell's pick-target
/// grammar (`domain`: `"node"` | `"handle"` | `"edge"`; `id`: a node's widget id, `"widgetId@port"`
/// for a handle, or a mirrored edge id — `"*"` picks whichever matching entity's screen anchor is
/// nearest the viewport center) — powers introduction-demonstration semantic targeting
/// (`IntroductionPoint::Entity`/`Curve`). Never errors: an unresolved domain/id returns
/// `{"visible":false}`.
///
/// 🔌️ A port is published as its CONNECTOR rect — the very rect `port_connector_handle_hit`
/// grabs a wire in, so anything that aims at what the host reports (a demonstration, an
/// assistive caller, a scripted drag) presses a point that wires.
pub fn entity_screen_json(viewport_state: &DagViewportState, graph: &DagGraphState, domain: &str, id: &str) -> String {
    #[derive(ToValue, FromValue)]
    struct EntityGeometry {
        visible: bool,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        rect: Option<[f64; 4]>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        polyline: Option<Vec<[f64; 2]>>,
    }
    use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let unresolved = EntityGeometry { visible: false, x: None, y: None, rect: None, polyline: None };
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
    let viewport_center = (viewport_state.width as f64 * 0.5, viewport_state.height as f64 * 0.5);

    let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
        let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
        ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
    };
    let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
        let node = graph.host_snapshot.nodes.iter().find(|node| node.id == widget_id)?;
        if let Some(index) = node.inputs().iter().position(|candidate| candidate.id == port) {
            return input_port_connector_bounds(node, index);
        }
        let index = node.outputs().iter().position(|candidate| candidate.id == port)?;
        output_port_connector_bounds(node, index)
    };
    let nearest_center_screen = |candidates: &[(f64, f64, f64, f64)]| -> Option<(f64, f64, f64, f64)> {
        let mut best: Option<((f64, f64, f64, f64), f64)> = None;
        for &bounds in candidates {
            let (_, screen_center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
            let distance = (screen_center.0 - viewport_center.0).hypot(screen_center.1 - viewport_center.1);
            if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                best = Some((bounds, distance));
            }
        }
        best.map(|(bounds, _)| bounds)
    };

    let bounds_result: Option<(f64, f64, f64, f64)> = match domain {
        "node" => {
            if id == "*" {
                let all: Vec<(f64, f64, f64, f64)> = graph
                    .host_snapshot
                    .nodes
                    .iter()
                    .map(|node| {
                        let b = dag_node_world_bounds(node);
                        (b.min_x, b.min_y, b.max_x, b.max_y)
                    })
                    .collect();
                nearest_center_screen(&all)
            } else {
                graph.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| {
                    let b = dag_node_world_bounds(node);
                    (b.min_x, b.min_y, b.max_x, b.max_y)
                })
            }
        }
        "handle" => {
            if id == "*" {
                let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                for node in &graph.host_snapshot.nodes {
                    for port in node.inputs().iter().chain(node.outputs().iter()) {
                        if let Some(bounds) = handle_world_bounds(&node.id, &port.id) {
                            all.push(bounds);
                        }
                    }
                }
                nearest_center_screen(&all)
            } else {
                id.split_once('@').and_then(|(widget_id, port)| handle_world_bounds(widget_id, port))
            }
        }
        "edge" => {
            let edge = if id == "*" { graph.host_snapshot.edges.first() } else { graph.host_snapshot.edges.iter().find(|edge| edge.id == id) };
            let Some(edge) = edge else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some((source_widget, source_port)) = edge.source.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some((target_widget, target_port)) = edge.target.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
            let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
            let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
            if !viewport_state.screen_point_is_on_surface(midpoint) {
                return semio_framework_pack_json::to_json_string(&unresolved);
            }
            return semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(midpoint.0), y: Some(midpoint.1), rect: None, polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]) });
        }
        _ => None,
    };

    let Some(bounds) = bounds_result else { return semio_framework_pack_json::to_json_string(&unresolved) };
    let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
    if !viewport_state.screen_point_is_on_surface(center) {
        return semio_framework_pack_json::to_json_string(&unresolved);
    }
    semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None })
}

impl RetireOwned for DagViewportState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{canvas_theme,width,height,dpr,last_screen_x,last_screen_y,wheel_zoom_active,wheel_zoom_render_lod,automatic_lod,forced_draw_lod,grid_visible,grid_snap_enabled,grid_factor,pan_anchor}=self;sequence(vec![deferred(canvas_theme),deferred(width),deferred(height),deferred(dpr),deferred(last_screen_x),deferred(last_screen_y),deferred(wheel_zoom_active),deferred(wheel_zoom_render_lod),deferred(automatic_lod),deferred(forced_draw_lod),deferred(grid_visible),deferred(grid_snap_enabled),deferred(grid_factor),deferred(pan_anchor)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.canvas_theme),deferred_birth_bytes_for(&self.width),deferred_birth_bytes_for(&self.height),deferred_birth_bytes_for(&self.dpr),deferred_birth_bytes_for(&self.last_screen_x),deferred_birth_bytes_for(&self.last_screen_y),deferred_birth_bytes_for(&self.wheel_zoom_active),deferred_birth_bytes_for(&self.wheel_zoom_render_lod),deferred_birth_bytes_for(&self.automatic_lod),deferred_birth_bytes_for(&self.forced_draw_lod),deferred_birth_bytes_for(&self.grid_visible),deferred_birth_bytes_for(&self.grid_snap_enabled),deferred_birth_bytes_for(&self.grid_factor),deferred_birth_bytes_for(&self.pan_anchor)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagGraphState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{host_snapshot,engine,node_id_map,handle_key_map,handle_port_shape,handle_port_visible,edge_id_map,edge_engine_ids,edge_route_style,pending_graph_edits,journal_refusal}=self;sequence(vec![deferred(host_snapshot),deferred(engine),deferred(node_id_map),deferred(handle_key_map),deferred(handle_port_shape),deferred(handle_port_visible),deferred(edge_id_map),deferred(edge_engine_ids),deferred(edge_route_style),deferred(pending_graph_edits),deferred(journal_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.host_snapshot),deferred_birth_bytes_for(&self.engine),deferred_birth_bytes_for(&self.node_id_map),deferred_birth_bytes_for(&self.handle_key_map),deferred_birth_bytes_for(&self.handle_port_shape),deferred_birth_bytes_for(&self.handle_port_visible),deferred_birth_bytes_for(&self.edge_id_map),deferred_birth_bytes_for(&self.edge_engine_ids),deferred_birth_bytes_for(&self.edge_route_style),deferred_birth_bytes_for(&self.pending_graph_edits),deferred_birth_bytes_for(&self.journal_refusal)])}
 fn controlled_retirement_supported()->bool{true}
}
