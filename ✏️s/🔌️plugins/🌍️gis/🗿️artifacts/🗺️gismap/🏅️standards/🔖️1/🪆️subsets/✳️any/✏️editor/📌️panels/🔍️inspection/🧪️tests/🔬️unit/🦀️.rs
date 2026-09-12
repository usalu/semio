use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_schema_and_visible_count() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION).await;
    assert!(json.contains(GIS_MAP_SCHEMA));
    assert!(json.contains(&format!("{}/{}", GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_IDS.len())));
    drop(json);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_inspection_tab_to_this_body() {
    let definition = definition();
    assert!(matches!(definition.group, PanelGroup::Details));
    assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_INSPECTION));
}
