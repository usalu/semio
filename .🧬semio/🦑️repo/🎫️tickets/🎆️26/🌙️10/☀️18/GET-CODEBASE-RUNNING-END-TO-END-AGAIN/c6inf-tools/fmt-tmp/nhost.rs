//! 🧭️ Normal board host: the composed `BoardHost` owner with pointer and render orchestration.
//!
//! Crate `semio-framework-os-infinite-board-normal-host`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, builtin_edge_tips, closed};
use semio_framework_os_infinite_board_engine::{ActiveUtility, Camera, CanvasPalette, distance, EdgeData, GestureStage, GraphPortMode, GRID_FACTOR_DEFAULT, IconPaintCache, Interaction, merge_pick_into_selection, pick_merge_mode_for_modifiers, rotate_point_about, SceneDescriptor, SELECTION_CLICK_MAX_DISTANCE_PX, selection_drag_enclosing, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SelectionOptions, transform_pivot_of, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, transform_ring_radius_world, TransformGumballFlags};
use semio_framework_os_infinite_board_normal_model::{BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_KEY_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardEventKind, BoardEventQueue, BoardOwnedEvent, BoardPayloadBuilder, BoardPointerSpan, BoardTransformDrag, brush_preview_snapshot_from_session_json, BrushCandidatePage, DEFAULT_BRUSH_NODE_SIZE, DEFAULT_SUGGESTION_OFFSET, drag_gesture_record, gesture_release_events, LINK_DRAG_MIN_DISTANCE_PX, NodeHandlePaintLayer, NormalPortError, REGION_DEFAULT_BRUSH_EXTENT_WORLD, StyleChromePass, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{board_draw_lod_for_frame, board_json_string_bytes, BoardDeleteKind, BoardDeletePlan, BoardDeletePlanningOperation, BoardDeletePlanningPhase, BoardPointerPlan, BoardPointerPlanFault, BoardPointerPlanKind, BoardSceneState, BoardStyleState, brush_compatible_candidates, drag_delta, handle_eligible_link_target_ring, link_compatible_handle_count_on_node, link_compatible_handle_ids_on_node, link_drag_compatible_target_node_ids, link_drag_ring_target_node_id, link_snap_commit_proximity_ok, nearest_link_snap_handle_world, node_drag_proximity_handle_pair, node_has_any_free_link_compatible_handle, node_sole_free_link_compatible_handle, planned_link_edge_id, resolve_node_hit_world};
use semio_framework_os_infinite_board_normal_focus::{active_link_wire_curve, append_edges_wires_and_link, append_indirect_handle_ring, append_nodes_and_handles_with_overlay_chrome, bake_node_icon_spans, board_apply_area_preselect, board_set_camera, BoardFocusState, BoardOutboxState, BoardPointerIntent, BoardPointerPhase, cancel_node_drag, clear_selection_on_background_click, commit_area_select_from_initial, indirect_ring_node_id, interaction_overlay_entity_ids, resolve_area_selection_with_initial, resolve_hit_world, resolve_hover_world, resolve_overview_free_link_handle_pointer_world, set_hovered_id, set_selection_ids_gestured, stage_selection_ids, sync_descriptor_with, sync_link_gesture_events, sync_selection_flags_to_objects, transform_selection_node_ids, try_begin_bounded_selection_drag_at, try_commit_link_edge};
use semio_framework_os_infinite_board_normal_gestures::{append_brush_preview_paint, append_drop_preview_paint, append_target_region_paint, begin_region_paint, BoardAuthorityStep, BoardBrushState, BoardClosingState, BoardCommitState, BoardEntityRetirement, BoardGesturesState, BoardHostClosePhase, BoardPointerCommitOperation, BoardPointerPlanRetirement, BoardPointerPublication, BoardRemovedEntity, brush_build_preview_with_offset, brush_clear_slot, brush_commit_preview, brush_enter_slot, brush_finish_slot, brush_nearest_slot_source, brush_pointer_move, brush_rebuild_preview, brush_sync_preview_events, brush_update_alt, cancel_region_drag, cancel_region_paint, cancel_transform_drag, commit_region_drag, commit_region_paint, commit_transform_drag, event_authority_terminal_is_empty, pointer_lane_is_direct, quarantine_world_content_step, resolve_pick_targets_world, step_delete_planning, try_begin_region_drag_at, update_region_drag, update_region_paint, update_transform_drag};
use semio_framework_os_infinite_board_normal_fill::NEUTRAL_BOARD_NAMING;
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::cell::{Cell, RefCell};
use std::mem::ManuallyDrop;
use semio_framework_graph_manifest::PropertyBag;
use geometry::distance_between;

    // #region board_host
    //#region 🧵️FillJob
impl BoardHost {
    /// 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
    pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.trim().is_empty() {
            self.brush.brush_slot_suggestions_active = false;
            self.brush.brush_slot_source_id = None;
            self.brush.brush_candidates.clear();
            self.brush.brush_candidate_index = 0;
            self.brush.brush_preview = None;
            self.brush.brush_preview_emit_key = None;
            self.brush.brush_candidates_emit_key = None;
            self.scene.bump_content_scene_generation();
            return Ok(());
        }
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::BrushSessionJson)?;
        let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        self.brush.brush_slot_source_id = source.clone();
        let mut candidates = BrushCandidatePage::default();
        if let Some(rows) = v.get("candidates").and_then(|x| x.as_array()) {
            if rows.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            for row in rows {
                let (node_kind_id, target_handle_index) = if let Some(node_kind_id) = row.as_str().map(str::trim).filter(|value| !value.is_empty()) {
                    (node_kind_id, 0)
                } else {
                    let node_kind_id = row.get("nodeKind").or_else(|| row.get("nodeKindId")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty()).ok_or(NormalPortError::EventCredits)?;
                    let target_handle_index = row.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0);
                    let target_handle_index = usize::try_from(target_handle_index).map_err(|_| NormalPortError::EventCredits)?;
                    (node_kind_id, target_handle_index)
                };
                candidates.push(node_kind_id, target_handle_index, 0.0).map_err(|_| NormalPortError::EventCredits)?;
            }
        }
        self.brush.brush_candidates = candidates;
        self.brush.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map_or(0, |i| i as usize);
        if self.brush.brush_candidates.is_empty() {
            self.brush.brush_candidate_index = 0;
        } else {
            self.brush.brush_candidate_index %= self.brush.brush_candidates.len();
        }
        self.brush.brush_preview = match (source.as_deref(), v.get("preview")) {
            (Some(source_id), Some(preview)) if !preview.is_null() => {
                let node = preview.get("node").filter(|n| !n.is_null());
                let edge = preview.get("edge").filter(|e| !e.is_null());
                match (node, edge) {
                    (Some(node), Some(edge)) => brush_preview_snapshot_from_session_json(node, edge, source_id),
                    _ => None,
                }
            }
            _ => None,
        };
        self.brush.brush_preview_emit_key = None;
        self.brush.brush_candidates_emit_key = None;
        self.brush.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
        if self.brush.brush_preview.is_none() && !self.brush.brush_candidates.is_empty() {
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        } else {
            brush_sync_preview_events(&mut self.brush, &mut self.outbox);
        }
        self.scene.bump_content_scene_generation();
        Ok(())
    }

    pub fn set_active_utility(&mut self, label: &str) {
        let next = match label {
            "brush" => ActiveUtility::Brush,
            "areaBrush" => ActiveUtility::AreaBrush,
            _ => ActiveUtility::Select,
        };
        if self.focus.active_utility == next {
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
        }
        cancel_region_paint(&mut self.scene, &mut self.gestures);
        cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures);
        self.focus.active_utility = next;
        self.focus.interaction = Interaction::None;
        self.scene.bump_content_scene_generation();
    }

    pub fn set_suggestion_offset(&mut self, distance: f64) {
        let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_SUGGESTION_OFFSET };
        if (self.brush.suggestion_offset - d).abs() < 1e-9 {
            return;
        }
        self.brush.suggestion_offset = d;
        if self.focus.active_utility == ActiveUtility::Brush {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn set_brush_kind_weights(&mut self, json: &str) {
        if json.is_empty() {
            return;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        self.brush.brush_node_kind_weights.clear();
        self.brush.brush_handle_kind_weights.clear();
        if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush.brush_node_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush.brush_handle_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if self.focus.active_utility != ActiveUtility::Brush {
            return;
        }
        if let Some(source) = self.brush.brush_slot_source_id.clone() {
            self.brush.brush_slot_source_id = None;
            brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, &source);
        } else {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn set_brush_node_size(&mut self, size: f64) {
        let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
        if (self.brush.brush_node_size - s).abs() < 1e-9 {
            return;
        }
        self.brush.brush_node_size = s;
        if self.focus.active_utility == ActiveUtility::Brush {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn brush_cycle_candidate(&mut self, forward: bool) {
        if self.brush.brush_candidates.len() < 2 {
            return;
        }
        let len = self.brush.brush_candidates.len();
        self.brush.brush_candidate_index = if forward { (self.brush.brush_candidate_index + 1) % len } else { (self.brush.brush_candidate_index + len - 1) % len };
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
    }

    pub fn brush_set_candidate_index(&mut self, index: usize) {
        if self.brush.brush_candidates.is_empty() {
            return;
        }
        self.brush.brush_candidate_index = index % self.brush.brush_candidates.len();
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
    }

    /// 🖌️ Opens a brush slot on a free handle (suggestions menu; works outside brush utility).
    pub fn brush_open_slot(&mut self, handle_id: &str) {
        if !self.scene.handles.contains_key(handle_id) {
            return;
        }
        brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, handle_id);
        self.brush.brush_slot_suggestions_active = true;
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(handle_id.to_string()));
    }

    /// 🎣️ Points the brush slot at a handle, or at nothing, WITHOUT opening the suggestions
    /// popup — the armed brush's own targeting channel, so a program driving the brush headlessly
    /// resolves the same candidate page a pointer hover would without claiming the popup's hover.
    pub fn brush_target_slot(&mut self, handle_id: Option<&str>) {
        match handle_id.filter(|handle_id| self.scene.handles.contains_key(*handle_id)) {
            Some(handle_id) => {
                brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, handle_id);
                brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
            }
            None => brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox),
        }
    }

    /// 🖌️ Commits the active brush preview and clears the slot.
    pub fn brush_commit_slot(&mut self) {
        brush_commit_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox);
    }

    /// 🖌️ Discards the active brush slot without placing.
    pub fn brush_cancel_slot(&mut self) {
        brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox);
    }

    pub fn step_event_authority(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        if context.should_yield() {
            return BoardAuthorityStep::Pending;
        }
        let faulted_before = self.outbox.event_schema_fault;
        if self.commit.pending_delete_planning.is_some() {
            if context.is_cancelled() {
                if let Some(planning) = self.commit.pending_delete_planning.as_mut() {
                    planning.cancelling = true;
                }
            }
            let result = step_delete_planning(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            return result;
        }
        if self.commit.pending_delete_operation.is_some() {
            if context.is_cancelled() {
                if let Some(operation) = self.commit.pending_delete_operation.as_mut() {
                    if !operation.claimed {
                        operation.cancelling = true;
                    }
                }
            }
            if self.commit.pending_delete_operation.as_ref().is_some_and(|operation| operation.cancelling && !operation.claimed) {
                self.commit.pending_delete_operation = None;
                context.consume_fuel(1);
                return BoardAuthorityStep::Cancelled;
            }
            step_pending_delete_operation(&mut self.scene, &mut self.focus, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            if self.outbox.event_schema_fault && !faulted_before {
                return BoardAuthorityStep::Fault;
            }
            return if self.commit.pending_delete_operation.is_some() { BoardAuthorityStep::Pending } else { BoardAuthorityStep::Complete };
        }
        BoardAuthorityStep::Complete
    }

    pub fn close_event_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if let Some(planning) = self.commit.pending_delete_planning.as_mut() {
            planning.cancelling = true;
        }
        if self.commit.pending_delete_planning.is_some() {
            let _ = step_delete_planning(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            return false;
        }
        if let Some(operation) = self.commit.pending_delete_operation.as_mut() {
            if !operation.claimed {
                operation.cancelling = true;
            }
        }
        if self.commit.pending_delete_operation.is_some() {
            let _ = self.step_event_authority(context);
            return false;
        }
        if self.outbox.event_overflow.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(batch) = self.outbox.event_batch_overflow.as_mut() {
            let _ = batch.pop();
            if batch.is_empty() {
                self.outbox.event_batch_overflow = None;
            }
            context.consume_fuel(1);
            return false;
        }
        let closed = self.outbox.events.close_step();
        context.consume_fuel(1);
        closed
    }

    pub(crate) fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
        if let Some(tid) = target_handle_id {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(tid.to_string()));
        } else {
            self.update_hover_from_world(world);
        }
    }

    /// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let world = self.scene.screen_to_world(Point::new(sx, sy));
        serde_json::to_string(&resolve_pick_targets_world(&self.scene, &self.focus, &self.style, &self.gestures, world)).unwrap_or_else(|_| "[]".into())
    }
}

