//! 🖥️ S studio CQRS — programs, app instances, media graph on `vcs`.

use vcs::{
    create_document_vcs_envelope, default_temporary_backbone_ref, materialize_document_projection,
    DocumentBackboneRef, DocumentVcs, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    Operation, OperationDiff, VcsError,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

pub const S_STUDIO_SCHEMA: &str = "s.studio";
pub const S_MEDIA_GRAPH_SCHEMA: &str = "s.media-graph";

//#region 🔖Schemas
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SSourceDocument {
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub source_document: SSourceDocument,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphPort {
    pub id: String,
    pub resource_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphNode {
    pub id: String,
    pub instance_id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub inputs: Vec<SMediaGraphPort>,
    pub outputs: Vec<SMediaGraphPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraph {
    pub schema: String,
    pub nodes: Vec<SMediaGraphNode>,
    pub edges: Vec<SMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SStudioProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<SAppInstance>,
    pub media_graph: SMediaGraph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGraphPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum StudioOp {
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    ApplyAppOperation {
        instance_id: String,
        next_source: SSourceDocument,
    },
    SpawnAppInstance {
        instance: SAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: SMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppSource {
        instance_id: String,
        inline: String,
    },
}

pub type SStudioVcs = DocumentVcs<SStudioProjection, StudioOp>;
pub type SStudioEnvelope = DocumentVcsEnvelope<SStudioProjection, StudioOp>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SStudioDocument {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: SStudioVcs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
}
//#endregion 🔖Schemas

//#region 🔖Projection
static S_ID: AtomicU64 = AtomicU64::new(0);

pub fn create_s_id(prefix: &str) -> String {
    let n = S_ID.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

pub fn empty_media_graph() -> SMediaGraph {
    SMediaGraph {
        schema: S_MEDIA_GRAPH_SCHEMA.into(),
        nodes: Vec::new(),
        edges: Vec::new(),
    }
}

pub fn default_studio_projection() -> SStudioProjection {
    SStudioProjection {
        programs: Vec::new(),
        active_program_id: None,
        active_alternative_id: None,
        app_instances: Vec::new(),
        media_graph: empty_media_graph(),
    }
}

pub fn create_empty_studio_document(id: &str, name: &str) -> SStudioDocument {
    SStudioDocument {
        schema: S_STUDIO_SCHEMA.into(),
        id: id.into(),
        name: name.into(),
        vcs: create_document_vcs_envelope(S_STUDIO_SCHEMA, id, default_studio_projection(), None).vcs,
        backbone: Some(default_temporary_backbone_ref(id)),
    }
}

pub fn apply_studio_operation(projection: &SStudioProjection, operation: &StudioOp) -> SStudioProjection {
    let mut next = projection.clone();
    match operation {
        StudioOp::SetActiveProgram { program_id } => {
            next.active_program_id = program_id.clone();
        }
        StudioOp::SetActiveAlternative { alternative_id } => {
            next.active_alternative_id = alternative_id.clone();
        }
        StudioOp::ApplyAppOperation {
            instance_id,
            next_source,
        } => {
            for instance in &mut next.app_instances {
                if instance.id == *instance_id {
                    instance.source_document = next_source.clone();
                }
            }
        }
        StudioOp::SpawnAppInstance { instance, position } => {
            if !next.programs.contains(&instance.program_id) {
                next.programs.push(instance.program_id.clone());
            }
            let node = SMediaGraphNode {
                id: create_s_id("node"),
                instance_id: instance.id.clone(),
                label: instance.label.clone(),
                x: position.x,
                y: position.y,
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            next.media_graph.nodes.push(node);
            next.app_instances.push(instance.clone());
        }
        StudioOp::RemoveAppInstance { instance_id } => {
            let node_id = next
                .media_graph
                .nodes
                .iter()
                .find(|node| node.instance_id == *instance_id)
                .map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != *instance_id);
            next.media_graph.nodes.retain(|node| node.instance_id != *instance_id);
            if let Some(node_id) = node_id {
                next.media_graph
                    .edges
                    .retain(|edge| edge.source_node_id != node_id && edge.target_node_id != node_id);
            }
        }
        StudioOp::ConnectMediaPorts { edge } => {
            next.media_graph.edges.push(edge.clone());
        }
        StudioOp::DisconnectMediaEdge { edge_id } => {
            next.media_graph.edges.retain(|edge| edge.id != *edge_id);
        }
        StudioOp::MoveMediaNode { node_id, x, y } => {
            for node in &mut next.media_graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        StudioOp::PatchAppSource { instance_id, inline } => {
            for instance in &mut next.app_instances {
                if instance.id == *instance_id {
                    instance.source_document.inline = Some(inline.clone());
                }
            }
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum StudioDiff {
    #[default]
    Empty,
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    ApplyAppOperation {
        instance_id: String,
        next_source: SSourceDocument,
    },
    SpawnAppInstance {
        instance: SAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: SMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppSource {
        instance_id: String,
        inline: String,
    },
}

impl OperationDiff<SStudioProjection> for StudioDiff {
    fn apply(&self, projection: &SStudioProjection) -> SStudioProjection {
        let op = match self {
            StudioDiff::Empty => return projection.clone(),
            StudioDiff::SetActiveProgram { program_id } => StudioOp::SetActiveProgram {
                program_id: program_id.clone(),
            },
            StudioDiff::SetActiveAlternative { alternative_id } => StudioOp::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            StudioDiff::ApplyAppOperation { instance_id, next_source } => StudioOp::ApplyAppOperation {
                instance_id: instance_id.clone(),
                next_source: next_source.clone(),
            },
            StudioDiff::SpawnAppInstance { instance, position } => StudioOp::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            StudioDiff::RemoveAppInstance { instance_id } => StudioOp::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            StudioDiff::ConnectMediaPorts { edge } => StudioOp::ConnectMediaPorts { edge: edge.clone() },
            StudioDiff::DisconnectMediaEdge { edge_id } => StudioOp::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            StudioDiff::MoveMediaNode { node_id, x, y } => StudioOp::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            StudioDiff::PatchAppSource { instance_id, inline } => StudioOp::PatchAppSource {
                instance_id: instance_id.clone(),
                inline: inline.clone(),
            },
        };
        apply_studio_operation(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, StudioDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<SStudioProjection> for StudioOp {
    type Diff = StudioDiff;

    fn diff(&self, _projection: &SStudioProjection) -> StudioDiff {
        match self {
            StudioOp::SetActiveProgram { program_id } => StudioDiff::SetActiveProgram {
                program_id: program_id.clone(),
            },
            StudioOp::SetActiveAlternative { alternative_id } => StudioDiff::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            StudioOp::ApplyAppOperation { instance_id, next_source } => StudioDiff::ApplyAppOperation {
                instance_id: instance_id.clone(),
                next_source: next_source.clone(),
            },
            StudioOp::SpawnAppInstance { instance, position } => StudioDiff::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            StudioOp::RemoveAppInstance { instance_id } => StudioDiff::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            StudioOp::ConnectMediaPorts { edge } => StudioDiff::ConnectMediaPorts { edge: edge.clone() },
            StudioOp::DisconnectMediaEdge { edge_id } => StudioDiff::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            StudioOp::MoveMediaNode { node_id, x, y } => StudioDiff::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            StudioOp::PatchAppSource { instance_id, inline } => StudioDiff::PatchAppSource {
                instance_id: instance_id.clone(),
                inline: inline.clone(),
            },
        }
    }

    fn backwards(&self, projection: &SStudioProjection) -> Vec<Self> {
        match self {
            StudioOp::SetActiveProgram { .. } => vec![StudioOp::SetActiveProgram {
                program_id: projection.active_program_id.clone(),
            }],
            StudioOp::SetActiveAlternative { .. } => vec![StudioOp::SetActiveAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
            StudioOp::ApplyAppOperation { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|i| i.id == *instance_id)
                .map(|instance| {
                    vec![StudioOp::ApplyAppOperation {
                        instance_id: instance_id.clone(),
                        next_source: instance.source_document.clone(),
                    }]
                })
                .unwrap_or_default(),
            StudioOp::SpawnAppInstance { instance, position } => vec![StudioOp::RemoveAppInstance {
                instance_id: instance.id.clone(),
            }],
            StudioOp::RemoveAppInstance { instance_id } => projection
                .app_instances
                .iter()
                .find(|i| i.id == *instance_id)
                .map(|instance| {
                    let node = projection
                        .media_graph
                        .nodes
                        .iter()
                        .find(|n| n.instance_id == *instance_id);
                    vec![StudioOp::SpawnAppInstance {
                        instance: instance.clone(),
                        position: MediaGraphPosition {
                            x: node.map(|n| n.x).unwrap_or(0.0),
                            y: node.map(|n| n.y).unwrap_or(0.0),
                        },
                    }]
                })
                .unwrap_or_default(),
            StudioOp::ConnectMediaPorts { edge } => vec![StudioOp::DisconnectMediaEdge {
                edge_id: edge.id.clone(),
            }],
            StudioOp::DisconnectMediaEdge { edge_id } => projection
                .media_graph
                .edges
                .iter()
                .find(|e| e.id == *edge_id)
                .map(|edge| vec![StudioOp::ConnectMediaPorts { edge: edge.clone() }])
                .unwrap_or_default(),
            StudioOp::MoveMediaNode { node_id, .. } => projection
                .media_graph
                .nodes
                .iter()
                .find(|n| n.id == *node_id)
                .map(|node| {
                    vec![StudioOp::MoveMediaNode {
                        node_id: node_id.clone(),
                        x: node.x,
                        y: node.y,
                    }]
                })
                .unwrap_or_default(),
            StudioOp::PatchAppSource { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|i| i.id == *instance_id)
                .map(|instance| {
                    vec![StudioOp::PatchAppSource {
                        instance_id: instance_id.clone(),
                        inline: instance.source_document.inline.clone().unwrap_or_default(),
                    }]
                })
                .unwrap_or_default(),
        }
    }
}

pub fn materialize_studio_projection(document: &SStudioDocument, applied_edit_ids: &[String]) -> Result<SStudioProjection, VcsError> {
    let envelope = SStudioEnvelope {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document.backbone.clone(),
        active_alternative_id: None,
    };
    materialize_document_projection(&envelope, applied_edit_ids)
}
//#endregion 🔖Projection

//#region 🔖StudioStore
pub struct StudioStore {
    inner: DocumentVcsStore<SStudioProjection, StudioOp>,
    name: String,
}

impl StudioStore {
    pub fn new(document: SStudioDocument) -> Self {
        let envelope = SStudioEnvelope {
            schema: document.schema,
            id: document.id,
            vcs: document.vcs,
            backbone: document.backbone,
            active_alternative_id: None,
        };
        Self {
            inner: DocumentVcsStore::new(envelope),
            name: document.name,
        }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<SStudioProjection, VcsError> {
        self.inner.projection()
    }

    pub fn document(&self) -> SStudioDocument {
        let envelope = self.inner.envelope();
        SStudioDocument {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            backbone: envelope.backbone.clone(),
        }
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        self.inner.dispatch_json(command_json)
    }

    pub fn dispatch_apply(&mut self, operations: Vec<StudioOp>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentVcsCommand::Apply {
            operations,
            description: None,
        })
    }

    pub fn sync_backbone(&self) -> Result<(), VcsError> {
        self.inner.sync_backbone()
    }

    pub fn load_backbone(&mut self) -> Result<(), VcsError> {
        self.inner.load_backbone()
    }
}
//#endregion 🔖StudioStore

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    use super::*;
    use std::sync::Mutex;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct StudioStoreHandle {
        store: Mutex<StudioStore>,
    }

    #[wasm_bindgen]
    impl StudioStoreHandle {
        #[wasm_bindgen(constructor)]
        pub fn new(document_json: &str) -> Result<StudioStoreHandle, JsValue> {
            let document: SStudioDocument =
                serde_json::from_str(document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self {
                store: Mutex::new(StudioStore::new(document)),
            })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            let mut store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            store.dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            let store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            let projection = store.projection().map_err(|e| JsValue::from_str(&e.to_string()))?;
            serde_json::to_string(&projection).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> Result<u32, JsValue> {
            let store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            Ok(store.generation() as u32)
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawns_app_instance_through_cqrs_dispatch() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            source_document: SSourceDocument {
                format: "draw.document".into(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        store
            .dispatch_apply(vec![StudioOp::SpawnAppInstance {
                instance: instance.clone(),
                position: MediaGraphPosition { x: 0.0, y: 0.0 },
            }])
            .expect("spawn");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.app_instances.len(), 1);
        assert_eq!(projection.media_graph.nodes.len(), 1);
    }

    #[test]
    fn undo_after_spawn() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            source_document: SSourceDocument {
                format: "draw.document".into(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        store
            .dispatch_apply(vec![StudioOp::SpawnAppInstance {
                instance,
                position: MediaGraphPosition { x: 0.0, y: 0.0 },
            }])
            .expect("spawn");
        store
            .dispatch_json(r#"{"kind":"undo"}"#)
            .expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }
}
//#endregion 🧪Tests
