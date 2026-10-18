//! 🎯️ Normal board focus and outbox components: selection, hover, picking, descriptor sync and event emission.
//!
//! Crate `semio-framework-os-infinite-board-normal-focus`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::BoardSnapshot;
use semio_framework_os_infinite_board_engine::{ActiveUtility, BoardElementStyleKind, decode_board_snapshot_json, EdgeData, EdgeStrokePattern, GestureStage, GraphPortMode, HandleData, Interaction, merge_ids_into_selection, NodeData, normalize_selection_mode, RegionData, SceneDescriptor, selection_drag_enclosing, selection_drag_shape, SelectionOptions, WireData};
use semio_framework_os_infinite_board_normal_model::{append_edge_tips_on_curve, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_snapshot_scene_descriptor, BoardDrawLod, BoardEventBatchReservation, BoardEventFault, BoardEventKind, BoardEventQueue, BoardEventReservation, BoardOwnedEvent, BOUNDED_DRAG_HIT_PAD_PX, chrome_pass_for_entity, DEFAULT_WIRE_KIND_ID, descriptor_admission, EDGE_HIT_TOLERANCE_PX, edge_stroke_for_kind_pattern, edge_stroke_for_style, explicit_style_kind, gesture_release_events, HANDLE_HIT_TOLERANCE_PX, lerp_color, locked_style_dim, node_stroke_for_style, NodeHandlePaintLayer, NodeIconSpan, NormalPortError, parse_catalog_tip_slot, parse_css_color, sorted_selection_ids, StyleChromePass, wire_stroke_for_style};
use semio_framework_os_infinite_board_normal_scene::{append_handle_marker, board_edge_curve, board_json_string_bytes, board_lod_uses_bounded_drag, BoardSceneState, BoardStyleState, BoardWheelPlan, current_draw_lod, edge_effectively_visible, edge_selectable, edge_world_bounds_for_cull, effective_handle_radius, handle_eligible_link_target_ring, handle_world_bounds_cull, handle_world_pos, handles_link_compatible_for_drag, indirect_handle_marker_radius_world, indirect_handle_world_bounds_cull, indirect_handle_world_pos, link_compatible_handle_count_on_node, link_compatible_handle_ids_on_node, link_drag_compatible_target_node_ids, link_drag_ring_target_node_id, link_drag_wire_curve_world, lod_disables_discrete_pick, node_sole_free_link_compatible_handle, paint_node_icon, wire_curve};
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::NativeDecodeControl;
use graph::{NodeShape, property_bag_from_value};
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_canvas::geom_sel::{cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
use geometry::{distance_between, distance_point_to_cubic_bezier};

impl BoardFocusState {
    pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
        if points.is_none() {
            self.selection_preview_crossing = false;
        }
        self.selection_screen_preview = points;
    }

    pub fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_screen_preview = None;
            self.selection_preview_crossing = false;
            return;
        }
        let last = *screen_points.last().unwrap_or(&start_screen);
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_screen_preview = Some(if self.selection_options.method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] });
    }

    pub(crate) fn is_preselecting(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 💠️ Live area-select preview ids, or committed selection when not preselecting.
    fn selection_chrome_ids(&self) -> BTreeSet<String> {
        if self.is_preselecting() || !self.preselect.is_empty() {
            self.preselect.clone()
        } else {
            self.selection.clone()
        }
    }

    /// 🧿️ True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
    pub fn is_dragging_area_select(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 🧭️ Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
    pub(crate) fn active_link_source_handle_id(&self) -> Option<&str> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
            _ => None,
        }
    }
}

