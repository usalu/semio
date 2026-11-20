extern crate semio_os_kernel as store;
#[path = "../../../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_process_process3d_modes::windows::workpiece::*;
use semio_s_artifact_process_process3d_modes::windows::workpiece;
use semio_s_artifact_process_process3d_config::Process3dConfig;
use semio_s_artifact_process_process3d_inferences::{processed_mesh, replay_process, ProcessKernelReplay, ProcessPreviewReplay};
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_plugin::SurfaceKind;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    let doc = Process3dSnapshot::default();
    let engagement = engagement(&doc, &Process3dConfig::default(), semio_s_artifact_process_process3d_config::PROCESS3D_DEFAULT_UTILITY, &semio_s_artifact_process_process3d_kit::editor::process3d::terminology::Process3dLabels::NATIVE_EN);
    assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options");
}

#[semio_framework_async_macros::async_test]
async fn render_world_scene_contains_processed_mesh() {
    let mut app = context::app();
    let node = context::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
}

/// 🪚️ The default timber document must reach the workpiece window as its replayed kernel mesh.
#[semio_framework_async_macros::async_test]
async fn render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box() {
    let snapshot = semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document();
    let scene = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(&snapshot);
    assert!(matches!(scene.stock.solid, semio_s_artifact_process_process3d_model::snapshot::WorkingSolid::Box { width, .. } if (width - 3.0).abs() < 1e-9), "timber fixture stock: {:?}", scene.stock.solid);
    let mesh = processed_mesh(&scene, None).expect("timber replay succeeds").expect("timber replay tessellates");
    let extent = |axis: usize| {
        let values = mesh.positions.iter().skip(axis).step_by(3).map(|value| f64::from(*value));
        values.clone().fold(f64::NEG_INFINITY, f64::max) - values.fold(f64::INFINITY, f64::min)
    };
    assert!((extent(0) - 3.0).abs() < 1e-3, "x extent {} is not the 3.0 beam length", extent(0));
    assert!(extent(1) < 1.0 && extent(2) < 1.0, "y/z extents {}/{} are not the 0.2/0.3 beam section", extent(1), extent(2));
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let tree = semio_plugin::built_to_component_tree(render(&snapshot, &Process3dConfig::default(), "select", &[]).expect("workpiece renders"));
    let published = semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| semio_plugin::artifact_app_laws::built_surface_scene::<semio_plugin::World3dScene>(root)).expect("workpiece scene");
    let instances: serde_json::Value = serde_json::from_str(&published.instances_json).unwrap();
    assert_eq!(instances[0]["id"], snapshot.stock_id);
    assert_eq!(instances[0]["meshId"], "processed");
}

