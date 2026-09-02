//! 🔍️ 🔍️ Writer play app commands command — `request-completions`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "request-completions")]
pub struct RequestCompletions {}

pub fn handle(_payload: &RequestCompletions, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}
