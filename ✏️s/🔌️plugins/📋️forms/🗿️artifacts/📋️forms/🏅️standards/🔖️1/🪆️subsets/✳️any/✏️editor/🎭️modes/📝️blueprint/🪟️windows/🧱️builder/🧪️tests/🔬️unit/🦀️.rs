
use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_blueprint_builder_cards() {
    let spec = FormsSnapshot::default();
    let config = FormsConfig::default();
    let labels = crate::editor::forms::terminology::forms_play_labels(&config);
    let node = render(&spec, &config, labels).expect("blueprint surface");
    let semio_framework_ui_contract::Component::Surface(props) = node.component else { panic!("blueprint must render a semantic surface") };
    let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).expect("block-list payload");
    let expected = crate::mutations::as_playbook_spec(&spec);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.steps_json).unwrap(), serde_json::to_value(&expected.steps).unwrap());
    assert!(scene.selected_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_block_list_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, FORMS_PLAY_BODY_BLUEPRINT);
    assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
}