pub(crate) fn step_pointer_commit_operation(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation) -> bool {
    match operation.plan.kind {
        BoardPointerPlanKind::Idle => true,
        BoardPointerPlanKind::Pan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
            true
        }
        BoardPointerPlanKind::FinishPan { camera } => match operation.phase {
            0 => {
                scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
                operation.phase = 1;
                false
            }
            _ => {
                focus.interaction = Interaction::None;
                true
            }
        },
        BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("pointer commit delta");
                if let Some(node) = scene.nodes.get_mut(operation.plan.id(delta.id)) {
                    operation.changed |= (node.x - delta.x).abs() > 1e-9 || (node.y - delta.y).abs() > 1e-9;
                    node.x = delta.x;
                    node.y = delta.y;
                }
                operation.cursor += 1;
                return false;
            }
            if operation.phase == 0 {
                if operation.changed {
                    scene.bump_content_scene_generation();
                }
                operation.phase = 1;
                return false;
            }
            if matches!(operation.plan.kind, BoardPointerPlanKind::FinishDrag) {
                focus.interaction = Interaction::None;
            }
            true
        }
        BoardPointerPlanKind::SelectionPreview { start, start_screen } => step_selection_preview_commit(scene, focus, outbox, operation, start, start_screen),
        BoardPointerPlanKind::SelectionCommit => step_selection_commit(scene, focus, outbox, operation),
        BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world, activated, start_screen } => match operation.phase {
            0 => {
                if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                outbox.link_compat_nodes_emit_key = Some(operation.plan.id(compat_key).to_owned());
                operation.phase = 2;
                false
            }
            2 => {
                outbox.link_target_ring_emit_key = Some(operation.plan.id(ring_key).to_owned());
                operation.phase = 3;
                false
            }
            3 => {
                focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                focus.hovered_kind = None;
                operation.phase = 4;
                false
            }
            4 => {
                let source_id = operation.plan.id(source).to_owned();
                let target_id = target.map(|span| operation.plan.id(span).to_owned());
                focus.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                operation.phase = 5;
                false
            }
            _ => {
                scene.bump_content_scene_generation();
                true
            }
        },
        BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key } => match operation.phase {
            0 => {
                if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                if let (Some(target), Some(edge)) = (target, edge) {
                    let source_id = operation.plan.id(source).to_owned();
                    let target_id = operation.plan.id(target).to_owned();
                    let edge_id = operation.plan.id(edge).to_owned();
                    let Some(source_row) = scene.handles.get(&source_id) else {
                        operation.faulted = true;
                        return true;
                    };
                    let Some(target_row) = scene.handles.get(&target_id) else {
                        operation.faulted = true;
                        return true;
                    };
                    let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
                    scene.edges.insert(
                        edge_id.clone(),
                        EdgeData { id: edge_id, source: source_id, target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                    );
                }
                operation.phase = 2;
                false
            }
            2 => {
                outbox.link_compat_nodes_emit_key = compat_key.map(|span| operation.plan.id(span).to_owned());
                operation.phase = 3;
                false
            }
            3 => {
                outbox.link_target_ring_emit_key = ring_key.map(|span| operation.plan.id(span).to_owned());
                operation.phase = 4;
                false
            }
            4 => {
                focus.interaction =
                    if let Some(target_node) = target_node { Interaction::LinkTargetNode { source_id: operation.plan.id(source).to_owned(), target_node_id: operation.plan.id(target_node).to_owned() } } else { Interaction::None };
                operation.phase = 5;
                false
            }
            5 => {
                focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                focus.hovered_kind = None;
                operation.phase = 6;
                false
            }
            _ => {
                scene.bump_content_scene_generation();
                true
            }
        },
        BoardPointerPlanKind::LinkRetain { hover } | BoardPointerPlanKind::Hover { hover } => {
            focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
            focus.hovered_kind = None;
            true
        }
        BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => step_brush_pointer_commit(scene, focus, style_state, brush, operation, source, hover, alt, commit_old),
    }
}

fn step_selection_preview_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation, start: Point, start_screen: Point) -> bool {
    match operation.phase {
        0 => {
            if !matches!(focus.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                operation.faulted = true;
                return true;
            }
            operation.phase = 1;
            false
        }
        1 => {
            if let Some(id) = focus.preselect.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if focus.preselect_removed.pop_first().is_some() {
                return false;
            }
            operation.phase = 3;
            false
        }
        3 => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection preview id");
                let id = operation.plan.id(delta.id).to_owned();
                focus.preselect.insert(id.clone());
                scene.set_pointer_selection_flag(&id, true);
                operation.cursor += 1;
                return false;
            }
            operation.cursor = 0;
            operation.phase = 4;
            false
        }
        4 => {
            let next = match &focus.interaction {
                Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => match operation.scan_after.as_ref() {
                    Some(after) => initial_ids.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next().cloned(),
                    None => initial_ids.first().cloned(),
                },
                _ => None,
            };
            if let Some(id) = next {
                operation.scan_after = Some(id.clone());
                if !focus.preselect.contains(&id) {
                    focus.preselect_removed.insert(id);
                }
                return false;
            }
            operation.scan_after = None;
            operation.phase = 5;
            false
        }
        5 => {
            if operation.cursor < operation.plan.point_len {
                let index = usize::from(operation.cursor);
                operation.points.push(operation.plan.points[index]);
                operation.screen_points.push(operation.plan.screen_points[index]);
                operation.cursor += 1;
                return false;
            }
            operation.cursor = 0;
            operation.phase = 6;
            false
        }
        6 => {
            if operation.screen_points.len() < 2 {
                operation.phase = 7;
                return false;
            }
            if focus.selection_options.method == "lasso" {
                if usize::from(operation.cursor) < operation.screen_points.len() {
                    operation.overlay_points.push(operation.screen_points[usize::from(operation.cursor)]);
                    operation.cursor += 1;
                    return false;
                }
            } else if operation.cursor < 4 {
                let last = *operation.screen_points.last().unwrap_or(&start_screen);
                let point = match operation.cursor {
                    0 => start_screen,
                    1 => Point::new(last.x, start_screen.y),
                    2 => last,
                    _ => Point::new(start_screen.x, last.y),
                };
                operation.overlay_points.push(point);
                operation.cursor += 1;
                return false;
            }
            operation.phase = 7;
            false
        }
        7 => {
            let previous = std::mem::replace(&mut focus.interaction, Interaction::None);
            let initial_ids = match previous {
                Interaction::SelectionPending { initial_ids, .. } => initial_ids,
                Interaction::Selection { initial_ids, points, screen_points, .. } => {
                    operation.retiring_points = points;
                    operation.retiring_screen_points = screen_points;
                    initial_ids
                }
                _ => {
                    operation.faulted = true;
                    return true;
                }
            };
            focus.selection_preview_crossing = !selection_drag_enclosing(focus.selection_options.method.as_str(), start_screen, &operation.screen_points);
            operation.retiring_overlay_points = focus.selection_screen_preview.take().unwrap_or_default();
            focus.selection_screen_preview = (!operation.overlay_points.is_empty()).then(|| std::mem::take(&mut operation.overlay_points));
            focus.interaction = Interaction::Selection { initial_ids, points: std::mem::take(&mut operation.points), screen_points: std::mem::take(&mut operation.screen_points), start, start_screen };
            if let Some((ids, removed, gesture)) = outbox.last_preselect_emit_sig.take() {
                operation.retiring_signature_b = ids;
                operation.retiring_signature_c = removed;
                operation.retiring_gestures[1] = gesture;
            }
            operation.phase = 8;
            false
        }
        8 => {
            if operation.retire_one() {
                return false;
            }
            true
        }
        _ => true,
    }
}

fn step_selection_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation) -> bool {
    match operation.phase {
        0 => {
            let previous = std::mem::replace(&mut focus.interaction, Interaction::None);
            match previous {
                Interaction::SelectionPending { initial_ids, .. } => operation.retiring_ids = initial_ids,
                Interaction::Selection { initial_ids, points, screen_points, .. } => {
                    operation.retiring_ids = initial_ids;
                    operation.retiring_points = points;
                    operation.retiring_screen_points = screen_points;
                }
                other => {
                    focus.interaction = other;
                    operation.faulted = true;
                    return true;
                }
            }
            operation.phase = 1;
            false
        }
        1 => {
            if let Some(id) = focus.selection.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if let Some(id) = focus.preselect.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 3;
            false
        }
        3 => {
            if focus.preselect_removed.pop_first().is_some() {
                return false;
            }
            operation.phase = 4;
            false
        }
        4 => {
            if focus.selection_exit_highlight.pop_first().is_some() {
                return false;
            }
            operation.phase = 5;
            false
        }
        5 => {
            if operation.retire_one() {
                return false;
            }
            operation.phase = 6;
            false
        }
        6 => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection commit id");
                let id = operation.plan.id(delta.id).to_owned();
                focus.selection.insert(id.clone());
                scene.set_pointer_selection_flag(&id, true);
                operation.cursor += 1;
                return false;
            }
            operation.phase = 7;
            false
        }
        7 => {
            if let Some((ids, gesture)) = outbox.last_select_emit_sig.take() {
                operation.retiring_signature_a = ids;
                operation.retiring_gestures[0] = gesture;
            }
            if let Some((ids, removed, gesture)) = outbox.last_preselect_emit_sig.take() {
                operation.retiring_signature_b = ids;
                operation.retiring_signature_c = removed;
                operation.retiring_gestures[1] = gesture;
            }
            operation.retiring_overlay_points = focus.selection_screen_preview.take().unwrap_or_default();
            focus.selection_preview_crossing = false;
            operation.phase = 8;
            false
        }
        8 => {
            if operation.retire_one() {
                return false;
            }
            operation.phase = 9;
            false
        }
        _ => {
            scene.bump_content_scene_generation();
            true
        }
    }
}

