
use super::*;
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn set_locale_updates_config_locale() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
}
