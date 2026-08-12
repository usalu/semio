//! 🔵️ Wires play app commands — adding an identity node to the board.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::schema::fixture_nodes;
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub kind: String,
    }

    pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        let document = doc.snapshot;
        let kind = if payload.kind.is_empty() { "identity" } else { payload.kind.as_str() };
        let id = format!("node-{}", fixture_nodes(&document.board_fixture).len() + 1);
        let node = dsl::to_dsl_value(&json!({
            "id": id,
            "nodeKind": kind,
            "shape": "circle",
            "x": 0.0,
            "y": 0.0,
            "radius": 24.0,
            "text": id,
            "handles": []
        }))
        .expect("node serializes");
        Ok(Emit { artifact_mutations: vec![WiresMutation::AddNode { node }], config_mutations: vec![WiresConfigMutation::SetSelection { ids: vec![id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddNode

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;

    #[test]
    fn add_node_appends_and_selects() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(fixture_nodes(&projection.board_fixture).len(), 1);
        assert!(find_board_node(&projection, "node-1").is_some());
    }
}
//#endregion 🧪️Tests
