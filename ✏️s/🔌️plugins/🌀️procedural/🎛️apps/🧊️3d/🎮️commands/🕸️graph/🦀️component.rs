//! 🕸️ Procedural3d play app commands — flow-graph editing, media-node moves, reorganize, and the
//! graph viewport/select/hover/pointer view commands.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String}

    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let selected = cfg.snapshot.selected_node_ids.clone();
        let mut host = host_from_fixture(fixture);
        let mut cleared = false;
        for operation in &sub_operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setFixture" => {
                    if let Some(new_fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                        host.replace_fixture(new_fixture);
                    }
                }
                "deleteSelection" => {
                    for id in &selected {
                        if host.remove_widget(id).is_ok() {
                            cleared = true;
                        }
                    }
                }
                "connect" => {
                    let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                    let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                    let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                    let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                        let _ = host.connect_ports(from, from_port, to, to_port);
                    }
                }
                _ => {}
            }
        }
        let operations = commit_fixture(fixture, &host.fixture);
        let config_mutations = if cleared { vec![Procedural3dConfigMutation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
        Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️MoveMediaNode
pub mod move_media_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-node")]
    pub struct MoveMediaNode {
        pub node_id: String,
        pub x: f64,
        pub y: f64}

    pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        let mut host = host_from_fixture(fixture);
        if host.move_widget(&payload.node_id, payload.x, payload.y).is_ok() {
            Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️MoveMediaNode

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        let mut host = host_from_fixture(fixture);
        if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
            Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️Reorganize

//#region 🔖️NodeGraphViewport
pub mod node_graph_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "viewport")]
    pub struct NodeGraphViewport {
        #[dsl(block)]
        pub camera: CameraJson}

    pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️NodeGraphViewport

//#region 🔖️NodeGraphSelect
pub mod node_graph_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "graph-select")]
    pub struct NodeGraphSelect {
        pub node_ids: Vec<String>}

    pub fn handle(payload: &NodeGraphSelect, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetSelection { node_ids: payload.node_ids.clone() }]))
    }
}
//#endregion 🔖️NodeGraphSelect

//#region 🔖️NodeGraphHover
pub mod node_graph_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "graph-hover")]
    pub struct NodeGraphHover {
        pub widget_id: Option<String>}

    pub fn handle(payload: &NodeGraphHover, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetHover { node_id: payload.widget_id.clone() }]))
    }
}
//#endregion 🔖️NodeGraphHover

//#region 🔖️GraphPointerDown
pub mod graph_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "graph-pointer-down")]
    pub struct GraphPointerDown {}

    pub fn handle(_payload: &GraphPointerDown, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️GraphPointerDown

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_artifact_mutations_via_reorganize_baseline() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Procedural3dCommand::Reorganize(reorganize::Reorganize {}));
        assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before);
    }
}
//#endregion 🧪️Tests
