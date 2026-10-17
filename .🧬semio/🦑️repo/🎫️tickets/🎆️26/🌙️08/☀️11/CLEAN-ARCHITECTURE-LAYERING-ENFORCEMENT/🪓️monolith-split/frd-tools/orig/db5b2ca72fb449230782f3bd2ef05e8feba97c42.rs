//! 🧪️ host tests (🧪️unit-3): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, World3dBuildContext, World3dScalarFieldStatus, WorldCursorWakeAuthority, WorldDrawRebuildStep};
use semio_framework_os_infinite_world_components::{TerrainTileMeshPayload, WorldMeshBuffers};
use semio_framework_os_infinite_world_scene::{step_world3d_terrain_retirement, WorldTerrainMeshPublicationStep};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::World3dShadowProfile;
use semio_framework_os_infinite_world_bridge::{apply_world3d_pick_filter, begin_world3d_dynamic_retirement, step_world3d_scene_bridge_close, sync_world3d_scene_selection, world3d_draw_rebuild_terminal_is_empty, World3dSceneBridgeCursor, World3dSceneBridgePhase, World3dSceneBridgeStep, World3dSceneMeshEntry, World3dSnapshotApplyStep};
use semio_framework_os_infinite_world_interaction::step_world3d_draw_rebuild;
use semio_framework_os_infinite_world_host::{apply_runtime_draw_flags, step_world3d_dynamic_retirement, step_world3d_scene_bridge, step_world3d_snapshot, sync_world3d_state, world3d_dynamic_retirement_terminal_is_empty, world3d_modelling_accessibility, World3dState};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_host::render_world_3d;
use ui_wgpu::wgpu::{ActionDescriptor, Rect, Rgba, SceneDraw3d, SurfaceKind, Theme, UiComponentSceneNode, UiPresence, Vec3, World3dScene};
use std::collections::HashMap;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{ScenePass3d, SceneSectionRole3d};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
    scene_with_selection_and_domain(selection_json, None)
}

/// 🪟️ Like `scene_with_selection`, but also stamps `World3dScene.domain_id`/`domain_granularity_id`
/// when `domain` is `Some((domain_id, granularity_id))` — exercises the app-bound-domain path of
/// `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id`.
fn scene_with_selection_and_domain(selection_json: &str, domain: Option<(&str, &str)>) -> UiComponentSceneNode {
    UiComponentSceneNode {
        presence: UiPresence::default(),
        host_id: "surface-1".into(),
        surface_id: "surface-1".into(),
        controller_id: "controller-1".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(World3dScene {
            snapshot: None,
            camera_json: r#"{"position":[4.0,4.0,4.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#.into(),
            meshes_json: r#"[{"id":"mesh-1","data":{"positions":[0,0,0,1,0,0,0,1,0],"normals":[0,0,1,0,0,1,0,0,1],"indices":[0,1,2],"faceIds":[10],"vertexIds":[1,2,3],"edgePositions":[0,0,0,1,0,0],"edgeIds":[5]}}]"#.into(),
            instances_json: r#"[{"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into(),
            instances_delta_json: None,
            selection_json: selection_json.into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            pick_targets_json: None,
            lod_json: None,
            presentation_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
            annotations: None,
            scalar_field: None,
            modelling_options: None,
            domain_id: domain.map(|(id, _)| id.to_string()),
            domain_granularity_id: domain.map(|(_, granularity)| granularity.to_string()),
            lanes: Vec::new(),
        }),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}

/// 🧹️ Retires everything a bridged surface holds — snapshot leases live in ONE process-wide
/// fixed pool, so a test that publishes one and walks away starves every later test with
/// `World3dSnapshotFault::Unavailable`.
fn retire_bridged_surface(state: &mut World3dState) {
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    for _ in 0..4_096 {
        if with_world_step_context(1, |context| terrain_test_retire_then_dynamic(state,context)) {
            break;
        }
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(state), "the surface must reach terminal empty");
}

/// 🌉️ Drives the staged bridge to its sealed lease, then the snapshot apply ladder to `Complete`.
pub(crate) fn drive_scene_bridge(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("scene bridge stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "scene bridge did not seal within its turn ceiling");
    }
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        assert_ne!(with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Fault, "draw rebuild faulted on turn {turn}");
        match with_world_step_context(64, |context| step_world3d_snapshot(state, context)) {
            World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("snapshot apply stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "snapshot apply did not complete within its turn ceiling");
    }
    for turn in 0..4_096 {
        let draw_step = with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context));
        assert_ne!(draw_step, WorldDrawRebuildStep::Fault, "draw retirement faulted on turn {turn}");
        let bridge_step = with_world_step_context(64, |context| step_world3d_scene_bridge(state, context));
        assert_ne!(bridge_step, World3dSceneBridgeStep::Fault, "bridge retirement faulted on turn {turn}");
        if world3d_draw_rebuild_terminal_is_empty(&state.scene) && state.bridge.scene_bridge_retired.is_none() && state.bridge.scene_bridge_close.is_none() && state.bridge.scene_bridge.is_none() {
            break;
        }
        assert!(turn < 4_095, "scene bridge retirement did not complete within its turn ceiling");
    }
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
}

