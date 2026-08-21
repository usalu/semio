//! 🧪️ 🧪️ Forms play app commands command — `reset-try`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reset-try")]
pub struct ResetTry {}

pub async fn handle(_payload: &ResetTry, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(reset_try_config_mutations()))
}
