//! 👁️ Flow play app commands — pure view interactions: camera, hover, spotlight, image replacement and
//! the per-widget live-eval preview toggle. All config-only.

use crate::apps::flow::config::{FlowConfig, FlowConfigOperation};
use crate::artifacts::flow::{op::FlowOperation, FlowFixture};
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️NodeGraphViewport
pub mod node_graph_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        #[dsl(block)]
        pub camera: CameraJson,
    }

    pub fn handle(payload: &NodeGraphViewport, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️NodeGraphViewport

//#region 🔖️NodeGraphHover
pub mod node_graph_hover {
    use super::*;

    /// 🖱️ Hover is surface-local: the renderer owns the highlight, so the app emits nothing.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-hover")]
    pub struct NodeGraphHover {}

    pub fn handle(_payload: &NodeGraphHover, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NodeGraphHover

//#region 🔖️OpenSpotlight
pub mod open_spotlight {
    use super::*;

    /// 🔦️ Opening the spotlight is renderer chrome; the commit comes back as `spotlightCommit`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-spotlight")]
    pub struct OpenSpotlight {}

    pub fn handle(_payload: &OpenSpotlight, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️OpenSpotlight

//#region 🔖️ReplaceImage
pub mod replace_image {
    use super::*;

    /// 🖼️ Opening the host file picker is renderer chrome; the picked media returns as a widget patch.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "replace-image")]
    pub struct ReplaceImage {
        pub id: String,
    }

    pub fn handle(_payload: &ReplaceImage, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️ReplaceImage

//#region 🔖️SetPreviewOff
pub mod set_preview_off {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-preview-off")]
    pub struct SetPreviewOff {
        pub ids: Vec<String>,
        pub value: bool,
    }

    pub fn handle(payload: &SetPreviewOff, _doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let mut next = cfg.projection.preview_off_node_ids.clone();
        if payload.value {
            for id in &payload.ids {
                if !next.contains(id) {
                    next.push(id.clone());
                }
            }
        } else {
            next.retain(|id| !payload.ids.contains(id));
        }
        Ok(Emit::config(vec![FlowConfigOperation::SetPreviewOff { node_ids: next }]))
    }
}
//#endregion 🔖️SetPreviewOff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};
    use serde_json::{json, Value};

    fn preview_off_ids(app: &mut semio_framework_plugin::VcsDocumentApp<crate::apps::flow::FlowPlayApp>) -> Value {
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
        dispatch(&mut app, FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: CameraJson { x: 30.0, y: -12.0, zoom: 2.0 } }));
        assert_ne!(before, render(&mut app, FLOW_PLAY_BODY_MAIN));
    }
}
//#endregion 🧪️Tests
