use super::*;
use crate::viewer::generation3d::config::Generation3dViewCamera;

fn default_document() -> Generation3dSnapshot {
    crate::standards::v1::subsets::any::schema::default_snapshot()
}

#[test]
fn definition_declares_a_world3d_surface_so_picking_can_resolve() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
}

#[test]
fn render_emits_real_tessellated_geometry_for_the_default_fixture() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    let eval = evaluate_fixture(&document.fixture);
    let payload = preview_payload(&eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    assert_ne!(payload.meshes_json, "[]", "the default fixture must evaluate and tessellate at least one preview mesh");
    assert_ne!(payload.instances_json, "[]");
    document.retire_cold();
}

/// 🕹️ Every instance must carry the channel-qualified `interactionId` the framework's declared
/// `handle` topology names — without it, a world pick resolves to an id `validate_state` prunes.
#[test]
fn every_instance_carries_a_channel_qualified_interaction_id() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    let eval = evaluate_fixture(&document.fixture);
    let payload = preview_payload(&eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    let instances = dsl::json::parse(&payload.instances_json).expect("instances json");
    let instances = instances.as_array().expect("instances array").clone();
    assert!(!instances.is_empty());
    for instance in &instances {
        let interaction_id = instance.get("interactionId").and_then(Value::as_str).expect("every instance declares an interactionId");
        assert!(interaction_id.contains('@'), "an interaction id must be channel-qualified: {interaction_id}");
        assert_eq!(instance.get("selected").and_then(Value::as_bool), Some(false));
        assert_eq!(instance.get("hovered").and_then(Value::as_bool), Some(false));
    }
    document.retire_cold();
}

/// 🕹️ The three-level match: naming the bare widget id marks every one of its channels' instances.
#[test]
fn marks_paint_hover_and_selection_from_a_bare_widget_id() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    let eval = evaluate_fixture(&document.fixture);
    let bare = preview_payload(&eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    let instances = dsl::json::parse(&bare.instances_json).expect("instances json");
    let first = instances.as_array().expect("instances array").first().cloned().expect("at least one instance");
    let interaction_id = first.get("interactionId").and_then(Value::as_str).expect("interactionId").to_string();
    let widget_id = interaction_id.split('@').next().expect("widget id").to_string();

    let marks = Generation3dViewMarks { hovered: std::iter::once(widget_id.clone()).collect(), selected: std::iter::once(widget_id).collect() };
    let marked = preview_payload(&eval, &document.fixture, &config, None, &marks);
    let marked_instances = dsl::json::parse(&marked.instances_json).expect("instances json");
    let marked_instances = marked_instances.as_array().expect("instances array").clone();
    let any_marked = marked_instances.iter().any(|instance| instance.get("selected").and_then(Value::as_bool) == Some(true) && instance.get("hovered").and_then(Value::as_bool) == Some(true));
    assert!(any_marked, "a bare widget id must mark every instance of that widget");
    assert!(!marked.selected_ids.is_empty(), "a marked instance must reach the selection payload");
    assert!(marked.hovered_id.is_some(), "a marked instance must reach the hover payload");
    document.retire_cold();
}

/// 👁️ The show mode really changes the payload, not just a flag: wireframe drops the triangle
/// channels and `shaded+edges` turns the scene's edge flag on.
#[test]
fn show_mode_changes_the_emitted_mesh_channels_and_edge_flag() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let eval = evaluate_fixture(&document.fixture);
    let shaded = Generation3dViewConfig::default();
    let wireframe = Generation3dViewConfig { show_mode: "wireframe".into(), ..Generation3dViewConfig::default() };
    let shaded_payload = preview_payload(&eval, &document.fixture, &shaded, None, &Generation3dViewMarks::default());
    let wireframe_payload = preview_payload(&eval, &document.fixture, &wireframe, None, &Generation3dViewMarks::default());
    assert_ne!(shaded_payload.meshes_json, wireframe_payload.meshes_json, "wireframe must drop the shaded triangle channels");
    assert!(preview_selection_json(&shaded, &shaded_payload).contains("\"showEdges\":false") || !preview_selection_json(&shaded, &shaded_payload).contains("\"showEdges\":true"));
    assert!(preview_selection_json(&wireframe, &wireframe_payload).contains("\"showEdges\":true"));
    document.retire_cold();
}

/// 🔒️ A viewer never mounts a gumball: a transform handle is a mutation affordance.
#[test]
fn the_selection_payload_never_arms_a_gumball() {
    let config = Generation3dViewConfig::default();
    let payload = ViewPreviewPayload { selected_ids: vec!["a@out#0".into()], ..ViewPreviewPayload::default() };
    let selection = preview_selection_json(&config, &payload);
    assert!(selection.contains("\"gumballActive\":false"), "a read-only surface must never arm a gumball: {selection}");
}

