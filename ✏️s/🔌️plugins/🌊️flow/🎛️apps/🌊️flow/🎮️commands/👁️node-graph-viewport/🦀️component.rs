//! 👁️ 👁️ Flow play app commands command — `node-graph-viewport`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};
    use serde_json::{json, Value};

    fn preview_off_ids(app: &mut semio_framework_plugin::VcsArtifactApp<crate::apps::flow::FlowPlayApp>) -> Value {
        let rendered: Value = serde_json::from_str(&render(app, FLOW_PLAY_BODY_MAIN)).expect("render json");
        rendered.pointer("/nodeGraph/previewOffJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    #[test]
    fn set_preview_off_toggles_ids_on_and_off_the_scene() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true }));
        assert_eq!(preview_off_ids(&mut app), json!(["slider"]));
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: false }));
        assert_eq!(preview_off_ids(&mut app), Value::Null, "an empty preview-off set is omitted from the scene");
    }

    #[test]
    fn node_graph_viewport_moves_the_camera() {
        let mut app = flow_app();
        let before = render(&mut app, FLOW_PLAY_BODY_MAIN);
        dispatch(&mut app, FlowCommand::NodeGraphViewport(NodeGraphViewport { camera: CameraJson { x: 30.0, y: -12.0, zoom: 2.0 } }));
        assert_ne!(before, render(&mut app, FLOW_PLAY_BODY_MAIN));
    }
}
//#endregion 🧪️Tests
