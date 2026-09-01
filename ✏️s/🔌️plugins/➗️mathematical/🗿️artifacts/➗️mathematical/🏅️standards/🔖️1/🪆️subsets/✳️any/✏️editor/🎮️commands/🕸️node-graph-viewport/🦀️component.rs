//! 🕸️ 🕸️ Mathematical play app commands command — `node-graph-viewport`.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::{MathematicalCamera, MathematicalSnapshot};
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 👁️ Config-only: the node-graph viewport never touches the document — it's written into `cfg`,
/// session-only, no VCS edit, no undo entry on the document store.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub camera: MathematicalCamera,
}

pub async fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    Ok(Emit::config(vec![MathematicalConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}