/// 📷️ The camera comes from the viewer's own config, so an orbit really moves the read-only view.
#[test]
fn render_uses_the_configured_preview_camera() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig { preview_camera: Generation3dViewCamera { position: [9.0, 8.0, 7.0], target: [1.0, 1.0, 1.0], fov: 33.0 }, ..Generation3dViewConfig::default() };
    let node = render(&document, &config, None, None, &Generation3dViewMarks::default()).expect("render");
    // 🚚️ `cameraJson` is an out-of-doc scene LANE since the paged-scene wave (26/09/02 P) and the
    // spine itself rides the surface `doc` as pack BYTES, so a `{node:?}` substring scan reads
    // neither — the assembled scene has to be decoded back out of the built surface node.
    let scene: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("viewer preview body must decode as an assembled world-3d scene");
    assert!(scene.camera_json.contains("\"fov\":33"), "the configured fov must reach the rendered scene: {}", scene.camera_json);
    document.retire_cold();
}

/// 🎚️ The window chrome must bind to this viewer's own actions, or the controls are decoration.
#[test]
fn the_window_chrome_binds_show_mode_lod_and_sun_to_viewer_actions() {
    let config = Generation3dViewConfig::default();
    let measures = preview_window_measures(&config, crate::viewer::generation3d::generation3d_view_action);
    assert_eq!(measures.len(), 3, "show mode, LOD and the sun group");
    let rendered = format!("{measures:?}");
    for action in ["setShowMode", "setLodMode", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"] {
        assert!(rendered.contains(action), "the preview chrome must dispatch {action}");
    }
}

/// 🧵️ A supplied evaluation is reused verbatim: the ephemeral transient really replaces the
/// per-repaint whole-fixture evaluation.
#[test]
fn a_supplied_evaluation_is_used_instead_of_re_evaluating() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    let empty_eval = "{}";
    let payload = preview_payload(empty_eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    assert_eq!(payload.meshes_json, "[]", "an empty supplied evaluation must yield an empty payload, proving it was not recomputed");
    document.retire_cold();
}

/// 🧊️ LAW: a hover-only or camera-only re-render of an UNCHANGED evaluation runs ZERO tessellations.
/// The mesh table is a pure function of (evaluation, deflection, shading mode, preview widgets) and
/// none of those move when the pointer does, so the second render must reuse the built table and the
/// third — which only changes the marks — must too (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_hover_only_re_render_tessellates_nothing() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    let eval = evaluate_fixture(&document.fixture);
    reset_preview_mesh_table();
    let cold = preview_payload(&eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    let after_cold = preview_tessellation_count();
    assert!(after_cold > 0, "the first render of a fresh document must actually tessellate");

    let warm = preview_payload(&eval, &document.fixture, &config, None, &Generation3dViewMarks::default());
    assert_eq!(preview_tessellation_count(), after_cold, "an identical re-render must not tessellate again");
    assert_eq!(warm.meshes_json, cold.meshes_json, "the reused mesh table must be byte-identical");

    let instances = dsl::json::parse(&cold.instances_json).expect("instances json");
    let first = instances.as_array().expect("instances array").first().cloned().expect("at least one instance");
    let interaction_id = first.get("interactionId").and_then(Value::as_str).expect("interactionId").to_string();
    let hovered_marks = Generation3dViewMarks { hovered: [interaction_id].into_iter().collect(), selected: std::collections::BTreeSet::new() };
    let hovered = preview_payload(&eval, &document.fixture, &config, None, &hovered_marks);
    assert_eq!(preview_tessellation_count(), after_cold, "moving the pointer must not tessellate anything");
    assert_eq!(hovered.meshes_json, cold.meshes_json, "hover changes instances, never meshes");
    assert!(hovered.hovered_id.is_some(), "the hovered instance must still be marked");
    document.retire_cold();
}

/// 🚦️ LAW: `render` NEVER evaluates. Without a published evaluation the window paints its empty
/// world and lets the `flowEvalTick` chain drive it — the removed fallback ran a whole synchronous
/// `FlowHost::evaluate` inside a pure render, racing the real chain on every repaint.
#[test]
fn render_without_a_published_evaluation_paints_the_empty_world() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let document = default_document();
    let config = Generation3dViewConfig::default();
    reset_preview_mesh_table();
    let before = preview_tessellation_count();
    let node = render(&document, &config, None, None, &Generation3dViewMarks::default()).expect("render");
    let scene: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("viewer preview body must decode as an assembled world-3d scene");
    assert_eq!(scene.meshes_json, "[]", "an unevaluated document must paint no meshes, not evaluate itself");
    assert_eq!(preview_tessellation_count(), before, "a pure render must never reach the geometry kernel");
    document.retire_cold();
}
