
use super::*;
use crate::editor::generation2d::Generation2dCommand;
use crate::editor::generation2d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn set_locale_updates_config_locale() {
    let mut app = app().await;
    dispatch(&mut app, Generation2dCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
}
