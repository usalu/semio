//! ⚙️ ⚙️ Writer play app commands command — `set-tab-size`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "tab-size")]
pub struct SetTabSize {
    pub value: u32,
}

pub fn handle(payload: &SetTabSize, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut settings = config.editor_settings.clone();
    settings.tab_size = payload.value.max(1);
    Ok(Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}
