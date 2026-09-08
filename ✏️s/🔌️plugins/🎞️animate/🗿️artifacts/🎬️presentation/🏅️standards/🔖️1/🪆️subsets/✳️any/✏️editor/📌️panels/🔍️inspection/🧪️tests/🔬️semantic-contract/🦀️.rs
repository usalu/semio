
use super::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn presentation_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let document = crate::default_presentation_snapshot();
    let (_, tiles) = crate::presentation_working_scene(&document);
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<AnimatePresentationLabels>(row["locale"].as_str().expect("locale"));
        let tree = project(crate::editor::animate::panels::artifact::render(&document, labels).expect("document"));
        assert_eq!(tree["children"][0]["component"]["label"], row["tiles"]);
        let catalogue = crate::editor::animate::panels::catalogue::render(&document, labels).expect("catalogue");
        let grid_args: Vec<serde_json::Value> = catalogue.children[0].children.iter().skip(1).take(2).map(|node| serde_json::to_value(&node.bindings[0].args).expect("independent binding oracle")).collect();
        let figure_args = serde_json::to_value(&catalogue.children[1].children[0].bindings[0].args).expect("independent figure binding oracle");
        let source_disabled = catalogue.children[1].children[1].children[0].disabled;
        let tree = project(catalogue);
        assert_eq!(tree["children"][0]["component"]["label"], row["templates"]);
        assert_eq!(tree["children"][1]["component"]["label"], row["figure"]);
        for (index, expected) in vectors["gridActions"].as_array().expect("grid actions").iter().enumerate() {
            assert_eq!(grid_args[index].as_object().expect("grid arguments").len(), expected.as_object().expect("grid vector").len());
            for field in ["columns", "rows"] {
                assert_eq!(grid_args[index][field].as_f64(), expected[field].as_f64());
            }
        }
        assert_eq!(figure_args, vectors["figureSource"]);
        assert!(source_disabled);
        let tree = project(render(&document, labels).expect("inspection"));
        assert_eq!(tree["children"][0]["component"]["label"], row["inspection"]);
        let fields = tree["children"][0]["children"].as_array().expect("summary fields");
        assert_eq!(serde_json::Value::Array(fields.iter().map(|node| node["component"]["label"].clone()).collect()), row["summary"]);
        assert_eq!(fields[0]["children"][0]["component"]["value"], "animate.presentation");
        assert_eq!(fields[1]["children"][0]["component"]["value"], tiles.len().to_string());
    }
    for node in [crate::editor::animate::modes::main::windows::tile_editor::render(&document).expect("editor canvas"), crate::viewer::animate::modes::view::windows::tile_editor::render(&document).expect("viewer canvas")] {
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
        let scene: semio_framework_plugin::Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
        assert_eq!(scene.camera_x, vectors["canvas"]["cameraX"].as_f64().expect("camera x"));
        assert_eq!(scene.camera_y, vectors["canvas"]["cameraY"].as_f64().expect("camera y"));
        assert_eq!(scene.zoom, vectors["canvas"]["zoom"].as_f64().expect("zoom"));
        let layers: serde_json::Value = serde_json::from_str(&scene.layers_json).expect("independent layers oracle");
        assert_eq!(layers.as_array().expect("layers").len(), tiles.len() + 1);
        assert_eq!(layers[0]["id"], "source-frame");
        project(node);
    }
}
