use crate::editor::puzzle2d::engine::board_host::testkit::*;
use crate::editor::puzzle2d::engine::canvas;
use crate::editor::puzzle2d::engine::canvas::geom_sel::cubic_bezier_point;
use crate::editor::puzzle2d::engine::canvas::Point;
use crate::editor::puzzle2d::engine::{compute_edge_bezier_points, handle_position_on_circle, BoardElementStyleKind, BoardHost, Interaction, NodeDescriptor, WireDescriptor};
use semio_framework_pack_json::json;
use crate::board_set_camera;
use semio_framework_test_fixtures::fixture_text;

/// 🔗️ Keeps the runtime kind-catalog JSON shape in sync with the compile-time `puzzle2d-default` manifest.
#[test]
fn puzzle2d_default_manifest_satisfies_board_host_validation() {
    let manifest: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(fixture_text!("../../../../../../../../../🛂️manifest.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let handle_kinds: Vec<semio_framework_pack_json::Value> =
        manifest["portKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "color": row["presentation"]["color"], "defaultWireKind": row["presentation"]["defaultWireKind"] })).collect();
    let wire_kinds: Vec<semio_framework_pack_json::Value> = manifest["wireKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "defaultEdgeKind": row["presentation"]["defaultEdgeKind"] })).collect();
    let edge_kinds: Vec<semio_framework_pack_json::Value> = manifest["edgeKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"] })).collect();
    let catalogs_json = json!({ "handleKinds": handle_kinds, "wireKinds": wire_kinds, "edgeKinds": edge_kinds }).to_string();

    let mut host = BoardHost::new();
    host.style.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");
    host.style.validate_against_manifest(&crate::standards::v1::subsets::any::schema::graph_manifest::puzzle2d_default::puzzle2d_default_manifest()).expect("runtime catalog must satisfy the compile-time puzzle2d-default manifest");
}

#[test]
fn board_host_defers_descriptor_sync_while_panning() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let _ = h.outbox.drain_events_json();
    h.pointer_down_screen(10.0, 10.0, 1, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    h.pointer_move_screen(80.0, 60.0, false, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let _ = h.outbox.drain_events_json();
    h.pointer_up_screen(80.0, 60.0, false, false, false);
    assert!(!defers_descriptor_sync_from_js(&h.focus, &h.gestures));
}

#[test]
fn board_host_defers_descriptor_sync_while_dragging_nodes() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let _ = h.outbox.drain_events_json();
    let start = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(start.x, start.y, 0, false, false);
    assert!(matches!(h.focus.interaction, Interaction::DragNodes { .. }));
    h.pointer_move_screen(start.x + 40.0, start.y, false, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"));
    h.pointer_up_screen(start.x + 40.0, start.y, false, false, false);
    assert!(!defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let end: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.outbox.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");
    let record = end.iter().find(|row| row["name"] == "gesture").expect("the release publishes one gesture record");
    let reached = h.scene.screen_to_world(Point::new(start.x + 40.0, start.y));
    let (dx, dy) = (record["payload"]["dx"].as_f64().expect("dx"), record["payload"]["dy"].as_f64().expect("dy"));
    assert_eq!(record["payload"]["kind"], "drag");
    assert!((dx - reached.x).abs() < 1e-6 && (dy - reached.y).abs() < 1e-6, "the record carries the grab-to-release offset: ({dx}, {dy}) vs {reached:?}");
}

#[test]
fn board_host_set_node_positions_updates_existing_nodes_only() {
    let mut h = BoardHost::new();
    h.scene.set_size(400, 300, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    h.scene.set_node_positions(&[("a".into(), 12.0, 34.0), ("missing".into(), 1.0, 2.0), ("a".into(), f64::NAN, 0.0)]);
    let node = h.scene.nodes.get("a").expect("node a should remain");
    assert!((node.x - 12.0).abs() < 0.001);
    assert!((node.y - 34.0).abs() < 0.001);
    assert!(h.scene.test_content_scene_generation() > gen_before, "moving nodes must invalidate cached world content");
    h.scene.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();
    let node = h.scene.nodes.get("a").expect("node a should remain");
    assert!((node.x - 90.0).abs() < 0.001);
    assert!((node.y - 110.0).abs() < 0.001);
}

#[test]
fn board_host_overlay_paint_state_json_matches_host_camera_lod_and_node_centers() {
    let mut h = BoardHost::new();
    h.scene.set_size(640, 480, 2.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_camera_silent(&mut h.scene, &mut h.outbox, 12.0, -8.0, 0.2);
    if let Some(n) = h.scene.nodes.get_mut("a") {
        n.x = 33.0;
        n.y = 44.0;
    }
    let raw: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&overlay_paint_state_json(&h.scene, &h.style), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("overlay paint state json");
    assert!((raw["camera"]["x"].as_f64().unwrap() - 12.0).abs() < 1e-9);
    assert!((raw["camera"]["y"].as_f64().unwrap() - (-8.0)).abs() < 1e-9);
    assert!((raw["camera"]["zoom"].as_f64().unwrap() - 0.2).abs() < 1e-9);
    assert_eq!(raw["lod"].as_str(), Some("overview"));
    let nodes = raw["nodes"].as_array().expect("nodes array");
    let a = nodes.iter().find(|row| row["id"].as_str() == Some("a")).expect("node a row");
    assert!((a["x"].as_f64().unwrap() - 33.0).abs() < 1e-9);
    assert!((a["y"].as_f64().unwrap() - 44.0).abs() < 1e-9);
}

#[test]
fn board_host_node_drag_invalidates_cached_world_content() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 80.0, s.y + 40.0, false, false, false);
    assert!(h.scene.test_content_scene_generation() > gen_before, "node drag must rebuild cached nodes/handles, not only edges");
    let node = h.scene.nodes.get("a").expect("dragged node");
    assert!(node.x.abs() > 1.0 || node.y.abs() > 1.0, "pointer move should translate node a away from origin");
}

#[test]
fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let with_automatic = h.encoded_scene_hint();
    assert!(with_automatic > 0, "sample scene should encode vector paths");
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("");
    let manual_follow_zoom = h.encoded_scene_hint();
    assert!(manual_follow_zoom > 0, "manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})");
    h.style.set_forced_draw_lod_label("overview");
    let pinned_overview = h.encoded_scene_hint();
    assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
    h.style.set_automatic_lod(true);
    let automatic_restored = h.encoded_scene_hint();
    assert_eq!(with_automatic, automatic_restored);
}

#[test]
fn board_host_pick_selection_never_sets_exit_highlight() {
    let mut h = BoardHost::new();
    h.scene.set_size(400, 300, 1.0);
    let mut d = sample_scene();
    d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
    h.sync_descriptor(&d).unwrap();
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let ev = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(ev.contains("\"exitHighlightIds\":[]"));
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["e1".into()]);
    let ev2 = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(ev2.contains("\"exitHighlightIds\":[]"));
}

#[test]
fn board_host_canvas_theme_keeps_explicit_element_state_colors() {
    let mut h = BoardHost::new();
    h.style.set_canvas_palette(&BoardPaletteOverlay {
        node_stroke_hovered:Some([1,2,3,255]),edge_stroke_hovered:Some([4,5,6,255]),
        handle_stroke_hovered:Some([7,8,9,255]),wire_stroke_hovered:Some([10,11,12,255]),
        ..BoardPaletteOverlay::default()
    });
    assert_eq!(h.style.canvas_theme.node_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.edge_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.handle_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.wire_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
}

#[test]
fn board_host_cancel_area_select_restores_initial_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
    let _ = h.outbox.drain_events_json();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    h.pointer_down_screen(5.0, 5.0, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    h.pointer_move_screen(20.0, 5.0, false, false, false);
    assert!(h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    assert!(h.cancel_area_select());
    assert!(!h.focus.is_dragging_area_select());
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("preselectCancel"));
    assert!(!ev.contains("\"select\""));
    assert_eq!(h.focus.selection.len(), 2);
    assert!(h.focus.selection.contains("a") && h.focus.selection.contains("b"));
    assert!(h.focus.preselect.is_empty());
}

#[test]
fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let hit = resolve_hit_world(&h.scene, &h.focus, &h.style, hp);
    assert_eq!(hit.as_deref(), Some("a:h0"));
    assert!(h.encoded_scene_hint() > 10);
}

#[test]
fn board_host_cached_content_includes_edge_vector_paths_at_overview_zoom() {
    let mut h = BoardHost::new();
    h.scene.set_size(1200, 800, 1.0);
    h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
    set_camera_silent(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.21);
    let with_edges = h.encoded_scene_hint();
    let without = link_test_scene_no_edge();
    h.sync_descriptor(&without).unwrap();
    let without_edges = h.encoded_scene_hint();
    assert!(with_edges > without_edges, "overview cached draw must encode edges (with={with_edges}, without={without_edges})");
}

#[test]
fn board_host_world_clip_changes_vector_encoding() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 600.0,
        y: 400.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    h.style.set_world_raster_tiling("none");
    let monolithic = h.encoded_scene_hint();
    h.style.set_world_raster_tiling("world-clip");
    let tiled = h.encoded_scene_hint();
    assert!(tiled >= monolithic);
}