fn step_brush_pointer_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, brush: &mut BoardBrushState, operation: &mut BoardPointerCommitOperation, source: Option<BoardPointerSpan>, hover: Option<BoardPointerSpan>, alt: bool, commit_old: bool) -> bool {
    match operation.phase {
        0 => {
            if focus.active_utility != ActiveUtility::Brush || matches!(focus.interaction, Interaction::Pan { .. }) {
                operation.faulted = true;
                return true;
            }
            operation.phase = 1;
            false
        }
        1 => {
            if operation.cursor < operation.plan.delta_len {
                let candidate = operation.plan.deltas[usize::from(operation.cursor)].expect("brush candidate");
                if operation.brush_candidates.push(operation.plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                    operation.faulted = true;
                    return true;
                }
                operation.cursor += 1;
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if commit_old {
                brush.brush_placement_serial = brush.brush_placement_serial.wrapping_add(1);
            }
            operation.phase = 3;
            false
        }
        3 => {
            brush.brush_alt_pressed = alt;
            brush.brush_slot_suggestions_active = false;
            operation.phase = 4;
            false
        }
        4 => {
            brush.brush_slot_source_id = source.map(|span| operation.plan.id(span).to_owned());
            operation.phase = 5;
            false
        }
        5 => {
            brush.brush_candidates = std::mem::take(&mut operation.brush_candidates);
            brush.brush_candidate_index = 0;
            operation.phase = 6;
            false
        }
        6 => {
            let offset = if alt { brush.suggestion_offset } else { 0.0 };
            brush.brush_preview = brush.brush_slot_source_id.as_deref().and_then(|source_id| brush.brush_candidates.first().and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
            operation.phase = 7;
            false
        }
        7 => {
            brush.brush_preview_emit_key = None;
            operation.phase = 8;
            false
        }
        8 => {
            brush.brush_candidates_emit_key = None;
            operation.phase = 9;
            false
        }
        9 => {
            focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
            focus.hovered_kind = None;
            operation.phase = 10;
            false
        }
        10 => {
            focus.interaction = Interaction::None;
            operation.phase = 11;
            false
        }
        _ => {
            scene.bump_content_scene_generation();
            true
        }
    }
}

// #region board_host
impl Default for BoardHost {
    fn default() -> Self {
        Self {
            scene: BoardSceneState {
                camera: Camera::default(),
                nodes: BTreeMap::new(),
                handles: BTreeMap::new(),
                edges: BTreeMap::new(),
                wires: BTreeMap::new(),
                regions: BTreeMap::new(),
                width: 1,
                height: 1,
                dpr: 1.0,
                content_scene_generation: 0,
                world_content_cache: RefCell::new(None),
                interaction_revision: 0,
            },
            focus: BoardFocusState {
                selection: BTreeSet::new(),
                preselect: BTreeSet::new(),
                preselect_removed: BTreeSet::new(),
                selection_exit_highlight: BTreeSet::new(),
                selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
                hovered_id: None,
                hovered_kind: None,
                highlighted_ids: BTreeSet::new(),
                interaction: Interaction::None,
                selection_screen_preview: None,
                selection_preview_crossing: false,
                active_utility: ActiveUtility::Select,
            },
            style: BoardStyleState {
                handle_kinds: BTreeMap::new(),
                wire_kinds: BTreeMap::new(),
                node_kinds: BTreeMap::new(),
                edge_kinds: BTreeMap::new(),
                edge_tips: builtin_edge_tips(),
                link_compat_rules: Vec::new(),
                world_raster_tiling: "world-clip".into(),
                canvas_theme: CanvasPalette::default(),
                grid_visible: true,
                grid_factor: GRID_FACTOR_DEFAULT,
                grid_snap_enabled: false,
                preserve_original_element_style: false,
                automatic_lod: true,
                forced_draw_lod: None,
                icon_paint_cache: IconPaintCache::new(),
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                naming: &NEUTRAL_BOARD_NAMING,
                placement_naming: None,
                port_mode: GraphPortMode::Ported,
            },
            brush: BoardBrushState {
                suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
                brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
                brush_slot_source_id: None,
                brush_candidates: BrushCandidatePage::default(),
                brush_candidate_index: 0,
                brush_preview: None,
                drop_preview: None,
                brush_candidates_emit_key: None,
                brush_preview_emit_key: None,
                brush_placement_serial: 0,
                brush_node_kind_weights: HashMap::new(),
                brush_handle_kind_weights: HashMap::new(),
                brush_alt_pressed: false,
                brush_slot_suggestions_active: false,
            },
            outbox: BoardOutboxState {
                events: BoardEventQueue::default(),
                event_overflow: None,
                event_batch_overflow: None,
                event_schema_fault: false,
                link_compat_nodes_emit_key: None,
                link_target_ring_emit_key: None,
                last_select_emit_sig: None,
                last_preselect_emit_sig: None,
                gesture_serial: 0,
            },
            gestures: BoardGesturesState {
                link_screen_preview: None,
                transform_flags: TransformGumballFlags::default(),
                transform_drag: None,
                region_drag: None,
                region_paint: None,
                area_brush_extent: (REGION_DEFAULT_BRUSH_EXTENT_WORLD, REGION_DEFAULT_BRUSH_EXTENT_WORLD),
            },
            commit: BoardCommitState {
                pending_delete_planning: None,
                pending_delete_operation: None,
                pending_pointer_commit: None,
                queued_pointer_commit: None,
                cancelled_pointer_plan_retirement: None,
                pointer_publication: None,
            },
            closing: BoardClosingState {
                opaque_scene_retirement: Cell::new(None),
                opaque_scene_fault: Cell::new(false),
                close_phase: BoardHostClosePhase::Events,
                close_entity_retirement: None,
                close_strings: std::array::from_fn(|_| None),
                close_string_len: 0,
                close_node_handles: None,
            },
        }
    }
}

impl BoardHost {
    pub fn close_nonopaque_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if self.closing.close_scratch_step() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.closing.close_entity_retirement.as_mut() {
            match retirement.step() {
                Ok(true) => self.closing.close_entity_retirement = None,
                Ok(false) => {}
                Err(()) => {
                    self.outbox.event_schema_fault = true;
                    return false;
                }
            }
            context.consume_fuel(1);
            return false;
        }
        match self.closing.close_phase {
            BoardHostClosePhase::Events => {
                if self.close_event_authority_step(context) {
                    self.closing.close_phase = BoardHostClosePhase::Pointer;
                }
                return false;
            }
            BoardHostClosePhase::Pointer => {
                if self.close_pointer_authority_step(context) {
                    self.closing.close_phase = BoardHostClosePhase::WorldScene;
                }
                return false;
            }
            BoardHostClosePhase::WorldScene => {
                if quarantine_world_content_step(&mut self.scene, &mut self.closing) {
                    self.closing.close_phase = BoardHostClosePhase::Icons;
                }
            }
            BoardHostClosePhase::Icons => {
                if self.style.icon_paint_cache.close_step() {
                    self.closing.close_phase = BoardHostClosePhase::Nodes;
                }
            }
            BoardHostClosePhase::Nodes => {
                if let Some((key, value)) = self.scene.nodes.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Node(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Handles;
                }
            }
            BoardHostClosePhase::Handles => {
                if let Some((key, value)) = self.scene.handles.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Handle(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Edges;
                }
            }
            BoardHostClosePhase::Edges => {
                if let Some((key, value)) = self.scene.edges.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Edge(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Wires;
                }
            }
            BoardHostClosePhase::Wires => {
                if let Some((key, value)) = self.scene.wires.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Wire(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Regions;
                }
            }
            BoardHostClosePhase::Regions => {
                if let Some((key, value)) = self.scene.regions.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.id);
                    self.closing.push_close_optional_string(value.label);
                } else if let Some(id) = self.gestures.region_drag.as_mut().and_then(|drag| drag.gesture.restore.as_mut()).and_then(BTreeSet::pop_first) {
                    self.closing.push_close_string(id);
                } else if let Some(drag) = self.gestures.region_drag.take() {
                    self.closing.push_close_string(drag.id);
                    self.closing.push_close_string(drag.gesture.id);
                } else {
                    self.gestures.region_paint = None;
                    self.closing.close_phase = BoardHostClosePhase::Selection;
                }
            }
            BoardHostClosePhase::Selection => {
                if self.focus.selection.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Preselect;
                }
            }
            BoardHostClosePhase::Preselect => {
                if self.focus.preselect.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::PreselectRemoved;
                }
            }
            BoardHostClosePhase::PreselectRemoved => {
                if self.focus.preselect_removed.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::SelectionExit;
                }
            }
            BoardHostClosePhase::SelectionExit => {
                if self.focus.selection_exit_highlight.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Highlighted;
                }
            }
            BoardHostClosePhase::Highlighted => {
                if self.focus.highlighted_ids.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Interaction;
                }
            }
            BoardHostClosePhase::Interaction => {
                if self.focus.close_interaction_step() {
                    self.closing.close_phase = BoardHostClosePhase::HandleKinds;
                }
            }
            BoardHostClosePhase::HandleKinds => {
                if let Some((key, value)) = self.style.handle_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.default_wire_kind);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::WireKinds;
                }
            }
            BoardHostClosePhase::WireKinds => {
                if let Some((key, value)) = self.style.wire_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.default_edge_kind);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::NodeKinds;
                }
            }
            BoardHostClosePhase::NodeKinds => {
                if let Some((key, value)) = self.style.node_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.icon);
                    self.closing.close_node_handles = Some(value.handles);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::EdgeKinds;
                }
            }
            BoardHostClosePhase::EdgeKinds => {
                if let Some((key, value)) = self.style.edge_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.source_tip);
                    self.closing.push_close_optional_string(value.target_tip);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::EdgeTips;
                }
            }
            BoardHostClosePhase::EdgeTips => {
                if self.style.edge_tips.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::LinkRules;
                }
            }
            BoardHostClosePhase::LinkRules => {
                if let Some(rule) = self.style.link_compat_rules.pop() {
                    self.closing.push_close_string(rule.source);
                    self.closing.push_close_string(rule.target);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Previews;
                }
            }
            BoardHostClosePhase::Previews => {
                if self.focus.selection_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) || self.gestures.link_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) {
                } else if let Some(preview) = self.brush.brush_preview.take() {
                    self.closing.push_close_string(preview.source_handle_id);
                    self.closing.push_close_string(preview.node_kind_id);
                    self.closing.push_close_optional_string(preview.icon_kind);
                    self.closing.close_node_handles = Some(preview.handles);
                } else if let Some(preview) = self.brush.drop_preview.take() {
                    self.closing.push_close_string(preview.node_kind_id);
                    self.closing.push_close_optional_string(preview.icon_kind);
                } else if self.gestures
                    .transform_drag
                    .as_mut()
                    .is_some_and(|drag| drag.start_positions.pop_first().is_some() || drag.start_handle_angles.pop_first().is_some())
                {
                } else if self.gestures.transform_drag.take().is_some() {
                } else {
                    self.focus.selection_screen_preview = None;
                    self.gestures.link_screen_preview = None;
                    self.closing.close_phase = BoardHostClosePhase::Strings;
                }
            }
            BoardHostClosePhase::Strings => {
                if let Some(value) = self.focus
                    .hovered_id
                    .take()
                    .or_else(|| self.outbox.link_compat_nodes_emit_key.take())
                    .or_else(|| self.outbox.link_target_ring_emit_key.take())
                    .or_else(|| self.brush.brush_slot_source_id.take())
                    .or_else(|| self.brush.brush_candidates_emit_key.take())
                    .or_else(|| self.brush.brush_preview_emit_key.take())
                {
                    drop(value);
                } else if let Some((left, right)) = self.focus.hovered_kind.take() {
                    self.closing.push_close_string(left);
                    self.closing.push_close_string(right);
                } else if let Some(naming) = self.style.placement_naming.take() {
                    self.closing.push_close_string(naming.brush_node_prefix);
                    self.closing.push_close_string(naming.brush_edge_prefix);
                } else if let Some((values, mode)) = self.outbox.last_select_emit_sig.as_mut() {
                    if let Some(value) = values.pop().or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.outbox.last_select_emit_sig = None;
                    }
                } else if let Some((left, right, mode)) = self.outbox.last_preselect_emit_sig.as_mut() {
                    if let Some(value) = left.pop().or_else(|| right.pop()).or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.outbox.last_preselect_emit_sig = None;
                    }
                } else {
                    drop(std::mem::take(&mut self.style.world_raster_tiling));
                    let method = std::mem::take(&mut self.focus.selection_options.method);
                    let mode = std::mem::take(&mut self.focus.selection_options.mode);
                    self.closing.push_close_string(method);
                    self.closing.push_close_string(mode);
                    self.closing.close_phase = BoardHostClosePhase::Weights;
                }
            }
            BoardHostClosePhase::Weights => {
                let released = self.brush.brush_node_kind_weights.keys().next().cloned().and_then(|key| self.brush.brush_node_kind_weights.remove_entry(&key)).is_some()
                    || self.brush.brush_handle_kind_weights.keys().next().cloned().and_then(|key| self.brush.brush_handle_kind_weights.remove_entry(&key)).is_some();
                if !released {
                    self.closing.close_phase = BoardHostClosePhase::Done;
                }
            }
            BoardHostClosePhase::Done => return self.nonopaque_terminal_is_empty(),
        }
        context.consume_fuel(1);
        false
    }

    pub fn nonopaque_terminal_is_empty(&self) -> bool {
        self.closing.close_phase == BoardHostClosePhase::Done
            && event_authority_terminal_is_empty(&self.outbox, &self.commit)
            && self.commit.pointer_authority_terminal_is_empty()
            && self.closing.close_entity_retirement.is_none()
            && self.closing.close_string_len == 0
            && self.closing.close_node_handles.is_none()
            && self.style.placement_naming.is_none()
            && self.scene.nodes.is_empty()
            && self.scene.handles.is_empty()
            && self.scene.edges.is_empty()
            && self.scene.wires.is_empty()
            && self.scene.regions.is_empty()
            && self.gestures.region_drag.is_none()
            && self.gestures.region_paint.is_none()
            && self.style.handle_kinds.is_empty()
            && self.style.wire_kinds.is_empty()
            && self.style.node_kinds.is_empty()
            && self.style.edge_kinds.is_empty()
            && self.style.edge_tips.is_empty()
            && self.style.link_compat_rules.is_empty()
            && self.focus.selection.is_empty()
            && self.focus.preselect.is_empty()
            && self.focus.preselect_removed.is_empty()
            && self.focus.selection_exit_highlight.is_empty()
            && self.focus.highlighted_ids.is_empty()
            && matches!(self.focus.interaction, Interaction::None)
            && self.gestures.transform_drag.is_none()
            && self.style.icon_paint_cache.terminal_is_empty()
            && self.scene.world_content_cache.borrow().is_none()
            && self.closing.opaque_scene_retirement.get().is_none()
    }
}

