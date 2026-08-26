//! ✍️ ✍️ Writer play app commands command — `format-document`.

use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::schema::format_writer_text;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
