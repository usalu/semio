use super::*;

#[test]
fn inspector_boolean_controls_and_literal_text_have_distinct_accessible_controls() {
    let boolean = editable_input("boolean", "Enabled", "shape", "enabled", None, "true", None).unwrap();
    let text = editable_input("text", "Text", "note", "text", None, "true", Some(InputKind::LongText)).unwrap();
    let boolean = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(boolean)).unwrap();
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(text)).unwrap();
    assert!(boolean.contains("checkbox"));
    assert!(boolean.contains("Enabled"));
    assert!(text.contains("longText"));
    assert!(text.contains("Text"));
}
