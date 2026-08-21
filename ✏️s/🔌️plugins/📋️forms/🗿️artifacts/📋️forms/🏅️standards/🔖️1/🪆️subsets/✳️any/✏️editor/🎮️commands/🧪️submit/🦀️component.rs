//! 🧪️ 🧪️ Forms play app commands command — `submit`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "submit")]
pub struct Submit {}

pub async fn handle(_payload: &Submit, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}
