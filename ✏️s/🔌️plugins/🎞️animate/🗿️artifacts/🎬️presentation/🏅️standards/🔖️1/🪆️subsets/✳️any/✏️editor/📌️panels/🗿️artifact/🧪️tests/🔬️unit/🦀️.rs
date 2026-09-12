use super::*;
use crate::editor::animate::unit_tests::context::{presentation_app, render as render_body};
use crate::editor::animate::PresentationCommand;

#[semio_framework_async_macros::async_test]
async fn document_lists_seeded_tiles() {
    use semio_framework_plugin::artifact_app_laws::meta;
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).await.expect("seed grid");
    let document = render_body(&mut app, PRESENTATION_PLAY_BODY_DOCUMENT).await;
    assert!(document.contains("tile-r0-c0"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_DOCUMENT));
}
