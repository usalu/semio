//! ✍️ Writer play app commands — document-mutating text operations: typed edits, discrete replacements,
//! JSON/fixture setters, example loading, formatting and rename commit.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::apps::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️TextEdit
pub mod text_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "text-edit")]
    pub struct TextEdit {
        pub text: String,
    }

    /// ⌨️ Keystroke-granular edits coalesce under a stable key so a typing burst amends into a few undo
    /// steps, not one-per-keystroke. Any interrupting command applies without this key and breaks the
    /// coalescing run.
    pub fn handle(payload: &TextEdit, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::amend(vec![WriterMutation::EditText(EditText { text: payload.text.clone() })], "writer-text-edit"))
    }
}
//#endregion 🔖️TextEdit

//#region 🔖️SetText
pub mod set_text {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-text")]
    pub struct SetText {
        pub text: String,
    }

    /// 🪙️ A discrete document replacement (unlike `TextEdit`'s keystroke bursts) — each call is its own
    /// undo step, so it must NOT share `TextEdit`'s coalescing key.
    pub fn handle(payload: &SetText, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: payload.text.clone() })]))
    }
}
//#endregion 🔖️SetText

//#region 🔖️SetSnapshot
pub mod set_snapshot {
    use super::*;

    /// 🔧️ `snapshot` is JSON text, not a nested `#[dsl(block)]` struct field — `WriterSnapshot` no
    /// longer implements `dsl::DslField` now that `document` is a composed `ArtifactChild<S>` slot
    /// (no `DslField` impl reachable from this crate, same gap `📐️cad`/`💠️lowpoly` hit for their own
    /// composed-child snapshot types). Functionally identical to `SetSnapshotJson` — kept as its own
    /// row for wire-format/manifest stability rather than folding the two together mid-ticket.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snapshot")]
    pub struct SetSnapshot {
        pub json: String,
    }

    pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(parse_document_json(&payload.json))
    }
}
//#endregion 🔖️SetSnapshot

//#region 🔖️OpenDocument
pub mod open_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-document")]
    pub struct OpenDocument {
        pub uri: String,
        pub text: String,
    }

    pub fn handle(payload: &OpenDocument, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let id = payload.uri.rsplit('/').next().unwrap_or("document").to_string();
        let ext = payload.uri.rsplit('.').next().filter(|s| *s != &id);
        let language_id = dsl::language_for_semio_content(payload.text.as_bytes())
            .or_else(|| ext.and_then(|e| dsl::language_for_extension(e)))
            .map(|spec| spec.id.to_string())
            .unwrap_or_else(|| "plaintext".to_string());
        eprintln!(
            "[DEBUG] writer.open_document uri={} language_id={} text_len={}",
            payload.uri,
            language_id,
            payload.text.len()
        );
        let document = writer_snapshot_with_text(crate::artifacts::writer::WRITER_DOCUMENT_SCHEMA, &id, &language_id, &payload.uri, &payload.text);
        Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
    }
}
//#endregion 🔖️OpenDocument

//#region 🔖️JsonSetters
/// 🙈️ Shared body for `SetSnapshotJson`/`SetFixtureJson` — both replace the whole document from a raw
/// JSON string, silently no-op'ing on a parse failure (dev-only chrome setters, never user-facing).
fn parse_document_json(json: &str) -> Emit<WriterMutation, WriterConfigMutation> {
    match serde_json::from_str::<WriterSnapshot>(json) {
        Ok(document) => Emit { effects: vec![reset_document_effect(&document)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}

pub mod set_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document-json")]
    pub struct SetSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &SetSnapshotJson, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(parse_document_json(&payload.json))
    }
}

pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    pub fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(parse_document_json(&payload.json))
    }
}
//#endregion 🔖️JsonSetters

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;
    use crate::artifacts::writer::schema::empty_writer_snapshot;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let document = match payload.example_id.as_str() {
            "jack" => jack_example_document(),
            "dag.jack" => dag_jack_example_document(),
            _ => empty_writer_snapshot(),
        };
        Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️FormatDocument
pub mod format_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "format-document")]
    pub struct FormatDocument {}

    pub fn handle(_payload: &FormatDocument, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let text = writer_text(document);
        let formatted = format_writer_text(&text, &document.language_id);
        let mut emit = Emit::config(vec![WriterConfigMutation::SetFormatSignal { value: config.format_signal + 1 }]);
        if formatted != text {
            emit.artifact_mutations = vec![WriterMutation::EditText(EditText { text: formatted })];
        }
        Ok(emit)
    }
}
//#endregion 🔖️FormatDocument

