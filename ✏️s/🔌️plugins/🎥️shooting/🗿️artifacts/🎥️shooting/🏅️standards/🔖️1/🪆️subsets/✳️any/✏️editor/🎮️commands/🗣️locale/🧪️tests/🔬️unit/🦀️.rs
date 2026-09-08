
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};

#[semio_framework_async_macros::async_test]
async fn set_locale_switches_the_resolved_label_locale() {
    use crate::editor::shooting::SHOOTING_PLAY_BODY_DOCUMENT;
    use crate::editor::shooting::testkit::render;

    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
    assert!(result.mutations.is_empty(), "locale is config-only");
    assert!(render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT).await.contains("Aufnahmen"), "the document panel now resolves German labels");
}
