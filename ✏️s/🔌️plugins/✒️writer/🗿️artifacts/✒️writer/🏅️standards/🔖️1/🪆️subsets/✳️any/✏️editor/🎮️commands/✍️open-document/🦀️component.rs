//! ✍️ ✍️ Writer play app commands command — `open-document`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::editor::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
