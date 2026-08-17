//! ✍️ ✍️ Writer play app commands command — `format-document`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::editor::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