//#region 🔖️Modelling
#[allow(non_snake_case)]
fn MODELLING_FIXTURE() -> &'static str {
    read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/📏️world3d-modelling/🔣️.json")
}

fn modelling_fixture() -> serde_json::Value {
    serde_json::from_str(MODELLING_FIXTURE()).expect("world3d modelling fixture")
}

fn state_with_guest_selection(granularity: &str) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &format!(r#"{{"granularity":"{granularity}","componentIds":[3,4],"hoveredComponent":{{"objectId":"a","mode":"{granularity}","id":3}},"targets":{{"vertex":true,"edge":true,"face":true}}}}"#));
    state
}

#[test]
fn the_scene_pick_filter_admits_exactly_the_fixture_granularity_on_the_native_selection_channels() {
    for row in modelling_fixture()["pickTargets"].as_array().unwrap() {
        let filter = ui_wgpu::wgpu::World3dPickGranularity::parse(row["filter"].as_str().unwrap()).unwrap();
        let mut state = state_with_guest_selection("face");
        apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, Some(filter));
        let targets = &row["targets"];
        assert_eq!((state.selection.selection_targets.vertex, state.selection.selection_targets.edge, state.selection.selection_targets.face), (targets["vertex"].as_bool().unwrap(), targets["edge"].as_bool().unwrap(), targets["face"].as_bool().unwrap()), "{filter:?}");
        let mode = if filter == ui_wgpu::wgpu::World3dPickGranularity::Shape { "mesh" } else { filter.as_str() };
        assert_eq!(state.selection.granularity, mode);
        let keeps = filter == ui_wgpu::wgpu::World3dPickGranularity::Face;
        assert_eq!(state.selection.component_ids.is_empty(), !keeps, "components of another granularity are dropped, those of the chosen one kept ({filter:?})");
        assert_eq!(state.selection.hovered_component_mode.is_some(), keeps);
    }
}

#[test]
fn the_scene_pick_filter_is_idempotent_and_absent_filter_leaves_the_guest_channels_alone() {
    let mut state = state_with_guest_selection("face");
    let before = (state.selection.granularity.clone(), state.selection.selection_targets.clone(), state.selection.component_ids.clone(), state.session.interaction_revision);
    apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, None);
    assert_eq!((state.selection.granularity.clone(), state.selection.selection_targets.clone(), state.selection.component_ids.clone(), state.session.interaction_revision), before);
    apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, Some(ui_wgpu::wgpu::World3dPickGranularity::Edge));
    let after_first = state.session.interaction_revision;
    assert_ne!(after_first, before.3);
    apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, Some(ui_wgpu::wgpu::World3dPickGranularity::Edge));
    assert_eq!(state.session.interaction_revision, after_first, "re-applying the same filter must not churn the view revision");
}

fn field_named(name: &str, values: serde_json::Value) -> ui_wgpu::wgpu::World3dScalarField {
    let entry = modelling_fixture()["valid"].as_array().unwrap().iter().find(|entry| entry["name"] == name).unwrap().clone();
    let mut value = entry["value"].clone();
    value["values"] = values;
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture scalar field")
}

//#region 🔖️ModellingWgpu
const UNIT_CUBE_MESHES: &str = r#"[{"id":"cube","data":{"positions":[0,0,0,1,0,0,1,1,0,0,1,0,0,0,1,1,0,1,1,1,1,0,1,1],"indices":[0,2,1,0,3,2,4,5,6,4,6,7,0,1,5,0,5,4,3,7,6,3,6,2,0,4,7,0,7,3,1,2,6,1,6,5]}}]"#;

fn modelling_cube_scene(options: Option<ui_wgpu::wgpu::World3dModellingOptions>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world fixture");
    world.meshes_json = UNIT_CUBE_MESHES.into();
    world.instances_json = r#"[{"id":"obj-1","meshId":"cube","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into();
    world.modelling_options = options;
    scene
}

fn paint_world(scene: &UiComponentSceneNode, state: &mut World3dState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, bounds: Rect) -> (ui_wgpu::wgpu::DrawList, ui_wgpu::wgpu::FontAtlas) {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    draw.set_screen_height(bounds.h);
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    {
        let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
            draw: &mut draw,
            overlay: None,
            atlas: &mut atlas,
            icons: None,
            input: &mut input,
            theme,
            scroll_offsets: &mut scroll,
            collapsed_sections: &mut collapsed,
            open_selects: &mut selects,
            interaction_maps: None,
            pick_clip: None,
            viewport_height: bounds.h,
        };
        render_world_3d(scene, bounds, &mut ctx, state, gpu, World3dShadowProfile::World);
    }
    (draw, atlas)
}

fn painted_pass(scene: &UiComponentSceneNode, state: &mut World3dState, theme: &ui_wgpu::wgpu::Theme, bounds: Rect, ready: impl Fn(&ScenePass3d) -> bool) -> ui_wgpu::wgpu::DrawList {
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    drive_scene_bridge(state, scene, bounds);
    for turn in 0..4_096 {
        let (draw, _) = paint_world(scene, state, &mut gpu, theme, bounds);
        if draw.scene_passes.last().is_some_and(&ready) {
            return draw;
        }
    }
    panic!("the world pass never satisfied its readiness predicate");
}