impl BoardOutboxState {
    fn reserve_selection_event(&mut self, kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Option<BoardEventReservation> {
        self.reserve_owned_event(BoardOwnedEvent::selection(kind, ids, anchor_ids, removed_ids, gesture))
    }

    pub fn reserve_owned_event(&mut self, event: Result<BoardOwnedEvent, BoardEventFault>) -> Option<BoardEventReservation> {
        let event = match event {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        match self.events.reserve_event(event) {
            Ok(reservation) => Some(reservation),
            Err(event) => {
                if self.event_overflow.is_none() {
                    self.event_overflow = Some(event);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub fn reserve_owned_batch(&mut self, first: Result<BoardOwnedEvent, BoardEventFault>, second: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> Option<BoardEventBatchReservation> {
        let first = match first {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        let mut batch = BoardEventBatchReservation::one(first);
        if let Some(second) = second {
            let second = match second {
                Ok(event) => event,
                Err(_) => {
                    self.event_schema_fault = true;
                    return None;
                }
            };
            if batch.push(second).is_err() {
                self.event_schema_fault = true;
                return None;
            }
        }
        match self.events.reserve_batch(batch) {
            Ok(batch) => Some(batch),
            Err(batch) => {
                if self.event_batch_overflow.is_none() {
                    self.event_batch_overflow = Some(batch);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub fn publish_event_reservation(&mut self, reservation: BoardEventReservation) {
        if self.events.publish_reserved(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    pub fn publish_event_batch(&mut self, reservation: BoardEventBatchReservation) {
        if self.events.publish_batch(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    //#region 🎬️Gestures
    /// 🪪️ Opens the next pointer gesture: a fresh id, nothing staged yet.
    pub fn begin_gesture(&mut self) -> GestureStage {
        self.gesture_serial = self.gesture_serial.wrapping_add(1);
        GestureStage { id: format!("gesture-{}", self.gesture_serial), select: None, restore: None }
    }

    /// 🏁️ Publishes one released gesture as ONE batch, so a host flushes `select` and the record together.
    pub fn publish_gesture_release(&mut self, gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) {
        let (first, second) = match gesture_release_events(gesture, record) {
            [Some(select), record] => (select, record),
            [None, Some(record)] => (record, None),
            [None, None] => return,
        };
        if let Some(reservation) = self.reserve_owned_batch(first, second) {
            self.publish_event_batch(reservation);
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn drain_events_json(&mut self) -> String {
        let mut out = String::from("[");
        let mut first = true;
        while let Some(event) = self.events.pop() {
            if !first {
                out.push(',');
            }
            first = false;
            event.write_json(&mut out);
        }
        if let Some(event) = self.event_overflow.take() {
            if !first {
                out.push(',');
            }
            event.write_json(&mut out);
        }
        out.push(']');
        out
    }

    pub fn pop_owned_event(&mut self) -> Option<BoardOwnedEvent> {
        if let Some(event) = self.events.pop().or_else(|| self.event_overflow.take()) {
            return Some(event);
        }
        let batch = self.event_batch_overflow.as_mut()?;
        let event = batch.pop();
        if batch.is_empty() {
            self.event_batch_overflow = None;
        }
        event
    }

    pub fn peek_owned_event(&self) -> Option<&BoardOwnedEvent> {
        self.events.iter().next().or(self.event_overflow.as_ref()).or_else(|| self.event_batch_overflow.as_ref().and_then(BoardEventBatchReservation::peek))
    }

    pub fn event_terminal_faulted(&self) -> bool {
        self.event_schema_fault
    }

    pub fn clear_link_gesture_events(&mut self) {
        let compat = self.link_compat_nodes_emit_key.is_some();
        let ring = self.link_target_ring_emit_key.is_some();
        let empty: &[String] = &[];
        let compat_event = compat.then(|| BoardOwnedEvent::link_compatible("", empty));
        let ring_event = ring.then(|| BoardOwnedEvent::link_ring("", None, empty));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = self.reserve_owned_batch(first, second) else {
            return;
        };
        if compat {
            self.link_compat_nodes_emit_key = None;
        }
        if ring {
            self.link_target_ring_emit_key = None;
        }
        self.publish_event_batch(reservation);
    }
}

/// 🕹️ Applies a press's selection change to the engine at once and stages its `select` row on
/// `gesture`, keeping the pre-press selection for a cancel. An unchanged selection stages nothing.
pub fn stage_selection_ids(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, gesture: &mut GestureStage, ids: &[String], merge: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    let previous = std::mem::replace(&mut focus.selection, next);
    gesture.restore.get_or_insert(previous);
    gesture.select = Some((sorted_selection_ids(&focus.selection), merge.map(str::to_string)));
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    outbox.last_select_emit_sig = None;
    focus.selection_exit_highlight.clear();
    sync_selection_flags_to_objects(scene, focus);
}

/// ↩️ Restores the selection a cancelled gesture's press staged; nothing is published.
pub fn restore_gesture_selection(scene: &mut BoardSceneState, focus: &mut BoardFocusState, gesture: GestureStage) {
    let Some(restore) = gesture.restore else {
        return;
    };
    focus.selection = restore;
    focus.selection_exit_highlight.clear();
    sync_selection_flags_to_objects(scene, focus);
    scene.bump_content_scene_generation();
}

/// ↩️ Abandons a live node drag with zero trace: every member returns to its grab-time position, the
/// press's staged selection is undone, and nothing reaches the document. The restored positions go out as
/// transient `nodeMove` preview rows only, so sibling panes that mirrored the drag snap back too.
pub fn cancel_node_drag(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, start_positions: BTreeMap<String, (f64, f64)>, gesture: GestureStage) {
    for (id, (x, y)) in &start_positions {
        if let Some(node) = scene.nodes.get_mut(id) {
            node.x = *x;
            node.y = *y;
            if let Ok(reservation) = BoardOwnedEvent::node_move(id, *x, *y).map_err(|_| ()).and_then(|event| outbox.events.reserve_event(event).map_err(|_| ())) {
                outbox.publish_event_reservation(reservation);
            }
        }
    }
    restore_gesture_selection(scene, focus, gesture);
    scene.bump_content_scene_generation();
}
//#endregion 🎬️Gestures

/// 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
pub fn clear_selection_on_background_click(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    if focus.selection.is_empty() {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &[], None, None, None) else {
        return;
    };
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    outbox.last_select_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection.clear();
    sync_selection_flags_to_objects(scene, focus);
    outbox.publish_event_reservation(reservation);
}

pub fn sync_selection_flags_to_objects(scene: &mut BoardSceneState, focus: &mut BoardFocusState) {
    let chrome = focus.selection_chrome_ids();
    for n in scene.nodes.values_mut() {
        n.selected = chrome.contains(&n.id);
    }
    for h in scene.handles.values_mut() {
        h.selected = chrome.contains(&h.id);
    }
    for e in scene.edges.values_mut() {
        e.selected = chrome.contains(&e.id);
    }
    for w in scene.wires.values_mut() {
        w.selected = chrome.contains(&w.id);
    }
    for r in scene.regions.values_mut() {
        r.selected = chrome.contains(&r.id);
    }
}

pub(crate) fn push_select_event(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    outbox.last_select_emit_sig = None;
    let mut sorted: Vec<_> = focus.selection.iter().cloned().collect();
    sorted.sort();
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
        return;
    };
    outbox.publish_event_reservation(reservation);
}

pub fn set_selection_ids(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
        return;
    };
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
    outbox.last_select_emit_sig = None;
    outbox.publish_event_reservation(reservation);
}

/// 🔇️ Updates committed selection without emitting `select` (controlled React sync).
pub fn set_selection_ids_silent(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
}

/// 🔇️ Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
pub fn set_preselect_state_silent(scene: &mut BoardSceneState, focus: &mut BoardFocusState, ids: &[String], removed_ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
    if focus.preselect == next && focus.preselect_removed == removed {
        return;
    }
    focus.preselect = next;
    focus.preselect_removed = removed;
    sync_selection_flags_to_objects(scene, focus);
}

pub fn set_selection_ids_gestured(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let sig = (sorted.clone(), gesture_owned);
    if next == focus.selection && outbox.last_select_emit_sig.as_ref() == Some(&sig) {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, gesture) else {
        return;
    };
    outbox.last_select_emit_sig = Some(sig);
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    if next != focus.selection {
        focus.selection_exit_highlight.clear();
        focus.selection = next;
        sync_selection_flags_to_objects(scene, focus);
    }
    outbox.publish_event_reservation(reservation);
}

/// 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
pub fn board_apply_area_preselect(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let sorted = sorted_selection_ids(&next);
    let removed = sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let sig = (sorted.clone(), removed.clone(), gesture_owned);
    if focus.preselect == next && outbox.last_preselect_emit_sig.as_ref() == Some(&sig) {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Preselect, &sorted, None, Some(&removed), gesture) else {
        return;
    };
    outbox.last_preselect_emit_sig = Some(sig);
    focus.preselect = next;
    focus.preselect_removed = anchor_ids.difference(&focus.preselect).cloned().collect();
    focus.set_hovered_id_silent(None);
    sync_selection_flags_to_objects(scene, focus);
    outbox.publish_event_reservation(reservation);
}

/// 🧿️ Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
pub fn commit_area_select_from_initial(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let sorted = sorted_selection_ids(&next);
    let anchor = sorted_selection_ids(initial_ids);
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, Some(&anchor), None, gesture) else {
        return;
    };
    outbox.last_select_emit_sig = None;
    outbox.last_preselect_emit_sig = None;
    focus.preselect.clear();
    focus.preselect_removed.clear();
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
    let _ = gesture_owned;
    outbox.publish_event_reservation(reservation);
}

/// 🧭️ Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
pub fn indirect_ring_node_id(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, lod: BoardDrawLod) -> Option<String> {
    if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        return None;
    }
    if let Interaction::LinkTargetNode { source_id, target_node_id } = &focus.interaction {
        if link_compatible_handle_count_on_node(scene, style, source_id, target_node_id) > 1 {
            return scene.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
        }
        return None;
    }
    if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &focus.interaction {
        if ring_handle_ids.len() > 1 {
            return scene.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
        }
        return None;
    }
    if let Interaction::LinkDragSnap { source_id, end_world, .. } = &focus.interaction {
        let ring_nid = link_drag_ring_target_node_id(scene, style, source_id, *end_world)?;
        if link_compatible_handle_count_on_node(scene, style, source_id, ring_nid.as_str()) > 1 {
            return Some(ring_nid);
        }
        return None;
    }
    if focus.active_link_source_handle_id().is_some() {
        return None;
    }
    let ring_nid = if focus.selection.len() == 1 {
        focus.selection.iter().next()?.clone()
    } else {
        return None;
    };
    let n = scene.nodes.get(&ring_nid).filter(|n| n.visible)?;
    if scene.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
        Some(ring_nid)
    } else {
        None
    }
}

fn sole_indirect_handle_hit_link_target(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    let Interaction::LinkTargetNode { source_id, target_node_id } = &focus.interaction else {
        return None;
    };
    let th = node_sole_free_link_compatible_handle(scene, style, source_id, target_node_id)?;
    let n = scene.nodes.get(target_node_id)?;
    if !n.visible {
        return None;
    }
    if !style.point_in_node_world(n, point) {
        return None;
    }
    Some(th)
}

fn sole_indirect_handle_hit_idle_selected_node(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if !matches!(focus.interaction, Interaction::None) {
        return None;
    }
    if focus.selection.len() != 1 {
        return None;
    }
    let nid = focus.selection.iter().next()?;
    if !scene.nodes.contains_key(nid) {
        return None;
    }
    let sole = scene.sole_eligible_indirect_handle_on_node(nid)?;
    let n = scene.nodes.get(nid)?;
    if !n.visible {
        return None;
    }
    if !style.point_in_node_world(n, point) {
        return None;
    }
    Some(sole)
}

pub(crate) fn indirect_ring_handle_eligible(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, handle_id: &str, ring_node_id: &str) -> bool {
    if scene.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
        return false;
    }
    if let Some(source_id) = focus.active_link_source_handle_id() {
        handle_eligible_link_target_ring(scene, style, handle_id, source_id)
    } else {
        scene.handle_eligible_indirect_connect_ring(handle_id)
    }
}

fn link_drag_target_ring_hit(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, source_id: &str, point: Point) -> Option<String> {
    if !matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        return None;
    }
    if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &focus.interaction {
        if active_source != source_id {
            return None;
        }
        let zoom = scene.camera.zoom;
        for hid in ring_handle_ids {
            let Some(h) = scene.handles.get(hid) else {
                continue;
            };
            if !handle_eligible_link_target_ring(scene, style, h.id.as_str(), source_id) {
                continue;
            }
            let Some(pos) = indirect_handle_world_pos(scene, style, h) else {
                continue;
            };
            let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
            if distance_between(point, pos) <= tol {
                return Some(h.id.clone());
            }
        }
        return None;
    }
    let node_id = link_drag_ring_target_node_id(scene, style, source_id, point)?;
    if link_compatible_handle_count_on_node(scene, style, source_id, node_id.as_str()) <= 1 {
        return None;
    }
    let zoom = scene.camera.zoom;
    for h in scene.handles.values().rev() {
        if h.node_id != node_id || !handle_eligible_link_target_ring(scene, style, h.id.as_str(), source_id) {
            continue;
        }
        let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
        if distance_between(point, pos) <= tol {
            return Some(h.id.clone());
        }
    }
    None
}

fn link_target_ring_snapshot(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, source_handle_id: &str) -> (Option<String>, Vec<String>) {
    if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &focus.interaction {
        if source_id == source_handle_id {
            return (ring_node_id.clone(), ring_handle_ids.clone());
        }
    }
    let node_id = match &focus.interaction {
        Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
        Interaction::LinkDragSnap { end_world, .. } => link_drag_ring_target_node_id(scene, style, source_handle_id, *end_world),
        _ => None,
    };
    let Some(nid) = node_id else {
        return (None, Vec::new());
    };
    if link_compatible_handle_count_on_node(scene, style, source_handle_id, nid.as_str()) <= 1 {
        return (None, Vec::new());
    }
    (Some(nid.clone()), link_compatible_handle_ids_on_node(scene, style, source_handle_id, nid.as_str()))
}

pub fn sync_link_gesture_events(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState) {
    if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = &focus.interaction {
        if compatible_node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
            outbox.event_schema_fault = true;
            return;
        }
        let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
        let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
        let compat_changed = outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
        let ring_changed = outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
        let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(source_id, compatible_node_ids));
        let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), ring_handle_ids));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = outbox.reserve_owned_batch(first, second) else {
            return;
        };
        if compat_changed {
            outbox.link_compat_nodes_emit_key = Some(compat_key);
        }
        if ring_changed {
            outbox.link_target_ring_emit_key = Some(ring_key);
        }
        outbox.publish_event_batch(reservation);
        return;
    }
    let Some(source) = focus.active_link_source_handle_id().map(str::to_string) else {
        outbox.clear_link_gesture_events();
        return;
    };
    let node_ids = link_drag_compatible_target_node_ids(scene, style, &source);
    if node_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
        outbox.event_schema_fault = true;
        return;
    }
    let compat_key = format!("{}|{}", source, node_ids.join(","));
    let (ring_node_id, ring_handle_ids) = link_target_ring_snapshot(scene, focus, style, &source);
    if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
        outbox.event_schema_fault = true;
        return;
    }
    let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
    let compat_changed = outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
    let ring_changed = outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
    let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(&source, &node_ids));
    let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(&source, ring_node_id.as_deref(), &ring_handle_ids));
    let (first, second) = match (compat_event, ring_event) {
        (Some(first), second) => (first, second),
        (None, Some(first)) => (first, None),
        (None, None) => return,
    };
    let Some(reservation) = outbox.reserve_owned_batch(first, second) else {
        return;
    };
    if compat_changed {
        outbox.link_compat_nodes_emit_key = Some(compat_key);
    }
    if ring_changed {
        outbox.link_target_ring_emit_key = Some(ring_key);
    }
    outbox.publish_event_batch(reservation);
}

pub fn active_link_wire_curve(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Option<CubicBez> {
    match &focus.interaction {
        Interaction::LinkDragSnap { source_id, target_id, end_world } => link_drag_wire_curve_world(scene, style, source_id.as_str(), target_id.as_deref(), *end_world),
        Interaction::LinkTargetNode { source_id, target_node_id } => link_drag_wire_curve_world(scene, style, source_id.as_str(), None, scene.node_center_world(target_node_id)?),
        Interaction::ExternalLinkPreview { source_id, end_world, .. } => link_drag_wire_curve_world(scene, style, source_id.as_str(), None, *end_world),
        Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => link_drag_wire_curve_world(scene, style, src.as_str(), Some(tgt.as_str()), Point::ZERO),
        _ => None,
    }
}

/// 🔗️ Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
pub fn resolve_overview_free_link_handle_pointer_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if !matches!(current_draw_lod(scene, style), BoardDrawLod::Overview) {
        return None;
    }
    if !focus.selection_options.select_handles {
        return None;
    }
    const MAX_D_WORLD: f64 = 2.25;
    let mut best: Option<(f64, String)> = None;
    for h in scene.handles.values() {
        if !scene.handle_effectively_visible(h.id.as_str()) || scene.handle_has_incident_edge(h.id.as_str()) {
            continue;
        }
        let Some(pos) = handle_world_pos(scene, style, h) else {
            continue;
        };
        let d = distance_between(point, pos);
        if d <= MAX_D_WORLD && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            best = Some((d, h.id.clone()));
        }
    }
    best.map(|(_, id)| id)
}

pub fn resolve_hover_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    let lod = current_draw_lod(scene, style);
    let zoom = scene.camera.zoom;
    if style.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
        if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_link_target(scene, focus, style, point) {
                return Some(hid);
            }
            if let Interaction::LinkDragSnap { source_id, .. } = &focus.interaction {
                if let Some(hid) = link_drag_target_ring_hit(scene, focus, style, source_id, point) {
                    return Some(hid);
                }
            }
        }
        if let Some(ring_node_id) = indirect_ring_node_id(scene, focus, style, lod) {
            for h in scene.handles.values().rev() {
                if h.node_id != ring_node_id || !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                if !indirect_ring_handle_eligible(scene, focus, style, h.id.as_str(), ring_node_id.as_str()) {
                    continue;
                }
                let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
            for h in scene.handles.values().rev() {
                if !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + effective_handle_radius(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_idle_selected_node(scene, focus, style, point) {
                if scene.handle_selectable(hid.as_str()) {
                    return Some(hid);
                }
            }
        }
    }
    for n in scene.nodes.values().rev() {
        if !scene.node_selectable(n.id.as_str()) {
            continue;
        }
        match n.shape {
            NodeShape::Rectangle => {
                let hw = style.scaled_node_width(n) / 2.0;
                let hh = style.scaled_node_height(n) / 2.0;
                if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                    return Some(n.id.clone());
                }
            }
            NodeShape::Circle => {
                if distance_between(point, Point::new(n.x, n.y)) <= style.scaled_node_radius(n) {
                    return Some(n.id.clone());
                }
            }
        }
    }
    for w in scene.wires.values().rev() {
        if !scene.wire_selectable(w) {
            continue;
        }
        if let Some(c) = wire_curve(scene, style, w) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                return Some(w.id.clone());
            }
        }
    }
    for e in scene.edges.values().rev() {
        if !edge_selectable(scene, style, e) {
            continue;
        }
        if let Some(c) = board_edge_curve(scene, style, e) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                return Some(e.id.clone());
            }
        }
    }
    None
}

