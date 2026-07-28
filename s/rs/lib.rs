//! 🖥️ S studio CQRS — programs, app instances, media graph on `vcs`.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use protocol::{Operation, OperationDiff};
use vcs::{DocumentVcs, VcsError};
use store::{create_document_envelope, materialize_document_projection, DocumentBackboneRef, DocumentCommand, DocumentEnvelope, DocumentStore};

pub const S_STUDIO_SCHEMA: &str = "s.studio";
pub const S_MEDIA_GRAPH_SCHEMA: &str = "s.media-graph";

//#region 🔖Schemas
/// @emoji 🔗 Handle to an app instance's own vcs document — app content is never embedded on the
/// studio document, only referenced (mirrors os-core's `OsDocumentRef`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SDocumentRef {
    pub document_id: String,
    pub schema: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    #[dsl(block)]
    pub document: SDocumentRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphPort {
    pub id: String,
    pub resource_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraph {
    pub schema: String,
    #[dsl(table)]
    pub nodes: Vec<SMediaGraphNode>,
    #[dsl(table)]
    pub edges: Vec<SMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "sstudio", layout = "lines")]
pub struct SStudioProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<SAppInstance>,
    #[dsl(block)]
    pub media_graph: SMediaGraph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MediaGraphPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum StudioOperation {
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    SpawnAppInstance {
        #[dsl(block)]
        instance: SAppInstance,
        #[dsl(block)]
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        #[dsl(block)]
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
}

pub type SStudioVcs = DocumentVcs<SStudioProjection, StudioOperation>;
pub type SStudioEnvelope = DocumentEnvelope<SStudioProjection, StudioOperation>;

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
    SMediaGraph { schema: S_MEDIA_GRAPH_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

pub fn default_studio_projection() -> SStudioProjection {
    SStudioProjection { programs: Vec::new(), active_program_id: None, active_alternative_id: None, app_instances: Vec::new(), media_graph: empty_media_graph() }
}

pub fn create_empty_studio_document(id: &str, name: &str) -> SStudioDocument {
    SStudioDocument { schema: S_STUDIO_SCHEMA.into(), id: id.into(), name: name.into(), vcs: create_document_envelope(S_STUDIO_SCHEMA, id, default_studio_projection(), None).vcs, backbone: None }
}

pub fn apply_studio_operation(projection: &SStudioProjection, operation: &StudioOperation) -> SStudioProjection {
    let mut next = projection.clone();
    match operation {
        StudioOperation::SetActiveProgram { program_id } => {
            next.active_program_id = program_id.clone();
        }
        StudioOperation::SetActiveAlternative { alternative_id } => {
            next.active_alternative_id = alternative_id.clone();
        }
        StudioOperation::SpawnAppInstance { instance, position } => {
            if !next.programs.contains(&instance.program_id) {
                next.programs.push(instance.program_id.clone());
            }
            let node = SMediaGraphNode { id: create_s_id("node"), instance_id: instance.id.clone(), label: instance.label.clone(), x: position.x, y: position.y, inputs: Vec::new(), outputs: Vec::new() };
            next.media_graph.nodes.push(node);
            next.app_instances.push(instance.clone());
        }
        StudioOperation::RemoveAppInstance { instance_id } => {
            let node_id = next.media_graph.nodes.iter().find(|node| node.instance_id == *instance_id).map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != *instance_id);
            next.media_graph.nodes.retain(|node| node.instance_id != *instance_id);
            if let Some(node_id) = node_id {
                next.media_graph.edges.retain(|edge| edge.source_node_id != node_id && edge.target_node_id != node_id);
            }
        }
        StudioOperation::ConnectMediaPorts { edge } => {
            next.media_graph.edges.push(edge.clone());
        }
        StudioOperation::DisconnectMediaEdge { edge_id } => {
            next.media_graph.edges.retain(|edge| edge.id != *edge_id);
        }
        StudioOperation::MoveMediaNode { node_id, x, y } => {
            for node in &mut next.media_graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
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
}

impl OperationDiff<SStudioProjection> for StudioDiff {
    fn apply(&self, projection: &SStudioProjection) -> SStudioProjection {
        let operation = match self {
            StudioDiff::Empty => return projection.clone(),
            StudioDiff::SetActiveProgram { program_id } => StudioOperation::SetActiveProgram { program_id: program_id.clone() },
            StudioDiff::SetActiveAlternative { alternative_id } => StudioOperation::SetActiveAlternative { alternative_id: alternative_id.clone() },
            StudioDiff::SpawnAppInstance { instance, position } => StudioOperation::SpawnAppInstance { instance: instance.clone(), position: position.clone() },
            StudioDiff::RemoveAppInstance { instance_id } => StudioOperation::RemoveAppInstance { instance_id: instance_id.clone() },
            StudioDiff::ConnectMediaPorts { edge } => StudioOperation::ConnectMediaPorts { edge: edge.clone() },
            StudioDiff::DisconnectMediaEdge { edge_id } => StudioOperation::DisconnectMediaEdge { edge_id: edge_id.clone() },
            StudioDiff::MoveMediaNode { node_id, x, y } => StudioOperation::MoveMediaNode { node_id: node_id.clone(), x: *x, y: *y },
        };
        apply_studio_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, StudioDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<SStudioProjection> for StudioOperation {
    type Diff = StudioDiff;

    fn diff(&self, _projection: &SStudioProjection) -> StudioDiff {
        match self {
            StudioOperation::SetActiveProgram { program_id } => StudioDiff::SetActiveProgram { program_id: program_id.clone() },
            StudioOperation::SetActiveAlternative { alternative_id } => StudioDiff::SetActiveAlternative { alternative_id: alternative_id.clone() },
            StudioOperation::SpawnAppInstance { instance, position } => StudioDiff::SpawnAppInstance { instance: instance.clone(), position: position.clone() },
            StudioOperation::RemoveAppInstance { instance_id } => StudioDiff::RemoveAppInstance { instance_id: instance_id.clone() },
            StudioOperation::ConnectMediaPorts { edge } => StudioDiff::ConnectMediaPorts { edge: edge.clone() },
            StudioOperation::DisconnectMediaEdge { edge_id } => StudioDiff::DisconnectMediaEdge { edge_id: edge_id.clone() },
            StudioOperation::MoveMediaNode { node_id, x, y } => StudioDiff::MoveMediaNode { node_id: node_id.clone(), x: *x, y: *y },
        }
    }

    fn backwards(&self, projection: &SStudioProjection) -> Vec<Self> {
        match self {
            StudioOperation::SetActiveProgram { .. } => vec![StudioOperation::SetActiveProgram { program_id: projection.active_program_id.clone() }],
            StudioOperation::SetActiveAlternative { .. } => vec![StudioOperation::SetActiveAlternative { alternative_id: projection.active_alternative_id.clone() }],
            StudioOperation::SpawnAppInstance { instance, .. } => vec![StudioOperation::RemoveAppInstance { instance_id: instance.id.clone() }],
            StudioOperation::RemoveAppInstance { instance_id } => projection
                .app_instances
                .iter()
                .find(|i| i.id == *instance_id)
                .map(|instance| {
                    let node = projection.media_graph.nodes.iter().find(|n| n.instance_id == *instance_id);
                    vec![StudioOperation::SpawnAppInstance { instance: instance.clone(), position: MediaGraphPosition { x: node.map(|n| n.x).unwrap_or(0.0), y: node.map(|n| n.y).unwrap_or(0.0) } }]
                })
                .unwrap_or_default(),
            StudioOperation::ConnectMediaPorts { edge } => vec![StudioOperation::DisconnectMediaEdge { edge_id: edge.id.clone() }],
            StudioOperation::DisconnectMediaEdge { edge_id } => projection.media_graph.edges.iter().find(|e| e.id == *edge_id).map(|edge| vec![StudioOperation::ConnectMediaPorts { edge: edge.clone() }]).unwrap_or_default(),
            StudioOperation::MoveMediaNode { node_id, .. } => projection.media_graph.nodes.iter().find(|n| n.id == *node_id).map(|node| vec![StudioOperation::MoveMediaNode { node_id: node_id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
        }
    }
}

pub fn materialize_studio_projection(document: &SStudioDocument, applied_edit_ids: &[String]) -> Result<SStudioProjection, VcsError> {
    let envelope = SStudioEnvelope { schema: document.schema.clone(), id: document.id.clone(), vcs: document.vcs.clone(), backbone: document.backbone.clone(), active_alternative_id: None, cursor: None };
    materialize_document_projection(&envelope, applied_edit_ids)
}
//#endregion 🔖Projection

//#region 🔖Dsl
// `impl store::DocumentDsl for SStudioProjection` is emitted automatically by the
// `#[derive(dsl::DslDocument)]` on `SStudioProjection` itself (see `🔖Schemas`) — no manual impl
// needed here. The former hand-rolled `mod studio_dsl` lexer/parser/printer has been removed now
// that the DSL round trip runs through the derive-generated printer/parser.
//#endregion 🔖Dsl

//#region 🔖OpText
// `impl protocol::OpText for StudioOperation` is emitted automatically by the `#[derive(dsl::DslOps)]`
// on `StudioOperation` itself (see `🔖Schemas`) — no manual impl needed here.
//#endregion 🔖OpText


//#region 🔖StudioStore
pub struct StudioStore {
    inner: DocumentStore<SStudioProjection, StudioOperation>,
    name: String,
}

impl StudioStore {
    pub fn new(document: SStudioDocument) -> Self {
        let envelope = SStudioEnvelope { schema: document.schema, id: document.id, vcs: document.vcs, backbone: document.backbone, active_alternative_id: None, cursor: None };
        Self { inner: DocumentStore::new(envelope), name: document.name }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<SStudioProjection, VcsError> {
        self.inner.projection()
    }

    pub fn document(&self) -> SStudioDocument {
        let envelope = self.inner.envelope();
        SStudioDocument { schema: envelope.schema.clone(), id: envelope.id.clone(), name: self.name.clone(), vcs: envelope.vcs.clone(), backbone: envelope.backbone.clone() }
    }

    pub fn dispatch_text(&mut self, command_text: &str) -> Result<(), VcsError> {
        self.inner.dispatch_text(command_text)
    }

    pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<(), VcsError> {
        self.inner.dispatch_binary(command_bytes)
    }

    pub fn dispatch_apply(&mut self, operations: Vec<StudioOperation>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentCommand::Apply { operations, description: None })
    }

    /// @emoji 📡 Pumps any queued inbound backbone messages into the edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.inner.tick()
    }

    /// @emoji 🔗 Resolves and attaches a backbone by uri inside the wasm sandbox (every scheme
    /// forwards to the host over the injected `BackboneChannelPort`, a pure queue).
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        self.inner.attach_backbone_uri(uri)
    }

    /// @emoji 🚧 Native attach is a documented no-operation: `s` only runs as a WASM plugin in the browser
    /// today (no native caller exists), and wiring its native path onto `framework/sync`'s
    /// `DocumentHost` is `s`'s own `DocumentApp` migration (WS-F's last wave), not this compile fix.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&mut self, _uri: &str) -> Result<(), VcsError> {
        Ok(())
    }

    pub fn detach_backbone(&mut self) {
        self.inner.detach_backbone();
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
            let document: SStudioDocument = serde_json::from_str(document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: Mutex::new(StudioStore::new(document)) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            let mut store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            store.dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            let mut store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            store.dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
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
        let instance = SAppInstance { id: "app-1".into(), program_id: "draw".into(), app_id: "draw".into(), label: "Draw".into(), yields: "graph.dag".into(), document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() } };
        store.dispatch_apply(vec![StudioOperation::SpawnAppInstance { instance: instance.clone(), position: MediaGraphPosition { x: 0.0, y: 0.0 } }]).expect("spawn");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.app_instances.len(), 1);
        assert_eq!(projection.media_graph.nodes.len(), 1);
    }

    #[test]
    fn undo_after_spawn() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SAppInstance { id: "app-1".into(), program_id: "draw".into(), app_id: "draw".into(), label: "Draw".into(), yields: "graph.dag".into(), document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() } };
        store.dispatch_apply(vec![StudioOperation::SpawnAppInstance { instance, position: MediaGraphPosition { x: 0.0, y: 0.0 } }]).expect("spawn");
        store.dispatch_text("undo").expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }

    //#region 🔖DslAndOpText
    fn sample_studio_projection() -> SStudioProjection {
        SStudioProjection {
            programs: vec!["draw".into(), "writer".into()],
            active_program_id: Some("draw".into()),
            active_alternative_id: None,
            app_instances: vec![SAppInstance {
                id: "app-1".into(),
                program_id: "draw".into(),
                app_id: "draw".into(),
                label: "Semio \"Emblem\"".into(),
                yields: "2d.drawing".into(),
                document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() },
            }],
            media_graph: SMediaGraph {
                schema: S_MEDIA_GRAPH_SCHEMA.into(),
                nodes: vec![SMediaGraphNode {
                    id: "node-1".into(),
                    instance_id: "app-1".into(),
                    label: "Draw\nNode".into(),
                    x: 40.0,
                    y: 80.0,
                    inputs: vec![SMediaGraphPort { id: "app-1:in".into(), resource_kind: "2d.drawing".into() }],
                    outputs: vec![SMediaGraphPort { id: "app-1:out".into(), resource_kind: "2d.drawing".into() }],
                }],
                edges: vec![SMediaGraphEdge {
                    id: "edge-1".into(),
                    source_node_id: "node-1".into(),
                    source_port_id: "app-1:out".into(),
                    target_node_id: "node-1".into(),
                    target_port_id: "app-1:in".into(),
                }],
            },
        }
    }

    #[test]
    fn studio_dsl_round_trips_empty_and_sample_projections() {
        store::test_support::assert_dsl_round_trip(&default_studio_projection());
        store::test_support::assert_dsl_round_trip(&sample_studio_projection());
        store::test_support::assert_dsl_pack_equivalence(&default_studio_projection());
        store::test_support::assert_dsl_pack_equivalence(&sample_studio_projection());
    }

    #[test]
    fn studio_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveProgram { program_id: Some("draw".into()) });
        store::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveProgram { program_id: None });
        store::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveAlternative { alternative_id: Some("alt-1".into()) });
        store::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveAlternative { alternative_id: None });
        let instance = SAppInstance {
            id: "app-2".into(),
            program_id: "writer".into(),
            app_id: "writer".into(),
            label: "Jack \"Notes\"".into(),
            yields: "text.document".into(),
            document: SDocumentRef { document_id: "doc-2".into(), schema: "writer.document".into() },
        };
        store::test_support::assert_op_line_round_trip(&StudioOperation::SpawnAppInstance { instance, position: MediaGraphPosition { x: 12.0, y: 24.0 } });
        store::test_support::assert_op_line_round_trip(&StudioOperation::RemoveAppInstance { instance_id: "app-1".into() });
        let edge = SMediaGraphEdge { id: "edge-2".into(), source_node_id: "node-1".into(), source_port_id: "p-out".into(), target_node_id: "node-2".into(), target_port_id: "p-in".into() };
        store::test_support::assert_op_line_round_trip(&StudioOperation::ConnectMediaPorts { edge });
        store::test_support::assert_op_line_round_trip(&StudioOperation::DisconnectMediaEdge { edge_id: "edge-1".into() });
        store::test_support::assert_op_line_round_trip(&StudioOperation::MoveMediaNode { node_id: "node-1".into(), x: 5.0, y: 6.0 });
    }

    #[test]
    fn studio_document_text_round_trips_through_the_store() {
        let envelope = create_document_envelope::<SStudioProjection, StudioOperation>(S_STUDIO_SCHEMA, "studio", sample_studio_projection(), None);
        let store: DocumentStore<SStudioProjection, StudioOperation> = DocumentStore::new(envelope);
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
