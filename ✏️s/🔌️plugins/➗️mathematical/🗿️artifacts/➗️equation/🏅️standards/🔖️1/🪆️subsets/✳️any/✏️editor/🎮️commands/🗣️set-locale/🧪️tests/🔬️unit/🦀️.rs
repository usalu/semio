
use super::*;
use crate::editor::equation::EquationCommand;
use crate::editor::equation::testkit::math_app;

#[semio_framework_async_macros::async_test]
async fn set_locale_writes_config_not_mutations() {
    let mut app = math_app().await;
    let result = app.dispatch_typed(EquationCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("locale");
    assert!(result.mutations.is_empty(), "setLocale must not emit a VCS operation");
}
