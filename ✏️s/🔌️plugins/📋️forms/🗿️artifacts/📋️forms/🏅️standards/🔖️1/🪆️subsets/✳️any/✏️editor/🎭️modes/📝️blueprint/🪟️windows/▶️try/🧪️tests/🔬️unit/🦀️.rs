use super::*;
use crate::editor::forms::unit_tests::context::{building_component_contributions, building_component_question, forms_app, render as render_body};
use crate::editor::forms::FORMS_PLAY_BODY_TRY as BODY_TRY;

#[semio_framework_async_macros::async_test]
async fn renders_try_wizard() {
    let mut app = forms_app().await;
    crate::editor::forms::unit_tests::context::dispatch(&mut app, crate::editor::forms::FormsCommand::SetActiveExample(crate::editor::forms::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    let json = render_body(&mut app, BODY_TRY).await;
    assert!(json.contains("forms-try"));
    assert!(json.contains("Step 1"));
}

#[semio_framework_async_macros::async_test]
async fn image_question_with_url_src_emits_image_node() {
    let question = FormQuestion { src: Some("https://example.com/picture.png".into()), ..crate::editor::forms::questions::question_shell("q-image".into(), "Picture".into(), "image".into()) };
    let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&semio_framework_plugin::ViewModel::default()), &semio_framework_plugin::ViewModel::default());
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("semantic component"))).expect("component JSON");
    assert!(json.contains(r#""type":"image""#));
    assert!(json.contains("https://example.com/picture.png"));
}

#[semio_framework_async_macros::async_test]
async fn extension_question_emits_external_slot_when_contribution_registered() {
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("fill-extension".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "fill-extension".into(), window_kind_id: FORMS_PLAY_WINDOW_TRY.into() }],
        ..Default::default()
    };
    let node = render_try_question(&building_component_question(), &Object::new(), &building_component_contributions(), Some("Required"), crate::editor::forms::terminology::forms_play_labels(&view), &view);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("semantic component"))).expect("component JSON");
    assert!(json.contains("\"type\":\"extension\""));
    assert!(json.contains("forms-module-procedural"));
    assert!(json.contains("fill-extension"));
    assert!(json.contains("windowKindId"));
    assert!(json.contains("Geometry"));
    assert!(json.contains("Required"));
}

#[semio_framework_async_macros::async_test]
async fn extension_question_falls_back_without_contribution() {
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("fill-extension".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "fill-extension".into(), window_kind_id: FORMS_PLAY_WINDOW_TRY.into() }],
        ..Default::default()
    };
    let node = render_try_question(&building_component_question(), &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&view), &view);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("semantic component"))).expect("component JSON");
    assert!(json.contains("Extension unavailable"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, FORMS_PLAY_BODY_TRY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn extension_answer_controls_require_a_live_fill_window() {
    let view = semio_framework_plugin::ViewModel::default();
    let result = render_try_question(&building_component_question(), &Object::new(), &building_component_contributions(), None, crate::editor::forms::terminology::forms_play_labels(&view), &view);
    assert!(result.is_err());
}
