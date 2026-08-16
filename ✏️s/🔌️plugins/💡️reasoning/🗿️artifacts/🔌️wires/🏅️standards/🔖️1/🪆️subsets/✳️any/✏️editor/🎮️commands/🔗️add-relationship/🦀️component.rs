//! 🔗️ 🔗️ Wires play app commands command — `add-relationship`.

use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_EDGE};
use crate::artifacts::wires::schema::fixture_edges;
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-relationship")]
pub struct AddRelationship {
    pub kind: String,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// the newly created edge is selected via a requested `interactionSelect` effect instead of a
/// `WiresConfigMutation::SetSelection`.
pub fn handle(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    let kind = if payload.kind.is_empty() { "owns" } else { payload.kind.as_str() };
    let edge_id = format!("edge-{}", fixture_edges(&crate::artifacts::wires::wires_working_board(document)).len() + 1);
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
    Ok(Emit { artifact_mutations: vec![crate::artifacts::wires::mutations::connect_nodes(edge, relationship)], effects: vec![wires_select_effect(&[edge_id], WIRES_GRANULARITY_EDGE, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{dispatch, new_app};
    use crate::editor::wires::WiresCommand;

    #[test]
    fn add_relationship_appends_edge_and_selects() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddRelationship(AddRelationship { kind: "owns".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(fixture_edges(&crate::artifacts::wires::wires_working_board(&projection)).len(), 1);
    }
}
//#endregion 🧪️Tests
