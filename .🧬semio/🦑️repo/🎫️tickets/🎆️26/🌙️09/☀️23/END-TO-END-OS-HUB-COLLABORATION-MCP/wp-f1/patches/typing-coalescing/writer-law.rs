
/// ⌨️ A typing run far longer than the store's fixed applied-edit ledger (64) — 1137 typed characters with pauses, caret moves
/// and corrections, one full-text `text-edit` per changed key — saves every key, and ONE undo reverts the whole run, ONE redo
/// restores it (ticket 26/09/23 F1, typing census).
#[semio_framework_async_macros::async_test]
async fn a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step() {
    let run = semio_framework_plugin::artifact_app_laws::typing_run();
    assert!(run.texts.len() > 64 * 10, "the run must outlast the edit ledger many times over");
    let mut app = new_app().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: run.initial.clone() })).await;
    for text in &run.texts {
        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: text.clone() })).await;
    }
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.expected);
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "undo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.initial, "one undo reverts the whole run");
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "redo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.expected, "one redo restores the whole run");
}