fn section_roles(draws: &[SceneDraw3d]) -> Vec<(String, ui_wgpu::wgpu::SceneSectionRole3d)> {
    draws.iter().flat_map(|draw| draw.instances.iter().map(|instance| (draw.mesh_key.clone(), instance.material.section))).collect()
}

#[test]
fn a_capped_section_clips_the_scene_and_brackets_the_cap_plane_between_two_stencil_toggles() {
    use ui_wgpu::wgpu::SceneSectionRole3d as Role;
    let fixture = modelling_fixture();
    let row = &fixture["sectionPlanes"][1];
    let section: ui_wgpu::wgpu::World3dSection = serde_json::from_value(row["section"].clone()).expect("fixture section");
    let options = ui_wgpu::wgpu::World3dModellingOptions { section: Some(section), ..Default::default() };
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&modelling_cube_scene(Some(options)), &mut state, &theme, bounds, |pass| pass.translucent_draws.iter().any(|draw| draw.instances.iter().any(|instance| instance.material.section == Role::Cap)));
    let pass = draw.scene_passes.last().unwrap();
    let want: Vec<f32> = row["plane"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
    assert_eq!(pass.section_clip.map(|plane| plane.to_vec()), Some(want), "the pass carries the fixture's three.js clip plane");
    assert_eq!(section_roles(&pass.draws), vec![("cube".to_string(), Role::Clipped)], "every scene instance is tested against the plane");
    let roles = section_roles(&pass.translucent_draws);
    assert_eq!(roles, vec![("cube".to_string(), Role::StencilToggle), ("gumball-plane".to_string(), Role::Cap), ("cube".to_string(), Role::StencilToggle)], "toggle, cap, toggle restores the stencil bit");
    let cap = pass.translucent_draws.iter().flat_map(|draw| &draw.instances).find(|instance| instance.material.section == Role::Cap).unwrap();
    let expected = theme.celebrate[1];
    assert_eq!(cap.color, [expected.r, expected.g, expected.b, 1.0], "the cap takes the secondary tone from the live theme");
    let axis = cap.model.transform_point(Vec3::new(0.0, 1.0, 0.0)).sub(cap.model.transform_point(Vec3::ZERO));
    let normal = [1.0, 1.0, 0.0].map(|component: f32| component / 2.0_f32.sqrt());
    for (got, want) in [axis.x, axis.y, axis.z].into_iter().zip(normal) {
        assert!((got / axis.length() - want).abs() < 1e-4, "the cap plane stands on the section normal");
    }
    let centre = cap.model.transform_point(Vec3::ZERO);
    assert_eq!([centre.x, centre.y, centre.z], [1.0, 2.0, 3.0], "the cap is centred on the section origin");
    retire_bridged_surface(&mut state);
}

#[test]
fn an_open_section_clips_without_a_cap_and_no_section_changes_nothing() {
    use ui_wgpu::wgpu::SceneSectionRole3d as Role;
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let open = ui_wgpu::wgpu::World3dModellingOptions { section: Some(ui_wgpu::wgpu::World3dSection::new([0.0, 0.0, 0.5], [0.0, 0.0, 1.0])), ..Default::default() };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&modelling_cube_scene(Some(open)), &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
    let pass = draw.scene_passes.last().unwrap();
    assert_eq!(pass.section_clip, Some([-0.0, -0.0, -1.0, 0.5]));
    assert_eq!(section_roles(&pass.draws), vec![("cube".to_string(), Role::Clipped)]);
    assert!(section_roles(&pass.translucent_draws).iter().all(|(_, role)| *role == Role::None), "an open section draws no stencil or cap");
    retire_bridged_surface(&mut state);
    let mut plain = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&modelling_cube_scene(None), &mut plain, &theme, bounds, |pass| !pass.draws.is_empty());
    let pass = draw.scene_passes.last().unwrap();
    assert_eq!(pass.section_clip, None);
    assert!(section_roles(&pass.draws).into_iter().chain(section_roles(&pass.translucent_draws)).all(|(_, role)| role == Role::None));
    retire_bridged_surface(&mut plain);
}

fn overlay_vertices(draw: &ui_wgpu::wgpu::DrawList) -> Vec<[f32; 2]> {
    draw.layers.iter().flat_map(|layer| layer.overlay_vector_vertices.iter().map(|vertex| vertex.position)).collect()
}

fn overlay_glyphs(draw: &ui_wgpu::wgpu::DrawList) -> usize {
    draw.layers.iter().flat_map(|layer| &layer.overlay_ui_instances).filter(|instance| instance.uv_rect != [0.0, 0.0, 1.0, 1.0]).count()
}

