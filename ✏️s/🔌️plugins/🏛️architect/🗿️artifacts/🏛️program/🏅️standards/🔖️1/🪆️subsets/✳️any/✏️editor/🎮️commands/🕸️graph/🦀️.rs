//! 🕸️ Architect play app commands — the node-graph surface's edit and viewport wires.

pub mod node_graph_edit {
    use crate::editor::architect::catalog::{find_adjacency, new_adjacency};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::registers::AdjacencyKind;
    use crate::schema::mutations as leaves;
    use crate::{EntityId, ProgramSnapshot};
    use semio_framework_value::DslValue as Value;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }


    /// 🛠️ A graph batch uses the framework's closed row vocabulary and one-step transaction; every invalid row
    /// refuses the batch before any mutation is published, and linked nodes disconnect each adjacency once.
    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let operations = semio_framework_pack_json::from_json_str::<Value>(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::from(format!("nodeGraphEdit operations are not JSON: {error}")))?;
        let rows = semio_framework_tool_machine::node_graph_edit_rows(&Value::object([("operations".into(), operations)])).map_err(Fault::from)?;
        let program = doc.snapshot;
        let mut emitted = Vec::new();
        let mut disconnected = std::collections::BTreeSet::new();
        let mut deleted = std::collections::BTreeSet::new();
        let mut adjacencies = program.adjacencies.clone();
        let element = |id: &str| program.elements.iter().any(|element| element.header.id.0 == id);
        for row in rows {
            match row {
                semio_framework_tool_machine::NodeGraphEditRow::Connect { source_node_id, target_node_id, .. } => {
                    if source_node_id == target_node_id || !element(&source_node_id) || !element(&target_node_id) || deleted.contains(&source_node_id) || deleted.contains(&target_node_id) {
                        return Err(Fault::from("nodeGraphEdit connect needs two existing distinct elements"));
                    }
                    let (a, b) = (EntityId(source_node_id), EntityId(target_node_id));
                    if adjacencies.iter().any(|row| ((row.element_a_id == a && row.element_b_id == b) || (row.element_a_id == b && row.element_b_id == a)) && !disconnected.contains(&row.header.id.0)) { continue; }
                    let kind = find_adjacency(program, &a, &b).map_or(AdjacencyKind::Preferred, |row| row.kind.clone());
                    let adjacency = new_adjacency(program, &a, &b, kind);
                    adjacencies.push(adjacency.clone());
                    emitted.push(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency, index: None }));
                }
                semio_framework_tool_machine::NodeGraphEditRow::Disconnect { synapse_id } => {
                    if !adjacencies.iter().any(|adjacency| adjacency.header.id.0 == synapse_id) {
                        return Err(Fault::from("nodeGraphEdit disconnect needs an existing adjacency"));
                    }
                    if disconnected.insert(synapse_id.clone()) {
                        emitted.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: EntityId(synapse_id) }));
                    }
                }
                semio_framework_tool_machine::NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                    if node_ids.iter().any(|id| !element(id)) || synapse_ids.iter().any(|id| !adjacencies.iter().any(|adjacency| &adjacency.header.id.0 == id)) {
                        return Err(Fault::from("nodeGraphEdit delete needs existing elements and adjacencies"));
                    }
                    for adjacency in &adjacencies {
                        if (synapse_ids.contains(&adjacency.header.id.0) || node_ids.contains(&adjacency.element_a_id.0) || node_ids.contains(&adjacency.element_b_id.0)) && disconnected.insert(adjacency.header.id.0.clone()) {
                            emitted.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: adjacency.header.id.clone() }));
                        }
                    }
                    for id in node_ids {
                        if deleted.insert(id.clone()) {
                            emitted.push(ProgramMutation::DeleteProgramElement(leaves::delete_program_element::DeleteProgramElement { id: EntityId(id) }));
                        }
                    }
                }
                _ => return Err(Fault::from("nodeGraphEdit operation is not supported by the program graph")),
            }
        }
        Ok(Emit::tool_once(crate::editor::architect::ARCHITECT_APP_ID, "nodeGraphEdit", doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()), emitted))
    }

}

pub mod node_graph_viewport {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_os_kernel::Viewport2d;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        #[dsl(block)]
        pub viewport: Viewport2d,
    }

    pub fn handle(_payload: &NodeGraphViewport, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
