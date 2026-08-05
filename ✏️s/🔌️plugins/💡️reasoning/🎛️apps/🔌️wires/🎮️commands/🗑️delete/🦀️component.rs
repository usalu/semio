//! 🗑️ Wires play app commands — deleting the current selection (nodes and/or edges).

use crate::apps::wires::config::{WiresConfig, WiresConfigOperation};
use crate::artifacts::wires::engine::{fixture_edges, find_board_node};
use crate::artifacts::wires::op::MindmapWiresOperation;
use crate::artifacts::wires::MindmapWiresDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &DocumentView<'_, MindmapWiresDocument>, cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<MindmapWiresOperation, WiresConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        let mut operations = Vec::new();
        for id in &config.selected_ids {
            if find_board_node(document, id).is_some() {
                operations.push(MindmapWiresOperation::RemoveNode { node_id: id.clone() });
            } else if fixture_edges(&document.board_fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                operations.push(MindmapWiresOperation::RemoveEdge { edge_id: id.clone() });
            }
        }
        let config_operations = if operations.is_empty() { Vec::new() } else { vec![WiresConfigOperation::SetSelection { ids: Vec::new() }] };
        Ok(Emit { document_operations: operations, config_operations, ..Default::default() })
    }
}
//#endregion 🔖️DeleteSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::commands::node::add_node;
    use crate::apps::wires::commands::selection::set_selection;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;
    use crate::artifacts::wires::engine::fixture_nodes;

    #[test]
    fn delete_selection_removes_node() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        dispatch(&mut app, WiresCommand::SetSelection(set_selection::SetSelection { ids: vec!["node-1".into()] }));
        dispatch(&mut app, WiresCommand::DeleteSelection(delete_selection::DeleteSelection {}));
        assert!(fixture_nodes(&app.projection().expect("projection").board_fixture).is_empty());
    }
}
//#endregion 🧪️Tests
