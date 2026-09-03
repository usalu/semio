//! 🔵️ 🔵️ Wires play app commands command — `add-node`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::schema::fixture_nodes;
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_NODE};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub kind: String,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// the newly created node is selected via a requested `interactionSelect` effect instead of a
/// `WiresConfigMutation::SetSelection`.
pub async fn handle(payload: &AddNode, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    let kind = if payload.kind.is_empty() { "identity" } else { payload.kind.as_str() };
    let id = format!("node-{}", fixture_nodes(&crate::artifacts::wires::wires_working_board(document)).len() + 1);
    let node = DslValue::object([
        ("id".into(), DslValue::String(id.clone())),
        ("nodeKind".into(), DslValue::String(kind.into())),
        ("shape".into(), DslValue::String("circle".into())),
        ("x".into(), DslValue::float(0.0)),
        ("y".into(), DslValue::float(0.0)),
        ("radius".into(), DslValue::float(24.0)),
        ("text".into(), DslValue::String(id.clone())),
        ("handles".into(), DslValue::Array(vec![])),
    ]);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::wires::mutations::create_node(node)], effects: vec![wires_select_effect(&[id], WIRES_GRANULARITY_NODE, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
    use crate::editor::wires::testkit::{dispatch, new_app};
    use crate::editor::wires::WiresCommand;

    #[semio_framework_async_macros::async_test]
    async fn add_node_appends_and_selects() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(AddNode { kind: "identity".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(fixture_nodes(&crate::artifacts::wires::wires_working_board(&projection)).len(), 1);
        assert!(find_board_node(&projection, "node-1").is_some());
    }
}
//#endregion 🧪️Tests