pub fn resolve_hit_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if lod_disables_discrete_pick(scene, style) {
        return None;
    }
    let zoom = scene.camera.zoom;
    let o = &focus.selection_options;
    if style.has_ports() && o.select_handles {
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_link_target(scene, focus, style, point) {
                return Some(hid);
            }
            if let Interaction::LinkDragSnap { source_id, .. } = &focus.interaction {
                if let Some(hid) = link_drag_target_ring_hit(scene, focus, style, source_id, point) {
                    return Some(hid);
                }
            }
        }
        if let Some(ring_node_id) = indirect_ring_node_id(scene, focus, style, current_draw_lod(scene, style)) {
            for h in scene.handles.values().rev() {
                if h.node_id != ring_node_id || !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                if !indirect_ring_handle_eligible(scene, focus, style, h.id.as_str(), ring_node_id.as_str()) {
                    continue;
                }
                let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
            for h in scene.handles.values().rev() {
                if !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + effective_handle_radius(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_idle_selected_node(scene, focus, style, point) {
                return Some(hid);
            }
        }
    }
    if o.select_nodes {
        for n in scene.nodes.values().rev() {
            if !scene.node_selectable(n.id.as_str()) {
                continue;
            }
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = style.scaled_node_width(n) / 2.0;
                    let hh = style.scaled_node_height(n) / 2.0;
                    if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                        return Some(n.id.clone());
                    }
                }
                NodeShape::Circle => {
                    if distance_between(point, Point::new(n.x, n.y)) <= style.scaled_node_radius(n) {
                        return Some(n.id.clone());
                    }
                }
            }
        }
    }
    if o.select_edges {
        for e in scene.edges.values().rev() {
            if !edge_selectable(scene, style, e) {
                continue;
            }
            if let Some(c) = board_edge_curve(scene, style, e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(e.id.clone());
                }
            }
        }
    }
    None
}