fn annotation_scene(layer: serde_json::Value) -> UiComponentSceneNode {
    let mut scene = modelling_cube_scene(None);
    scene.world_3d.as_mut().unwrap().annotations = Some(semio_framework_pack_json::from_json_str(&layer.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("annotation layer"));
    scene
}

#[test]
fn annotations_paint_the_shared_projection_as_screen_constant_overlay_geometry() {
    let fixture = modelling_fixture();
    let layer = fixture["annotationProjection"]["layer"].clone();
    let scene = annotation_scene(layer.clone());
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(40.0, 30.0, 640.0, 480.0);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
    let pass = draw.scene_passes.last().unwrap();
    let typed: ui_wgpu::wgpu::World3dAnnotationLayer = semio_framework_pack_json::from_json_str(&layer.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let projected = ui_wgpu::wgpu::project_world3d_annotations(&typed, &pass.view_proj, bounds.w, bounds.h);
    assert!(projected.iter().any(|entry| entry.visible) && projected.iter().any(|entry| !entry.visible));
    let vertices = overlay_vertices(&draw);
    let near = |x: f64, y: f64| vertices.iter().any(|vertex| (f64::from(vertex[0]) - (x + f64::from(bounds.x))).abs() < 1.0 && (f64::from(vertex[1]) - (y + f64::from(bounds.y))).abs() < 1.0);
    for entry in projected.iter().filter(|entry| entry.visible) {
        for line in &entry.lines {
            assert!(near(line[0], line[1]) && near(line[2], line[3]), "{}: the projected line is painted at its screen position", entry.id);
        }
        for arrow in &entry.arrows {
            assert!(near(arrow.x, arrow.y), "{}: the arrow tip is painted on its anchor", entry.id);
        }
    }
    let hidden_only = projected.iter().filter(|entry| !entry.visible).count();
    assert!(hidden_only >= 2);
    let glyphs = overlay_glyphs(&draw);
    let visible_text: usize = typed.items.iter().zip(&projected).filter(|(_, entry)| entry.visible && entry.label.is_some()).map(|(item, _)| item.text().resolve("en").chars().count()).sum();
    assert_eq!(glyphs, visible_text, "exactly the visible labels are written, in English by default");
    retire_bridged_surface(&mut state);
}

#[test]
fn annotation_sizes_stay_in_screen_pixels_when_the_camera_moves_away() {
    let layer = serde_json::json!({"items":[{"kind":"dimension","id":"d","from":[0,0,0],"to":[1,0,0],"offset":[0,-0.2,0],"text":{"en":"1","de":"1"}}]});
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
    let mut arrow_sizes = Vec::new();
    for distance in [4.0, 40.0] {
        let mut scene = annotation_scene(layer.clone());
        scene.world_3d.as_mut().unwrap().camera_json = format!(r#"{{"position":[{distance},{distance},{distance}],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}}"#);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let draw = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
        let typed: ui_wgpu::wgpu::World3dAnnotationLayer = semio_framework_pack_json::from_json_str(&layer.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let projected = ui_wgpu::wgpu::project_world3d_annotations(&typed, &draw.scene_passes.last().unwrap().view_proj, bounds.w, bounds.h);
        let arrows = &projected[0].arrows;
        assert_eq!(arrows.len(), 2, "the original dimension projects both arrowheads");
        let triangles: Vec<_> = draw.layers.iter().flat_map(|layer| layer.overlay_vector_vertices.iter().map(|vertex| vertex.position)).collect();
        for expected in arrows {
            let arrow = triangles.chunks_exact(3).find(|triangle| (f64::from(triangle[0][0]) - expected.x).abs() < 0.01 && (f64::from(triangle[0][1]) - expected.y).abs() < 0.01).expect("the dimension arrowhead is painted at its projected anchor");
            let tip_to_base = ((arrow[1][0] + arrow[2][0]) / 2.0 - arrow[0][0]).hypot((arrow[1][1] + arrow[2][1]) / 2.0 - arrow[0][1]);
            arrow_sizes.push(tip_to_base);
        }
        retire_bridged_surface(&mut state);
    }
    assert_eq!(arrow_sizes.len(), 4);
    assert!(arrow_sizes.iter().all(|size| (size - 9.0).abs() < 0.01), "both arrowheads are nine pixels at every distance: {arrow_sizes:?}");
}

#[test]
fn annotation_text_follows_the_active_language() {
    let layer = serde_json::json!({"items":[{"kind":"leader","id":"m","anchor":[0.5,0.5,0.5],"labelOffsetPx":[10,10],"text":{"en":"Centre point","de":"Mitte"}}]});
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
    let mut counts = Vec::new();
    for german in [false, true] {
        let mut scene = annotation_scene(layer.clone());
        scene.world_3d.as_mut().unwrap().annotations = scene.world_3d.as_ref().unwrap().annotations.clone().map(|mut layer| {
            if let ui_wgpu::wgpu::World3dAnnotation::Leader(leader) = &mut layer.items[0] {
                leader.text = ui_wgpu::wgpu::World3dText::new("Centre point", "Mitte");
            }
            layer
        });
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.set_modelling_locale(german);
        let draw = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
        counts.push(overlay_glyphs(&draw));
        retire_bridged_surface(&mut state);
    }
    assert_eq!(counts, vec!["Centre point".chars().count(), "Mitte".chars().count()]);
}

fn legend_scene(values: serde_json::Value) -> UiComponentSceneNode {
    let mut scene = modelling_cube_scene(None);
    scene.world_3d.as_mut().unwrap().scalar_field = Some(field_named("scalar-vertex-inferno", values));
    let world = scene.world_3d.as_mut().unwrap();
    world.scalar_field.as_mut().unwrap().mesh_id = "cube".into();
    scene
}

fn ramp_slices(draw: &ui_wgpu::wgpu::DrawList) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| &layer.overlay_ui_instances).filter(|instance| instance.uv_rect == [0.0, 0.0, 1.0, 1.0] && instance.rect[2] == 12.0).map(|instance| instance.color).collect()
}

#[test]
fn an_applied_scalar_field_paints_its_legend_ramp_top_down_in_the_fixture_ramp_colours() {
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
    let scene = legend_scene(serde_json::json!([0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 1.0]));
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
    assert_eq!(state.overlay.scalar_field_status(), World3dScalarFieldStatus::Applied);
    let slices = ramp_slices(&draw);
    assert_eq!(slices.len(), 32);
    let ramp = ui_wgpu::wgpu::World3dColorRamp::Inferno;
    let colour = |t: f64| {
        let [r, g, b] = ramp.sample(t);
        let linear = ui_wgpu::wgpu::Rgba::from_srgb8(r, g, b, 255);
        [linear.r, linear.g, linear.b, 1.0]
    };
    assert_eq!(slices[0], colour(1.0 - 0.5 / 32.0), "the top of the bar is the range maximum");
    assert_eq!(slices[31], colour(0.5 / 32.0), "the bottom of the bar is the range minimum");
    let fixture = modelling_fixture();
    for sample in fixture["ramps"]["inferno"]["samples"].as_array().unwrap() {
        let hex = sample["hex"].as_str().unwrap();
        let [r, g, b] = ramp.sample(sample["t"].as_f64().unwrap());
        assert_eq!(format!("#{r:02x}{g:02x}{b:02x}"), hex, "the legend ramp is the fixture's three.js ramp");
    }
    let rects: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| &layer.overlay_ui_instances).filter(|instance| instance.uv_rect == [0.0, 0.0, 1.0, 1.0] && instance.rect[2] == 12.0).map(|instance| instance.rect).collect();
    assert!(rects.windows(2).all(|pair| pair[1][1] > pair[0][1]), "slices run from the top downwards");
    assert!(rects[0][0] >= 8.0 && rects[31][1] + rects[31][3] <= bounds.h - 8.0, "the legend sits inside the viewport's lower-left margin");
    retire_bridged_surface(&mut state);
}

#[test]
fn a_scalar_field_that_does_not_fit_its_mesh_announces_the_mismatch_instead_of_a_legend() {
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
    let scene = legend_scene(serde_json::json!([0.0, 1.0]));
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let draw = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
    assert_eq!(state.overlay.scalar_field_status(), World3dScalarFieldStatus::Mismatch);
    assert!(ramp_slices(&draw).is_empty());
    assert_eq!(overlay_glyphs(&draw), ui_wgpu::wgpu::world3d_modelling_strings("en").mismatch.chars().count());
    retire_bridged_surface(&mut state);
    let mut german = World3dState::new("surface-1".into(), "controller-1".into());
    german.set_modelling_locale(true);
    let draw = painted_pass(&scene, &mut german, &theme, bounds, |pass| !pass.draws.is_empty());
    assert_eq!(overlay_glyphs(&draw), ui_wgpu::wgpu::world3d_modelling_strings("de").mismatch.chars().count());
    retire_bridged_surface(&mut german);
}

#[test]
fn the_modelling_overlays_publish_a_text_alternative_in_the_active_language() {
    let theme = ui_wgpu::wgpu::Theme::default();
    let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
    let mut scene = legend_scene(serde_json::json!([0.0, 0.1, 0.2, 0.3, null, 0.5, 0.6, 1.0]));
    scene.world_3d.as_mut().unwrap().annotations = Some(semio_framework_pack_json::from_json_str(&modelling_fixture()["valid"][0]["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture layer"));
    let mut labels = Vec::new();
    for german in [false, true] {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.set_modelling_locale(german);
        let _ = painted_pass(&scene, &mut state, &theme, bounds, |pass| !pass.draws.is_empty());
        let entries = world3d_modelling_accessibility(&state.session, &state.overlay, &scene);
        assert_eq!(entries[0].role, "list");
        assert!(entries.iter().filter(|entry| entry.role == "listitem" && entry.depth == 1).count() >= 4, "one list item per annotation");
        assert!(entries.iter().all(|entry| !entry.polite), "an applied legend is not a live region");
        labels.push(entries.iter().map(|entry| entry.label.clone()).collect::<Vec<_>>());
        retire_bridged_surface(&mut state);
    }
    assert_eq!(labels[0][0], "Measurements");
    assert_eq!(labels[1][0], "Messungen");
    assert_eq!(labels[0][1], "Dimension: Width 40 mm");
    assert_eq!(labels[1][1], "Bemaßung: Breite 40 mm");
    assert!(labels[0].contains(&"Legend: Wall thickness".to_string()) && labels[0].contains(&"No data".to_string()) && labels[0].contains(&"0.25 mm".to_string()));
    assert!(labels[1].contains(&"Legende: Wandstaerke".to_string()) && labels[1].contains(&"Keine Daten".to_string()) && labels[1].contains(&"0,25 mm".to_string()));
    let mut mismatch_state = World3dState::new("surface-1".into(), "controller-1".into());
    let mismatch = legend_scene(serde_json::json!([0.0, 1.0]));
    let _ = painted_pass(&mismatch, &mut mismatch_state, &theme, bounds, |pass| !pass.draws.is_empty());
    let entries = world3d_modelling_accessibility(&mismatch_state.session, &mismatch_state.overlay, &mismatch);
    assert_eq!(entries.len(), 1);
    assert_eq!((entries[0].role, entries[0].polite, entries[0].label.as_str()), ("status", true, "The scalar field does not match the mesh"));
    retire_bridged_surface(&mut mismatch_state);
}

/// 🧊️ Renders one painted world draw list through the real prepared pipeline on a headless GPU and reads the pixels back, `None` when the machine has no adapter.
#[cfg(not(target_arch = "wasm32"))]
fn render_on_gpu(scene: &UiComponentSceneNode, state: &mut World3dState, theme: &ui_wgpu::wgpu::Theme, size: u32, ready: impl Fn(&ScenePass3d) -> bool) -> Option<Vec<u8>> {
    use semio_framework_job::{InteractiveJob, StepOutcome};
    use ui_gpu::{RasterTextureCleanupStep, RasterTextureWitness};
    use ui_wgpu::wgpu::{PreparedRasterKeepCursorV1, PreparedRasterKeepStepV1, PreparedRenderGate, PreparedRenderInput, PreparedRenderJob, UiPresentToken};
use ui_gpu::{GpuContext};
    let source = match semio_framework_async::block_on(GpuContext::headless(1, 1)) {
        Ok(source) => source,
        Err(error) if error.starts_with("offscreen adapter:") => return None,
        Err(error) => panic!("headless GPU {error}"),
    };
    let bounds = Rect::new(0.0, 0.0, size as f32, size as f32);
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    drive_scene_bridge(state, scene, bounds);
    let mut draw = None;
    let mut readied = false;
    for _ in 0..512 {
        let (painted, _) = paint_world(scene, state, &mut resources, theme, bounds);
        let ready = painted.scene_passes.last().is_some_and(&ready);
        draw = Some(painted);
        if ready {
            readied = true;
            break;
        }
    }
    assert!(readied, "the world pass never satisfied its readiness predicate");
    let draw = draw.expect("a painted frame");
    let mut input = PreparedRenderInput::try_new(7, 1, draw, None, 0.0).unwrap_or_else(|rejected| panic!("prepared input refused: {}", rejected.fault()));
    while !resources.append_step(&mut input).unwrap_or_else(|_| panic!("world resources refused")) {}
    let mut job = PreparedRenderJob::try_new(input).unwrap_or_else(|rejected| panic!("prepared job refused: {}", rejected.fault()));
    let mut packet = None;
    for _ in 0..8_192 {
        match with_world_step_context(1_000, |context| job.step(context)) {
            StepOutcome::Yield => {}
            mut outcome @ StepOutcome::Complete(_) => {
                packet = job.take_packet();
                InteractiveJob::begin_close(&mut job);
                while !outcome.terminal_is_empty(){let step=outcome.close_step(world_original_fixture_grant()).expect("original prepared outcome close");assert!(step.progress().fits(world_original_fixture_grant()));}
                break;
            }
            other => panic!("prepared job ended in {other:?}: {:?}", job.fault()),
        }
    }
    while !matches!(PreparedRenderJob::close_step(&mut job,world_original_fixture_grant()),semio_framework_job::InteractiveJobCloseStep::Complete{..}) {}
    let mut packet = packet.expect("a prepared packet");
    let mut gpu = source.offscreen(size, size).expect("offscreen target");
    gpu.begin_prepared(&UiPresentToken::mint_for_current_thread(), &PreparedRenderGate::default(), &packet, packet.scene_revision(), packet.preview_generation()).expect("packet admitted");
    let witness = RasterTextureWitness { scene_revision: packet.scene_revision(), preview_generation: packet.preview_generation(), operation: 1 };
    gpu.begin_raster_ownership(witness).expect("raster ownership");
    let mut keep = PreparedRasterKeepCursorV1::default();
    loop {
        match packet.raster_keep_step(&mut keep) {
            PreparedRasterKeepStepV1::Pending => {}
            PreparedRasterKeepStepV1::Key(key) => gpu.publish_raster_ownership(witness, key).expect("raster key"),
            PreparedRasterKeepStepV1::Complete => {
                gpu.seal_raster_ownership(witness).expect("raster seal");
                break;
            }
        }
    }
    for index in 0..packet.uploads().len() {
        while !gpu.apply_prepared_upload_step(&packet, index, witness, witness).expect("upload step") {}
    }
    let mut cursor = gpu.begin_prepared_present(&packet, witness).expect("present cursor");
    while !gpu.prepared_present_step(&packet, &mut cursor).expect("present step") {
        assert!(gpu.take_missing_world_mesh().is_none(), "every drawn mesh is resident");
    }
    let mut readback = gpu.begin_prepared_readback(&cursor).expect("readback");
    while !gpu.prepared_readback_step(&mut readback).expect("readback step") {}
    let rgba = readback.take_rgba().expect("pixels");
    cursor.begin_close();
    while !cursor.close_step() {}
    while !gpu.abort_presented_rasters_step(witness).expect("abort rasters") {}
    while !matches!(gpu.close_raster_upload_step(), RasterTextureCleanupStep::Complete) {}
    while !gpu.close_mesh_upload_step() {}
    while !gpu.close_raster_table_step().expect("raster table") {}
    while !gpu.close_mesh_table_step() {}
    while !packet.retirement_is_empty(){let step=packet.close_step(world_original_fixture_grant());assert!(step.progress().fits(world_original_fixture_grant()));}
    Some(rgba)
}

fn top_camera(scene: &mut UiComponentSceneNode) {
    scene.world_3d.as_mut().unwrap().camera_json = r#"{"position":[0.5,0.5,6.0],"target":[0.5,0.5,0.5],"up":[0.0,1.0,0.0],"fov":30.0}"#.into();
}

fn side_camera(scene: &mut UiComponentSceneNode) {
    scene.world_3d.as_mut().unwrap().camera_json = r#"{"position":[6.0,0.5,0.5],"target":[0.5,0.5,0.5],"up":[0.0,0.0,1.0],"fov":30.0}"#.into();
}

fn pixel(rgba: &[u8], size: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * size + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
}

fn distance(left: [u8; 4], right: [u8; 4]) -> i32 {
    (0..3).map(|channel| (i32::from(left[channel]) - i32::from(right[channel])).abs()).sum()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn on_a_real_gpu_a_section_removes_the_far_half_and_the_cap_closes_the_cut_without_leaking_stencil() {
    use ui_wgpu::wgpu::SceneSectionRole3d as Role;
    const SIZE: u32 = 128;
    let theme = ui_wgpu::wgpu::Theme::default();
    let section = |cap: bool| {
        let section = ui_wgpu::wgpu::World3dSection::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]);
        Some(ui_wgpu::wgpu::World3dModellingOptions { section: Some(if cap { section.capped(ui_wgpu::wgpu::World3dTone::Danger) } else { section }), ..Default::default() })
    };
    let render = |options: Option<ui_wgpu::wgpu::World3dModellingOptions>, camera: fn(&mut UiComponentSceneNode)| -> Option<Vec<u8>> {
        let mut scene = modelling_cube_scene(options.clone());
        camera(&mut scene);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let capped = options.as_ref().and_then(|options| options.section).is_some_and(|section| section.cap.is_some());
        let pixels = render_on_gpu(&scene, &mut state, &theme, SIZE, |pass| !pass.draws.is_empty() && (!capped || pass.translucent_draws.iter().any(|draw| draw.instances.iter().any(|instance| instance.material.section == Role::Cap))));
        retire_bridged_surface(&mut state);
        pixels
    };
    let Some(whole) = render(None, top_camera) else {
        return;
    };
    let open = render(section(false), top_camera).expect("same adapter");
    let capped = render(section(true), top_camera).expect("same adapter");
    let background = pixel(&whole, SIZE, 3, 3);
    let centre = (SIZE / 2, SIZE / 2);
    assert_ne!(pixel(&whole, SIZE, centre.0, centre.1), background, "the whole cube covers the image centre");
    let tone = theme.error;
    let cap_pixel = pixel(&capped, SIZE, centre.0, centre.1);
    assert!(distance(cap_pixel, pixel(&open, SIZE, centre.0, centre.1)) > 60, "the cap changes the cut: {cap_pixel:?} against {:?}", pixel(&open, SIZE, centre.0, centre.1));
    let dominant = |channels: [f32; 3]| (0..3).max_by(|a, b| channels[*a].total_cmp(&channels[*b])).unwrap();
    assert_eq!(dominant([f32::from(cap_pixel[0]), f32::from(cap_pixel[1]), f32::from(cap_pixel[2])]), dominant([tone.r, tone.g, tone.b]), "the cap is painted in the danger tone");
    for (x, y) in [(3, 3), (SIZE - 4, 3), (3, SIZE - 4), (SIZE - 4, SIZE - 4), (SIZE / 2, 6)] {
        assert_eq!(pixel(&capped, SIZE, x, y), pixel(&open, SIZE, x, y), "outside the cube the capped render equals the open render at ({x}, {y}): the stencil bit never leaks");
    }
    let side_whole = render(None, side_camera).expect("same adapter");
    let side_cut = render(section(false), side_camera).expect("same adapter");
    let side_background = pixel(&side_whole, SIZE, 3, 3);
    let (x, upper, lower) = (SIZE / 2, SIZE / 2 - 12, SIZE / 2 + 12);
    assert_ne!(pixel(&side_whole, SIZE, x, upper), side_background, "the upper half of the cube is drawn without the section");
    assert_eq!(pixel(&side_cut, SIZE, x, upper), side_background, "the section removes the side its normal points to");
    assert_eq!(pixel(&side_cut, SIZE, x, lower), pixel(&side_whole, SIZE, x, lower), "the kept half is untouched");
}
//#endregion 🔖️ModellingWgpu

fn terrain_test_grant()->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64}}

