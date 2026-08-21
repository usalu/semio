//! 💬️ 💬️ Writer play app commands command — `engagement-input`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub async fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    if payload.value != config.engagement_input {
        Ok(Emit::config(vec![WriterConfigMutation::SetEngagementInput { value: payload.value.clone() }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
    } else {
        Ok(Emit::default())
    }
}
