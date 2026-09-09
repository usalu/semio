use super::*;
use crate::editor::shooting::testkit::{render as render_body, shooting_app};

#[semio_framework_async_macros::async_test]
async fn document_lists_shots_and_assets() {
    let mut app = shooting_app().await;
    let json = render_body(&mut app, SHOOTING_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("Overview Svg"));
    assert!(json.contains("Base"));
}
