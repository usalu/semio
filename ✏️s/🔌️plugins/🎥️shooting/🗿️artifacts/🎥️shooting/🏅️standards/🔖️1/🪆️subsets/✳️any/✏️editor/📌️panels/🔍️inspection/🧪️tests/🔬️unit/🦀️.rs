use super::*;
use crate::editor::shooting::unit_tests::context::{render as render_body, shooting_app};

#[semio_framework_async_macros::async_test]
async fn inspector_falls_back_to_the_active_shot() {
    let mut app = shooting_app().await;
    let json = render_body(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await;
    assert!(json.contains("Shot"));
}
