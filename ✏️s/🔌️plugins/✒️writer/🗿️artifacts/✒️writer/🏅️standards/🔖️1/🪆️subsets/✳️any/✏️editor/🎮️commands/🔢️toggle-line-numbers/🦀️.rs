//! ⚙️ ⚙️ Writer play app commands command — `toggle-line-numbers`.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "toggle-line-numbers")]
pub struct ToggleLineNumbers {}

pub fn handle(_payload: &ToggleLineNumbers, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut settings = config.editor_settings.clone();
    settings.show_line_numbers = !settings.show_line_numbers;
    Ok(Emit::config(vec![WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
