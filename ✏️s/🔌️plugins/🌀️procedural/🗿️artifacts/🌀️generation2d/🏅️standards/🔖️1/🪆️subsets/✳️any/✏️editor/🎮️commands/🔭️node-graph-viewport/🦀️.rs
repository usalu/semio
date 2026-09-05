//! 🕸️ 🕸️ Generation2d play app commands command — `node-graph-viewport`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub viewport_json: String,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    match dsl::json::from_json_str::<CameraJson>(&payload.viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![Generation2dConfigMutation::SetCamera { camera }])),
        Err(_) => Ok(Emit::default()),
    }
}
