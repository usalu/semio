//! 👁️ 👁️ Flow play app commands command — `node-graph-viewport`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub camera: CameraJson,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app, render, FlowApp};
    use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};
    use serde_json::{json, Value};

    async fn preview_off_ids(app: &mut FlowApp) -> Value {
        let rendered: Value = serde_json::from_str(&render(app, FLOW_PLAY_BODY_MAIN)).expect("render json");
        rendered.pointer("/nodeGraph/previewOffJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    #[semio_framework_async_macros::async_test]
    async fn set_preview_off_toggles_ids_on_and_off_the_scene() {
        use crate::editor::flow::commands::set_preview_off;
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true }));
        assert_eq!(preview_off_ids(&mut app), json!(["slider"]));
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: false }));
        assert_eq!(preview_off_ids(&mut app), Value::Null, "an empty preview-off set is omitted from the scene");
    }

    #[semio_framework_async_macros::async_test]
    async fn node_graph_viewport_moves_the_camera() {
        let mut app = flow_app();
        let before = render(&mut app, FLOW_PLAY_BODY_MAIN);
        dispatch(&mut app, FlowCommand::NodeGraphViewport(NodeGraphViewport { camera: CameraJson { x: 30.0, y: -12.0, zoom: 2.0 } }));
        assert_ne!(before, render(&mut app, FLOW_PLAY_BODY_MAIN));
    }
}
//#endregion 🧪️Tests
