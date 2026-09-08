
use super::*;
use crate::editor::vcs::VcsCommand;
use crate::editor::vcs::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn vcs_demo_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::SetLocale(SetLocale { value: "de-DE".into() }));
}

/// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more passing
/// a `ViewModel` into `render`/`app_labels` for this purpose (mirrors `shooting_ui`'s identical test).
#[semio_framework_async_macros::async_test]
async fn vcs_labels_resolve_german_locale() {
    use crate::editor::vcs::{VCS_PLAY_BODY_DOCUMENT, VCS_PLAY_BODY_EDITOR, VCS_PLAY_BODY_INSPECTION};
    let mut instance = app().await;
    dispatch(&mut instance, VcsCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;

    let editor = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_EDITOR).await;
    assert!(editor.contains("Aktionen"));
    assert!(editor.contains("Rückgängig"));
    assert!(editor.contains("Wiederholen"));
    assert!(editor.contains("Zähler"));

    let inspection = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_INSPECTION).await;
    assert!(inspection.contains("Titel"));
    assert!(inspection.contains("Notizen"));
    assert!(inspection.contains("Schlagwörter"));

    let document_tree = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_DOCUMENT).await;
    assert!(document_tree.contains("Alternativen"));
    assert!(document_tree.contains("Checkpoints"));
    assert!(!document_tree.contains("\"Alternatives\""));
}
