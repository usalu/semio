
use super::*;
use semio_framework_plugin::Component;

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
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert_eq!(scene.buffer, "playground.custom");
    assert_eq!(scene.language.as_deref(), Some("playground"));
    assert_eq!(scene.settings_json, None, "editable window must not stamp readOnly");
}
