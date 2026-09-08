
use crate::editor::lowpoly::testkit::{app, render};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_primitives() {
    let mut a = app().await;
    let json = render(&mut a, super::LOWPOLY_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("lowpoly-play-catalogue.box"));
    assert!(json.contains("Cube"));
    assert!(json.contains("Ico Sphere"));
}