//#region 🔖️CommitRename
pub mod commit_rename {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "commit-rename")]
    pub struct CommitRename {
        pub text: String,
    }

    pub fn handle(payload: &CommitRename, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let text = writer_text(document);
        let selection = config.editor_selection.clone().unwrap_or(WriterEditorSelection { start: 0, end: 0 });
        if selection.start == selection.end {
            if let Some(symbol) = jack_symbol_at_offset(&text, selection.start) {
                if symbol.kind == JackSymbolKind::Variable {
                    let renamed = apply_jack_rename(&text, &symbol.occurrences, &payload.text);
                    return Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: renamed })]));
                }
            }
        }
        if selection.start <= selection.end && selection.end <= text.len() {
            let mut updated = text.clone();
            updated.replace_range(selection.start..selection.end, &payload.text);
            return Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: updated })]));
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️CommitRename

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::writer::commands::text::{commit_rename, format_document, set_active_example};
    use crate::apps::writer::testkit::{app_with_jack, dispatch, new_app};
    use crate::apps::writer::WriterCommand;
    use crate::artifacts::writer::schema::jack_variable_occurrences;
    use crate::artifacts::writer::{writer_text, WriterSnapshot};
    use semio_framework::kernel::HostEffect;
    use semio_framework_plugin::PluginApp;

    const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

    /// 🌱️ Decodes the `HostEffect::LoadDocument` pack every whole-document-replace command now
    /// emits (`SetSnapshot` is banned — see `reset_document_effect`'s doc comment) — the standard
    /// way this file's tests observe a replaced document, mirroring `📐️cad`'s own
    /// `import_cad_file_action_imports_...` tests.
    fn loaded_document(result: &semio_framework_plugin::InvocationResult) -> WriterSnapshot {
        let HostEffect::LoadDocument { pack, .. } = result.requested_effects.first().expect("expected a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        <WriterSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack")
    }

    #[test]
    fn text_edit_burst_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for text in ["h", "he", "hel", "hell", "hello"] {
            dispatch(&mut app, WriterCommand::TextEdit(super::text_edit::TextEdit { text: text.into() }));
        }
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "hello");
        // The whole typing burst shares one coalesce key, so a single undo restores the pre-burst buffer
        // rather than backing out one keystroke at a time.
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "", "coalesced typing collapses to one undo step");
    }

    #[test]
    fn format_artifact_reformats_jack_query() {
        let mut app = app_with_jack();
        dispatch(&mut app, WriterCommand::SetText(super::set_text::SetText { text: "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name".into() }));
        let result = app.dispatch_typed(WriterCommand::FormatDocument(format_document::FormatDocument {}), &semio_framework_plugin::testkit::meta("local")).expect("format");
        assert_eq!(result.mutations.len(), 1);
        assert!(writer_text(&app.snapshot().expect("projection")).contains('\n'));
    }

    #[test]
    fn format_document_without_change_emits_no_operation() {
        // A no-operation format (already-formatted or non-jack empty doc) bumps the format signal but must
        // not record a history entry.
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::FormatDocument(format_document::FormatDocument {}), &semio_framework_plugin::testkit::meta("local")).expect("format");
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn set_text_action_updates_projection() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetText(super::set_text::SetText { text: "MATCH (a) RETURN a".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set text");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "MATCH (a) RETURN a");
    }

    #[test]
    fn set_text_undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        dispatch(&mut app, WriterCommand::SetText(super::set_text::SetText { text: "first".into() }));
        dispatch(&mut app, WriterCommand::SetText(super::set_text::SetText { text: "second".into() }));
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "second");
        let undo = app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert!(undo.mutations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "first");
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(writer_text(&app.snapshot().expect("projection")), "second");
    }

    #[test]
    fn commit_rename_renames_all_spans_at_the_config_selection() {
        let mut app = app_with_jack();
        let occurrences = jack_variable_occurrences(CANONICAL_QUERY, "a");
        assert_eq!(occurrences.len(), 3);
        let (start, _) = occurrences[0];
        // 🎯️ `CommitRename` reads the rename target off `WriterConfig::editor_selection` — set it via
        // a real selection command first (mirrors what the editor surface does before offering rename).
        dispatch(&mut app, WriterCommand::SetEditorSelection(crate::apps::writer::commands::selection::set_editor_selection::SetEditorSelection { start, end: start }));
        let result = app.dispatch_typed(WriterCommand::CommitRename(commit_rename::CommitRename { text: "piece".into() }), &semio_framework_plugin::testkit::meta("local")).expect("commit rename");
        assert_eq!(result.mutations.len(), 1);
        let text = writer_text(&app.snapshot().expect("projection"));
        assert_eq!(text.matches("piece").count(), 3);
        assert_eq!(text.matches("a:Piece").count(), 0);
    }

    /// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright) —
    /// `setActiveExample` now surfaces as a `HostEffect::LoadDocument` carrying the replacement
    /// document's pack bytes, exactly like `📐️cad`'s `importCadFile` (`reset_document_effect`).
    #[test]
    fn set_active_example_loads_jack_fixture() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }), &semio_framework_plugin::testkit::meta("local")).expect("load");
        assert!(result.mutations.is_empty(), "whole-document replace is an effect, not an in-history mutation");
        let projection = loaded_document(&result);
        assert_eq!(projection.id, "jack");
        assert!(writer_text(&projection).contains("MATCH"));
    }

    #[test]
    fn set_active_example_loads_dag_jack_fixture() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "dag.jack".into() }), &semio_framework_plugin::testkit::meta("local")).expect("load");
        assert!(result.mutations.is_empty());
        assert_eq!(loaded_document(&result).id, "dag-jack");
    }

    #[test]
    fn set_active_example_falls_back_to_empty_document() {
        let mut app = app_with_jack();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }), &semio_framework_plugin::testkit::meta("local")).expect("load");
        assert!(result.mutations.is_empty());
        let projection = loaded_document(&result);
        assert_eq!(projection.id, "empty");
        assert_eq!(writer_text(&projection), "");
    }
}
//#endregion 🧪️Tests