#[test]
fn board_host_silent_selection_keeps_cached_world_content_warm() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    assert_eq!(h.scene.test_content_scene_generation(), gen_before, "selection chrome must paint via dynamic fill/stroke layers without rebuilding cached icons");
    assert_ne!(h.encoded_scene_hint(), neutral_hint, "selected node fill appears in overlay fill layer at normal LOD");
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected));
}

#[test]
fn board_host_selected_node_keeps_selected_style_when_hovered() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected), "committed selection chrome should beat hover while pointer is over the node");
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Hovered), "unselected nodes should still use hover chrome");
}

#[test]
fn board_host_dragging_selected_node_keeps_selected_style_at_detail_lod() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    assert!(matches!(h.focus.interaction, Interaction::DragNodes { .. }));
    assert_eq!(h.focus.hovered_id.as_deref(), Some("a"));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected), "node drag should keep primary selected paint at detail LOD");
}

#[test]
fn board_host_drag_emits_node_move() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let _ = h.outbox.drain_events_json();
    let w = Point::new(0.0, 0.0);
    let s = h.scene.world_to_screen(w);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"));
}

#[test]
fn board_host_compact_discrete_hit_selects_and_drags_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.5);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let _ = h.outbox.drain_events_json();
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, Point::new(0.0, 0.0)).as_deref(), Some("a"));
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, Point::new(150.0, 0.0)).is_none());
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
}

