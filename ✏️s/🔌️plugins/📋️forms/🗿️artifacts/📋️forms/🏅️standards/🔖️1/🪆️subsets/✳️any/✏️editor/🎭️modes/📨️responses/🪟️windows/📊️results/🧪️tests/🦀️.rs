use crate::{schema::response::FormsAnswer, schema::response::FormsResponse};
use crate::FormsSnapshot;
use semio_plugin::TreeWindows;
use crate::editor::forms::modes::responses::results::*;
use crate::editor::forms::terminology::forms_play_labels;
use semio_plugin::plugin_app_close_prelude::Component;
use semio_plugin::{TreeWindowRequest, ViewModel};
use semio_framework_test_fixtures::fixture_text;

#[test]
fn shared_response_fixture_renders_original_labels_and_values() {
    let input: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut snapshot = FormsSnapshot::default();
    snapshot.responses = semio_framework_pack_json::from_json_str(&input["cases"][1]["responses"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let view = ViewModel { tree_viewport_rows: Some(32), ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    let node = render(&snapshot, forms_play_labels(&view), &TreeWindows::for_body(&view, BODY)).unwrap();
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
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
        answers: (0..100).map(|answer| FormsAnswer { question_id: format!("q-{answer}"), label: format!("Question {answer}"), kind: "text".into(), value: semio_framework_value::DslValue::String(format!("Answer {answer}")) }).collect(),
    }).collect();
    let view = ViewModel { tree_viewport_rows: Some(8), tree_windows: vec![TreeWindowRequest { body_key: BODY.into(), node_key: RESPONSES.into(), open: Some(true), offset: 70, rows: 3 }], ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
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
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
}

#[test]
fn submitted_long_answer_retains_a_read_only_paged_view() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️fixtures/📝️answer.json")).unwrap();
    let recipe = &fixture["answer"];
    let answer = recipe["unit"].as_str().unwrap().repeat(recipe["repeat"].as_u64().unwrap() as usize);
    assert_eq!(answer.len(), recipe["bytes"].as_u64().unwrap() as usize);
    assert!(answer.len() > 4096);
    let value = FormsAnswer { question_id: "long-answer".into(), label: "Long Answer".into(), kind: "longText".into(), value: semio_framework_value::DslValue::String(answer.clone()) };
    let mut snapshot = FormsSnapshot::default();
    snapshot.responses = vec![FormsResponse { id: "long-response".into(), submitted_at: 1790545740000, definition_version: "revision-a".into(), answers: vec![value] }];
    let view = ViewModel { tree_viewport_rows: Some(32), ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    let node = render(&snapshot, forms_play_labels(&view), &TreeWindows::for_body(&view, BODY)).unwrap();
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
    let tree: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(json.contains("framework.scene.text.buffer"));
    assert!(json.contains("readOnly"));
    assert!(!json.contains("setTryValue"));
    assert_eq!(response_carrier_text(response_scene(&tree).unwrap()), answer);
}

fn response_carrier_text(value: &serde_json::Value) -> String {
    let mut output = String::new();
    if value["component"]["type"] == "text" {
        if let Some(text) = value["component"]["value"].as_str() { output.push_str(text); }
        if let Some(attributes) = value["component"]["dataAttributes"].as_object() { for text in attributes.values().filter_map(serde_json::Value::as_str) { output.push_str(text); } }
    }
    if let Some(children) = value["children"].as_array() { for child in children { output.push_str(&response_carrier_text(child)); } }
    output
}

fn response_scene(value: &serde_json::Value) -> Option<&serde_json::Value> {
    if value["key"] == "long-answer.answer" { return Some(value); }
    value["children"].as_array()?.iter().find_map(response_scene)
}