#[semio_framework_async_macros::async_test]
async fn workpiece_surface_replays_the_cursor_through_empty_and_restored_geometry() {
    use semio_s_artifact_process_process3d_config::Process3dConfig;
    use semio_s_artifact_process_process3d_kit::editor::process3d::terminology::Process3dLabels;
    use semio_s_artifact_process_process3d_model::snapshot::{process_working_scene_to_snapshot, Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, WorkingSolid, Workshop};

    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/workpiece-replay.json")).unwrap();
    let dimension = |part: &str, field: &str| fixture[part][field].as_f64().unwrap();
    let box_solid = |part: &str, width: &str| WorkingSolid::Box { width: dimension(part, width), depth: dimension(part, "depth"), height: dimension(part, "height") };
    let cut = |id: &str, tool, pose| ProcessStep { id: id.into(), label: id.into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool, pose } };
    let stock = Stock { id: "preview-stock".into(), solid: box_solid("stock", "width"), ..Default::default() };
    let mut scene = ProcessWorkingScene {
        stock,
        steps: vec![
            cut("preview-cut", box_solid("cut", "width"), Pose { position: [dimension("cut", "x"), 0.0, 0.0], ..Default::default() }),
            cut("complete-cut", box_solid("completeCut", "width"), Pose::default()),
            cut("empty-cut", box_solid("cut", "width"), Pose::default()),
            ProcessStep {
                id: "restore".into(),
                label: "Restore".into(),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Attach { component: box_solid("restore", "width"), pose: Pose::default() },
            },
        ],
    };
    let project = |scene: &ProcessWorkingScene, cursor, expected: &str, has_mesh| {
        let snapshot = process_working_scene_to_snapshot(scene, Workshop::default());
        let config = Process3dConfig { resolved_up_to: Some(cursor), ..Default::default() };
        let status = workpiece::engagement(&snapshot, &config, "select", &Process3dLabels::NATIVE_EN).status.unwrap();
        assert!(status[0].text.ends_with(&format!("{} m³", fixture["expected"][expected].as_str().unwrap())), "{status:?}");
        let built = workpiece::render(&snapshot, &config, "select", std::slice::from_ref(&snapshot.stock_id)).expect("workpiece renders");
        let rendered = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(built)).expect("workpiece projection");
        let published = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(&rendered).expect("published workpiece scene");
        assert_eq!(published.domain_id.as_deref(), Some("geometry"));
        let selection: serde_json::Value = serde_json::from_str(&published.selection_json).unwrap();
        assert_eq!(selection["ids"], if has_mesh { serde_json::json!(["preview-stock"]) } else { serde_json::json!([]) });
        let meshes: serde_json::Value = serde_json::from_str(&published.meshes_json).unwrap();
        let instances: serde_json::Value = serde_json::from_str(&published.instances_json).unwrap();
        assert_eq!(meshes.as_array().unwrap().len(), has_mesh as usize);
        assert_eq!(instances.as_array().unwrap().len(), has_mesh as usize);
        if has_mesh {
            assert_eq!(meshes[0]["id"], "processed");
            assert_eq!(instances[0]["id"], "preview-stock");
        }
        meshes
    };
    let tiny = ProcessWorkingScene {
        stock: Stock { solid: box_solid("tinyStock", "width"), ..scene.stock.clone() },
        steps: Vec::new(),
    };
    project(&tiny, 0, "tinyStock", true);
    let stock_mesh = project(&scene, 0, "stock", true);
    let cut_mesh = project(&scene, 1, "cut", true);
    assert_ne!(stock_mesh, cut_mesh);
    let ProcessMeasure::Cut { tool, .. } = &mut scene.steps[0].measure else { panic!("fixture cut") };
    *tool = box_solid("cut", "editedWidth");
    let edited_mesh = project(&scene, 1, "editedCut", true);
    assert_ne!(cut_mesh, edited_mesh);
    project(&scene, 2, "completeCut", false);
    project(&scene, 3, "emptyCut", false);
    let restored_mesh = project(&scene, 4, "restored", true);
    assert_ne!(edited_mesh, restored_mesh);

    let mut replay = ProcessKernelReplay::new();
    assert!(replay_process(&mut replay, &scene, Some(3)).unwrap().is_none());
    assert!(replay_process(&mut replay, &scene, Some(4)).unwrap().is_some());
    assert!(replay_process(&mut replay, &scene, Some(2)).unwrap().is_none());

    let mut unused_tool = scene.clone();
    let ProcessMeasure::Cut { tool, .. } = &mut unused_tool.steps[2].measure else { panic!("fixture empty cut") };
    *tool = box_solid("restore", "invalidWidth");
    assert!(replay_process(&mut replay, &unused_tool, Some(3)).unwrap().is_none());

    for format in ["step", "glb"] {
        assert!(semio_s_artifact_process_process3d_io::export_process3d_model(&scene, Some(3), format).unwrap().is_none());
    }
    assert!(semio_s_artifact_process_process3d_io::export_process3d_model(&scene, Some(4), "glb").unwrap().is_some());

    let ProcessMeasure::Attach { component, .. } = &mut scene.steps[3].measure else { panic!("fixture attachment") };
    *component = WorkingSolid::Box { width: dimension("restore", "invalidWidth"), depth: 1.0, height: 1.0 };
    assert!(ProcessPreviewReplay::default().evaluate(&scene, Some(4)).is_err());
    assert!(semio_s_artifact_process_process3d_io::export_process3d_model(&scene, Some(4), "glb").is_err());
}
