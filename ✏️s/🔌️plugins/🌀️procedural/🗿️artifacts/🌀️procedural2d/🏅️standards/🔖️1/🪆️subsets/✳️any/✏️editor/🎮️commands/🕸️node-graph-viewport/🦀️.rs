//! 🕸️ 🕸️ Procedural2d play app commands command — `node-graph-viewport`.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub viewport_json: String,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    match dsl::json::from_json_str::<CameraJson>(&payload.viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![Procedural2dConfigMutation::SetCamera { camera }])),
        Err(_) => Ok(Emit::default()),
    }
}