impl BoardHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧠️ Normal directed graph host: no handles, edges reference node ids.
    pub fn new_normal() -> Self {
        let mut host = Self::default();
        host.style.port_mode = GraphPortMode::Normal;
        host.focus.selection_options.select_handles = false;
        host
    }

    /// 🔗️ Applies or clears a host-driven link preview session (cross-surface mirror).
    pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::ExternalLinkPreviewJson)?;
        let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
        if source.is_empty() {
            if matches!(self.focus.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
            }
            return Ok(());
        }
        let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
        let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
        let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.focus.interaction = Interaction::ExternalLinkPreview { source_id: source, end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
        sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
        Ok(())
    }
}

// #region board_host
impl BoardHost {
    /// 🖍️ The area brush owns the whole pointer while it is armed: a press anchors a region
    /// rectangle instead of picking, so the same gesture can never also start a marquee.
    ///
    /// 🎯️ LAST of all the pick paths: a region is the board's backdrop, so it is only grabbed
    /// once the nodes, handles, edges and the rotate ring have all missed.
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        self.focus.set_selection_screen_preview(None);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if button == 1 {
                self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            } else if button == 0 {
                begin_region_paint(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.gestures, world);
            }
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            if button == 1 {
                self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            }
            return;
        }
        let hit = resolve_hit_world(&self.scene, &self.focus, &self.style, world).or_else(|| resolve_overview_free_link_handle_pointer_world(&self.scene, &self.focus, &self.style, world));
        if let Interaction::LinkTargetNode { source_id, target_node_id } = self.focus.interaction.clone() {
            self.focus.interaction = Interaction::None;
            self.outbox.clear_link_gesture_events();
            if button == 0 {
                if let Some(th) = node_sole_free_link_compatible_handle(&self.scene, &self.style, &source_id, &target_node_id) {
                    if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                        try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &th, Some(BoardEventKind::IndirectConnect));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| self.scene.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && handle_eligible_link_target_ring(&self.scene, &self.style, id.as_str(), source_id.as_str())) {
                    try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.focus.interaction.clone() {
            if button == 0 {
                if let Some(target_node_id) = ring_node_id {
                    if let Some(th) = node_sole_free_link_compatible_handle(&self.scene, &self.style, &source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.focus.interaction = Interaction::None;
                            self.outbox.clear_link_gesture_events();
                            try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &th, Some(BoardEventKind::IndirectConnect));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                    self.focus.interaction = Interaction::None;
                    self.outbox.clear_link_gesture_events();
                    try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        let pick_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
        if button == 0 && !merge_from_modifiers && self.try_begin_transform_drag_at(world) {
            return;
        }
        if button == 0 && !merge_from_modifiers && try_begin_bounded_selection_drag_at(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, world) {
            return;
        }
        if button == 1 {
            self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            return;
        }
        if let Some(ref hid) = hit {
            if let Some(node) = self.scene.nodes.get(hid) {
                if node.draggable && !node.locked {
                    let nid = hid.clone();
                    let nx = node.x;
                    let ny = node.y;
                    let mut gesture = self.outbox.begin_gesture();
                    let members_before: Vec<String> = self.focus.selection.iter().filter(|id| self.scene.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                    let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                    let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                    if !drag_group_before || force_pick_merge {
                        let next = merge_pick_into_selection(&self.focus.selection, &nid, pick_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        stage_selection_ids(&mut self.scene, &mut self.focus, &mut self.outbox, &mut gesture, &ids, merge_from_modifiers.then_some(pick_mode.as_str()));
                    }
                    let members: Vec<String> = self.focus.selection.iter().filter(|id| self.scene.nodes.get(*id).is_some_and(|n| n.draggable && !n.locked)).cloned().collect();
                    let drag_group = members.contains(&nid) && members.len() > 1;
                    let mut start_positions = BTreeMap::new();
                    for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                        if let Some(n) = self.scene.nodes.get(id) {
                            start_positions.insert(id.clone(), (n.x, n.y));
                        }
                    }
                    self.focus.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
                    set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, hit);
                    return;
                }
            }
        }
        if let Some(ref hid) = hit {
            if button == 0 && self.scene.handles.contains_key(hid) && !self.scene.handle_has_incident_edge(hid.as_str()) {
                let next = merge_pick_into_selection(&self.focus.selection, hid, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
                self.focus.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(hid.clone()));
                return;
            }
        }
        if hit.is_none() && button == 0 && try_begin_region_drag_at(&mut self.scene, &mut self.focus, &mut self.outbox, &mut self.gestures, world) {
            return;
        }
        if hit.is_none() && button == 0 {
            self.focus.interaction = Interaction::SelectionPending { initial_ids: self.focus.selection.clone(), start: world, start_screen: screen };
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        self.focus.interaction = Interaction::None;
        if let Some(id) = hit {
            let next = merge_pick_into_selection(&self.focus.selection, &id, pick_mode.as_str());
            let ids: Vec<_> = next.iter().cloned().collect();
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(id));
        } else {
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &[], gesture);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if let Interaction::Pan { origin, start_screen } = self.focus.interaction.clone() {
                let delta = screen - start_screen;
                board_set_camera(&mut self.scene, &mut self.outbox, origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom);
                self.focus.interaction = Interaction::Pan { origin, start_screen };
                return;
            }
            update_region_paint(&mut self.scene, &mut self.style, &mut self.gestures, world);
            return;
        }
        if self.gestures.region_drag.is_some() {
            update_region_drag(&mut self.scene, &mut self.style, &mut self.gestures, world);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            match std::mem::replace(&mut self.focus.interaction, Interaction::None) {
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    board_set_camera(&mut self.scene, &mut self.outbox, nx, ny, origin.zoom);
                    self.focus.interaction = Interaction::Pan { origin, start_screen };
                }
                _ => {
                    self.focus.interaction = Interaction::None;
                    brush_pointer_move(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, world);
                }
            }
            return;
        }
        if self.gestures.transform_drag.is_some() {
            update_transform_drag(&mut self.scene, &mut self.outbox, &mut self.gestures, world, self.style.grid_snap_enabled);
            return;
        }
        match std::mem::replace(&mut self.focus.interaction, Interaction::None) {
            Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta } => {
                let Some(delta) = drag_delta(&self.scene, &self.style, world, &primary_id, offset, &start_positions) else {
                    self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                };
                let mut event_count = 0usize;
                let mut event_bytes = 0usize;
                for (id, (ox0, oy0)) in &start_positions {
                    if self.scene.nodes.contains_key(id) {
                        event_count += 1;
                        let Some(total) = board_node_move_owned_bytes(id, ox0 + delta.x, oy0 + delta.y).and_then(|bytes| event_bytes.checked_add(bytes)) else {
                            self.outbox.event_schema_fault = true;
                            self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                            return;
                        };
                        event_bytes = total;
                    }
                }
                if self.outbox.events.reserve(event_count, event_bytes).is_err() {
                    self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                }
                let mut geometry_changed = false;
                for (id, (ox0, oy0)) in &start_positions {
                    if let Some(n) = self.scene.nodes.get_mut(id) {
                        let mx = ox0 + delta.x;
                        let my = oy0 + delta.y;
                        if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                            geometry_changed = true;
                        }
                        n.x = mx;
                        n.y = my;
                        let event = BoardOwnedEvent::node_move(id, mx, my).expect("node move event was exactly preflighted");
                        self.outbox.events.push(event).expect("node move event credits were reserved before mutation");
                    }
                }
                if geometry_changed {
                    self.scene.bump_content_scene_generation();
                }
                let proximity_pair = if start_positions.len() == 1 { node_drag_proximity_handle_pair(&self.scene, &self.style, primary_id.as_str()) } else { None };
                self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, delta };
            }
            Interaction::Pan { origin, start_screen } => {
                let delta = screen - start_screen;
                let nx = origin.x - delta.x / origin.zoom;
                let ny = origin.y - delta.y / origin.zoom;
                board_set_camera(&mut self.scene, &mut self.outbox, nx, ny, origin.zoom);
                self.focus.interaction = Interaction::Pan { origin, start_screen };
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                    self.focus.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                } else {
                    let points = vec![start, world];
                    let screen_points = vec![start_screen, screen];
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    board_apply_area_preselect(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &ids, gesture);
                    self.focus.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.focus.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(world);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = world;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let initial = initial_ids.clone();
                let pts = points.clone();
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial, start, &pts, merge_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let merge_from_modifiers = ctrl_or_meta || shift;
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                board_apply_area_preselect(&mut self.scene, &mut self.focus, &mut self.outbox, &initial, &ids, gesture);
                self.focus.sync_selection_screen_overlay(start_screen, &screen_points);
                self.focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
            }
            Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                    let optional_target_handle_id = nearest_link_snap_handle_world(&self.scene, &self.style, &source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.focus.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
                } else {
                    self.focus.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                    self.update_hover_from_world(world);
                }
            }
            Interaction::LinkDragSnap { source_id, .. } => {
                let optional_target_handle_id = nearest_link_snap_handle_world(&self.scene, &self.style, &source_id, world);
                self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                self.focus.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                self.focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                self.focus.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                self.update_hover_from_world(world);
            }
            Interaction::None => {
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if matches!(self.focus.interaction, Interaction::Pan { .. }) {
                self.focus.interaction = Interaction::None;
            }
            commit_region_paint(&mut self.scene, &mut self.outbox, &mut self.gestures);
            return;
        }
        if commit_region_drag(&mut self.scene, &mut self.outbox, &mut self.gestures) {
            self.update_hover_from_world(world);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            if matches!(self.focus.interaction, Interaction::Pan { .. }) {
                self.focus.interaction = Interaction::None;
            }
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if commit_transform_drag(&mut self.outbox, &mut self.gestures) {
            self.update_hover_from_world(world);
            return;
        }
        let grabbed = std::mem::take(&mut self.focus.interaction);
        match grabbed {
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(ref target_handle_id) = target_id {
                    if link_snap_commit_proximity_ok(&self.scene, &self.style, target_handle_id, world) && try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, target_handle_id, Some(BoardEventKind::ProximityConnect)) {
                        self.focus.interaction = Interaction::None;
                        self.outbox.clear_link_gesture_events();
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(target_node_id) = resolve_node_hit_world(&self.scene, &self.style, world) {
                    let source_node_id = self.scene.handles.get(&source_id).map(|h| h.node_id.clone());
                    if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                        if let Some(sole_target) = node_sole_free_link_compatible_handle(&self.scene, &self.style, source_id.as_str(), target_node_id.as_str()) {
                            try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &sole_target, Some(BoardEventKind::IndirectConnect));
                            self.outbox.clear_link_gesture_events();
                        } else {
                            self.focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(target_node_id));
                            sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
                        }
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::LinkAtSourceHandle { .. } => {
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::DragNodes { start_positions, proximity_pair, gesture, delta, .. } => {
                let record = drag_gesture_record(&gesture, &start_positions, delta, proximity_pair.as_ref());
                self.outbox.publish_gesture_release(&gesture, record);
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                let _ = (start, start_screen);
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    clear_selection_on_background_click(&mut self.scene, &mut self.focus, &mut self.outbox);
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                    let gesture = Some(merge_mode.as_str());
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &[start], merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
                }
                self.focus.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                points.push(world);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_from_modifiers = ctrl_or_meta || shift;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                if click_only {
                    commit_area_select_from_initial(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &[], gesture);
                } else {
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    commit_area_select_from_initial(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &ids, gesture);
                }
                self.focus.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { .. } => {
                self.focus.interaction = grabbed;
                self.update_hover_from_world(world);
            }
            _ => {
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }
}

impl BoardHost {
    pub fn pointer_leave_screen(&mut self, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        if cancel_region_paint(&mut self.scene, &mut self.gestures) || cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if cancel_transform_drag(&mut self.scene, &mut self.gestures) {
            return;
        }
        if matches!(self.focus.interaction, Interaction::None) {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        }
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        begin_pointer_cancel(&mut self.scene, &mut self.commit);
        let cancelled_region = cancel_region_paint(&mut self.scene, &mut self.gestures) | cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) | cancel_transform_drag(&mut self.scene, &mut self.gestures);
        let cancelled_brush = self.brush.brush_preview.is_some() || self.brush.brush_slot_source_id.is_some();
        self.brush.brush_slot_suggestions_active = false;
        self.brush.brush_slot_source_id = None;
        self.brush.brush_candidates.clear();
        self.brush.brush_candidate_index = 0;
        self.brush.brush_preview = None;
        self.brush.brush_preview_emit_key = None;
        self.brush.brush_candidates_emit_key = None;
        let interaction = std::mem::take(&mut self.focus.interaction);
        let cancelled_interaction = !matches!(interaction, Interaction::None);
        match interaction {
            Interaction::DragNodes { start_positions, gesture, .. } => cancel_node_drag(&mut self.scene, &mut self.focus, &mut self.outbox, start_positions, gesture),
            Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => {
                self.focus.selection = initial_ids;
                self.focus.preselect.clear();
                self.focus.preselect_removed.clear();
                self.outbox.last_preselect_emit_sig = None;
                self.outbox.last_select_emit_sig = None;
                sync_selection_flags_to_objects(&mut self.scene, &mut self.focus);
                self.focus.set_selection_screen_preview(None);
                self.scene.bump_content_scene_generation();
            }
            Interaction::LinkAtSourceHandle { .. }
            | Interaction::LinkDragSnap { .. }
            | Interaction::LinkTargetNode { .. }
            | Interaction::ExternalLinkPreview { .. } => {
                self.gestures.link_screen_preview = None;
                self.outbox.link_compat_nodes_emit_key = None;
                self.outbox.link_target_ring_emit_key = None;
                self.scene.bump_content_scene_generation();
            }
            Interaction::Pan { .. } | Interaction::None => {}
        }
        cancelled_region || cancelled_brush || cancelled_interaction
    }

    /// ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
    pub fn cancel_area_select(&mut self) -> bool {
        if cancel_region_paint(&mut self.scene, &mut self.gestures) || cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) {
            return true;
        }
        if cancel_transform_drag(&mut self.scene, &mut self.gestures) {
            return true;
        }
        let reservation = match &self.focus.interaction {
            Interaction::Selection { initial_ids, .. } => {
                let event = BoardOwnedEvent::id_list(BoardEventKind::PreselectCancel, "ids", initial_ids.iter().map(String::as_str));
                let Some(reservation) = self.outbox.reserve_owned_event(event) else {
                    return false;
                };
                Some(reservation)
            }
            _ => None,
        };
        let prev = std::mem::replace(&mut self.focus.interaction, Interaction::None);
        match prev {
            Interaction::DragNodes { start_positions, gesture, .. } => {
                cancel_node_drag(&mut self.scene, &mut self.focus, &mut self.outbox, start_positions, gesture);
                true
            }
            Interaction::SelectionPending { .. } => {
                self.focus.set_selection_screen_preview(None);
                true
            }
            Interaction::Selection { initial_ids, .. } => {
                self.focus.set_selection_screen_preview(None);
                self.focus.preselect.clear();
                self.focus.preselect_removed.clear();
                self.outbox.last_preselect_emit_sig = None;
                self.focus.selection = initial_ids;
                sync_selection_flags_to_objects(&mut self.scene, &mut self.focus);
                self.scene.bump_content_scene_generation();
                self.outbox.last_select_emit_sig = None;
                self.outbox.publish_event_reservation(reservation.expect("selection cancellation reserved one event"));
                true
            }
            other => {
                self.focus.interaction = other;
                false
            }
        }
    }

    /// ⭕️ Pivot and ring radius when the rotate gumball is live: the select utility is
    /// active, the rotate flag is on, and at least one selected node exists to turn.
    pub fn transform_gumball_geometry(&self) -> Option<(Point, f64)> {
        if self.focus.active_utility != ActiveUtility::Select || !self.gestures.transform_flags.rotate_enabled {
            return None;
        }
        let ids = transform_selection_node_ids(&self.scene, &self.focus);
        if ids.is_empty() {
            return None;
        }
        let centers: Vec<Point> = ids.iter().filter_map(|id| self.scene.nodes.get(id)).map(|n| Point::new(n.x, n.y)).collect();
        let pivot = transform_pivot_of(&centers)?;
        let mut corners = Vec::with_capacity(centers.len() * 2);
        for id in &ids {
            let Some(node) = self.scene.nodes.get(id) else { continue };
            let bounds = self.style.node_world_bounds(node, 0.0);
            corners.push(Point::new(bounds.min_x, bounds.min_y));
            corners.push(Point::new(bounds.max_x, bounds.max_y));
        }
        Some((pivot, transform_ring_radius_world(pivot, &corners, self.scene.camera.zoom)))
    }

    /// 🩺️ The gumball's whole probe-visible state: which handles are composed, whether the
    /// ring is drawn, and the live drag's pivot/angle. Read as `data-board-transform-json`.
    pub fn transform_gumball_json(&self) -> String {
        let geometry = self.gestures.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry());
        let mut out = String::from("{\"move\":");
        out.push_str(if self.gestures.transform_flags.move_enabled { "true" } else { "false" });
        out.push_str(",\"rotate\":");
        out.push_str(if self.gestures.transform_flags.rotate_enabled { "true" } else { "false" });
        out.push_str(",\"ringVisible\":");
        out.push_str(if geometry.is_some() { "true" } else { "false" });
        out.push_str(",\"dragging\":");
        out.push_str(if self.gestures.transform_drag.is_some() { "true" } else { "false" });
        out.push_str(",\"radians\":");
        out.push_str(&self.gestures.transform_drag.as_ref().map_or(0.0, |drag| drag.radians).to_string());
        if let Some((pivot, radius)) = geometry {
            out.push_str(",\"pivot\":{\"x\":");
            out.push_str(&pivot.x.to_string());
            out.push_str(",\"y\":");
            out.push_str(&pivot.y.to_string());
            out.push_str("},\"radius\":");
            out.push_str(&radius.to_string());
        }
        out.push('}');
        out
    }

    /// 🔄️ Grabs the rotate ring. Asked BEFORE any node/handle/edge hit test, so a ring that
    /// crosses a node still rotates instead of starting a drag on whatever sits under it.
    fn try_begin_transform_drag_at(&mut self, world: Point) -> bool {
        let Some((pivot, radius_world)) = self.transform_gumball_geometry() else {
            return false;
        };
        if !transform_ring_hit(pivot, radius_world, self.scene.camera.zoom, world) {
            return false;
        }
        let ids = transform_selection_node_ids(&self.scene, &self.focus);
        let mut start_positions = BTreeMap::new();
        for id in &ids {
            if let Some(node) = self.scene.nodes.get(id) {
                if node.draggable && !node.locked {
                    start_positions.insert(id.clone(), (node.x, node.y));
                }
            }
        }
        if start_positions.is_empty() {
            return false;
        }
        let start_handle_angles = self.scene.handles.iter().filter(|(_, handle)| start_positions.contains_key(&handle.node_id)).map(|(id, handle)| (id.clone(), handle.angle)).collect();
        let gesture = self.outbox.begin_gesture();
        self.gestures.transform_drag = Some(BoardTransformDrag { gesture, pivot, grab: world, radius_world, radians: 0.0, start_positions, start_handle_angles });
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        true
    }

    /// ⭕️ Paints the ring, its pivot dot and the live grab spoke.
    ///
    /// ⭕️ While the gesture is live the ring is the one grabbed at press time: re-deriving it from
    /// the turning nodes would make the band breathe under the cursor for a non-circular selection.
    pub(crate) fn append_transform_gumball_paint(&self, scene: &mut Scene, world_space: bool) {
        let Some((pivot, radius_world)) = self.gestures.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry()) else {
            return;
        };
        let center = self.scene.draw_space_point(pivot, world_space);
        let radius = self.scene.draw_space_len(radius_world, world_space);
        let color = if self.gestures.transform_drag.is_some() { self.style.canvas_theme.node_stroke_selected } else { self.style.canvas_theme.node_stroke };
        let mut ring_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
        if self.gestures.transform_drag.is_none() {
            ring_stroke.set_dash_pattern(vec![6.0, 5.0]);
        }
        scene.stroke(&ring_stroke, Affine::IDENTITY, color, None, &Circle::new(center, radius));
        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &Circle::new(center, self.scene.draw_space_len(TRANSFORM_RING_HIT_TOLERANCE_PX / self.scene.camera.zoom.max(1e-9) * 0.25, world_space)));
        if let Some(ref drag) = self.gestures.transform_drag {
            let spoke = rotate_point_about(drag.pivot, Point::new(drag.pivot.x + radius_world, drag.pivot.y), drag.radians);
            let mut path = semio_framework_canvas::BezPath::new();
            path.move_to(center);
            path.line_to(self.scene.draw_space_point(spoke, world_space));
            scene.stroke(&Stroke::new(ui_styling::strokes::SELECTION_PREVIEW), Affine::IDENTITY, color, None, &path);
        }
    }
    //#endregion 🕹️TransformGumball
}

