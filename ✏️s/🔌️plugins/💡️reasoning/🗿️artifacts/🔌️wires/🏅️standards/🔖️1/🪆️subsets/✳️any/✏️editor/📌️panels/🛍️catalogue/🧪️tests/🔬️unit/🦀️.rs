use super::*;
use crate::editor::wires::testkit::{metabolism_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_identity_and_relationship_kinds() {
    let mut app = metabolism_app().await;
    let json = render_body(&mut app, WIRES_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("Identity kinds"));
    assert!(json.contains("Relationship kinds"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_CATALOGUE));
}
