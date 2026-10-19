use semio_framework_plugin::BuiltNode;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::editor::sequence::terminology::SequenceLabels;
#[allow(unused_imports)]
use crate::editor::sequence::editor_vocabulary::ui_label;
#[allow(unused_imports)]
use crate::{SequenceHostSnapshot, SequenceStep};
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTreeBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::PluginAssemblyError;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::UiFixedList;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
#[allow(unused_imports)]
use semio_framework_ui_contract as ui;
use crate::editor::sequence::panels::inspection::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn sequence_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️panels.json")).expect("neutral panels");
    let document = neural_engine::ColdOwner::new(crate::default_snapshot());
    let fixture = neural_engine::ColdOwner::new(document.to_host_snapshot());
    for row in vectors["cases"].as_array().expect("locales") {
        let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
        let labels = crate::editor::sequence::terminology::sequence_play_labels(&view_state);
        let document = project(crate::editor::sequence::panels::document::render(&fixture, labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("document tree"));
        assert_eq!(document["children"][0]["component"]["label"], row["steps"]);
        assert_eq!(document["children"][1]["component"]["label"], row["edges"]);
        let catalogue = project(crate::editor::sequence::panels::catalogue::render(&fixture, labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("catalogue"));
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
