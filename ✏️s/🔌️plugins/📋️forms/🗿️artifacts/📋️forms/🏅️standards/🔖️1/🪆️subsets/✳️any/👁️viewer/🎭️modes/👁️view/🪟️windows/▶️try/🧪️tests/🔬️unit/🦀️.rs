use semio_plugin::SurfaceKind;
use crate::viewer::forms::modes::view::windows::try_wizard::{text};
use crate::viewer::forms::modes::view::windows::try_wizard::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_canvas2d_try_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::building_component_spec();
    let node = render(&document, &[], &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).unwrap();
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("\"container\""));
}

#[semio_framework_async_macros::async_test]
async fn render_falls_back_to_a_placeholder_for_an_empty_document() {
    let document = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[]);
    let node = render(&document, &[], &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).unwrap();
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("No steps"));
}
