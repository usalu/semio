//! 🔍️ 🔍️ Writer play app commands command — `request-completions`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "request-completions")]
pub struct RequestCompletions {}

pub async fn handle(_payload: &RequestCompletions, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}
