//! ✍️ ✍️ Writer play app commands command — `text-edit`.

use crate::op::{EditText, WriterMutation};
use crate::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
