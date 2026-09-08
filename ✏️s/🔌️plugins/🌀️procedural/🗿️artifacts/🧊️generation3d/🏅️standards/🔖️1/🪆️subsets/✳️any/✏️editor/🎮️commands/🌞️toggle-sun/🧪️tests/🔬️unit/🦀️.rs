
use super::*;
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn toggle_sun_never_mutates_the_document() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation3dCommand::ToggleSun(ToggleSun {})).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "toggleSun must not mutate the document");
}
