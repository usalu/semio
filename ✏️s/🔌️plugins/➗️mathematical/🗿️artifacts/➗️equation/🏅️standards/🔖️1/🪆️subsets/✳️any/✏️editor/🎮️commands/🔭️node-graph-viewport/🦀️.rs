//! 🕸️ 🕸️ Equation play app commands command — `node-graph-viewport`.

use crate::op::EquationMutation;
use crate::EquationSnapshot;
use semio_framework::Viewport2d;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 👁️ Config-only: the node-graph viewport never touches the document — it's written into `cfg`,
/// session-only, no VCS edit, no undo entry on the document store.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub viewport: Viewport2d,
}

pub fn handle(_payload: &NodeGraphViewport, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
