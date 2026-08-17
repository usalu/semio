//! ✍️ ✍️ Writer play app commands command — `set-active-example`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::editor::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
