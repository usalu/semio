use super::*;
use protocol::Inference;
use serde_json::Value;

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn instances(scene: &semio_framework_plugin::World3dScene) -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(&scene.instances_json).expect("instances are a JSON array")
}

fn meshes(scene: &semio_framework_plugin::World3dScene) -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(&scene.meshes_json).expect("meshes are a JSON array")
}

#[semio_framework_async_macros::async_test]
async fn every_wall_of_the_demo_is_one_picking_instance_with_an_inline_mesh() {
    let (snapshot, inference) = demo();
    let scene = scene(&snapshot, &inference, &BimWorldWindowConfig::default(), &[], &[], 1);
    let ids: Vec<String> = instances(&scene).iter().map(|instance| instance["id"].as_str().expect("id").to_string()).collect();
    assert_eq!(ids, snapshot.walls.keys().cloned().collect::<Vec<_>>());
    for instance in instances(&scene) {
        assert_eq!(instance["interactionId"], instance["id"]);
        assert_eq!(instance["interactionGranularityId"], "wall");
    }
    for mesh in meshes(&scene) {
        let positions = mesh["data"]["positions"].as_array().expect("inline positions");
        assert!(!positions.is_empty() && positions.len() % 3 == 0);
        assert_eq!(mesh["data"]["colors"].as_array().expect("vertex colours").len() * 3, positions.len() * 4);
    }
}

#[semio_framework_async_macros::async_test]
async fn the_scene_names_the_elements_domain_so_picks_are_not_index_addressed() {
    let (snapshot, inference) = demo();
    let scene = scene(&snapshot, &inference, &BimWorldWindowConfig::default(), &[], &[], 1);
    assert_eq!(scene.domain_id.as_deref(), Some(BIM_ELEMENT_DOMAIN));
    assert!(scene.domain_granularity_id.is_some());
}

#[semio_framework_async_macros::async_test]
async fn the_selection_marks_its_instances_and_the_hover_marks_another() {
    let (snapshot, inference) = demo();
    let scene = scene(&snapshot, &inference, &BimWorldWindowConfig::default(), &["w-south".to_string()], &["w-east".to_string()], 1);
    let flags = |id: &str, flag: &str| instances(&scene).iter().find(|instance| instance["id"] == id).map(|instance| instance[flag].as_bool().expect("flag")).expect("instance");
    assert!(flags("w-south", "selected") && !flags("w-north", "selected"));
    assert!(flags("w-east", "hovered") && !flags("w-south", "hovered"));
}

#[semio_framework_async_macros::async_test]
async fn isolating_or_hiding_a_storey_filters_the_instances() {
    let (snapshot, inference) = demo();
    let count = |config: BimWorldWindowConfig| instances(&scene(&snapshot, &inference, &config, &[], &[], 1)).len();
    assert_eq!(count(BimWorldWindowConfig::default()), 4);
    assert_eq!(count(BimWorldWindowConfig { isolated_storey: "st-ground".into(), ..BimWorldWindowConfig::default() }), 4);
    assert_eq!(count(BimWorldWindowConfig { isolated_storey: "st-first".into(), ..BimWorldWindowConfig::default() }), 0);
    assert_eq!(count(BimWorldWindowConfig { hidden_storeys: vec!["st-ground".into()], ..BimWorldWindowConfig::default() }), 0);
}

#[semio_framework_async_macros::async_test]
async fn the_section_plane_exists_only_while_enabled_and_follows_the_axis() {
    let off = BimWorldWindowConfig::default();
    assert!(section_options(&off).is_none());
    let on = BimWorldWindowConfig { section_enabled: true, section_axis: "x".into(), section_offset: 2.0, ..off };
    let section = section_options(&on).and_then(|options| options.section).expect("section plane");
    assert_eq!((section.origin, section.normal), ([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
}

#[semio_framework_async_macros::async_test]
async fn an_unnavigated_window_frames_the_content_and_a_navigated_one_keeps_its_camera() {
    let (snapshot, inference) = demo();
    let solids = visible_solids(&inference, &BimWorldWindowConfig::default());
    let camera = framing_camera(&solids);
    assert!(camera.target[0] > 0.0 && camera.target[0] < 8.0, "targets the middle of the 8 m house, got {:?}", camera.target);
    let navigated = BimWorldWindowConfig { framed: true, camera: store::Viewport3dOrbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: None }, ..BimWorldWindowConfig::default() };
    let kept = scene(&snapshot, &inference, &navigated, &[], &[], 1);
    assert!(kept.camera_json.contains("\"position\":[1,2,3]") || kept.camera_json.contains("[1.0,2.0,3.0]"), "{}", kept.camera_json);
    assert_eq!(framing_camera(&[]), INITIAL_CAMERA);
}

#[semio_framework_async_macros::async_test]
async fn the_world_window_is_a_world3d_surface_bound_to_the_elements_domain() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert_eq!(definition.interactions.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn the_phase_filter_draws_only_the_elements_of_its_phase() {
    let (mut snapshot, _) = demo();
    snapshot.walls.get_mut("w-south").expect("wall").phase = crate::Phase::Demolished;
    snapshot.walls.get_mut("w-east").expect("wall").phase = crate::Phase::Existing;
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let drawn = |view_phase: &str| -> Vec<String> { instances(&scene(&snapshot, &inference, &BimWorldWindowConfig { view_phase: view_phase.into(), ..BimWorldWindowConfig::default() }, &[], &[], 1)).iter().map(|instance| instance["id"].as_str().expect("id").to_string()).collect() };
    assert_eq!(drawn("").len(), 4, "an empty filter and 'all' draw every element");
    assert_eq!(drawn("all"), drawn(""));
    assert_eq!(drawn("demolished"), ["w-south"]);
    assert_eq!(drawn("existing"), ["w-east"]);
    assert_eq!(drawn("new"), ["w-north", "w-west"]);
    assert!(drawn("temporary").is_empty());
    assert_eq!(drawn("planned").len(), 4, "an unknown key filters nothing instead of hiding the model");
    assert_eq!(view_phase(&BimWorldWindowConfig { view_phase: "Demolished".into(), ..BimWorldWindowConfig::default() }), ViewPhase::Demolished);
}
