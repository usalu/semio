extern crate semio_os_kernel as store;
use semio_s_artifact_process_process3d_harness as context;
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

#[semio_framework_async_macros::async_test]
async fn renderer_camera_gesture_republishes_the_pose_without_resetting_the_field_of_view() {
    use semio_s_artifact_process_process3d_commands::camera::set_camera::SetCamera;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/camera-gesture.json")).unwrap();
    assert!(SetCamera::from_viewport(&semio_framework::DslValue::from(&fixture["malformedCamera"])).is_none());
    let fov = fixture["retainedFieldOfView"].as_f64().unwrap();
    let mut app = context::app();
    context::settled_action(&mut app, "setCamera", Some(&semio_framework::DslValue::from(&fixture["seed"])));
    let before = app.snapshot().unwrap();
    let args = semio_framework::DslValue::from(&fixture["args"]);
    let (_, receipt) = context::settled_action(&mut app, "setCamera", Some(&args));
    assert!(!context::published_a_document_mutation(&receipt));
    assert_eq!(app.snapshot().unwrap(), before);
    let projection = context::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
    let world = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(&projection).expect("published workpiece scene");
    let camera: serde_json::Value = serde_json::from_str(&world.camera_json).unwrap();
    assert_eq!(camera["position"], fixture["args"]["camera"]["position"]);
    assert_eq!(camera["target"], fixture["args"]["camera"]["target"]);
    assert_eq!(camera["fov"].as_f64(), Some(fov));
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
    let tree = semio_plugin::built_to_component_tree(render(&snapshot, &Process3dConfig::default(), "select", &[], semio_plugin::Locale::En).expect("workpiece renders"));
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
        let built = workpiece::render(&snapshot, &config, "select", std::slice::from_ref(&snapshot.stock_id), semio_plugin::Locale::En).expect("workpiece renders");
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

#[semio_framework_async_macros::async_test]
async fn workpiece_hover_is_local_and_click_commits_at_the_preview_center() {
    use semio_plugin::artifact_app_laws::{decode_fixture_scene_with_lanes, dispatch_fixture_action, fixture_identity, meta};
    use semio_plugin::{ActionMeta, ArtifactEditor, Effect, PluginApp, ViewModel, World3dScene};
    use semio_s_artifact_process_process3d_editor::Process3dPlayApp;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/workpiece-placement.json")).unwrap();
    let metres = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|coordinate| coordinate.as_f64().unwrap()).collect::<Vec<_>>();
    let args = |value: &serde_json::Value| semio_framework_pack_json::from_json_str::<semio_framework::DslValue>(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for invalid in fixture["invalid"].as_array().unwrap() {
        for verb in ["worldPointerMove", "worldPointerDown"] {
            assert!(Process3dPlayApp::command_from_action(verb, &mut Some(args(invalid)), &mut None).is_err());
        }
    }
    let action = |app: &mut context::Process3dApp, view: &ViewModel, verb: &str, values: serde_json::Value| {
        let metadata = ActionMeta { view_state: Some(std::sync::Arc::new(view.clone())), ..meta("local") };
        let admitted = semio_framework_async::poll::resolve_ready(dispatch_fixture_action(&mut app.0, verb, Some(args(&values)), &metadata, &mut fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("dispatch workpiece gesture");
        let result = semio_framework_async::poll::resolve_ready(semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx())).expect("settle reserved action");
        let receipt = context::settle(app);
        let effects = result.requested_effects.into_iter().chain(receipt.effects.iter().cloned()).filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } => Some((window_id, utility_id)),
            _ => None,
        }).collect::<Vec<_>>();
        (receipt, effects)
    };
    let render = |app: &mut context::Process3dApp, view: &ViewModel| {
        let tree = context::render_with_view(app, PROCESS_3D_PLAY_BODY_MAIN, view);
        decode_fixture_scene_with_lanes::<World3dScene>(&tree).expect("workpiece scene")
    };
    let mut application = context::app();
    context::install_contributions(&mut application, context::demonstrator_contributions_pack());
    let before = application.snapshot().expect("published snapshot").step_payloads.len();
    let mut view = context::main_window_view();
    let mut second = view.clone();
    let sibling_id = fixture["windows"]["sibling"].as_str().unwrap();
    let primary_id = view.window_id.clone().unwrap();
    second.window_id = Some(sibling_id.into());
    let sibling = semio_plugin::ViewWindowInstance { id: sibling_id.into(), window_kind_id: workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into() };
    view.window_instances.push(sibling.clone());
    second.window_instances.push(sibling);
    let stock = render(&mut application, &view);
    for case in fixture["cases"].as_array().unwrap() {
        let utility = case["utility"].as_str().unwrap();
        view.active_utility_id = Some(utility.into());
        second.active_utility_id = Some(utility.into());
        let (receipt, effects) = action(&mut application, &view, "worldPointerMove", serde_json::json!({ "pane": "", "position": case["position"] }));
        assert!(effects.is_empty());
        assert!(!context::published_a_document_mutation(&receipt));
        assert_eq!(application.snapshot().expect("published snapshot").step_payloads.len(), before);
        let hovered = render(&mut application, &view);
        assert_eq!(hovered.meshes_json, stock.meshes_json);
        assert_eq!(hovered.instances_json, stock.instances_json);
        assert!(render(&mut application, &second).engagement_preview_json.is_none());
        let items: Vec<serde_json::Value> = serde_json::from_str(hovered.engagement_preview_json.as_deref().expect("visible wireframe")).unwrap();
        assert_eq!(items.len() as u64, case["segments"].as_u64().unwrap());
        for item in &items {
            assert_eq!(item["kind"], "segment");
            assert_eq!(item["role"], utility);
        }
        for axis in 0..3 {
            let (min, max) = items.iter().flat_map(|item| [item["from"][axis].as_f64().unwrap(), item["to"][axis].as_f64().unwrap()]).fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), coordinate| (min.min(coordinate), max.max(coordinate)));
            assert!(((min + max) / 2.0 - case["position"][axis].as_f64().unwrap()).abs() < 1e-9);
            assert!((max - min - case["dimensions"][axis].as_f64().unwrap()).abs() < 1e-9);
        }
        let primary_preview = hovered.engagement_preview_json;
        action(&mut application, &second, "engagementInput", serde_json::json!({ "value": utility }));
        let (_, effects) = action(&mut application, &second, "engagementSubmit", serde_json::json!({ "windowId": primary_id }));
        assert_eq!(effects, vec![(sibling_id.to_owned(), utility.to_owned())]);
        action(&mut application, &second, "worldPointerMove", serde_json::json!({ "position": case["position"] }));
        assert!(render(&mut application, &second).engagement_preview_json.is_some());
        let (_, effects) = action(&mut application, &second, "engagementAbort", serde_json::json!({ "windowId": primary_id }));
        assert_eq!(effects, vec![(sibling_id.to_owned(), "select".to_owned())]);
        assert!(render(&mut application, &second).engagement_preview_json.is_none());
        assert_eq!(render(&mut application, &view).engagement_preview_json, primary_preview);
    }
    view.active_utility_id = Some("cut".into());
    second.active_utility_id = Some("cut".into());
    let position = &fixture["place"];
    action(&mut application, &view, "worldPointerMove", serde_json::json!({ "position": fixture["cases"][0]["position"] }));
    let primary_preview = render(&mut application, &view).engagement_preview_json;
    assert!(primary_preview.is_some());
    action(&mut application, &second, "worldPointerMove", serde_json::json!({ "position": position }));
    assert!(render(&mut application, &second).engagement_preview_json.is_some());
    let (receipt, effects) = action(&mut application, &second, "worldPointerDown", serde_json::json!({ "position": position, "windowId": primary_id }));
    assert!(context::published_a_document_mutation(&receipt));
    assert_eq!(effects, vec![(sibling_id.to_owned(), "select".to_owned())]);
    let snapshot = application.snapshot().expect("published snapshot");
    assert_eq!(snapshot.step_payloads.len(), before + 1);
    let semio_s_artifact_process_process3d_model::snapshot::ProcessMeasure::Cut { pose, .. } = &snapshot.step_payloads.last().unwrap().measure else { panic!("cut placement") };
    assert_eq!(pose.position.to_vec(), metres(position));
    assert!(render(&mut application, &second).engagement_preview_json.is_none());
    assert_eq!(render(&mut application, &view).engagement_preview_json, primary_preview);

    let chosen = &fixture["chosenCapability"];
    let machine = chosen["machineId"].as_str().unwrap();
    let capability = chosen["capabilityId"].as_str().unwrap();
    let utility = chosen["utility"].as_str().unwrap();
    let position = &chosen["positionMetres"];
    let start = application.snapshot().unwrap();
    let count = start.step_payloads.len();
    let resize = |width| serde_json::json!({"target": start.stock_id, "field": "width", "value": width});
    let select = |capability: &str| serde_json::json!({"machineId": machine, "capabilityId": capability});
    let install = serde_json::json!({"machineId": machine, "catalogId": chosen["catalogId"]});
    assert!(context::published_a_document_mutation(&action(&mut application, &view, "addWorkshopMachine", install).0));
    assert!(action(&mut application, &view, "placeCapability", select(capability)).1.is_empty(), "default timber exceeds the reach rule");
    action(&mut application, &view, "patchInspector", resize(chosen["validWidthMetres"].as_f64().unwrap()));

    let arm = |app: &mut context::Process3dApp, view: &ViewModel, capability| {
        let (receipt, effects) = action(app, view, "placeCapability", select(capability));
        assert!(!context::published_a_document_mutation(&receipt));
        assert_eq!(effects, [(view.window_id.clone().unwrap(), utility.to_owned())]);
    };
    arm(&mut application, &view, capability);
    arm(&mut application, &second, chosen["siblingCapabilityId"].as_str().unwrap());
    view.active_utility_id = Some(utility.into());
    second.active_utility_id = Some(utility.into());
    let preview = |app: &mut context::Process3dApp, view: &ViewModel| render(app, view).engagement_preview_json;
    assert!(preview(&mut application, &view).is_none());
    let hover = |app: &mut context::Process3dApp, window| action(app, window, "worldPointerMove", serde_json::json!({"position": position}));
    for window in [&view, &second] {
        assert!(!context::published_a_document_mutation(&hover(&mut application, window).0));
    }
    let mill_preview = preview(&mut application, &view).unwrap();
    assert!(action(&mut application, &view, "placeCapability", select(capability)).1.is_empty(), "repeat arming is a no-op");
    assert_eq!(preview(&mut application, &view).as_deref(), Some(mill_preview.as_str()));
    let items: Vec<serde_json::Value> = serde_json::from_str(&mill_preview).unwrap();
    assert_eq!(items.len() as u64, chosen["segments"].as_u64().unwrap());
    assert!(items.iter().all(|item| item["role"] == chosen["measure"]));
    for axis in 0..3 {
        let (min, max) = items.iter().flat_map(|item| [item["from"][axis].as_f64().unwrap(), item["to"][axis].as_f64().unwrap()]).fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), coordinate| (min.min(coordinate), max.max(coordinate)));
        assert!(((min + max) / 2.0 - position[axis].as_f64().unwrap()).abs() < 1e-9);
        assert!((max - min - chosen["dimensionsMetres"][axis].as_f64().unwrap()).abs() < 1e-9);
    }
    let sibling_preview = preview(&mut application, &second).expect("independent bore wireframe");
    assert!(sibling_preview.contains("\"role\":\"drill\""));

    let select_effect = [(primary_id, "select".to_owned())];
    assert_eq!(action(&mut application, &view, "engagementAbort", serde_json::json!({})).1, select_effect);
    assert!(preview(&mut application, &view).is_none());
    assert_eq!(preview(&mut application, &second).as_deref(), Some(sibling_preview.as_str()));
    let click = |app: &mut context::Process3dApp| action(app, &view, "worldPointerDown", serde_json::json!({"position": position}));
    assert!(!context::published_a_document_mutation(&click(&mut application).0), "a delayed click after abort places nothing");

    arm(&mut application, &view, capability);
    let (receipt, effects) = click(&mut application);
    assert!(context::published_a_document_mutation(&receipt));
    assert_eq!(effects, select_effect);
    let placed = application.snapshot().unwrap();
    assert_eq!(placed.step_payloads.len(), count + 1);
    let step = placed.step_payloads.last().unwrap();
    let origin = step.origin.as_ref().unwrap();
    assert_eq!((origin.machine_id.as_str(), origin.capability_id.as_str()), (machine, capability));
    use semio_s_artifact_process_process3d_model::snapshot::{ProcessMeasure, WorkingSolid};
    let ProcessMeasure::Cut { tool: WorkingSolid::Box { width, depth, height }, pose } = &step.measure else { panic!("expected the robotic mill box"); };
    assert_eq!(pose.position.to_vec(), metres(position));
    assert_eq!(vec![*width, *depth, *height], metres(&chosen["dimensionsMetres"]));
    assert_eq!(preview(&mut application, &second).as_deref(), Some(sibling_preview.as_str()));

    arm(&mut application, &view, capability);
    action(&mut application, &view, "patchInspector", resize(chosen["tooLargeWidthMetres"].as_f64().unwrap()));
    hover(&mut application, &view);
    let stale = render(&mut application, &view);
    assert!(stale.engagement_preview_json.is_none());
    let selection: serde_json::Value = serde_json::from_str(&stale.selection_json).unwrap();
    assert_eq!(selection["engagementSessionActive"], true, "the stale press still reaches the disarming reducer");
    let (receipt, effects) = click(&mut application);
    assert!(!context::published_a_document_mutation(&receipt), "stock constraints are checked again at commit");
    assert_eq!(effects, select_effect);
    assert_eq!(application.snapshot().unwrap().step_payloads, placed.step_payloads);
}
