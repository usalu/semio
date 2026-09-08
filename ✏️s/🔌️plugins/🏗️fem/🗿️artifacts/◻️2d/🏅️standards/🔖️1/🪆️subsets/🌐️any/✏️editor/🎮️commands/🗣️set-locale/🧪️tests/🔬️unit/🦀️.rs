
use super::*;
use crate::editor::fem2d::Fem2dCommand;
use crate::editor::fem2d::testkit::{dispatch, fem2d_app};

#[semio_framework_async_macros::async_test]
async fn set_locale_action_writes_config_not_artifact_mutations() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, Fem2dCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    assert!(result.mutations.is_empty());
    assert_eq!(app.snapshot().expect("snapshot"), before);
}
