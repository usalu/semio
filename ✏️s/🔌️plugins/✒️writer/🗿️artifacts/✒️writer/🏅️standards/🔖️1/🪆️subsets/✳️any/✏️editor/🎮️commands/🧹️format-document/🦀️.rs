//! ✍️ ✍️ Writer play app commands command — `format-document`.

use crate::op::{EditText, WriterMutation};
use crate::schema::format_writer_text;
use crate::{writer_text, WriterSnapshot};
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "format-document")]
pub struct FormatDocument {}

pub fn handle(_payload: &FormatDocument, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let text = writer_text(document);
    let formatted = format_writer_text(&text, &document.language_id);
    let mut emit = Emit::config(vec![WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { value: config.format_signal + 1 })]);
    if formatted != text {
        emit.artifact_mutations = vec![WriterMutation::EditText(EditText { text: formatted })];
    }
    Ok(emit)
}
