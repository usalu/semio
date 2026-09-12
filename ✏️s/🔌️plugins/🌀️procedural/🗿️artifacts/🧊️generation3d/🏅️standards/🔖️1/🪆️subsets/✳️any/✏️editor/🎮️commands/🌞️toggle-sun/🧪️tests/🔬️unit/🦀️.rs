use super::*;
use crate::editor::generation3d::unit_tests::context::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn toggle_sun_never_mutates_the_document() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::ToggleSun(ToggleSun {})).await;
    assert_eq!(context::snapshot(&app), before, "toggleSun must not mutate the document");
}
