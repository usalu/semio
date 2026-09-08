
use super::*;
use crate::editor::gis2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_layer_toggles() {
    let mut app = app().await;
    assert!(render_body(&mut app, GIS2D_PLAY_BODY_CATALOGUE).await.contains("gis2d-play-catalogue.layer.water"));
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_catalogue_tab_to_this_body() {
    assert_eq!(definition().body_key.as_deref(), Some(GIS2D_PLAY_BODY_CATALOGUE));
}
