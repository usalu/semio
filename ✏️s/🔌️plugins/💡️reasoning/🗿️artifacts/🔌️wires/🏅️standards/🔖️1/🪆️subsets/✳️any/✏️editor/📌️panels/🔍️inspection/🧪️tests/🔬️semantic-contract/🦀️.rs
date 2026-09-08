
use super::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn wires_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let document = crate::empty_wires_snapshot();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels::<crate::editor::wires::terminology::WiresLabels>(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::locale_from_str(row["locale"].as_str().expect("locale")), ..Default::default() });
        let tree = project(crate::editor::wires::panels::document::render(&document, labels).expect("document"));
        assert_eq!(tree["children"][0]["component"]["label"], row["identities"]);
        assert_eq!(tree["children"][1]["component"]["label"], row["relationships"]);
        let tree = project(crate::editor::wires::panels::catalogue::render(&document.wires_fixture, labels).expect("catalogue"));
        assert_eq!(tree["children"][0]["component"]["label"], row["identityKinds"]);
        assert_eq!(tree["children"][1]["component"]["label"], row["relationshipKinds"]);
        let tree = project(render(&document, labels).expect("inspection"));
        let lines = tree["children"][0]["children"].as_array().expect("summary").iter().map(|node| node["component"]["value"].clone()).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(lines), row["summary"]);
    }
    let board = crate::wires_working_board(&document);
    for node in [crate::editor::wires::modes::edit::windows::canvas::render(&board, &document.wires_fixture).expect("editor canvas"), crate::viewer::wires::modes::view::windows::canvas::render(&document).expect("viewer canvas")] {
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
        let scene: semio_framework_plugin::Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
        assert_eq!(scene.camera_x, vectors["canvas"]["cameraX"].as_f64().expect("camera x"));
        assert_eq!(scene.camera_y, vectors["canvas"]["cameraY"].as_f64().expect("camera y"));
        assert_eq!(scene.zoom, vectors["canvas"]["zoom"].as_f64().expect("zoom"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.layers_json).expect("independent layers oracle"), vectors["canvas"]["layers"]);
        project(node);
    }
}