impl BoardFocusState {
    pub fn close_interaction_step(&mut self) -> bool {
        match &mut self.interaction {
            Interaction::None | Interaction::Pan { .. } => {
                self.interaction = Interaction::None;
                true
            }
            Interaction::DragNodes { primary_id, start_positions, proximity_pair, gesture, .. } => {
                if let Some((id, _)) = start_positions.pop_first() {
                    drop(id);
                    return false;
                }
                if let Some(id) = gesture.select.as_mut().and_then(|(ids, _)| ids.pop()).or_else(|| gesture.restore.as_mut().and_then(BTreeSet::pop_first)) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(gesture));
                if let Some((left, right)) = proximity_pair.take() {
                    if right.is_empty() {
                        drop(left);
                    } else {
                        drop(right);
                        *proximity_pair = Some((left, String::new()));
                    }
                    return false;
                }
                drop(std::mem::take(primary_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::SelectionPending { initial_ids, .. } => {
                if let Some(id) = initial_ids.pop_first() {
                    drop(id);
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::Selection { initial_ids, points, screen_points, .. } => {
                if initial_ids.pop_first().is_some() || points.pop().is_some() || screen_points.pop().is_some() {
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkAtSourceHandle { source_id, .. } => {
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(target) = target_id.take() {
                    drop(target);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                if !target_node_id.is_empty() {
                    drop(std::mem::take(target_node_id));
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } => {
                if let Some(id) = compatible_node_ids.pop().or_else(|| ring_handle_ids.pop()).or_else(|| ring_node_id.take()) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
        }
    }
}

impl BoardFocusState {
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.selection_options.method = method.into();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_edges = select_edges;
        self.selection_options.select_handles = select_handles;
    }

    fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() {
            return None;
        }
        if self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return Some(BoardElementStyleKind::Hovered);
        }
        if self.hovered_id.is_none() {
            if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                if hover_domain == domain && hover_kind == element_kind {
                    return Some(BoardElementStyleKind::Hovered);
                }
            }
        }
        None
    }

    fn highlighted_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() || self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return None;
        }
        if self.highlighted_ids.contains(id) {
            return Some(BoardElementStyleKind::Highlighted);
        }
        None
    }

    pub fn is_preselect_active(&self) -> bool {
        self.is_preselecting() || !self.preselect.is_empty()
    }

    /// 🎨️ During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
    fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
        if self.is_preselect_active() {
            if self.preselect.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Highlighted;
            }
            return BoardElementStyleKind::Neutral;
        }
        if self.selection.contains(id) {
            return BoardElementStyleKind::Selected;
        }
        if self.highlighted_ids.contains(id) {
            return BoardElementStyleKind::Highlighted;
        }
        BoardElementStyleKind::Neutral
    }

    fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = explicit_style_kind(e.style.as_deref()) {
            return locked_style_dim(kind, e.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                    return locked_style_dim(kind, e.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(e.id.as_str()) {
                    return locked_style_dim(kind, e.locked);
                }
                self.resolve_interaction_style_kind(e.id.as_str())
            }
        };
        locked_style_dim(kind, e.locked)
    }

    pub(crate) fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = explicit_style_kind(w.style.as_deref()) {
            return locked_style_dim(kind, w.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                    return locked_style_dim(kind, w.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(w.id.as_str()) {
                    return locked_style_dim(kind, w.locked);
                }
                self.resolve_interaction_style_kind(w.id.as_str())
            }
        };
        locked_style_dim(kind, w.locked)
    }
}

/// 🔗️ Clears host-driven link preview without touching local link drags.
pub fn clear_external_link_preview(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    if matches!(focus.interaction, Interaction::ExternalLinkPreview { .. }) {
        focus.interaction = Interaction::None;
        outbox.clear_link_gesture_events();
    }
}

pub fn board_set_camera(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64) {
    set_camera_internal(scene, outbox, x, y, zoom, true);
}

/// 🔇️ Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
pub fn set_camera_silent(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64) {
    set_camera_internal(scene, outbox, x, y, zoom, false);
}

fn set_camera_internal(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64, emit_event: bool) {
    let zoom = semio_framework_canvas::camera::clamp_zoom(zoom);
    if (scene.camera.x - x).abs() < 1e-9 && (scene.camera.y - y).abs() < 1e-9 && (scene.camera.zoom - zoom).abs() < 1e-9 {
        return;
    }
    let reservation = if emit_event {
        let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::camera(x, y, zoom)) else {
            return;
        };
        Some(reservation)
    } else {
        None
    };
    scene.camera.x = x;
    scene.camera.y = y;
    scene.camera.zoom = zoom;
    scene.interaction_revision = scene.interaction_revision.wrapping_add(1);
    if let Some(reservation) = reservation {
        outbox.publish_event_reservation(reservation);
    }
}

fn ids_matching_kind_hover(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    let Some((domain, kind_id)) = focus.hovered_kind.as_ref() else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    match domain.as_str() {
        "node" => {
            for node in scene.nodes.values() {
                if &node.node_kind == kind_id && !focus.selection.contains(&node.id) {
                    ids.push(node.id.clone());
                }
            }
        }
        "handle" => {
            for handle in scene.handles.values() {
                if &handle.handle_kind == kind_id && !focus.selection.contains(&handle.id) {
                    ids.push(handle.id.clone());
                }
            }
        }
        "edge" => {
            for edge in scene.edges.values() {
                if &edge.edge_kind == kind_id && !focus.selection.contains(&edge.id) {
                    ids.push(edge.id.clone());
                }
            }
        }
        "wire" => {
            for wire in scene.wires.values() {
                if &wire.wire_kind == kind_id && !focus.selection.contains(&wire.id) {
                    ids.push(wire.id.clone());
                }
            }
        }
        _ => {}
    }
    ids
}

pub fn resolve_node_style_kind(focus: &BoardFocusState, style_state: &BoardStyleState, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
    if let Some(kind) = explicit_style_kind(n.style.as_deref()) {
        return locked_style_dim(kind, n.locked);
    }
    let kind = match pass {
        StyleChromePass::CachedBase => {
            if style_state.preserve_original_element_style {
                BoardElementStyleKind::Original
            } else {
                BoardElementStyleKind::Neutral
            }
        }
        StyleChromePass::InteractionOverlay => {
            if let Some(kind) = focus.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                return locked_style_dim(kind, n.locked);
            }
            if let Some(kind) = focus.highlighted_style_kind(n.id.as_str()) {
                return locked_style_dim(kind, n.locked);
            }
            focus.resolve_interaction_style_kind(n.id.as_str())
        }
    };
    locked_style_dim(kind, n.locked)
}

pub(crate) fn resolve_handle_style_kind(focus: &BoardFocusState, style_state: &BoardStyleState, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
    if let Some(kind) = explicit_style_kind(h.style.as_deref()) {
        return locked_style_dim(kind, h.locked);
    }
    let kind = match pass {
        StyleChromePass::CachedBase => {
            if style_state.preserve_original_element_style {
                BoardElementStyleKind::Original
            } else {
                BoardElementStyleKind::Neutral
            }
        }
        StyleChromePass::InteractionOverlay => {
            if let Some(kind) = focus.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                return locked_style_dim(kind, h.locked);
            }
            if let Some(kind) = focus.highlighted_style_kind(h.id.as_str()) {
                return locked_style_dim(kind, h.locked);
            }
            focus.resolve_interaction_style_kind(h.id.as_str())
        }
    };
    locked_style_dim(kind, h.locked)
}

