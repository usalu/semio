//! ⚙️ ⚙️ Writer play app commands command — `set-line-height`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "line-height")]
pub struct SetLineHeight {
    pub value: u32,
}

pub async fn handle(payload: &SetLineHeight, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut settings = config.editor_settings.clone();
    settings.line_height = payload.value;
    Ok(Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}
