//! 🕸️ Architect play app commands — the node-graph surface's edit and viewport wires.

pub mod node_graph_edit {
    use crate::apps::architect::catalog::{find_adjacency, new_adjacency};
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::registers::AdjacencyKind;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use protocol::CollectionMutation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &DocumentView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let edit_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
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
                        emitted.push(ProgramMutation::SetAdjacency { adjacency: new_adjacency(program, &a, &b, kind) });
                    }
                }
                "deleteSelection" => {
                    if let Some(ids) = operation.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        for id in ids {
                            emitted.push(ProgramMutation::Elements(CollectionMutation::Remove { id: EntityId(id.clone()) }));
                            for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id) {
                                emitted.push(ProgramMutation::ClearAdjacency { id: adjacency.header.id.clone() });
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
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::apps::architect::modes::edit::windows::graph::GraphCamera;
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        pub viewport_json: String,
    }

    pub fn handle(payload: &NodeGraphViewport, _doc: &DocumentView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(camera) = serde_json::from_str::<GraphCamera>(&payload.viewport_json) else {
            return Ok(Emit::default());
        };
        let mut next = cfg.snapshot.clone();
        next.graph_camera_x = camera.x;
        next.graph_camera_y = camera.y;
        next.graph_camera_zoom = camera.zoom;
        Ok(Emit::config(snapshot(next)))
    }
}
