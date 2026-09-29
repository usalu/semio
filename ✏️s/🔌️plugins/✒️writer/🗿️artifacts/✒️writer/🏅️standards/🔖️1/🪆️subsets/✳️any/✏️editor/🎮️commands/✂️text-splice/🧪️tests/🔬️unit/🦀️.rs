use crate::editor::writer::commands::set_text;
use crate::editor::writer::unit_tests::context::{dispatch, history_verb, new_app};
use crate::editor::writer::WriterCommand;
use crate::writer_text;
use semio_framework_plugin::PluginApp;

fn keystroke(previous: &str, next: &str, seq: u64) -> WriterCommand {
    let splice = semio_framework_plugin::TextSplice::from_edit(previous, next, semio_framework_plugin::TEXT_SPLICE_CONTEXT_SCALARS).expect("a keystroke changes the text");
    let caret = next.len();
    WriterCommand::TextSplice(super::TextSplice { start: splice.start, deleted: splice.deleted, insert: splice.insert, before: splice.before, after: splice.after, seq, anchor: caret, caret })
}

/// ⌨️ A typing run sent as one splice per keystroke lands as the typed text and ONE undo reverts it (same coalescing key as
/// the whole-text verb it replaces for splice-typing hosts).
#[semio_framework_async_macros::async_test]
async fn a_spliced_typing_run_lands_and_undoes_as_one_step() {
    let mut app = new_app().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "Doc: ".into() })).await;
    let mut text = String::from("Doc: ");
    for (seq, char) in "hello".chars().enumerate() {
        let next = format!("{text}{char}");
        dispatch(&mut app, keystroke(&text, &next, seq as u64 + 1)).await;
        text = next;
    }
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "Doc: hello");
    history_verb(&mut app, "undo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "Doc: ", "one undo reverts the spliced run");
}

/// 🧷️ A splice authored against an older text still lands between its context after the text changed elsewhere.
#[semio_framework_async_macros::async_test]
async fn a_splice_relocates_by_its_context() {
    let mut app = new_app().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "oh, hello world".into() })).await;
    let authored = keystroke("hello world", "hello big world", 1);
    dispatch(&mut app, authored).await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "oh, hello big world");
}