pub(crate) fn terrain_test_retire_then_dynamic(state:&mut World3dState,context:&mut semio_framework_job::StepContext<'_>)->bool {
    let grant=terrain_test_grant();
    let terrain=step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,grant,context);
    assert!(terrain.ownership.fits(grant));
    match terrain.step {
        WorldTerrainMeshPublicationStep::Idle=>step_world3d_dynamic_retirement(state,context),
        WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>false,
        WorldTerrainMeshPublicationStep::Fault(fault)=>panic!("typed terrain retirement refusal: {fault:?}"),
    }
}

#[test]
fn terrain_retirement_zero_grant_preserves_original_owner_without_bool_bridge() {
    let mut state=World3dState::new("surface".into(),"controller".into());
    let payload=TerrainTileMeshPayload{positions:vec![0.0;9],normals:vec![0.0;9],indices:vec![0,1,2],uvs:vec![0.0;6]};
    let pointer=payload.positions.as_ptr();
    state.terrain.terrain_pending=Some(((0,0,0),payload,100,1,1));
    let zero=semio_framework_value::retained_clone::RetainedCloneGrant::default();
    with_world_step_context(1,|context|{let result=step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,zero,context);assert_eq!(result.step,WorldTerrainMeshPublicationStep::Pending);assert_eq!(result.ownership,Default::default());});
    assert_eq!(state.terrain.terrain_pending.as_ref().unwrap().1.positions.as_ptr(),pointer);
    begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
    for _ in 0..8192 {if with_world_step_context(1,|context|terrain_test_retire_then_dynamic(&mut state,context)){break;}}
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[test]
fn scene_bridge_cursor_close_preserves_original_mesh_attribute_custody_and_physical_receipts(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let mut state=World3dState::new("surface".into(),"controller".into());
    let(cursor,birth)=observe(||{
        let mut mesh=WorldMeshBuffers::default();
        mesh.attributes.insert("original-color".into(),semio_framework::MeshAttribute{domain:semio_framework::MeshAttributeDomain::Vertex,semantic:semio_framework::MeshAttributeSemantic::Color,interpolation:semio_framework::MeshAttributeInterpolation::Linear,values:vec![semio_framework_value::DslValue::String("original native payload".into())],indices:Some(vec![0])});
        World3dSceneBridgeCursor{digest:1,camera_digest:1,meshes_json:"original wire".into(),instances_json:"[]".into(),camera_json:"{}".into(),camera_changed:false,phase:World3dSceneBridgePhase::ScalarAttributes,meshes:vec![World3dSceneMeshEntry{id:"mesh".into(),data:mesh,url:None,kind:None}],instances:Vec::new(),camera:None,mesh_cursor:0,scalar_mesh_cursor:0,scalar_slot_cursor:0,retiring_mesh:false,mesh_digest:None,scalar_field:None}
    });
    assert_eq!(birth.released_bytes,0);
    assert!(birth.requested_bytes<=world_original_fixture_grant().maximum_capacity_bytes);
    let pointer=cursor.meshes[0].data.attributes.slot_entry(0).unwrap().1.values.as_ptr();state.bridge.scene_bridge=Some(cursor);
    let grant=world_original_fixture_grant();
    let(progress,removal)=observe(||state.bridge.scene_bridge.as_mut().unwrap().meshes[0].data.attributes.retire_slot_if(0,|_,_|false,grant).unwrap());assert_eq!(progress.copied_bytes,std::mem::size_of::<(String,semio_framework::MeshAttribute)>());assert_eq!((removal.requested_bytes,removal.released_bytes),(0,0));
    assert_eq!(state.bridge.scene_bridge.as_ref().unwrap().meshes[0].data.attributes.retained_slot_entry(0).unwrap().1.values.as_ptr(),pointer);
    let mut zero=semio_framework_value::RetainedCloneProgress::default();let mut sequence=0;
    let mut context=semio_framework_job::StepContext::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX,semio_framework_value::RetainedCloneGrant::default()),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut zero);
    let(_,heap)=observe(||step_world3d_scene_bridge_close(&mut state.bridge,&mut context));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(state.bridge.scene_bridge.is_some());assert!(state.bridge.scene_bridge_close.is_none());
    let(mut born,mut released,mut turns)=(birth.requested_bytes,0,0);
    while state.bridge.scene_bridge.is_some()||state.bridge.scene_bridge_close.is_some(){
        let(receipt,heap)=with_world_step_context(1,|context|{let(_,heap)=observe(||step_world3d_scene_bridge_close(&mut state.bridge,context));(context.retained_progress(),heap)});
        assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<16384);
    }
    assert_eq!(born,released);
}
