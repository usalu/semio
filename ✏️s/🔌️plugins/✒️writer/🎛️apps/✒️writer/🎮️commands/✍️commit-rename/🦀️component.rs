//! ✍️ ✍️ Writer play app commands command — `commit-rename`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::apps::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