#[test]
fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("minimap");
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let gap = Point::new(150.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, gap).is_none());
    let s = h.scene.world_to_screen(gap);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
    let zoom = 0.1;
    let dx = 50.0 / zoom;
    let dy = 30.0 / zoom;
    let a = h.scene.nodes.get("a").unwrap();
    let b = h.scene.nodes.get("b").unwrap();
    assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
    assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
}

#[test]
fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("overview");
    set_overview_lod(&mut h);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let gap = Point::new(150.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, gap).is_none());
    let s = h.scene.world_to_screen(gap);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false, false);
    h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
}

#[test]
fn board_host_detail_lod_resolves_direct_handle_hit() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let probe = Point::new(hp.x + 2.0, hp.y);
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, probe).as_deref(), Some("a:h0"));
}

#[test]
fn board_host_multi_select_drag_moves_each_selected_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 100.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    h.focus.set_selection_options("rectangle", "additive", true, true, true);
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let w = Point::new(0.0, 0.0);
    let s = h.scene.world_to_screen(w);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false, false);
    h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false, false);
    let a = h.scene.nodes.get("a").expect("node a");
    let b = h.scene.nodes.get("b").expect("node b");
    assert!((a.x - 10.0).abs() < 1e-6);
    assert!((a.y - 5.0).abs() < 1e-6);
    assert!((b.x - 110.0).abs() < 1e-6);
    assert!((b.y - 5.0).abs() < 1e-6);
    let sorted: Vec<_> = h.focus.selection.iter().cloned().collect();
    assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn board_host_selection_target_edges_skips_node_geometry() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "invertive", false, true, false);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let inside_node_a = Point::new(0.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, inside_node_a).is_none());
    let on_edge = Point::new(150.0, 0.0);
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, on_edge).as_deref(), Some("e1"));
}