#[cfg(any(test, feature = "testkit"))]
fn commit_pointer_legacy_differential(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, plan: &BoardPointerPlan) -> bool {
    match plan.kind {
        BoardPointerPlanKind::Idle => {}
        BoardPointerPlanKind::Pan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
        }
        BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => unreachable!("drag plans are committed through the retained pointer authority"),
        BoardPointerPlanKind::FinishPan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
            focus.interaction = Interaction::None;
        }
        BoardPointerPlanKind::SelectionPreview { start, start_screen } => {
            let grabbed = std::mem::take(&mut focus.interaction);
            let initial_ids = match grabbed {
                Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => initial_ids,
                other => {
                    focus.interaction = other;
                    return false;
                }
            };
            let next: BTreeSet<String> = plan.selection_ids().map(str::to_string).collect();
            focus.preselect_removed = initial_ids.difference(&next).cloned().collect();
            focus.preselect = next;
            outbox.last_preselect_emit_sig = None;
            sync_selection_flags_to_objects(scene, focus);
            let points = plan.points[..usize::from(plan.point_len)].to_vec();
            let screen_points = plan.screen_points[..usize::from(plan.point_len)].to_vec();
            focus.sync_selection_screen_overlay(start_screen, &screen_points);
            focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
        }
        BoardPointerPlanKind::SelectionCommit => {
            if !matches!(focus.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                return false;
            }
            focus.interaction = Interaction::None;
            focus.selection = plan.selection_ids().map(str::to_string).collect();
            focus.preselect.clear();
            focus.preselect_removed.clear();
            focus.selection_exit_highlight.clear();
            outbox.last_select_emit_sig = None;
            outbox.last_preselect_emit_sig = None;
            focus.selection_screen_preview = None;
            focus.selection_preview_crossing = false;
            sync_selection_flags_to_objects(scene, focus);
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkMove { source, target, hover, end_world, activated, start_screen, .. } => {
            if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                return false;
            }
            let source_id = plan.id(source).to_string();
            let target_id = target.map(|target| plan.id(target).to_string());
            let hover_id = hover.map(|hover| plan.id(hover).to_string());
            let node_ids = link_drag_compatible_target_node_ids(scene, style_state, &source_id);
            let ring_node_id = activated.then(|| link_drag_ring_target_node_id(scene, style_state, &source_id, end_world)).flatten().filter(|node_id| link_compatible_handle_count_on_node(scene, style_state, &source_id, node_id) > 1);
            let ring_handle_ids = ring_node_id.as_deref().map(|node_id| link_compatible_handle_ids_on_node(scene, style_state, &source_id, node_id)).unwrap_or_default();
            outbox.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
            outbox.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(",")));
            focus.hovered_id = hover_id;
            focus.hovered_kind = None;
            focus.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, .. } => {
            if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                return false;
            }
            let source_id = plan.id(source).to_string();
            let target_id = target.map(|target| plan.id(target).to_string());
            let edge_id = edge.map(|edge| plan.id(edge).to_string());
            let target_node_id = target_node.map(|target_node| plan.id(target_node).to_string());
            let hover_id = hover.map(|hover| plan.id(hover).to_string());
            if let (Some(target_id), Some(edge_id)) = (target_id, edge_id) {
                let Some(source_row) = scene.handles.get(&source_id) else {
                    return false;
                };
                let Some(target_row) = scene.handles.get(&target_id) else {
                    return false;
                };
                let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
                scene.edges.insert(
                    edge_id.clone(),
                    EdgeData { id: edge_id, source: source_id.clone(), target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                );
            }
            if let Some(target_node_id) = target_node_id {
                let node_ids = link_drag_compatible_target_node_ids(scene, style_state, &source_id);
                let ring_handle_ids = link_compatible_handle_ids_on_node(scene, style_state, &source_id, &target_node_id);
                outbox.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                outbox.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
                focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
            } else {
                outbox.link_compat_nodes_emit_key = None;
                outbox.link_target_ring_emit_key = None;
                focus.interaction = Interaction::None;
            }
            focus.hovered_id = hover_id;
            focus.hovered_kind = None;
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkRetain { hover } => {
            if !matches!(focus.interaction, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                return false;
            }
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
        }
        BoardPointerPlanKind::Hover { hover } => {
            if !matches!(focus.interaction, Interaction::None) {
                return false;
            }
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
        }
        BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => {
            if focus.active_utility != ActiveUtility::Brush || matches!(focus.interaction, Interaction::Pan { .. }) {
                return false;
            }
            let source_id = source.map(|source| plan.id(source).to_string());
            let mut candidates = BrushCandidatePage::default();
            for candidate in plan.deltas[..usize::from(plan.delta_len)].iter().flatten() {
                if candidates.push(plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                    return false;
                }
            }
            if commit_old {
                brush.brush_placement_serial = brush.brush_placement_serial.wrapping_add(1);
            }
            brush.brush_alt_pressed = alt;
            brush.brush_slot_suggestions_active = false;
            brush.brush_slot_source_id = source_id.clone();
            brush.brush_candidates = candidates;
            brush.brush_candidate_index = 0;
            let offset = if alt { brush.suggestion_offset } else { 0.0 };
            brush.brush_preview = source_id.as_deref().and_then(|source_id| brush.brush_candidates.first().and_then(|candidate| brush_build_preview_with_offset(scene, style_state, brush, source_id, candidate, offset)));
            brush.brush_preview_emit_key = None;
            brush.brush_candidates_emit_key = None;
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
            focus.interaction = Interaction::None;
            scene.bump_content_scene_generation();
        }
    }
    scene.interaction_revision = plan.revision.wrapping_add(1);
    true
}

