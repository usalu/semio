
use super::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire inspected semantic tree");
    serde_json::from_str(&text).expect("independent semantic JSON oracle")
}

#[test]
fn shooting_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let mut snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let cfg = ShootingConfig::default();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<ShootingLabels>(row["locale"].as_str().expect("locale"));
        let node = render(&snapshot, &cfg, labels).expect("shot inspector");
        let fields = &node.children[0].children;
        let bindings: Vec<serde_json::Value> = [0, 3, 4].into_iter().map(|i| serde_json::to_value(&fields[i].children[0].bindings[0]).expect("independent binding oracle")).collect();
        let tree = project(node);
        assert_eq!(tree["children"][0]["component"]["label"], row["shot"]);
        let fields = tree["children"][0]["children"].as_array().expect("shot fields");
        assert_eq!(serde_json::Value::Array(fields.iter().map(|field| field["component"]["label"].clone()).collect()), row["fields"]);
        for (index, binding) in bindings.iter().enumerate() {
            assert_eq!(binding["args"]["field"], vectors["editableFields"][index]);
            assert_eq!(binding["args"]["shotIds"][0], snapshot.active_shot_id);
            assert_eq!(binding["trigger"], "change");
        }
    }
    for node in [crate::editor::shooting::modes::edit::windows::scene::render(&snapshot, &cfg).expect("editor scene"), crate::viewer::shooting::modes::view::windows::scene::render(&snapshot).expect("viewer scene")] {
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("3D surface") };
        let scene: semio_framework_plugin::World3dScene = semio_framework_ui_scene::decode(props).expect("packed scene");
        let frame: serde_json::Value = serde_json::from_str(scene.frame_json.as_deref().expect("active frame")).expect("independent frame oracle");
        assert_eq!(frame, vectors["frame"]);
        project(node);
    }
    let node = crate::editor::shooting::modes::edit::windows::icon::render(&snapshot, &cfg).expect("icon scene");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("icon surface") };
    let scene: semio_framework_plugin::IconRenderScene = semio_framework_ui_scene::decode(props).expect("packed icon");
    let request: serde_json::Value = serde_json::from_str(&scene.request_json).expect("independent icon request oracle");
    assert_eq!(request["assetUrl"], vectors["iconAssetUrl"]);
    project(node);
    snapshot.shots.clear();
    snapshot.active_shot_id.clear();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<ShootingLabels>(row["locale"].as_str().expect("locale"));
        let tree = project(render(&snapshot, &cfg, labels).expect("empty inspector"));
        assert_eq!(tree["children"][0]["component"]["label"], row["summary"]);
    }
}
