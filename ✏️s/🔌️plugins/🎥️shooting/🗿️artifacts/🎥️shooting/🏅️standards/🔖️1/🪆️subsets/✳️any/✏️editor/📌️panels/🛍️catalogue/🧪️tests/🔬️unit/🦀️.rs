use super::*;
use crate::editor::shooting::testkit::{render as render_body, shooting_app};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_the_shot_presets_and_glb_asset() {
    let mut app = shooting_app().await;
    let json = render_body(&mut app, SHOOTING_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("Add Shot"));
    assert!(json.contains("Add Asset"));
    assert!(json.contains("SVG Rectangle"));
    assert!(json.contains("GLB Asset"));
}