/// 💠️ Entity ids whose selection/preselect/hover chrome tints fills and strokes without rebuilding {@link BoardHost.world_content_cache}.
pub fn interaction_overlay_entity_ids(scene: &BoardSceneState, focus: &BoardFocusState) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    if focus.is_preselect_active() {
        ids.extend(focus.preselect.iter().cloned());
        ids.extend(focus.selection.iter().cloned());
        ids.extend(focus.preselect_removed.iter().cloned());
    } else {
        ids.extend(focus.selection.iter().cloned());
        ids.extend(focus.selection_exit_highlight.iter().cloned());
    }
    if let Some(ref hover_id) = focus.hovered_id {
        if !focus.is_preselect_active() && !focus.selection.contains(hover_id) {
            ids.insert(hover_id.clone());
        }
    }
    if focus.hovered_id.is_none() && !focus.is_preselect_active() {
        for id in ids_matching_kind_hover(scene, focus) {
            ids.insert(id);
        }
    }
    for id in focus.highlighted_ids.iter() {
        if !focus.selection.contains(id) && focus.hovered_id.as_deref() != Some(id.as_str()) {
            ids.insert(id.clone());
        }
    }
    ids
}

pub(crate) fn resolve_edge_stroke_paint(focus: &BoardFocusState, style: &BoardStyleState, e: &EdgeData, chrome_pass: StyleChromePass, lod: BoardDrawLod, lod_scale_width: f64) -> (Color, Stroke, f64) {
    let style_kind = focus.resolve_edge_style_kind(e, chrome_pass);
    let chrome = edge_stroke_for_style(&style.canvas_theme, style_kind);
    let kind_def = style.edge_kinds.get(e.edge_kind.as_str());
    let base_color = kind_def.and_then(|d| d.color).unwrap_or(style.canvas_theme.edge_stroke);
    let stroke_color = match style_kind {
        BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
        _ if lod == BoardDrawLod::Minimap => chrome,
        _ => lerp_color(base_color, chrome, 0.55),
    };
    let catalog_w = kind_def.map_or(2.0, |d| d.stroke_width);
    let width_mult = match style_kind {
        BoardElementStyleKind::Selected => ui_styling::strokes::EDGE_SELECTED_MULT,
        BoardElementStyleKind::Hovered => ui_styling::strokes::EDGE_HOVERED_MULT,
        _ => 1.0,
    };
    let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
    let pattern = kind_def.map_or(EdgeStrokePattern::Solid, |d| d.pattern);
    (stroke_color, edge_stroke_for_kind_pattern(pattern, width), width)
}

impl BoardFocusState {
    fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
        selection_drag_shape(self.selection_options.method.as_str(), start, points)
    }

    fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        const STEPS: usize = 24;
        let mut samples = Vec::with_capacity(STEPS + 1);
        for i in 0..=STEPS {
            let t = i as f64 / STEPS as f64;
            samples.push(cubic_bezier_point(c, t));
        }
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                samples.iter().all(|&p| point_in_polygon(p, polygon))
            } else {
                samples.iter().all(|&p| world_box_contains_point(box_, p))
            }
        } else if lasso {
            (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
        } else {
            (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
        }
    }
}

