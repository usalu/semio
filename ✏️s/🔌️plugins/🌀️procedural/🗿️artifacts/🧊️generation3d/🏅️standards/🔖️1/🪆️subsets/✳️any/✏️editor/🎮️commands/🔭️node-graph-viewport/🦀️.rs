//! 🕸️ 🕸️ Generation3d play app commands command — `node-graph-viewport`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub camera: CameraJson,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetCamera(crate::editor::generation3d::config::SetCamera { camera: payload.camera.clone() })]))
}
