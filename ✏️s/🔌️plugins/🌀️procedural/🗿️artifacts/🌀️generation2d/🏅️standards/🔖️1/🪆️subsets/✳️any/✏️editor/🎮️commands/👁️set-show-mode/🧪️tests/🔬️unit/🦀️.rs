use super::*;
use crate::editor::generation2d::testkit::{app, dispatch};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_show_mode_is_config_only() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation2dCommand::SetShowMode(SetShowMode { value: "wire".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot"), before);
}
