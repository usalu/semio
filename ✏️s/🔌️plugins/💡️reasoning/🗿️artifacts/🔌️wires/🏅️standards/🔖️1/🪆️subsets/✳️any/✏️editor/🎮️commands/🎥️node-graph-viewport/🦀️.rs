//! 🎥️ Routes one canvas viewport into its concrete window configuration owner.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use semio_framework::Viewport2d;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub viewport: Viewport2d,
}

pub fn handle(_payload: &NodeGraphViewport, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
