use super::*;
use semio_framework_plugin::ArtifactEditor;

#[test]
fn inspection_selection_matches_shared_authoring_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️authoring.json")).unwrap();
    let steps: Vec<crate::FormStep> = dsl::json::from_json_str(&fixture["steps"].to_string()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let selected: Vec<String> = serde_json::from_value(case["selected"].clone()).unwrap();
        let actual = inspection_model(&steps, &selected);
        let oracle: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&actual)).unwrap();
        assert_eq!(oracle, case["expected"], "{}", case["name"]);
    }
}

#[test]
fn inspection_controls_bind_to_real_typed_commands() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️authoring.json")).unwrap();
    let steps: Vec<crate::FormStep> = dsl::json::from_json_str(&fixture["steps"].to_string()).unwrap();
    let spec = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "authoring".into(), "1".into(), Some("Authoring".into()), &steps);
    let view = semio_framework_plugin::ViewModel::default();
    for case in fixture["cases"].as_array().unwrap() {
        let selected: Vec<String> = serde_json::from_value(case["selected"].clone()).unwrap();
        let node = render(&spec, &Default::default(), &selected, &view, &semio_framework_plugin::TreeWindows::for_body(&view, FORMS_PLAY_BODY_INSPECTION)).unwrap();
        let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert!(text.contains("forms-play-inspector.title"), "{}", case["name"]);
        assert!(assert_bound_commands(&tree) > 0, "{} must expose authoring commands", case["name"]);
    }
}

fn assert_bound_commands(value: &serde_json::Value) -> usize {
    let mut count = 0;
    if let Some(bindings) = value.get("bindings").and_then(serde_json::Value::as_array) {
        for binding in bindings {
            let action = binding["action"]["name"].as_str().expect("a semantic binding must name its action");
            count += 1;
            let mut args = binding["args"].as_object().cloned().unwrap_or_default();
            if binding["trigger"] == "change" { args.insert("value".into(), serde_json::json!("Edited")); }
            let args = crate::editor::forms::unit_tests::context::action_args(&serde_json::Value::Object(args));
            crate::editor::forms::FormsPlayApp::command_from_action(action, Some(&args)).unwrap_or_else(|error| panic!("{action}: {}", error.message));
        }
    }
    match value {
        serde_json::Value::Array(items) => count += items.iter().map(assert_bound_commands).sum::<usize>(),
        serde_json::Value::Object(entries) => count += entries.values().map(assert_bound_commands).sum::<usize>(),
        _ => {}
    }
    count
}
