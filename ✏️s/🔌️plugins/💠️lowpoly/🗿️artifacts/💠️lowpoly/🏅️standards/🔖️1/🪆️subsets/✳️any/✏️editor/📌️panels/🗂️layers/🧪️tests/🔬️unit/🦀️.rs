use crate::editor::lowpoly::unit_tests::context::{app, render};

#[semio_framework_async_macros::async_test]
async fn layers_panel_lists_the_base_layer() {
    let mut a = app().await;
    let json = render(&mut a, super::LOWPOLY_PLAY_BODY_LAYERS).await;
    assert!(json.contains("lowpoly-layer:0"));
}