fn selection_draggable_node_members(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    focus.selection.iter().filter(|id| scene.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
}

fn selection_union_bounds_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Option<WorldBox> {
    let mut corners: Vec<Point> = Vec::new();
    for id in &focus.selection {
        let Some(n) = scene.nodes.get(id) else {
            continue;
        };
        let b = style.node_world_bounds(n, 0.0);
        corners.push(Point::new(b.min_x, b.min_y));
        corners.push(Point::new(b.max_x, b.max_y));
    }
    world_box_from_points(&corners)
}

/// 📐️ Every selected node id in engine order — the rotate gesture's members, pivot
/// contributors and event ids all come from this one list.
pub fn transform_selection_node_ids(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    focus.selection.iter().filter(|id| scene.nodes.contains_key(*id)).cloned().collect()
}

/// 📦️ Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
pub fn try_begin_bounded_selection_drag_at(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, world: Point) -> bool {
    if !board_lod_uses_bounded_drag(scene, style) {
        return false;
    }
    let members: Vec<String> = selection_draggable_node_members(scene, focus).into_iter().filter(|id| scene.nodes.get(id).is_some_and(|n| !n.locked)).collect();
    if members.is_empty() {
        return false;
    }
    let Some(bounds) = selection_union_bounds_world(scene, focus, style) else {
        return false;
    };
    let pad = BOUNDED_DRAG_HIT_PAD_PX / scene.camera.zoom.max(1e-9);
    if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
        return false;
    }
    let primary_id = members
        .iter()
        .min_by(|a, b| {
            let da = scene.nodes.get(*a).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
            let db = scene.nodes.get(*b).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
        .unwrap_or_else(|| members[0].clone());
    let (px0, py0) = scene.nodes.get(&primary_id).map_or((0.0, 0.0), |n| (n.x, n.y));
    let mut start_positions = BTreeMap::new();
    for id in &members {
        if let Some(n) = scene.nodes.get(id) {
            start_positions.insert(id.clone(), (n.x, n.y));
        }
    }
    let gesture = outbox.begin_gesture();
    focus.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
    set_hovered_id(scene, focus, outbox, None);
    true
}

fn selection_contains_node(focus: &BoardFocusState, style: &BoardStyleState, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
    let bounds = style.node_world_bounds(n, 0.0);
    let lasso = focus.selection_options.method == "lasso";
    if enclosing {
        if lasso {
            polygon_contains_world_box(polygon, bounds)
        } else {
            world_box_contains_box(box_, bounds)
        }
    } else if lasso {
        polygon_intersects_world_box(polygon, bounds)
    } else {
        world_boxes_overlap(box_, bounds)
    }
}

fn selection_contains_handle(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
    let Some(pos) = handle_world_pos(scene, style, h) else {
        return false;
    };
    let pad = effective_handle_radius(scene, style, h).max(1.0);
    let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
    let lasso = focus.selection_options.method == "lasso";
    if enclosing {
        if lasso {
            polygon_contains_world_box(polygon, bounds)
        } else {
            world_box_contains_box(box_, bounds)
        }
    } else if lasso {
        polygon_intersects_world_box(polygon, bounds)
    } else {
        world_boxes_overlap(box_, bounds)
    }
}

pub fn resolve_area_selection_with_initial(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
    let Some((box_, enclosing, ref polygon)) = focus.selection_drag_shape_world(start, points) else {
        return initial.clone();
    };
    let mut hits = BTreeSet::new();
    let o = &focus.selection_options;
    if o.select_nodes {
        for n in scene.nodes.values() {
            if scene.node_selectable(n.id.as_str()) && selection_contains_node(focus, style, n, box_, enclosing, polygon) {
                hits.insert(n.id.clone());
            }
        }
    }
    if o.select_handles {
        for h in scene.handles.values() {
            if scene.handle_selectable(h.id.as_str()) && selection_contains_handle(scene, focus, style, h, box_, enclosing, polygon) {
                hits.insert(h.id.clone());
            }
        }
    }
    if o.select_edges {
        for e in scene.edges.values() {
            if !edge_selectable(scene, style, e) {
                continue;
            }
            if let Some(c) = board_edge_curve(scene, style, e) {
                if focus.selection_contains_edge(c, box_, enclosing, polygon) {
                    hits.insert(e.id.clone());
                }
            }
        }
    }
    merge_ids_into_selection(initial, &hits, merge_mode)
}

pub fn test_resolve_node_style_kind(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, node_id: &str) -> Option<BoardElementStyleKind> {
    scene.nodes.get(node_id).map(|n| resolve_node_style_kind(focus, style, n, StyleChromePass::InteractionOverlay))
}

/// 🧩️ BoardHost focus component: selection, preselect, hover and highlight state with the active utility.
pub struct BoardFocusState {
    pub selection: BTreeSet<String>,
    /// 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
    pub preselect: BTreeSet<String>,
    /// 💠️ During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
    pub preselect_removed: BTreeSet<String>,
    /// 💠️ After commit: ids dropped in the last `select` transition only.
    pub selection_exit_highlight: BTreeSet<String>,
    pub selection_options: SelectionOptions,
    pub hovered_id: Option<String>,
    /// 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
    pub hovered_kind: Option<(String, String)>,
    /// 💠️ Externally driven highlight ids (e.g. cross-panel variable binding); below hover, above neutral.
    pub highlighted_ids: BTreeSet<String>,
    pub interaction: Interaction,
    /// Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
    pub selection_screen_preview: Option<Vec<Point>>,
    /// ↔ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
    pub selection_preview_crossing: bool,
    /// 🖌️ Active viewport utility (`select` suppresses brush slot logic).
    pub active_utility: ActiveUtility,
}

/// 🧩️ BoardHost outbox component: the outgoing board event queue and its emit dedupe signatures.
pub struct BoardOutboxState {
    pub events: BoardEventQueue,
    pub event_overflow: Option<BoardOwnedEvent>,
    pub event_batch_overflow: Option<BoardEventBatchReservation>,
    pub event_schema_fault: bool,
    /// 📡️ Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
    pub link_compat_nodes_emit_key: Option<String>,
    /// 📡️ Dedupes {@code linkTargetRing} emissions while a link wire is active.
    pub link_target_ring_emit_key: Option<String>,
    /// 📡️ Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
    pub last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
    /// 📡️ Dedupes `preselect` emissions during area-select drag.
    pub last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
    /// 🪪️ Serial of the last pointer gesture opened ([`GestureStage::id`] is `gesture-<serial>`).
    pub gesture_serial: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPhase {
    Move,
    Up,
    Leave,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardPointerIntent {
    pub phase: BoardPointerPhase,
    pub x: f64,
    pub y: f64,
    pub shift: bool,
    pub ctrl_or_meta: bool,
    pub alt: bool,
}

pub(crate) fn board_edge_event_owned_bytes(id: &str, source: &str, target: &str) -> Option<usize> {
    let bytes = 27usize.checked_add(board_json_string_bytes(id))?.checked_add(board_json_string_bytes(source))?.checked_add(board_json_string_bytes(target))?;
    (bytes <= BOARD_EVENT_PAYLOAD_BYTE_CAPACITY).then_some(bytes)
}

impl BoardFocusState {
    /// 🔇️ Updates hover chrome without emitting `hover` (controlled React sync).
    pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        self.hovered_id = id;
        self.hovered_kind = None;
    }

    /// 💠️ Externally driven highlight set (cross-panel binding); does not emit events.
    pub fn set_highlighted_ids(&mut self, ids: Vec<String>) {
        let next: BTreeSet<String> = ids.into_iter().collect();
        if self.highlighted_ids == next {
            return;
        }
        self.highlighted_ids = next;
    }

    /// 💠️ Current externally driven highlight ids as JSON array.
    pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {
        Ok(serde_json::to_string(&self.highlighted_ids.iter().cloned().collect::<Vec<_>>())?)
    }

    /// 🔇️ Mirrors controlled kind hover without emitting `hover`.
    pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        self.hovered_id = None;
        self.hovered_kind = next_kind;
    }
}

/// 🚚️ One descriptor sync; `announce_new_edges` decides whether edges absent from the current
/// scene emit `edgeCreate`. A snapshot parse passes `false`: it clears the scene first, so every
/// edge is "new" — announcing them echoed the WHOLE document back to the plugin as edge
/// creations after each re-parse (fill placements re-committing themselves) and, undrained,
/// they exhausted the event credits so the next parse of any edged document was refused
/// (three blank panes after one drag, 2026-09-17).
pub fn sync_descriptor_with(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, outbox: &mut BoardOutboxState, desc: &SceneDescriptor, announce_new_edges: bool) -> Result<(), NormalPortError> {
    descriptor_admission(desc)?;
    if matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
        focus.interaction = Interaction::None;
        outbox.clear_link_gesture_events();
    }
    if announce_new_edges {
        let mut created_edge_count = 0usize;
        let mut created_edge_bytes = 0usize;
        for edge in desc.edges.iter().filter(|edge| !scene.edges.contains_key(&edge.id)) {
            created_edge_count = created_edge_count.checked_add(1).ok_or(NormalPortError::EventCredits)?;
            let bytes = board_edge_event_owned_bytes(&edge.id, &edge.source, &edge.target).ok_or(NormalPortError::EventCredits)?;
            created_edge_bytes = created_edge_bytes.checked_add(bytes).ok_or(NormalPortError::EventCredits)?;
        }
        outbox.events.reserve(created_edge_count, created_edge_bytes).map_err(|_| NormalPortError::EventCredits)?;
    }
    let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
    let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
    let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
    let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
    let want_regions: BTreeSet<_> = desc.regions.iter().map(|r| r.id.clone()).collect();
    scene.regions.retain(|id, _| want_regions.contains(id));
    scene.edges.retain(|id, _| want_edges.contains(id));
    scene.wires.retain(|id, _| want_wires.contains(id));
    scene.handles.retain(|id, _| want_handles.contains(id));
    scene.nodes.retain(|id, _| want_nodes.contains(id));
    for n in &desc.nodes {
        let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
            NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
        };
        let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
        let properties = n.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.nodes.insert(
            n.id.clone(),
            NodeData {
                id: n.id.clone(),
                x: n.x,
                y: n.y,
                shape,
                radius,
                width,
                height,
                scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                draggable: n.draggable.unwrap_or(true),
                selected: n.selected.unwrap_or(false),
                visible: n.visible.unwrap_or(true),
                locked: n.locked.unwrap_or(false),
                root: n.root.unwrap_or(false),
                style: n.style.clone(),
                text: n.text.clone(),
                icon_kind: n.icon_kind.clone(),
                node_kind,
                properties,
            },
        );
    }
    for h in &desc.handles {
        let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
        let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            None => None,
            Some(s) => Some(parse_css_color(s).ok_or_else(|| NormalPortError::InvalidHandleColor(h.id.clone(), s.to_string()))?),
        };
        let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let properties = h.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.handles.insert(
            h.id.clone(),
            HandleData {
                id: h.id.clone(),
                node_id: h.node_id.clone(),
                angle: h.angle,
                radius: h.radius.unwrap_or(ui_styling::radii::HANDLE_DEFAULT),
                scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                selected: h.selected.unwrap_or(false),
                visible: h.visible.unwrap_or(true),
                locked: h.locked.unwrap_or(false),
                style: h.style.clone(),
                handle_kind: kind,
                color_fill,
                icon_kind,
                properties,
            },
        );
    }
    for e in &desc.edges {
        let existed = scene.edges.contains_key(&e.id);
        let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
        let source_tip = parse_catalog_tip_slot(e.source_tip.as_deref());
        let target_tip = parse_catalog_tip_slot(e.target_tip.as_deref());
        let properties = e.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.edges.insert(
            e.id.clone(),
            EdgeData {
                id: e.id.clone(),
                source: e.source.clone(),
                target: e.target.clone(),
                selected: e.selected.unwrap_or(false),
                visible: e.visible.unwrap_or(true),
                locked: e.locked.unwrap_or(false),
                style: e.style.clone(),
                edge_kind,
                source_tip,
                target_tip,
                properties,
            },
        );
        if announce_new_edges && !existed {
            let event = BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &e.id, &e.source, &e.target).expect("new descriptor edges were exactly preflighted");
            outbox.events.push(event).expect("new descriptor edge credits were reserved before mutation");
        }
    }
    for w in &desc.wires {
        let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let (end_x, end_y) = match &target {
            Some(_) => (None, None),
            None => {
                let x = match w.end_x {
                    Some(v) if v.is_finite() => Some(v),
                    _ => None,
                };
                let y = match w.end_y {
                    Some(v) if v.is_finite() => Some(v),
                    _ => None,
                };
                if x.is_none() || y.is_none() {
                    continue;
                }
                (x, y)
            }
        };
        let wire_kind = w
            .wire_kind
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| scene.handles.get(w.source.as_str()).map(|h| style_state.resolve_default_wire_kind_for_handle(h)))
            .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
        let properties = w.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.wires.insert(
            w.id.clone(),
            WireData {
                id: w.id.clone(),
                source: w.source.clone(),
                target,
                end_x,
                end_y,
                selected: w.selected.unwrap_or(false),
                visible: w.visible.unwrap_or(true),
                locked: w.locked.unwrap_or(false),
                style: w.style.clone(),
                wire_kind,
                properties,
            },
        );
    }
    for r in &desc.regions {
        scene.regions.insert(
            r.id.clone(),
            RegionData {
                id: r.id.clone(),
                x: r.x,
                y: r.y,
                width: r.width,
                height: r.height,
                label: r.label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
                hidden: r.hidden.unwrap_or(false),
                locked: r.locked.unwrap_or(false),
                selected: r.selected.unwrap_or(false),
            },
        );
    }
    if !focus.is_preselect_active() {
        let mut new_selection = BTreeSet::new();
        for r in &desc.regions {
            if r.selected == Some(true) {
                new_selection.insert(r.id.clone());
            }
        }
        for n in &desc.nodes {
            if n.selected == Some(true) {
                new_selection.insert(n.id.clone());
            }
        }
        for h in &desc.handles {
            if h.selected == Some(true) {
                new_selection.insert(h.id.clone());
            }
        }
        for e in &desc.edges {
            if e.selected == Some(true) {
                new_selection.insert(e.id.clone());
            }
        }
        for w in &desc.wires {
            if w.selected == Some(true) {
                new_selection.insert(w.id.clone());
            }
        }
        let prev_sel = focus.selection.clone();
        if prev_sel != new_selection {
            focus.selection_exit_highlight.clear();
        }
        focus.selection = new_selection;
        if prev_sel != focus.selection {
            push_select_event(focus, outbox);
        }
    }
    sync_selection_flags_to_objects(scene, focus);
    scene.bump_content_scene_generation();
    Ok(())
}