impl semio_framework_canvas::canvas_content::CanvasContent for BoardHost {
    fn build_scene(&self) -> Scene {
        self.build_vector_scene()
    }

    fn clear_color(&self) -> Color {
        self.style.canvas_theme.raster_clear
    }
}
// #endregion board_host

pub struct BoardHost {
    /// Entities, camera, viewport and the cached world content of one board.
    pub scene: BoardSceneState,
    /// Selection, preselect, hover and highlight state with the active utility.
    pub focus: BoardFocusState,
    /// Style catalogs, themes, LOD and grid policy.
    pub style: BoardStyleState,
    /// Brush candidates, previews and suggestion state.
    pub brush: BoardBrushState,
    /// The outgoing board event queue and its emit dedupe signatures.
    pub outbox: BoardOutboxState,
    /// Transform, region and link gesture state.
    pub gestures: BoardGesturesState,
    /// Pending pointer commit and delete operations.
    pub commit: BoardCommitState,
    /// Close and opaque scene retirement state.
    pub closing: BoardClosingState,
}

// #region board_host
pub struct BoardHostRetirement {
    pub host: std::mem::ManuallyDrop<BoardHost>,
    pub released: bool,
}

impl BoardHostRetirement {
    pub fn new(host: BoardHost) -> Self {
        Self { host: std::mem::ManuallyDrop::new(host), released: false }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.released {
            return true;
        }
        if !self.host.close_nonopaque_step(context) {
            return false;
        }
        assert!(self.host.nonopaque_terminal_is_empty(), "BoardHost nonopaque terminal witness precedes shallow release");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.host) };
        self.released = true;
        true
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.released
    }
}

impl Drop for BoardHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.released, "BoardHostRetirement must reach terminal-empty before release");
    }
}

pub(crate) fn board_node_move_owned_bytes(id: &str, x: f64, y: f64) -> Option<usize> {
    if id.len() > BOARD_EVENT_KEY_BYTE_CAPACITY || !x.is_finite() || !y.is_finite() {
        return None;
    }
    17usize.checked_add(board_json_string_bytes(id))?.checked_add(x.to_string().len())?.checked_add(y.to_string().len())?.checked_add(id.len())
}

impl BoardHost {
    // #region board_host
    /// 🔗️ Authoring sync: every edge the descriptor adds is announced as an `edgeCreate` event.
    pub fn sync_descriptor(&mut self, desc: &SceneDescriptor) -> Result<(), NormalPortError> {
        sync_descriptor_with(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, desc, true)
    }

