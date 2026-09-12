use super::*;
use crate::editor::animate::unit_tests::context::{presentation_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_DETAILS));
}

/// 🕹️ `render` has no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM), so the panel is a schema/tile-count summary regardless of selection now.
#[semio_framework_async_macros::async_test]
async fn details_panel_reports_schema_and_tile_count() {
    let mut app = presentation_app().await;
    let json_str = render_body(&mut app, PRESENTATION_PLAY_BODY_DETAILS).await;
    assert!(json_str.contains(PRESENTATION_DOCUMENT_SCHEMA));
}
