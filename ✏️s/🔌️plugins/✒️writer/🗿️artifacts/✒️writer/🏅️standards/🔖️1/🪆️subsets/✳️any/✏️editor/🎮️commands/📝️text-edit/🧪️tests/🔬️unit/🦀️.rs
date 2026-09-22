use crate::editor::writer::commands::{commit_rename, format_document, set_active_example, set_text};
use crate::editor::writer::unit_tests::context::{app_with_jack, dispatch, new_app};
use crate::editor::writer::WriterCommand;
use crate::schema::jack_variable_occurrences;
use crate::{writer_text, WriterSnapshot};
use semio_framework::kernel::Effect;
use semio_framework_plugin::PluginApp;

const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

/// 🌱️ Decodes the `Effect::LoadDocument` pack every whole-document-replace command now
/// emits (`SetSnapshot` is banned — see `reset_document_effect`'s doc comment) — the standard
/// way this file's tests observe a replaced document, mirroring `📐️cad`'s own
/// `import_cad_file_action_imports_...` tests.
fn loaded_document(result: &semio_framework_plugin::InvocationResult) -> WriterSnapshot {
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("expected a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    <WriterSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack")
}

#[semio_framework_async_macros::async_test]
async fn text_edit_burst_coalesces_into_one_undo_step() {
    let mut app = new_app().await;
    for text in ["h", "he", "hel", "hell", "hello"] {
        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: text.into() })).await;
    }
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "hello");
    // The whole typing burst shares one coalesce key, so a single undo restores the pre-burst buffer
    // rather than backing out one keystroke at a time.
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "undo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "", "coalesced typing collapses to one undo step");
}

#[semio_framework_async_macros::async_test]
async fn format_artifact_reformats_jack_query() {
    let mut app = app_with_jack().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name".into() })).await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::FormatDocument(format_document::FormatDocument {})).await;
    // 🧾️ A MOUNTED app never surfaces its operations in `result.mutations`: the edit lands through
    // the retained publication lane and `InvocationResult::mutations` stays empty (fleet-brief
    // stale-test bucket 3). Asserting a count of 1 there tested the UNMOUNTED shape this harness
    // stopped using; the projection assertion that follows is the real proof the edit landed.
    assert!(result.mutations.is_empty(), "a mounted dispatch publishes through its receipt lanes, not result.mutations: {:?}", result.mutations);
    assert!(writer_text(&app.snapshot().expect("projection")).contains('\n'));
}

#[semio_framework_async_macros::async_test]
async fn format_document_without_change_emits_no_operation() {
    // A no-operation format (already-formatted or non-jack empty doc) bumps the format signal but must
    // not record a history entry.
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::FormatDocument(format_document::FormatDocument {})).await;
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn set_text_action_updates_projection() {
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "MATCH (a) RETURN a".into() })).await;
    // 🧾️ Mounted dispatch — see the note on `format_artifact_reformats_jack_query`.
    assert!(result.mutations.is_empty(), "a mounted dispatch publishes through its receipt lanes, not result.mutations: {:?}", result.mutations);
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "MATCH (a) RETURN a");
}

#[semio_framework_async_macros::async_test]
async fn set_text_undo_redo_round_trips_through_the_wrapper() {
    let mut app = new_app().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "first".into() })).await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: "second".into() })).await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "second");
    let undo = crate::editor::writer::unit_tests::context::history_verb(&mut app, "undo").await;
    assert!(undo.mutations.is_empty());
    assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "first");
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "redo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "second");
}

#[semio_framework_async_macros::async_test]
async fn commit_rename_renames_all_spans_at_the_config_selection() {
    let mut app = app_with_jack().await;
    let occurrences = jack_variable_occurrences(CANONICAL_QUERY, "a");
    assert_eq!(occurrences.len(), 3);
    let (start, _) = occurrences[0];
    // 🎯️ `CommitRename` reads the rename target from the exact main-window transient selection — set it via
    // a real selection command first (mirrors what the editor surface does before offering rename).
    dispatch(&mut app, WriterCommand::SetEditorSelection(crate::editor::writer::commands::set_editor_selection::SetEditorSelection { start, end: start })).await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::CommitRename(commit_rename::CommitRename { text: "piece".into() })).await;
    // 🧾️ Mounted dispatch — see the note on `format_artifact_reformats_jack_query`.
    assert!(result.mutations.is_empty(), "a mounted dispatch publishes through its receipt lanes, not result.mutations: {:?}", result.mutations);
    let text = writer_text(&app.snapshot().expect("projection"));
    assert_eq!(text.matches("piece").count(), 3);
    assert_eq!(text.matches("a:Piece").count(), 0);
}

/// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright) —
/// `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the replacement
/// document's pack bytes, exactly like `📐️cad`'s `importCadFile` (`reset_document_effect`).
/// 📚️ `"demo"` is the ONE example id this subset PUBLISHES (`📚️examples/🎬️demo`), and the document it
/// loads is the jack fixture. The dispatched id used to be `"jack"`, which
/// `document_for_example_id` has never published — it fell through to the empty document, so this law
/// proved the opposite of its own name.
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_jack_fixture() {
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    assert!(result.mutations.is_empty(), "whole-document replace is an effect, not an in-history mutation");
    let projection = loaded_document(&result);
    assert_eq!(projection.id, "jack");
    assert!(writer_text(&projection).contains("MATCH"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_dag_jack_fixture() {
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "dag.jack".into() })).await;
    assert!(result.mutations.is_empty());
    assert_eq!(loaded_document(&result).id, "dag-jack");
}

/// 🪹 An id this app does not publish resets to the EMPTY document rather than faulting (the navbar
/// dispatches whatever its combobox holds). The EMPTY id is not that case: it means "load my default
/// example", exactly like `📽️animate`'s own `setActiveExample` — so this law now files an id the app
/// genuinely does not publish.
#[semio_framework_async_macros::async_test]
async fn set_active_example_falls_back_to_empty_document() {
    let mut app = app_with_jack().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "not-a-published-example".into() })).await;
    assert!(result.mutations.is_empty());
    let projection = loaded_document(&result);
    assert_eq!(projection.id, "empty");
    assert_eq!(writer_text(&projection), "");
}
