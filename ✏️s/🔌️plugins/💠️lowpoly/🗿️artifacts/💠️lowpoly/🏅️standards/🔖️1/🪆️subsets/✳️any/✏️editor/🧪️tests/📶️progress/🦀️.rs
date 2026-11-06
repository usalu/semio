//! 📶️ Operation progress and exact cancellation stay visible in both Lowpoly editing windows.
use semio_plugin::{ArtifactEditor, ArtifactView, ConfigView, HistoryView, ViewModel};
use semio_plugin::app::ArtifactOperationProgress;
use semio_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyPlayApp;
use serde_json::Value;

fn find<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"] == key { return Some(node); }
    node["children"].as_array()?.iter().find_map(|child| find(child, key))
}

fn has_text(value: &Value, text: &str) -> bool {
    match value {
        Value::String(value) => value == text,
        Value::Array(values) => values.iter().any(|value| has_text(value, text)),
        Value::Object(values) => values.values().any(|value| has_text(value, text)),
        _ => false,
    }
}

fn scene(body: &str, locale: Locale, operations: &[ArtifactOperationProgress]) -> Value {
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history).with_operations(operations);
    let config = LowpolyConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let tree = LowpolyPlayApp::render(body, &doc, &cfg, &ViewModel::new(locale, Terminology::Native)).unwrap();
    serde_json::from_str(&semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap()).unwrap()
}

#[test]
fn editing_windows_present_localized_progress_and_exact_cancellation() {
    let fixture: Value = serde_json::from_str(include_str!("status.json")).unwrap();
    let definition = semio_s_artifact_lowpoly_lowpoly_editor::create_lowpoly_app();
    let editor = semio_plugin::EditorApp::<LowpolyPlayApp>::default();
    let runtime_id = semio_framework_async::poll::resolve_ready(editor.surface_id());
    assert_eq!(definition.id, fixture["controller"].as_str().unwrap());
    assert_eq!(definition.controller_id, definition.id);
    assert_eq!(runtime_id, definition.id);
    for case in fixture["cases"].as_array().unwrap() {
        let locale = match case["locale"].as_str().unwrap() { "en" => Locale::En, "de" => Locale::De, _ => panic!("explicit fixture locale") };
        let operations = [ArtifactOperationProgress {
            operation_id: u64::from_str_radix(fixture["operationId"].as_str().unwrap(), 16).unwrap(),
            generation: u64::from_str_radix(fixture["generation"].as_str().unwrap(), 16).unwrap(),
            label: LocalizedLabel::native("Modeling", "Modellierung"),
            completed_units: fixture["completed"].as_u64().unwrap(),
            cancelling: case["cancelling"].as_bool().unwrap(),
        }];
        let key = format!("framework.operation.{}", fixture["operationId"].as_str().unwrap());
        for body in fixture["bodies"].as_array().unwrap() {
            let body = body.as_str().unwrap();
            let tree = scene(body, locale, &operations);
            assert_eq!(tree["key"], format!("{body}.with-progress"));
            assert!(find(&tree, body).is_some(), "the original editing surface remains present");
            assert!(has_text(&find(&tree, &key).unwrap()["accessibility"], case["label"].as_str().unwrap()));
            let status = find(&tree, &format!("{key}.status")).unwrap();
            assert_eq!(status["accessibility"]["live"], "polite");
            assert!(has_text(&status["component"], case["caption"].as_str().unwrap()));
            let progress = find(&tree, &format!("{key}.progress")).unwrap();
            assert_eq!(progress["component"]["type"], "progress");
            let cancel = find(&tree, &format!("{key}.cancel")).unwrap();
            assert!(has_text(&cancel["component"], case["cancel"].as_str().unwrap()));
            assert_eq!(cancel["disabled"], case["cancelling"]);
            let binding = &cancel["bindings"][0];
            assert_eq!(binding["trigger"], "activate");
            assert_eq!(binding["action"]["scope"].as_str(), Some(definition.controller_id.as_str()));
            assert_eq!(binding["action"]["name"], "cancelTypedOperation");
            assert_eq!(binding["action"]["version"], 1);
            assert_eq!(binding["args"]["operationId"], fixture["operationId"]);
            assert_eq!(binding["args"]["generation"], fixture["generation"]);
            assert!(find(&scene(body, locale, &[]), "framework.operations").is_none());
        }
        assert!(find(&scene(fixture["excludedBody"].as_str().unwrap(), locale, &operations), "framework.operations").is_none());
    }
    let semio_framework::kernel::UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = LowpolyPlayApp::operation_progress_scope() else { panic!("progress changes target the editing windows") };
    assert_eq!(serde_json::to_value(window_bodies).unwrap(), fixture["bodies"]);
    assert!(panel_bodies.is_empty());
    assert!(!utilities && !tools && !engagements && !measures && !labels);
}
