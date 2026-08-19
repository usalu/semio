//! ✍️ ✍️ Writer play app commands command — `set-text`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-text")]
pub struct SetText {
    pub text: String,
}

/// 🪙️ A discrete document replacement (unlike `TextEdit`'s keystroke bursts) — each call is its own
/// undo step, so it must NOT share `TextEdit`'s coalescing key.
pub async fn handle(payload: &SetText, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: payload.text.clone() })]))
}