#[test]
fn board_host_additive_click_merges_edge_into_existing_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "additive", true, true, true);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    let on_edge = Point::new(150.0, 0.0);
    let s = h.scene.world_to_screen(on_edge);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    let mut got: Vec<_> = h.focus.selection.iter().cloned().collect();
    got.sort();
    assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
}

#[test]
fn board_host_selection_change_does_not_bump_content_scene_generation() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    let selected_hint = h.encoded_scene_hint();
    assert_ne!(selected_hint, neutral_hint);
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    assert_eq!(h.encoded_scene_hint(), neutral_hint);
}

#[test]
fn board_host_hover_change_does_not_bump_content_scene_generation() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(h.scene.test_content_scene_generation(), gen, "hover must paint via dynamic overlay chrome without rebuilding cached icons");
    let hovered_hint = h.encoded_scene_hint();
    assert_ne!(hovered_hint, neutral_hint);
    h.focus.set_hovered_id_silent(None);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    assert_eq!(h.encoded_scene_hint(), neutral_hint);
}

#[test]
fn board_host_background_click_deselect_skips_preselect_events() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let desc = sample_scene();
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let _ = h.outbox.drain_events_json();
    let away = Point::new(5000.0, 5000.0);
    let s = h.scene.world_to_screen(away);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    h.pointer_move_screen(s.x + 1.0, s.y, false, false, false);
    let mid = h.outbox.drain_events_json();
    assert!(!mid.contains("preselect"), "background click path must not emit preselect");
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(h.focus.selection.contains("a"));
    h.pointer_up_screen(s.x, s.y, false, false, false);
    assert!(h.focus.selection.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    let fin = h.outbox.drain_events_json();
    assert!(fin.contains("select"));
    assert!(!fin.contains("preselect"));
    assert!(fin.contains("\"exitHighlightIds\":[]"));
}

#[test]
fn board_host_background_click_without_drag_clears_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let away = Point::new(5000.0, 5000.0);
    let s = h.scene.world_to_screen(away);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_up_screen(s.x, s.y, false, false, false);
    assert!(h.focus.selection.is_empty());
}

#[test]
fn board_host_rectangle_area_select_includes_handles_with_nodes() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "invertive", true, true, true);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let _ = h.outbox.drain_events_json();
    let w0 = Point::new(-90.0, -70.0);
    let w1 = Point::new(90.0, 90.0);
    let s0 = h.scene.world_to_screen(w0);
    let s1 = h.scene.world_to_screen(w1);
    h.pointer_down_screen(s0.x, s0.y, 0, false, false);
    h.pointer_move_screen(s1.x, s1.y, false, false, false);
    h.pointer_up_screen(s1.x, s1.y, false, false, false);
    let mut got: Vec<_> = h.focus.selection.iter().cloned().collect();
    got.sort();
    assert!(got.contains(&"a".to_string()));
    assert!(got.contains(&"a:h0".to_string()));
}

#[test]
fn board_host_area_select_preselect_matches_selected_chrome() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let _ = h.outbox.drain_events_json();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    let w_down = Point::new(350.0, -50.0);
    let w_mid = Point::new(270.0, 50.0);
    let w_end = Point::new(265.0, 48.0);
    let s_down = h.scene.world_to_screen(w_down);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    let s_mid = h.scene.world_to_screen(w_mid);
    let s_end = h.scene.world_to_screen(w_end);
    h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
    assert!(h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.preselect.contains("b"), "preview should include node b");
    assert!(h.focus.preselect_removed.contains("a"));
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(!h.focus.selection.contains("b"), "committed selection unchanged during preselect");
    let frozen = h.focus.preselect_removed.clone();
    h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert_eq!(frozen, h.focus.preselect_removed);
    h.pointer_up_screen(s_end.x, s_end.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection.contains("b"));
    assert!(!h.focus.selection.contains("a"));
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
}