pub fn clear_scene(scene: &mut BoardSceneState, focus: &mut BoardFocusState) {
    scene.regions.clear();
    scene.edges.clear();
    scene.wires.clear();
    scene.handles.clear();
    scene.nodes.clear();
    focus.selection.clear();
    focus.preselect.clear();
    focus.preselect_removed.clear();
    focus.selection_exit_highlight.clear();
}

/// 🎲️ Replaces the whole board with owned snapshot facts, ALL OR NOTHING: the descriptor is built and admitted
/// first, and only a snapshot that will really paint clears the live scene. Clearing before the
/// refusal left the panes blank until the next parse — the refusal and "the board went empty"
/// were the same event (2026-09-17 battery, `16-inspection/inspector-fresh-on-open`).
pub fn load_board_snapshot(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, f: BoardSnapshot) -> bool {
    let port_mode = match f.schema.as_str() {
        "board.normal.undirected.v1" => GraphPortMode::Normal,
        "board.ports.directed.v1" => GraphPortMode::Ported,
        _ => return false,
    };
    let has_ports = port_mode.has_ports();
    let camera = f.camera.as_ref().map(|camera| (camera.x, camera.y, camera.zoom));
    let Some(desc) = board_snapshot_scene_descriptor(f, has_ports) else {
        return false;
    };
    if descriptor_admission(&desc).is_err() {
        return false;
    }
    style.port_mode = port_mode;
    if !has_ports {
        focus.selection_options.select_handles = false;
    }
    if let Some((x, y, zoom)) = camera {
        board_set_camera(scene, outbox, x, y, zoom);
    }
    clear_scene(scene, focus);
    sync_descriptor_with(scene, focus, style, outbox, &desc, false).is_ok()
}

pub fn append_indirect_handle_ring(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
    for h in scene_state.handles.values() {
        if h.node_id != node_id || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if !indirect_ring_handle_eligible(scene_state, focus, style, h.id.as_str(), node_id) {
            continue;
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = indirect_handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = indirect_handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass);
        let stroke_px = 2.0_f64;
        let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((style.canvas_theme.indirect_handle_fill, style.canvas_theme.indirect_handle_stroke, stroke_px)) } else { None };
        append_handle_marker(scene_state, style, scene, h, wp, indirect_handle_marker_radius_world(scene_state, style, h), false, style_kind, paint_override, world_space, NodeHandlePaintLayer::Full, false);
    }
}

fn paint_node_geometry(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, n: &NodeData, lod: BoardDrawLod, world_space: bool, layer: NodeHandlePaintLayer, chrome_pass: StyleChromePass, link_compat: bool) {
    let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
    let resolved_style_kind = resolve_node_style_kind(focus, style, n, chrome_pass);
    let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
    let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
    let stroke_c = node_stroke_for_style(&style.canvas_theme, style_kind);
    let fill = if lod == BoardDrawLod::Minimap { stroke_c } else { style.resolve_node_fill_color(n, &style.canvas_theme, style_kind) };
    let sw = 2.0_f64;
    let paint_fill = if lod == BoardDrawLod::Minimap {
        matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill)
    } else {
        matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill) && !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
    };
    let paint_stroke = draw_node_stroke && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
    let paint_icons = draw_node_icons && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
    match n.shape {
        NodeShape::Circle => {
            let c = scene_state.draw_space_point(Point::new(n.x, n.y), world_space);
            let r = scene_state.draw_space_len(style.scaled_node_radius(n), world_space);
            let circle = Circle::new(c, r);
            if paint_fill {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            }
            if paint_stroke {
                scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
            }
            if paint_icons {
                paint_node_icon(scene_state, style, scene, n, world_space, style_kind, Some(&circle), None);
            }
        }
        NodeShape::Rectangle => {
            let hw = style.scaled_node_width(n) / 2.0;
            let hh = style.scaled_node_height(n) / 2.0;
            let p0 = scene_state.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
            let p1 = scene_state.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
            let rect = Rect::from_points(p0, p1);
            if paint_fill {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
            }
            if paint_stroke {
                scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &rect);
            }
            if paint_icons {
                paint_node_icon(scene_state, style, scene, n, world_space, style_kind, None, Some(rect));
            }
        }
    }
}

pub fn append_nodes_and_handles_with_overlay_chrome(
    scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState,
    scene: &mut Scene,
    tile_filter: Option<&WorldBox>,
    lod: BoardDrawLod,
    world_space: bool,
    only_ids: Option<&BTreeSet<String>>,
    overlay_ids: &BTreeSet<String>,
    layer: NodeHandlePaintLayer,
) {
    let pad = scene_state.drawable_cull_pad_world();
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| link_drag_compatible_target_node_ids(scene_state, style, s).into_iter().collect()).unwrap_or_default();
    for h in scene_state.handles.values() {
        if !draw_handles || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&h.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass_for_entity(&h.id, overlay_ids));
        append_handle_marker(scene_state, style, scene, h, wp, effective_handle_radius(scene_state, style, h), draw_handle_icons, style_kind, None, world_space, layer, true);
    }
    let paint_node = |scene: &mut Scene, n: &NodeData, chrome_pass: StyleChromePass| {
        paint_node_geometry(scene_state, focus, style, scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
    };
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        if overlay_ids.contains(&n.id) {
            continue;
        }
        paint_node(scene, n, StyleChromePass::CachedBase);
    }
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        if !overlay_ids.contains(&n.id) {
            continue;
        }
        paint_node(scene, n, StyleChromePass::InteractionOverlay);
    }
}

fn append_nodes_and_handles(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, chrome_pass: StyleChromePass, layer: NodeHandlePaintLayer) {
    let pad = scene_state.drawable_cull_pad_world();
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| link_drag_compatible_target_node_ids(scene_state, style, s).into_iter().collect()).unwrap_or_default();
    for h in scene_state.handles.values() {
        if !draw_handles || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&h.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass);
        append_handle_marker(scene_state, style, scene, h, wp, effective_handle_radius(scene_state, style, h), draw_handle_icons, style_kind, None, world_space, layer, true);
    }
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        paint_node_geometry(scene_state, focus, style, scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
    }
}

