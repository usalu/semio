//! 🔍️ 🔍️ Writer play app commands command — `lint-document`.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "lint-document")]
pub struct LintDocument {}

pub fn handle(_payload: &LintDocument, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { value: config.lint_signal + 1 }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
