//! 🕸️ 🕸️ Equation play app commands command — `node-graph-viewport`.

use crate::artifacts::equation::op::EquationMutation;
use crate::artifacts::equation::{EquationCamera, EquationSnapshot};
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 👁️ Config-only: the node-graph viewport never touches the document — it's written into `cfg`,
/// session-only, no VCS edit, no undo entry on the document store.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub camera: EquationCamera,
}

pub async fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    Ok(Emit::config(vec![EquationConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}
