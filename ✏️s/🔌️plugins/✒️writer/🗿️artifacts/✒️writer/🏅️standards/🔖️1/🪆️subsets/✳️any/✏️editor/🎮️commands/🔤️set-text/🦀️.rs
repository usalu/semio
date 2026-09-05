//! ✍️ ✍️ Writer play app commands command — `set-text`.

use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-text")]
pub struct SetText {
    pub text: String,
}

/// 🪙️ A discrete document replacement (unlike `TextEdit`'s keystroke bursts) — each call is its own
/// undo step, so it must NOT share `TextEdit`'s coalescing key.
pub fn handle(payload: &SetText, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: payload.text.clone() })]))
}
