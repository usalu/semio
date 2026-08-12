//! 🕸️ Procedural2d play app commands — node-graph editing, media-node moves/connects, reorganize,
//! and the graph viewport/select/hover view commands.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String}

    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let selected = cfg.snapshot.selected_ids.clone();
        let mut cleared = false;
        let operations = host_operations(fixture, |host| {
            for operation in &sub_operations {
                match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                    "setFixture" => {
                        if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                            host.replace_fixture(fixture);
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
        });
        let config_mutations = if cleared { vec![Procedural2dConfigMutation::SetSelection { ids: Vec::new() }] } else { Vec::new() };
        Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️MoveMediaNode
pub mod move_media_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-media-node")]
    pub struct MoveMediaNode {
        pub node_id: String,
        pub x: f64,
        pub y: f64}

    pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        Ok(Emit::mutations(host_operations(fixture, |host| {
            let _ = host.move_widget(&payload.node_id, payload.x, payload.y);
        })))
    }
}
//#endregion 🔖️MoveMediaNode

//#region 🔖️ConnectMediaPorts
pub mod connect_media_ports {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "connect-media-ports")]
    pub struct ConnectMediaPorts {
        pub source_node_id: String,
        pub source_port_id: String,
        pub target_node_id: String,
        pub target_port_id: String}

    pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        Ok(Emit::mutations(host_operations(fixture, |host| {
            let _ = host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id);
        })))
    }
}
//#endregion 🔖️ConnectMediaPorts

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        Ok(Emit::mutations(host_operations(fixture, |host| {
            let _ = host.reorganize(r#"{"orientation":"leftRight"}"#);
        })))
    }
}
//#endregion 🔖️Reorganize

//#region 🔖️NodeGraphViewport
pub mod node_graph_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        pub viewport_json: String}

    pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        match serde_json::from_str::<CameraJson>(&payload.viewport_json) {
            Ok(camera) => Ok(Emit::config(vec![Procedural2dConfigMutation::SetCamera { camera }])),
            Err(_) => Ok(Emit::default())}
    }
}
//#endregion 🔖️NodeGraphViewport

//#region 🔖️NodeGraphSelect
pub mod node_graph_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-select")]
    pub struct NodeGraphSelect {
        pub ids: Vec<String>}

    pub fn handle(payload: &NodeGraphSelect, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigMutation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️NodeGraphSelect

//#region 🔖️NodeGraphHover
pub mod node_graph_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-hover")]
    pub struct NodeGraphHover {}

    pub fn handle(_payload: &NodeGraphHover, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NodeGraphHover

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn reorganize_emits_operations() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture;
        dispatch(&mut app, Procedural2dCommand::Reorganize(reorganize::Reorganize {}));
        let after = app.snapshot().expect("snapshot").fixture;
        assert_ne!(before.layout, after.layout);
    }

    #[test]
    fn node_graph_viewport_sets_camera() {
        let mut app = app();
        dispatch(&mut app, Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: serde_json::to_string(&CameraJson { x: 1.0, y: 2.0, zoom: 3.0 }).unwrap() }));
    }
}
//#endregion 🧪️Tests
