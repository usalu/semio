use super::*;

#[test]
fn definition_declares_an_editable_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "replace-text"), "editable text window must carry the replace-text catalog action");
}

#[test]
fn render_carries_the_schema_field_as_editable_text() {
    let document = PlaygroundSnapshot { schema: "playground.custom".into() };
    let node = render(&document, semio_framework_plugin::Locale::En).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    assert_eq!(scene.buffer, "playground.custom");
    assert_eq!(scene.language.as_deref(), Some("playground"));
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("explicit draft settings")).expect("settings json");
    assert_eq!((settings["readOnly"].as_bool(), settings["commit"].as_str(), settings["editAction"].as_str()), (Some(false), Some("explicit"), Some("textEdit")), "an editable text window is the kit's explicit draft");
}