    /// 🎯️ Target regions are the backdrop: painted before the first entity layer, always in
    /// world space, so nothing they overlap is ever hidden behind them.
    fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
        self.closing.advance_opaque_scene_retirement_step();
        let generation = self.scene.content_scene_generation;
        let cam_aff = self.scene.camera_content_affine();
        let overlay_ids = interaction_overlay_entity_ids(&self.scene, &self.focus);
        if !self.scene.regions.is_empty() || self.gestures.region_paint.is_some() {
            let mut region_layer = Scene::new();
            append_target_region_paint(&self.scene, &self.focus, &self.style, &self.gestures, &mut region_layer, true);
            scene.append(&region_layer, Some(cam_aff));
        }
        let mut fill_layer = Scene::new();
        append_nodes_and_handles_with_overlay_chrome(&self.scene, &self.focus, &self.style, &mut fill_layer, None, lod, true, None, &overlay_ids, NodeHandlePaintLayer::Fill);
        scene.append(&fill_layer, Some(cam_aff));
        let mut cache = self.scene.world_content_cache.borrow_mut();
        let needs_rebuild = cache.as_ref().is_none_or(|c| c.generation != generation || c.lod != lod);
        if needs_rebuild && self.closing.opaque_scene_retirement.get().is_none() {
            if cache.is_some() {
                let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
                    self.closing.opaque_scene_fault.set(true);
                    return;
                };
                let WorldContentCache { scene: stale, .. } = cache.take().expect("stale world content cache was witnessed occupied");
                self.closing.publish_opaque_scene_retirement(token, stale);
            }
            let mut content = Scene::new();
            let spans = bake_node_icon_spans(&self.scene, &self.focus, &self.style, &mut content, lod);
            *cache = Some(WorldContentCache { generation, lod, scene: content, spans });
        }
        if let Some(cached) = cache.as_ref() {
            self.scene.append_baked_node_icons(scene, cached, cam_aff);
        }
        let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if edges_in_world_space {
            let mut edge_layer = Scene::new();
            append_edges_wires_and_link(&self.scene, &self.focus, &self.style, &mut edge_layer, None, lod, true, None, Some(&overlay_ids));
            scene.append(&edge_layer, Some(cam_aff));
        } else {
            append_edges_wires_and_link(&self.scene, &self.focus, &self.style, scene, None, lod, false, None, Some(&overlay_ids));
        }
        let mut stroke_layer = Scene::new();
        append_nodes_and_handles_with_overlay_chrome(&self.scene, &self.focus, &self.style, &mut stroke_layer, None, lod, false, None, &overlay_ids, NodeHandlePaintLayer::Stroke);
        scene.append(&stroke_layer, None);
        if let Some(c) = active_link_wire_curve(&self.scene, &self.focus, &self.style) {
            let link_wire_stroke = Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT);
            let link_wire_color = self.style.canvas_theme.node_stroke;
            let p0 = self.scene.draw_space_point(c.p0(), false);
            let p1 = self.scene.draw_space_point(c.p1(), false);
            let p2 = self.scene.draw_space_point(c.p2(), false);
            let p3 = self.scene.draw_space_point(c.p3(), false);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
        if self.style.has_ports() {
            if let Some(node_id) = indirect_ring_node_id(&self.scene, &self.focus, &self.style, lod) {
                append_indirect_handle_ring(&self.scene, &self.focus, &self.style, scene, None, &node_id, StyleChromePass::CachedBase, false);
            }
        }
        let previews_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if previews_in_world_space {
            let mut preview_layer = Scene::new();
            if self.brush.drop_preview.is_some() {
                append_drop_preview_paint(&self.scene, &self.style, &self.brush, &mut preview_layer, lod, true);
            }
            if self.focus.active_utility == ActiveUtility::Brush || self.brush.brush_preview.is_some() {
                append_brush_preview_paint(&self.scene, &self.style, &self.brush, &mut preview_layer, lod, true);
            }
            self.append_transform_gumball_paint(&mut preview_layer, true);
            scene.append(&preview_layer, Some(cam_aff));
        } else {
            if self.brush.drop_preview.is_some() {
                append_drop_preview_paint(&self.scene, &self.style, &self.brush, scene, lod, false);
            }
            if self.focus.active_utility == ActiveUtility::Brush || self.brush.brush_preview.is_some() {
                append_brush_preview_paint(&self.scene, &self.style, &self.brush, scene, lod, false);
            }
            self.append_transform_gumball_paint(scene, false);
        }
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut inner = Scene::new();
        let lod = board_draw_lod_for_frame(&self.scene, &self.style);
        if self.style.grid_visible && !self.style.wheel_zoom_active {
            let grid_color = self.style.canvas_theme.grid_minor_stroke;
            if lod != BoardDrawLod::Minimap {
                self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_LARGE, self.style.grid_step_large_world(), 0.0);
                match lod {
                    BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                        self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MEDIUM, self.style.grid_step_medium_world(), 0.0);
                    }
                    BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                }
                if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_SMALL, self.style.grid_step_small_world(), 0.0);
                }
                if lod == BoardDrawLod::Micro {
                    self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MICRO, self.style.grid_step_micro_world(), 0.0);
                }
            }
        }
        if let Some(ref pts) = self.focus.selection_screen_preview {
            if pts.len() >= 2 {
                let mut path = semio_framework_canvas::BezPath::new();
                path.move_to(pts[0]);
                for p in pts.iter().skip(1) {
                    path.line_to(*p);
                }
                path.close_path();
                inner.fill(FillRule::NonZero, Affine::IDENTITY, self.style.canvas_theme.selection_preview_fill, None, &path);
                let mut preview_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
                if self.focus.selection_preview_crossing {
                    preview_stroke.set_dash_pattern(vec![5.0, 4.0]);
                }
                inner.stroke(&preview_stroke, Affine::IDENTITY, self.style.canvas_theme.selection_preview_stroke, None, &path);
            }
        }
        self.append_cached_world_content(&mut inner, lod);
        let scale = self.scene.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            inner
        } else {
            let mut scene = Scene::new();
            scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
            scene
        }
    }

    pub fn encoded_scene_hint(&self) -> usize {
        let s = self.build_vector_scene();
        s.path_count()
    }

    pub fn update_hover_from_world(&mut self, world: Point) {
        let next = resolve_hover_world(&self.scene, &self.focus, &self.style, world);
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, next);
    }

    pub fn plan_pointer(&self, intent: BoardPointerIntent) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if self.scene.interaction_revision == u64::MAX || (matches!(intent.phase, BoardPointerPhase::Move | BoardPointerPhase::Up) && pointer_lane_is_direct(&self.focus, &self.gestures)) {
            return Err(BoardPointerPlanFault::Unsupported);
        }
        let screen = Point::new(intent.x, intent.y);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::Brush && !matches!(self.focus.interaction, Interaction::Pan { .. }) {
            return plan_brush_pointer(&self.scene, &self.focus, &self.style, &self.brush, intent, world);
        }
        match (intent.phase, &self.focus.interaction) {
            (BoardPointerPhase::Move, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Pan { camera }))
            }
            (BoardPointerPhase::Move, Interaction::DragNodes { primary_id, offset, start_positions, .. }) => plan_drag_pointer(&self.scene, &self.style, world, (primary_id, *offset, start_positions), None),
            (_, Interaction::SelectionPending { initial_ids, start, start_screen }) => plan_selection_pending_pointer(&self.scene, &self.focus, &self.style, intent, initial_ids, *start, *start_screen, screen, world),
            (_, Interaction::Selection { initial_ids, points, screen_points, start, start_screen }) => plan_selection_pointer(&self.scene, &self.focus, &self.style, intent, initial_ids, points, screen_points, *start, *start_screen, screen, world),
            (BoardPointerPhase::Move, Interaction::LinkAtSourceHandle { source_id, start_screen }) => plan_link_move_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, Some(*start_screen), screen, world),
            (BoardPointerPhase::Move, Interaction::LinkDragSnap { source_id, .. }) => plan_link_move_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, None, screen, world),
            (BoardPointerPhase::Up, Interaction::LinkDragSnap { source_id, target_id, .. }) => plan_link_finish_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, target_id.as_deref(), world),
            (BoardPointerPhase::Up, Interaction::LinkAtSourceHandle { source_id, .. }) => plan_link_finish_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, None, world),
            (BoardPointerPhase::Move, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) => plan_link_retain_pointer(&self.scene, &self.focus, &self.style, world),
            (BoardPointerPhase::Up, Interaction::LinkTargetNode { source_id, .. }) => plan_link_clear_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, world),
            (BoardPointerPhase::Up, Interaction::ExternalLinkPreview { .. }) => plan_link_retain_pointer(&self.scene, &self.focus, &self.style, world),
            (BoardPointerPhase::Move, Interaction::None) => plan_hover_pointer(&self.scene, &self.focus, resolve_hover_world(&self.scene, &self.focus, &self.style, world).as_deref()),
            (BoardPointerPhase::Up, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::FinishPan { camera }))
            }
            (BoardPointerPhase::Up, Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, .. }) => plan_drag_pointer(&self.scene, &self.style, world, (primary_id, *offset, start_positions), Some((gesture, proximity_pair.as_ref()))),
            (BoardPointerPhase::Up, Interaction::None) => Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Idle)),
            (BoardPointerPhase::Leave, Interaction::None) => plan_hover_pointer(&self.scene, &self.focus, None),
            (BoardPointerPhase::Leave, _) => Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Idle)),
        }
    }

    pub fn step_pointer_commit(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        let Some(mut operation) = self.commit.pending_pointer_commit.take() else {
            return BoardAuthorityStep::Complete;
        };
        if context.should_yield() {
            self.commit.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if context.is_cancelled() && operation.phase == 0 {
            operation.cancelling = true;
        }
        if self.scene.interaction_revision != operation.plan.revision && operation.phase == 0 {
            operation.cancelling = true;
            operation.faulted = true;
        }
        if operation.cancelling {
            if operation.cursor < operation.plan.delta_len {
                operation.plan.deltas[usize::from(operation.cursor)] = None;
                operation.cursor += 1;
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.retire_one() || operation.points.pop().is_some() || operation.screen_points.pop().is_some() || operation.overlay_points.pop().is_some() {
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.scan_after.take().is_some() {
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            return if operation.faulted { BoardAuthorityStep::Fault } else { BoardAuthorityStep::Cancelled };
        }
        let complete = step_pointer_commit_operation(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, &mut operation);
        context.consume_fuel(1);
        if !complete {
            self.commit.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if operation.faulted {
            return BoardAuthorityStep::Fault;
        }
        let completed_revision = operation.plan.revision;
        if self.scene.interaction_revision != completed_revision {
            return BoardAuthorityStep::Fault;
        }
        self.scene.interaction_revision = completed_revision.wrapping_add(1);
        if operation.plan.output_len > 2 {
            self.commit.pointer_publication = Some(BoardPointerPublication { bytes: operation.plan.output, len: operation.plan.output_len });
        }
        if let Some(mut queued) = self.commit.queued_pointer_commit.take() {
            if queued.revision != completed_revision {
                return BoardAuthorityStep::Fault;
            }
            queued.revision = self.scene.interaction_revision;
            self.commit.pending_pointer_commit = Some(BoardPointerCommitOperation::new(queued));
            return BoardAuthorityStep::Pending;
        }
        BoardAuthorityStep::Complete
    }

    pub fn step_pointer_cancel(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.commit.pending_pointer_commit.is_some() {
            let _ = self.step_pointer_commit(context);
            return false;
        }
        let Some(retirement) = self.commit.cancelled_pointer_plan_retirement.as_mut() else {
            return true;
        };
        if retirement.close_step() {
            self.commit.cancelled_pointer_plan_retirement = None;
        }
        context.consume_fuel(1);
        false
    }

    pub fn close_pointer_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if let Some(operation) = self.commit.pending_pointer_commit.as_mut() {
            if operation.cursor == 0 {
                operation.cancelling = true;
            }
            let _ = self.step_pointer_commit(context);
            return false;
        }
        if self.commit.queued_pointer_commit.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.commit.cancelled_pointer_plan_retirement.as_mut() {
            if retirement.close_step() {
                self.commit.cancelled_pointer_plan_retirement = None;
            }
            context.consume_fuel(1);
            return false;
        }
        if let Some(publication) = self.commit.pointer_publication.as_mut() {
            if publication.close_step() {
                self.commit.pointer_publication = None;
            }
            return false;
        }
        true
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn plan_delete_selection(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Result<BoardDeletePlan, BoardEventFault> {
    let mut plan = BoardDeletePlan::new(scene.interaction_revision);
    for id in focus.selection.iter().filter(|id| scene.edges.contains_key(*id)) {
        let edge = scene.edges.get(id).expect("selected edge exists");
        plan.push_entity(BoardDeleteKind::Edge, id, &edge.properties)?;
    }
    if style.has_ports() {
        for (node_id, node) in scene.nodes.iter().filter(|(node_id, _)| focus.selection.contains(*node_id) || scene.handles.values().any(|handle| handle.node_id == **node_id && focus.selection.contains(&handle.id))) {
            for (handle_id, handle) in scene.handles.iter().filter(|(_, handle)| handle.node_id == *node_id) {
                for (wire_id, wire) in scene.wires.iter().filter(|(_, wire)| wire.source == *handle_id || wire.target.as_ref() == Some(handle_id)) {
                    plan.push_entity(BoardDeleteKind::Wire, wire_id, &wire.properties)?;
                }
                for (edge_id, edge) in scene.edges.iter().filter(|(_, edge)| edge.source == *handle_id || edge.target == *handle_id) {
                    plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                }
                plan.push_entity(BoardDeleteKind::Handle, handle_id, &handle.properties)?;
            }
            plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
        }
    } else {
        for (node_id, node) in scene.nodes.iter().filter(|(node_id, _)| focus.selection.contains(*node_id)) {
            for (edge_id, edge) in scene.edges.iter().filter(|(_, edge)| edge.source == *node_id || edge.target == *node_id) {
                plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
            }
            plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
        }
    }
    Ok(plan)
}

pub(crate) fn step_pending_delete_operation(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, commit: &mut BoardCommitState) {
    let Some(mut operation) = commit.pending_delete_operation.take() else {
        return;
    };
    if let Some(retiring) = operation.retiring_entity.as_mut() {
        match retiring.step() {
            Ok(true) => {
                debug_assert!(retiring.terminal_is_empty());
                operation.retiring_entity = None;
            }
            Ok(false) => {}
            Err(()) => outbox.event_schema_fault = true,
        }
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if !operation.claimed {
        if scene.interaction_revision != operation.plan.revision {
            outbox.event_schema_fault = true;
            return;
        }
        let event_count = operation.plan.emitted_event_count() + 1;
        let Some(event_bytes) = operation.plan.emitted_event_bytes().and_then(|bytes| bytes.checked_add(operation.select_event.as_ref()?.owned_bytes())) else {
            outbox.event_schema_fault = true;
            return;
        };
        if outbox.events.claim(event_count, event_bytes).is_err() {
            commit.pending_delete_operation = Some(operation);
            return;
        }
        operation.claimed = true;
        operation.remaining_claimed_items = event_count as u16;
        operation.remaining_claimed_bytes = event_bytes;
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.mutation_cursor < operation.plan.len {
        let entry = operation.plan.entries[usize::from(operation.mutation_cursor)].expect("delete plan entry");
        let id = operation.plan.id(entry.id);
        let retiring = match entry.kind {
            BoardDeleteKind::Edge => {
                let removed = scene.edges.remove(id).map(BoardRemovedEntity::Edge);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Node => {
                let removed = scene.nodes.remove(id).map(BoardRemovedEntity::Node);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Handle => {
                let removed = scene.handles.remove(id).map(BoardRemovedEntity::Handle);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Wire => {
                let removed = scene.wires.remove(id).map(BoardRemovedEntity::Wire);
                focus.selection.remove(id);
                removed
            }
        };
        operation.mutation_cursor += 1;
        operation.retiring_entity = retiring.map(BoardEntityRetirement::new);
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.publication_cursor < operation.plan.len {
        let entry = operation.plan.entries[usize::from(operation.publication_cursor)].expect("delete plan entry");
        operation.publication_cursor += 1;
        let id = operation.plan.id(entry.id);
        let event = match entry.kind {
            BoardDeleteKind::Edge => Some(BoardOwnedEvent::id(BoardEventKind::EdgeDelete, id).expect("delete edge payload was exactly preflighted")),
            BoardDeleteKind::Node => Some(BoardOwnedEvent::id(BoardEventKind::NodeDelete, id).expect("delete node payload was exactly preflighted")),
            BoardDeleteKind::Handle | BoardDeleteKind::Wire => None,
        };
        if let Some(event) = event {
            operation.remaining_claimed_items -= 1;
            operation.remaining_claimed_bytes -= event.owned_bytes();
            outbox.events.push_claimed(event).expect("delete event owns claimed queue credits");
        }
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if let Some(event) = operation.select_event.take() {
        operation.remaining_claimed_items -= 1;
        operation.remaining_claimed_bytes -= event.owned_bytes();
        outbox.events.push_claimed(event).expect("delete selection owns claimed queue credits");
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if let Some(id) = focus.selection_exit_highlight.pop_first() {
        drop(id);
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.remaining_claimed_items != 0 || operation.remaining_claimed_bytes != 0 {
        let _ = outbox.events.release_claim(usize::from(operation.remaining_claimed_items), operation.remaining_claimed_bytes);
        outbox.event_schema_fault = true;
    }
    scene.bump_content_scene_generation();
    scene.interaction_revision = operation.plan.revision.wrapping_add(1);
}

pub fn board_delete_selection(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, commit: &mut BoardCommitState) {
    if commit.pending_delete_planning.is_some() || commit.pending_delete_operation.is_some() {
        return;
    }
    let mut select_builder = BoardPayloadBuilder::new();
    if select_builder.raw("{\"ids\":[").is_err() {
        outbox.event_schema_fault = true;
        return;
    }
    commit.pending_delete_planning = Some(BoardDeletePlanningOperation {
        plan: BoardDeletePlan::new(scene.interaction_revision),
        phase: BoardDeletePlanningPhase::SelectedEdges,
        scan_after: None,
        node_after: None,
        handle_after: None,
        relation_after: None,
        node_id: None,
        handle_id: None,
        node_relevant: false,
        property_audit: None,
        select_builder,
        select_first: true,
        fault: None,
        cancelling: false,
    });
}

fn plan_selection_pending_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, intent: BoardPointerIntent, initial_ids: &BTreeSet<String>, start: Point, start_screen: Point, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    match intent.phase {
        BoardPointerPhase::Move if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX => Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle)),
        BoardPointerPhase::Move => {
            let points = [start, world];
            let screen_points = [start_screen, screen];
            let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
            let next = resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &points, merge_mode.as_str());
            let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
            scene.build_selection_pointer_plan(BoardPointerPlanKind::SelectionPreview { start, start_screen }, &next, initial_ids, gesture, &points, &screen_points)
        }
        BoardPointerPhase::Up => {
            let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
            let next = if intent.ctrl_or_meta || intent.shift { resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &[start], merge_mode.as_str()) } else { BTreeSet::new() };
            let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
            scene.build_selection_pointer_plan(BoardPointerPlanKind::SelectionCommit, &next, initial_ids, gesture, &[], &[])
        }
        BoardPointerPhase::Leave => Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle)),
    }
}

fn plan_selection_pointer(
    scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState,
    intent: BoardPointerIntent,
    initial_ids: &BTreeSet<String>,
    points: &[Point],
    screen_points: &[Point],
    start: Point,
    start_screen: Point,
    screen: Point,
    world: Point,
) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    if intent.phase == BoardPointerPhase::Leave {
        return Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle));
    }
    if points.len() != screen_points.len() || points.len() >= BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let mut next_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
    let mut next_screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
    next_points[..points.len()].copy_from_slice(points);
    next_screen_points[..screen_points.len()].copy_from_slice(screen_points);
    let mut len = points.len();
    let last_screen = screen_points.last().copied().unwrap_or(start_screen);
    let add_point = intent.phase == BoardPointerPhase::Up || focus.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
    if add_point {
        next_points[len] = world;
        next_screen_points[len] = screen;
        len += 1;
    } else if len > 0 {
        next_points[len - 1] = world;
        next_screen_points[len - 1] = screen;
    }
    let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
    let end_screen = next_screen_points[len.saturating_sub(1)];
    let click_only = intent.phase == BoardPointerPhase::Up && distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
    let next = if click_only { BTreeSet::new() } else { resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &next_points[..len], merge_mode.as_str()) };
    let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
    let kind = if intent.phase == BoardPointerPhase::Up { BoardPointerPlanKind::SelectionCommit } else { BoardPointerPlanKind::SelectionPreview { start, start_screen } };
    scene.build_selection_pointer_plan(kind, &next, initial_ids, gesture, &next_points[..len], &next_screen_points[..len])
}

fn plan_link_move_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, start_screen: Option<Point>, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let activated = start_screen.is_none_or(|start_screen| distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX);
    let target_id = activated.then(|| nearest_link_snap_handle_world(scene, style, source_id, world)).flatten();
    let hover_id = target_id.clone().or_else(|| resolve_hover_world(scene, focus, style, world));
    let node_ids = link_drag_compatible_target_node_ids(scene, style, source_id);
    let ring_node_id = activated.then(|| link_drag_ring_target_node_id(scene, style, source_id, world)).flatten().filter(|node_id| link_compatible_handle_count_on_node(scene, style, source_id, node_id) > 1);
    let ring_handle_ids = ring_node_id.as_deref().map(|node_id| link_compatible_handle_ids_on_node(scene, style, source_id, node_id)).unwrap_or_default();
    if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let compat_key = format!("{}|{}", source_id, node_ids.join(","));
    let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let events = [
        (outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str())).then(|| BoardOwnedEvent::link_compatible(source_id, &node_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str())).then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), &ring_handle_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let target = target_id.as_deref().map(|target_id| plan.push_id(target_id)).transpose()?;
    let hover = hover_id.as_deref().map(|hover_id| plan.push_id(hover_id)).transpose()?;
    let compat_key = plan.push_id(&compat_key)?;
    let ring_key = plan.push_id(&ring_key)?;
    plan.kind = BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world: world, activated, start_screen: start_screen.unwrap_or(screen) };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

fn plan_link_finish_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, target_id: Option<&str>, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let direct_target = target_id.filter(|target_id| link_snap_commit_proximity_ok(scene, style, target_id, world));
    let mut target_node_id = None;
    let mut edge_target = direct_target.map(str::to_string);
    let mut also_emit = direct_target.map(|_| BoardEventKind::ProximityConnect);
    if edge_target.is_none() {
        if let Some(node_id) = resolve_node_hit_world(scene, style, world).filter(|node_id| scene.handles.get(source_id).is_some_and(|source| source.node_id.as_str() != node_id.as_str())) {
            if let Some(sole_target) = node_sole_free_link_compatible_handle(scene, style, source_id, &node_id) {
                edge_target = Some(sole_target);
                also_emit = Some(BoardEventKind::IndirectConnect);
            } else if node_has_any_free_link_compatible_handle(scene, style, source_id, &node_id) {
                target_node_id = Some(node_id);
            }
        }
    }
    let edge_id = edge_target.as_deref().and_then(|target| planned_link_edge_id(scene, style, source_id, target));
    if edge_id.is_none() {
        edge_target = None;
        also_emit = None;
    }
    let hover_id = target_node_id.clone().or_else(|| resolve_hover_world(scene, focus, style, world));
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let mut events: [Option<BoardOwnedEvent>; 5] = std::array::from_fn(|_| None);
    let mut compat_key_owned = None;
    let mut ring_key_owned = None;
    if let (Some(edge_id), Some(edge_target)) = (edge_id.as_deref(), edge_target.as_deref()) {
        events[0] = Some(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        if let Some(also_emit) = also_emit {
            events[1] = Some(BoardOwnedEvent::edge(also_emit, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        }
    }
    if let Some(target_node_id) = target_node_id.as_deref() {
        let node_ids = link_drag_compatible_target_node_ids(scene, style, source_id);
        let ring_handle_ids = link_compatible_handle_ids_on_node(scene, style, source_id, target_node_id);
        compat_key_owned = Some(format!("{}|{}", source_id, node_ids.join(",")));
        ring_key_owned = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
        events[2] = Some(BoardOwnedEvent::link_compatible(source_id, &node_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        events[3] = Some(BoardOwnedEvent::link_ring(source_id, Some(target_node_id), &ring_handle_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
    } else {
        let empty: &[String] = &[];
        events[2] = outbox.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
        events[3] = outbox.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    }
    if focus.hovered_id != hover_id || focus.hovered_kind.is_some() {
        events[4] = Some(BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
    }
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let target = edge_target.as_deref().map(|target| plan.push_id(target)).transpose()?;
    let edge = edge_id.as_deref().map(|edge| plan.push_id(edge)).transpose()?;
    let target_node = target_node_id.as_deref().map(|target_node| plan.push_id(target_node)).transpose()?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    let compat_key = compat_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
    let ring_key = ring_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

fn plan_link_retain_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_id = resolve_hover_world(scene, focus, style, world);
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let event = (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
        .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
        .transpose()
        .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkRetain { hover };
    plan.seal_optional_events(&[event])?;
    Ok(plan)
}

fn plan_link_clear_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_id = resolve_hover_world(scene, focus, style, world);
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let empty: &[String] = &[];
    let events = [
        outbox.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        outbox.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkFinish { source, target: None, edge: None, target_node: None, hover, compat_key: None, ring_key: None };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

fn plan_hover_pointer(scene: &BoardSceneState, focus: &BoardFocusState, hover_id: Option<&str>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_kind = hover_id.and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let event = (focus.hovered_id.as_deref() != hover_id || focus.hovered_kind.is_some())
        .then(|| BoardOwnedEvent::hover(hover_id, hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
        .transpose()
        .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let hover = hover_id.map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::Hover { hover };
    plan.seal_optional_events(&[event])?;
    Ok(plan)
}

fn plan_brush_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, brush: &BoardBrushState, intent: BoardPointerIntent, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let next_source = if intent.phase == BoardPointerPhase::Move { brush_nearest_slot_source(scene, style, brush, world) } else { None };
    let offset = if intent.alt || brush.brush_slot_suggestions_active { brush.suggestion_offset } else { 0.0 };
    let next_candidates = if let Some(source_id) = next_source.as_deref() {
        if brush.brush_slot_source_id.as_deref() == Some(source_id) {
            brush.brush_candidates.clone()
        } else {
            let source = scene.handles.get(source_id).ok_or(BoardPointerPlanFault::Unsupported)?;
            brush_compatible_candidates(scene, style, source).ok_or(BoardPointerPlanFault::ItemCredits)?
        }
    } else {
        BrushCandidatePage::default()
    };
    if next_candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let next_index = if brush.brush_slot_source_id == next_source { brush.brush_candidate_index.min(next_candidates.len().saturating_sub(1)) } else { 0 };
    let next_preview = next_source.as_deref().and_then(|source_id| next_candidates.get(next_index).and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
    let leaving_old = brush.brush_slot_source_id.is_some() && brush.brush_slot_source_id != next_source;
    let commit_old = intent.alt && leaving_old && brush.brush_preview.is_some();
    let old_preview = brush.brush_slot_source_id.as_deref().and_then(|source_id| brush.brush_candidates.get(brush.brush_candidate_index).and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
    let hover_id = next_source.clone();
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let serial = brush.brush_placement_serial.wrapping_add(1);
    let node_id = format!("{}{serial}", style.brush_node_prefix());
    let edge_id = format!("{}{serial}", style.brush_edge_prefix());
    let events = [
        commit_old.then(|| old_preview.as_ref().ok_or(BoardEventFault::Schema).and_then(|preview| BoardOwnedEvent::brush_place(preview, &node_id, &edge_id))).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        Some(BoardOwnedEvent::brush_preview(next_preview.as_ref()).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
        Some(BoardOwnedEvent::brush_candidates(next_source.as_deref().unwrap_or(""), &next_candidates, next_index, false).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = next_source.as_deref().map(|source| plan.push_id(source)).transpose()?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    for candidate in next_candidates.iter() {
        plan.push_delta(candidate.node_kind_id, candidate.target_handle_index as f64, 0.0)?;
    }
    plan.kind = BoardPointerPlanKind::Brush { source, hover, alt: intent.alt, commit_old };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

/// 🧲️ A drag frame (`DragMove`) or its release (`FinishDrag`) as a retained plan: every member's resolved
/// position, and for the release the staged `select` plus the ONE gesture record, sealed as one page.
fn plan_drag_pointer(scene: &BoardSceneState, style: &BoardStyleState, world: Point, drag: (&str, Vec2, &BTreeMap<String, (f64, f64)>), release: Option<(&GestureStage, Option<&(String, String)>)>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let (primary_id, offset, start_positions) = drag;
    if start_positions.len() > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let delta = drag_delta(scene, style, world, primary_id, offset, start_positions).ok_or(BoardPointerPlanFault::Unsupported)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, if release.is_some() { BoardPointerPlanKind::FinishDrag } else { BoardPointerPlanKind::DragMove });
    for (id, (x, y)) in start_positions {
        plan.push_delta(id, x + delta.x, y + delta.y)?;
    }
    if let Some((gesture, proximity_pair)) = release {
        let [select, record] = gesture_release_events(gesture, drag_gesture_record(gesture, start_positions, delta, proximity_pair));
        plan.seal_optional_events(&[select.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?, record.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?])?;
    }
    Ok(plan)
}

pub fn begin_pointer_commit(scene: &mut BoardSceneState, commit: &mut BoardCommitState, plan: BoardPointerPlan) -> Result<(), BoardPointerPlan> {
    if commit.pointer_publication.is_some() || scene.interaction_revision != plan.revision || !plan.requires_retained_commit() {
        return Err(plan);
    }
    if let Some(active) = commit.pending_pointer_commit.as_ref() {
        if active.plan.revision != plan.revision || !matches!(active.plan.kind, BoardPointerPlanKind::DragMove) || !matches!(plan.kind, BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag) {
            return Err(plan);
        }
        match commit.queued_pointer_commit.as_ref().map(|queued| queued.kind) {
            None => {
                commit.queued_pointer_commit = Some(plan);
                return Ok(());
            }
            Some(BoardPointerPlanKind::DragMove) => {
                commit.queued_pointer_commit = Some(plan);
                return Ok(());
            }
            Some(_) => return Err(plan),
        }
    }
    commit.pending_pointer_commit = Some(BoardPointerCommitOperation::new(plan));
    Ok(())
}

pub fn begin_pointer_cancel(scene: &mut BoardSceneState, commit: &mut BoardCommitState) {
    let revision = scene.interaction_revision.wrapping_add(1);
    scene.interaction_revision = revision;
    if let Some(operation) = commit.pending_pointer_commit.as_mut() {
        operation.cancelling = true;
        operation.plan.revision = revision;
    }
    if commit.cancelled_pointer_plan_retirement.is_none() {
        commit.cancelled_pointer_plan_retirement = commit.queued_pointer_commit.take().map(BoardPointerPlanRetirement::new);
    }
}
