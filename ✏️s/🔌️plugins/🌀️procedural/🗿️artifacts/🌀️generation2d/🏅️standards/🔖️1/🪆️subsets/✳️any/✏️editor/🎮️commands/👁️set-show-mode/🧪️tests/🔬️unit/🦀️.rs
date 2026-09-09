use super::*;
use crate::editor::generation2d::testkit::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_show_mode_is_config_only() {
    let mut app = app().await;
    let before = snapshot_read(&app);
    dispatch(&mut app, Generation2dCommand::SetShowMode(SetShowMode { value: "wire".into() })).await;
    let after = snapshot_read(&app);
    let unchanged = after == before;
    close(app);
    assert!(unchanged, "setShowMode writes only the config lane");
}
