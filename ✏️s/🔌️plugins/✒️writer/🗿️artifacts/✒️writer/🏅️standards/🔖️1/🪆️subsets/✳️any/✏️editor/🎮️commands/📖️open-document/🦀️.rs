//! ✍️ ✍️ Writer play app commands command — `open-document`.

use crate::editor::writer::reset_document_effect;
use crate::op::WriterMutation;
use crate::{writer_snapshot_with_text, WriterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-document")]
pub struct OpenDocument {
    pub uri: String,
    pub text: String,
}

pub(crate) fn emit(payload: &OpenDocument) -> Emit<WriterMutation, NoConfigMutation> {
    let id = payload.uri.rsplit('/').next().unwrap_or("document").to_string();
    let ext = payload.uri.rsplit('.').next().filter(|s| *s != &id);
    let language_id = dsl::language_for_semio_content(payload.text.as_bytes()).or_else(|| ext.and_then(|e| dsl::language_for_extension(e))).map(|spec| spec.id.to_string()).unwrap_or_else(|| "plaintext".to_string());
    eprintln!("[DEBUG] writer.open_document uri={} language_id={} text_len={}", payload.uri, language_id, payload.text.len());
    let document = writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, &id, &language_id, &payload.uri, &payload.text);
    Emit { effects: vec![reset_document_effect(&document)], ..Default::default() }
}

pub fn handle(payload: &OpenDocument, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(emit(payload))
}
