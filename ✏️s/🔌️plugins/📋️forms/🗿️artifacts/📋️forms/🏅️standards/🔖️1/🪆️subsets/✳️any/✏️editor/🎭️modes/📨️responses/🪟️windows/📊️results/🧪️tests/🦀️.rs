use super::*;
use crate::editor::forms::terminology::forms_play_labels;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

#[test]
fn shared_response_fixture_renders_original_labels_and_values() {
    let input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧬️schema/📨️response/📤️export/🧫️fixtures/🔣️.json")).unwrap();
    let mut snapshot = FormsSnapshot::default();
    snapshot.responses = dsl::json::from_json_str(&input["cases"][1]["responses"].to_string()).unwrap();
    let view = ViewModel { tree_viewport_rows: Some(32), ..Default::default() };
    let node = render(&snapshot, forms_play_labels(&view), &TreeWindows::for_body(&view, BODY)).unwrap();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("Name, full"));
    assert!(json.contains("Renamed label"));
    assert!(json.contains("Zustimmung"));
    assert!(json.contains("exportResponses"));
    assert!(json.contains("discardResponse"));
}

#[test]
fn response_browser_windows_submissions_and_answers() {
    let mut snapshot = FormsSnapshot::default();
    snapshot.responses = (0..120).map(|index| FormsResponse {
        id: format!("response-{index}"), submitted_at: 1790545740000, definition_version: "revision-a".into(),
        answers: (0..100).map(|answer| FormsAnswer { question_id: format!("q-{answer}"), label: format!("Question {answer}"), kind: "text".into(), value: dsl::DslValue::String(format!("Answer {answer}")) }).collect(),
    }).collect();
    let view = ViewModel { tree_viewport_rows: Some(8), tree_windows: vec![TreeWindowRequest { body_key: BODY.into(), node_key: RESPONSES.into(), open: Some(true), offset: 70, rows: 3 }], ..Default::default() };
    let node = render(&snapshot, forms_play_labels(&view), &TreeWindows::for_body(&view, BODY)).unwrap();
    let section = node.children.iter().find(|node| node.key.as_str() == RESPONSES).unwrap();
    let Component::TreeSection(props) = &section.component else { panic!("responses section"); };
    assert_eq!(props.window.unwrap().total, 120);
    assert_eq!(props.window.unwrap().offset, 70);
    assert!(section.children.len() <= 3);
    let first = section.children.iter().next().unwrap();
    assert_eq!(first.key.as_str(), "response-70");
    let Component::TreeItem(props) = &first.component else { panic!("response row"); };
    assert_eq!(props.window.unwrap().total, 100);
    assert!(first.children.len() < 100);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
}
