//! 🕸️ Architect play app commands — the node-graph surface's edit and viewport wires.

pub mod node_graph_edit {
    use dsl::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::registers::AdjacencyKind;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use crate::editor::architect::catalog::{find_adjacency, new_adjacency};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use dsl::DslValue as Value;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    pub async fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let edit_operations: Vec<Value> = dsl::json::from_json_str(&payload.operations_json).unwrap_or_default();
        let mut emitted = Vec::new();
        for operation in edit_operations {
            match operation.get("operation").and_then(Value::as_str).unwrap_or("") {
                "connect" => {
                    let source = operation.get("sourceNodeId").and_then(Value::as_str);
                    let target = operation.get("targetNodeId").and_then(Value::as_str);
                    if let (Some(source), Some(target)) = (source, target) {
                        let a = EntityId(source.into());
                        let b = EntityId(target.into());
                        let kind = find_adjacency(program, &a, &b).map_or(AdjacencyKind::Preferred, |row| row.kind.clone());
                        emitted.push(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::mutation::ConnectAdjacency { adjacency: new_adjacency(program, &a, &b, kind) }));
                    }
                }
                "deleteSelection" => {
                    if let Some(ids) = operation.get("nodeIds").and_then(|value| <Vec<String> as dsl::FromValue>::from_value(value.clone()).ok()) {
                        for id in ids {
                            emitted.push(ProgramMutation::DeleteProgramElement(leaves::delete_program_element::mutation::DeleteProgramElement { id: EntityId(id.clone()) }));
                            for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id) {
                                emitted.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::mutation::DisconnectAdjacency { id: adjacency.header.id.clone() }));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if emitted.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(emitted))
        }
    }
}

pub mod node_graph_viewport {
    use dsl::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::editor::architect::modes::edit::windows::graph::GraphCamera;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        pub viewport_json: String,
    }

    pub async fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(camera) = dsl::json::from_json_str::<GraphCamera>(&payload.viewport_json) else {
            return Ok(Emit::default());
        };
        let mut next = cfg.snapshot.clone();
        next.graph_camera_x = camera.x;
        next.graph_camera_y = camera.y;
        next.graph_camera_zoom = camera.zoom;
        Ok(Emit::config(snapshot(next)))
    }
}