#[test]
fn board_host_area_select_from_empty_keeps_selection_until_commit() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    let _ = h.outbox.drain_events_json();
    let w_down = Point::new(350.0, -50.0);
    let w_mid = Point::new(270.0, 50.0);
    let s_down = h.scene.world_to_screen(w_down);
    let s_mid = h.scene.world_to_screen(w_mid);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.is_dragging_area_select());
    assert!(h.focus.preselect.contains("b"));
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection.is_empty());
    h.pointer_up_screen(s_mid.x, s_mid.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection.contains("b"));
    assert!(h.focus.preselect.is_empty());
}

#[test]
fn board_host_minimap_pointer_move_hovers_node_under_cursor() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    h.style.set_forced_draw_lod_label("minimap");
    h.sync_descriptor(&sample_scene()).unwrap();
    let center = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_move_screen(center.x, center.y, false, false, false);
    assert_eq!(h.focus.hovered_id.as_deref(), Some("a"));
    let away = h.scene.world_to_screen(Point::new(5000.0, 5000.0));
    h.pointer_move_screen(away.x, away.y, false, false, false);
    assert!(h.focus.hovered_id.is_none());
}

#[test]
fn board_host_minimap_preselect_matches_selected_chrome() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["b".into()]);
    let _ = h.outbox.drain_events_json();
    let selected_hint = h.encoded_scene_hint();
    assert!(selected_hint > neutral_hint, "minimap selected chrome should add visible vector encoding over neutral state");
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    let w_down = Point::new(350.0, -50.0);
    let w_end = Point::new(265.0, 48.0);
    let s_down = h.scene.world_to_screen(w_down);
    let s_end = h.scene.world_to_screen(w_end);
    set_transform_flags(&mut h.scene, &mut h.gestures, true, false);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
    assert!(h.focus.is_dragging_area_select(), "with the rotate ring off, a press beside the selection opens the marquee");
    assert!(h.focus.preselect.contains("b"));
    h.focus.set_selection_screen_preview(None);
    let preselect_hint = h.encoded_scene_hint();
    assert!(preselect_hint > neutral_hint, "minimap preselect should add visible selected chrome over neutral minimap rendering");
}

#[test]
fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let neutral_hint = h.encoded_scene_hint();
    assert!(!matches!(h.focus.interaction, Interaction::Selection { .. }));
    set_preselect_state_silent(&mut h.scene, &mut h.focus, &["b".into()], &[]);
    assert!(h.scene.nodes.get("b").is_some_and(|n| n.selected));
    assert!(h.scene.nodes.get("a").is_some_and(|n| !n.selected));
    let preselect_hint = h.encoded_scene_hint();
    assert!(preselect_hint > neutral_hint, "silent minimap preselect should paint selected chrome without an active area-select interaction");
}

#[test]
fn board_host_hover_tracks_visible_wires() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.edges.clear();
    desc.wires.push(WireDescriptor { id: "w1".into(), source: "a:h0".into(), target: None, end_x: Some(220.0), end_y: Some(0.0), selected: None, style: None, wire_kind: None, user_data: None, visible: None, locked: None });
    h.sync_descriptor(&desc).unwrap();
    let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
    let probe = cubic_bezier_point(curve, 0.5);
    h.update_hover_from_world(probe);
    assert_eq!(h.focus.hovered_id.as_deref(), Some("w1"));
}