pub fn append_edges_wires_and_link(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, overlay_ids: Option<&BTreeSet<String>>) {
    let edge_sw = if world_space { scene_state.edge_world_stroke_width(lod) } else { scene_state.edge_screen_stroke_width_px(lod) };
    for e in scene_state.edges.values() {
        if !edge_effectively_visible(scene_state, style, e) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&e.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(eb) = edge_world_bounds_for_cull(scene_state, style, e) else { continue };
            if !world_boxes_overlap(*tb, eb) {
                continue;
            }
        }
        if let Some(c) = board_edge_curve(scene_state, style, e) {
            let p0 = scene_state.draw_space_point(c.p0(), world_space);
            let p1 = scene_state.draw_space_point(c.p1(), world_space);
            let p2 = scene_state.draw_space_point(c.p2(), world_space);
            let p3 = scene_state.draw_space_point(c.p3(), world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| chrome_pass_for_entity(&e.id, ids));
            let (stroke_color, edge_stroke, stroke_w) = resolve_edge_stroke_paint(focus, style, e, chrome_pass, lod, edge_sw);
            scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
            let (source_tip, target_tip) = style.resolve_edge_tips(e);
            append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
        }
    }
    let wire_sw = 2.25_f64;
    let wire_stroke = Stroke::new(wire_sw);
    for w in scene_state.wires.values() {
        if !scene_state.wire_effectively_visible(w) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&w.id) {
                continue;
            }
        }
        if let Some(c) = wire_curve(scene_state, style, w) {
            let p0 = scene_state.draw_space_point(c.p0(), world_space);
            let p1 = scene_state.draw_space_point(c.p1(), world_space);
            let p2 = scene_state.draw_space_point(c.p2(), world_space);
            let p3 = scene_state.draw_space_point(c.p3(), world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| chrome_pass_for_entity(&w.id, ids));
            let wc = wire_stroke_for_style(&style.canvas_theme, focus.resolve_wire_style_kind(w, chrome_pass));
            scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
        }
    }
    let link_wire_sw = 2.85_f64;
    let link_wire_stroke = Stroke::new(link_wire_sw);
    let link_wire_color = style.canvas_theme.node_stroke;
    if let Some(c) = active_link_wire_curve(scene_state, focus, style) {
        let p0 = scene_state.draw_space_point(c.p0(), world_space);
        let p1 = scene_state.draw_space_point(c.p1(), world_space);
        let p2 = scene_state.draw_space_point(c.p2(), world_space);
        let p3 = scene_state.draw_space_point(c.p3(), world_space);
        let curve = CubicBez::new(p0, p1, p2, p3);
        scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
    }
}

/// 🎨️ Bakes each visible node's icon (and its handle icons) as a contiguous span at the node's current center.
pub fn bake_node_icon_spans(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, lod: BoardDrawLod) -> Vec<NodeIconSpan> {
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|source| link_drag_compatible_target_node_ids(scene_state, style, source).into_iter().collect()).unwrap_or_default();
    let mut spans = Vec::new();
    for node in scene_state.nodes.values() {
        if !node.visible {
            continue;
        }
        let start = scene.command_len();
        if draw_handles {
            for handle in scene_state.handles.values() {
                if handle.node_id != node.id || !scene_state.handle_effectively_visible(handle.id.as_str()) {
                    continue;
                }
                let Some(world) = handle_world_pos(scene_state, style, handle) else { continue };
                let style_kind = resolve_handle_style_kind(focus, style, handle, StyleChromePass::CachedBase);
                append_handle_marker(scene_state, style, scene, handle, world, effective_handle_radius(scene_state, style, handle), draw_handle_icons, style_kind, None, true, NodeHandlePaintLayer::Icons, true);
            }
        }
        paint_node_geometry(scene_state, focus, style, scene, node, lod, true, NodeHandlePaintLayer::Icons, StyleChromePass::CachedBase, link_compat_nodes.contains(&node.id));
        let end = scene.command_len();
        if end > start {
            spans.push(NodeIconSpan { id: node.id.clone(), start, end, x: node.x, y: node.y });
        }
    }
    spans
}

pub fn set_hovered_id(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, id: Option<String>) {
    let event_kind = id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    if focus.hovered_id == id && focus.hovered_kind.is_none() {
        return;
    }
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::hover(id.as_deref(), event_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
        return;
    };
    focus.hovered_id = id;
    focus.hovered_kind = None;
    outbox.publish_event_reservation(reservation);
}

/// 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
pub fn set_hovered_kind(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, domain: Option<String>, kind_id: Option<String>) {
    let next_kind = domain.zip(kind_id);
    if focus.hovered_id.is_none() && focus.hovered_kind == next_kind {
        return;
    }
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::hover(None, next_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
        return;
    };
    focus.hovered_id = None;
    focus.hovered_kind = next_kind;
    outbox.publish_event_reservation(reservation);
}

pub fn wheel_screen(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, sx: f64, sy: f64, delta_y: f64) {
    let plan = scene.plan_wheel(sx, sy, delta_y);
    let _ = commit_wheel(scene, outbox, &plan);
}

pub fn commit_wheel(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, plan: &BoardWheelPlan) -> bool {
    if scene.interaction_revision != plan.revision || [scene.camera.x.to_bits(), scene.camera.y.to_bits(), scene.camera.zoom.to_bits()] != [plan.expected.x.to_bits(), plan.expected.y.to_bits(), plan.expected.zoom.to_bits()] {
        return false;
    }
    set_camera_silent(scene, outbox, plan.next.x, plan.next.y, plan.next.zoom);
    scene.interaction_revision = plan.revision.wrapping_add(1);
    true
}

pub fn try_commit_link_edge(scene: &mut BoardSceneState, style_state: &mut BoardStyleState, outbox: &mut BoardOutboxState, source_handle_id: &str, target_handle_id: &str, also_emit: Option<BoardEventKind>) -> bool {
    if source_handle_id == target_handle_id {
        return false;
    }
    if !scene.handle_selectable(source_handle_id) || !scene.handle_selectable(target_handle_id) {
        return false;
    }
    let Some(source_row) = scene.handles.get(source_handle_id) else {
        return false;
    };
    let Some(target_row) = scene.handles.get(target_handle_id) else {
        return false;
    };
    if source_row.node_id == target_row.node_id {
        return false;
    }
    if !handles_link_compatible_for_drag(scene, style_state, source_row, target_row) {
        return false;
    }
    if scene.handle_has_incident_edge(source_handle_id) || scene.handle_has_incident_edge(target_handle_id) {
        return false;
    }
    for e in scene.edges.values() {
        if e.source == source_handle_id && e.target == target_handle_id {
            return false;
        }
    }
    let mut n = scene.edges.len().saturating_add(1);
    let id = loop {
        let candidate = format!("edge-link-{n}");
        if !scene.edges.contains_key(&candidate) {
            break candidate;
        }
        n = n.saturating_add(1);
    };
    let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
    let Some(reservation) = outbox.reserve_owned_batch(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &id, source_handle_id, target_handle_id), also_emit.map(|kind| BoardOwnedEvent::edge(kind, &id, source_handle_id, target_handle_id)))
    else {
        return false;
    };
    scene.edges.insert(
        id.clone(),
        EdgeData {
            id,
            source: source_handle_id.to_string(),
            target: target_handle_id.to_string(),
            selected: false,
            visible: true,
            locked: false,
            style: None,
            edge_kind,
            source_tip: None,
            target_tip: None,
            properties: semio_framework_graph_manifest::PropertyBag::new(),
        },
    );
    outbox.publish_event_batch(reservation);
    true
}

pub fn load_board_snapshot_json(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState,source:&str,control:&mut NativeDecodeControl<'_>)->bool{match decode_board_snapshot_json(source,control){Ok(snapshot)=>load_board_snapshot(scene, focus, style, outbox, snapshot),Err(_)=>false}}
