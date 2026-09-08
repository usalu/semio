
use super::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn sequence_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️panels.json")).expect("neutral panels");
    let document = neural_engine::ColdOwner::new(crate::default_snapshot());
    let fixture = neural_engine::ColdOwner::new(document.to_fixture());
    for row in vectors["cases"].as_array().expect("locales") {
        let config = crate::editor::sequence::config::SequenceConfig { locale: row["locale"].as_str().expect("locale").into(), ..Default::default() };
        let labels = crate::editor::sequence::terminology::sequence_play_labels(&config);
        let document = project(crate::editor::sequence::panels::document::render(&fixture, labels).expect("document tree"));
        assert_eq!(document["children"][0]["component"]["label"], row["steps"]);
        assert_eq!(document["children"][1]["component"]["label"], row["edges"]);
        let catalogue = project(crate::editor::sequence::panels::catalogue::render(&fixture, labels).expect("catalogue"));
        assert_eq!(catalogue["children"][0]["children"][0]["component"]["label"], row["firstAction"]);
        let inspector = project(render(&fixture, &["step-2".into()], labels).expect("selected inspector"));
        assert_eq!(inspector["children"][0]["component"]["label"], row["selectedHeading"]);
        assert_eq!(inspector["children"][0]["children"][1]["component"]["value"], row["selectedLine"]);
        for (selection, expected) in [(Vec::<String>::new(), &row["prompt"]), (vec!["missing".into()], &row["missing"])] {
            let inspector = project(render(&fixture, &selection, labels).expect("inspector message"));
            assert_eq!(&inspector["children"][0]["children"][0]["component"]["value"], expected);
        }
    }
}