/// 🎫️ The puzzle board naming is the plugin's wire vocabulary, byte for byte: every fill stage keeps its
/// `puzzle2d-fill-*` progress label, placements keep their `puzzle2d.brush.` / `puzzle2d.fill.` prefixes, and both
/// board constructors install it.
#[test]
fn puzzle_board_naming_declares_the_plugin_wire_vocabulary() {
    use crate::editor::puzzle2d::engine::board_host::{puzzle_board_host, puzzle_board_host_normal, PUZZLE_2D_BOARD_NAMING};
    use crate::editor::puzzle2d::engine::BoardFillStage;
    let labels = [
        (BoardFillStage::ResetSources, "puzzle2d-fill-reset-sources"),
        (BoardFillStage::PrepareSources, "puzzle2d-fill-prepare-sources"),
        (BoardFillStage::SelectTarget, "puzzle2d-fill-select-target"),
        (BoardFillStage::ResetCandidates, "puzzle2d-fill-reset-candidates"),
        (BoardFillStage::PrepareCandidates, "puzzle2d-fill-prepare-candidates"),
        (BoardFillStage::ScanCompatibility, "puzzle2d-fill-scan-compatibility"),
        (BoardFillStage::SelectCandidate, "puzzle2d-fill-select-candidate"),
        (BoardFillStage::ConstructPreview, "puzzle2d-fill-construct-preview"),
        (BoardFillStage::ScanHostCollision, "puzzle2d-fill-scan-host-collision"),
        (BoardFillStage::ScanVirtualCollision, "puzzle2d-fill-scan-virtual-collision"),
        (BoardFillStage::AcceptCandidate, "puzzle2d-fill-accept-candidate"),
        (BoardFillStage::AcceptNodeId, "puzzle2d-fill-accept-node-id"),
        (BoardFillStage::AcceptEdgeId, "puzzle2d-fill-accept-edge-id"),
        (BoardFillStage::AcceptEdgeKind, "puzzle2d-fill-accept-edge-kind"),
        (BoardFillStage::AcceptNodeKind, "puzzle2d-fill-accept-node-kind"),
        (BoardFillStage::AcceptSourceHandle, "puzzle2d-fill-accept-source-handle"),
        (BoardFillStage::AcceptTargetHandle, "puzzle2d-fill-accept-target-handle"),
        (BoardFillStage::AcceptIcon, "puzzle2d-fill-accept-icon"),
        (BoardFillStage::AcceptVirtualNode, "puzzle2d-fill-accept-virtual-node"),
        (BoardFillStage::AcceptSourceConnection, "puzzle2d-fill-accept-source-connection"),
        (BoardFillStage::AcceptHandles, "puzzle2d-fill-accept-handles"),
        (BoardFillStage::AcceptHandleId, "puzzle2d-fill-accept-handle-id"),
        (BoardFillStage::AcceptHandleVirtual, "puzzle2d-fill-accept-handle-virtual"),
        (BoardFillStage::AcceptHandlePublish, "puzzle2d-fill-accept-handle-publish"),
        (BoardFillStage::PublishPlanPrefix, "puzzle2d-fill-publish-prefix"),
        (BoardFillStage::Complete, "puzzle2d-fill-complete"),
    ];
    for (stage, label) in labels {
        assert_eq!((PUZZLE_2D_BOARD_NAMING.fill_stage_label)(stage), label);
    }
    assert_eq!(
        (PUZZLE_2D_BOARD_NAMING.brush_node_prefix, PUZZLE_2D_BOARD_NAMING.brush_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_node_prefix, PUZZLE_2D_BOARD_NAMING.fill_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_stale_operation_fault),
        ("puzzle2d.brush.", "puzzle2d.brush.edge.", "puzzle2d.fill.", "puzzle2d.fill.edge.", "stale-puzzle2d-fill-operation")
    );
    assert_eq!(PUZZLE_2D_BOARD_NAMING.placement_naming().to_json(), r#"{"brushNodePrefix":"puzzle2d.brush.","brushEdgePrefix":"puzzle2d.brush.edge."}"#, "the scene wire form other hosts mint with");
    assert!(std::ptr::eq(puzzle_board_host().style.naming(), &PUZZLE_2D_BOARD_NAMING));
    assert!(std::ptr::eq(puzzle_board_host_normal().style.naming(), &PUZZLE_2D_BOARD_NAMING));
}
