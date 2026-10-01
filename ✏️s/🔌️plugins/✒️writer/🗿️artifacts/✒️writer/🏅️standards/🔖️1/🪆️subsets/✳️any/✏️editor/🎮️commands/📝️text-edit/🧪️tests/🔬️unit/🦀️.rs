use crate::editor::writer::commands::{commit_rename, format_document, set_active_example, set_text};
use crate::editor::writer::unit_tests::context::{app_with_jack, dispatch, end_typing_run, new_app, type_delivery};
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

/// ⌨️ One whole-text `textEdit` delivery of the main window's typing run.
async fn type_text(app: &mut crate::editor::writer::unit_tests::context::WriterApp, text: &str, now_ms: u64) {
    type_delivery(app, "textEdit", vec![("text".into(), dsl::DslValue::String(text.into()))], now_ms).await;
}

/// ⚖️ LAW (design §13.2): a typing burst is ONE run — nothing lands while it is open (the render reads it as the overlay), the
/// host's commit signal publishes it as ONE edit stamped with its `TransactionRef`, and ONE undo reverts the whole burst.
#[semio_framework_async_macros::async_test]
async fn a_typing_burst_is_one_run_one_edit_and_one_undo_step() {
    let mut app = new_app().await;
    let edits = app.edit_transactions().len();
    for (index, text) in ["h", "he", "hel", "hell", "hello"].into_iter().enumerate() {
        type_text(&mut app, text, 10_000 + 100 * index as u64).await;
    }
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "", "nothing lands while the run is open");
    assert_eq!(writer_text(&app.rendered_snapshot()), "hello", "every render reads the run");
    assert_eq!(app.edit_transactions().len(), edits, "no history micro-mutation");
    end_typing_run(&mut app, "textEdit", "blur", 10_500).await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "hello");
    let transactions = app.edit_transactions();
    assert_eq!(transactions.len(), edits + 1, "one run is one edit");
    let transaction = transactions.last().cloned().flatten().expect("the run's edit carries its TransactionRef");
    assert!(transaction.id.starts_with("tx-") && transaction.tool.ends_with("#textEdit"), "{transaction:?}");
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "undo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), "", "one undo reverts the whole run");
}

/// ⌨️ A typing run far longer than the store's fixed applied-edit ledger (64) — 1137 typed characters with pauses, caret moves
/// and corrections, one full-text `text-edit` per changed key, never idle past the run's bound — is ONE run: it never spends the
/// ledger, lands as ONE edit, and ONE undo reverts it, ONE redo restores it (ticket 26/09/23 F1, typing census).
#[semio_framework_async_macros::async_test]
async fn a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step() {
    let run = semio_framework_plugin::artifact_app_laws::typing_run();
    assert!(run.texts.len() > 64 * 10, "the run must outlast the edit ledger many times over");
    let mut app = new_app().await;
    dispatch(&mut app, WriterCommand::SetText(set_text::SetText { text: run.initial.clone() })).await;
    for (index, text) in run.texts.iter().enumerate() {
        type_text(&mut app, text, 20_000 + 40 * index as u64).await;
        semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *app);
    }
    assert_eq!(writer_text(&app.rendered_snapshot()), run.expected);
    end_typing_run(&mut app, "textEdit", "idle", 20_000 + 40 * run.texts.len() as u64 + 750).await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.expected);
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "undo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.initial, "one undo reverts the whole run");
    crate::editor::writer::unit_tests::context::history_verb(&mut app, "redo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.expected, "one redo restores the whole run");
}

/// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): ONE uninterrupted 10 000-keystroke typing run at the maximum rate — one
/// full-text `text-edit` per key with corrections, no pressure drain between keys — applies every key in order to what the
/// window shows, never faults, and costs the same per key at its end as at its start: a key folds into the run's ONE net leaf
/// and refolds the overlay, never touching the store (the median key of the last 1 000 must stay within 3 × the first 1 000).
/// The run then lands as ONE edit, and maintenance returns its retirements below their pressure bound.
#[semio_framework_async_macros::async_test]
async fn a_ten_thousand_keystroke_burst_applies_every_key_in_order() {
    const KEYS: [char; 8] = ['a', 'q', 'ß', 'ü', '€', '𝄞', ' ', '\n'];
    let mut app = new_app().await;
    let edits = app.edit_transactions().len();
    let mut model = String::new();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut key_nanos = Vec::with_capacity(10_000);
    for index in 0..10_000usize {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        if index % 7 == 6 {
            model.pop();
        } else {
            if model.chars().count() == 48 {
                model.remove(0);
            }
            model.push(KEYS[(seed >> 33) as usize % KEYS.len()]);
        }
        let started = std::time::Instant::now();
        type_text(&mut app, &model, 100_000 + 10 * index as u64).await;
        key_nanos.push(started.elapsed().as_nanos());
        assert_eq!(writer_text(&app.rendered_snapshot()), model, "key {index} applies in order");
    }
    assert_eq!(app.edit_transactions().len(), edits, "the open run never spends the edit ledger");
    let median = |window: &[u128]| {
        let mut sorted = window.to_vec();
        sorted.sort_unstable();
        sorted[sorted.len() / 2]
    };
    let (first, last) = (median(&key_nanos[..1_000]), median(&key_nanos[9_000..]));
    assert!(last <= first.saturating_mul(3), "per-key cost stays flat over one uninterrupted run: the median key of the last 1 000 took {last} ns against {first} ns for the first 1 000");
    end_typing_run(&mut app, "textEdit", "blur", 300_000).await;
    assert_eq!((writer_text(&app.snapshot().expect("projection")), app.edit_transactions().len()), (model, edits + 1), "the run lands as ONE edit");
    semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *app);
    assert!(!PluginApp::maintenance_under_pressure(&*app), "maintenance returns the burst's retirements below their pressure bound");
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
    dispatch(&mut app, WriterCommand::SetEditorSelection(crate::editor::writer::commands::set_editor_selection::SetEditorSelection { start, end: start, splice: 0 })).await;
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
