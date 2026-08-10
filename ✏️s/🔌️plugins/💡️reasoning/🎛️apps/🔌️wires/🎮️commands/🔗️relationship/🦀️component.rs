//! 🔗️ Wires play app commands — adding a relationship edge between two identities.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::engine::fixture_edges;
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️AddRelationship
pub mod add_relationship {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-relationship")]
    pub struct AddRelationship {
        pub kind: String,
    }

    pub fn handle(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        let document = doc.snapshot;
        let kind = if payload.kind.is_empty() { "owns" } else { payload.kind.as_str() };
        let edge_id = format!("edge-{}", fixture_edges(&document.board_fixture).len() + 1);
        let edge = dsl::to_dsl_value(&json!({
            "id": edge_id,
            "edgeKind": format!("wires.{kind}"),
            "source": "node-1",
            "target": "node-2"
        }))
        .expect("edge serializes");
        let relationship = dsl::to_dsl_value(&json!({
            "edgeId": edge_id,
            "kind": kind,
            "sourceIdentityId": 1,
            "targetIdentityId": 2
        }))
        .expect("relationship serializes");
        Ok(Emit { artifact_mutations: vec![WiresMutation::AddRelationship { edge, relationship }], config_mutations: vec![WiresConfigMutation::SetSelection { ids: vec![edge_id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddRelationship

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;

    #[test]
    fn add_relationship_appends_edge_and_selects() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddRelationship(add_relationship::AddRelationship { kind: "owns".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(fixture_edges(&projection.board_fixture).len(), 1);
    }
}
//#endregion 🧪️Tests
