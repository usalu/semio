//! 🖥️ Semios studio CQRS — programs, app instances, media graph on `framework_vcs`.

use framework_vcs::{
    materialize_document_projection, ApplyDocumentOp, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, VcsError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

pub const SEMIOS_STUDIO_SCHEMA: &str = "semios.studio/v1";
pub const SEMIOS_MEDIA_GRAPH_SCHEMA: &str = "semios.media-graph/v1";

//#region 🔖Schemas
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosSourceDocument {
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
pub struct SemiosAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub source_document: SemiosSourceDocument,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosMediaGraphPort {
    pub id: String,
    pub resource_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosMediaGraphNode {
    pub id: String,
    pub instance_id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub inputs: Vec<SemiosMediaGraphPort>,
    pub outputs: Vec<SemiosMediaGraphPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosMediaGraphV1 {
    pub schema: String,
    pub nodes: Vec<SemiosMediaGraphNode>,
    pub edges: Vec<SemiosMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosStudioProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<SemiosAppInstance>,
    pub media_graph: SemiosMediaGraphV1,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemiosStudioDocumentV1 {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: framework_vcs::DocumentVcs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<framework_vcs::DocumentBackboneRef>,
}

pub type SemiosStudioEnvelope = DocumentVcsEnvelope;
//#endregion 🔖Schemas

//#region 🔖Projection
static SEMIOS_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn create_semios_id(prefix: &str) -> String {
    let n = SEMIOS_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

pub fn empty_media_graph() -> SemiosMediaGraphV1 {
    SemiosMediaGraphV1 {
        schema: SEMIOS_MEDIA_GRAPH_SCHEMA.into(),
        nodes: Vec::new(),
        edges: Vec::new(),
    }
}

pub fn default_studio_projection() -> SemiosStudioProjection {
    SemiosStudioProjection {
        programs: Vec::new(),
        active_program_id: None,
        active_alternative_id: None,
        app_instances: Vec::new(),
        media_graph: empty_media_graph(),
    }
}

pub fn create_empty_studio_document(id: &str, name: &str) -> SemiosStudioDocumentV1 {
    SemiosStudioDocumentV1 {
        schema: SEMIOS_STUDIO_SCHEMA.into(),
        id: id.into(),
        name: name.into(),
        vcs: framework_vcs::DocumentVcs {
            initial_projection: serde_json::to_value(default_studio_projection()).expect("projection"),
            operations: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
        },
        backbone: Some(framework_vcs::DocumentBackboneRef {
            kind: "dev".into(),
            uri: "dev://studio.json".into(),
        }),
    }
}

fn projection_from_value(value: &Value) -> SemiosStudioProjection {
    serde_json::from_value(value.clone()).expect("studio projection")
}

fn value_from_projection(projection: &SemiosStudioProjection) -> Value {
    serde_json::to_value(projection).expect("studio projection value")
}

fn apply_studio_operation(projection: &SemiosStudioProjection, operation: &Value) -> SemiosStudioProjection {
    let mut next = projection.clone();
    let op = operation.get("op").and_then(|v| v.as_str()).unwrap_or("");
    let payload = operation.get("payload").cloned().unwrap_or(json!({}));
    match op {
        "setActiveProgram" => {
            next.active_program_id = payload.get("programId").and_then(|v| v.as_str()).map(str::to_string);
        }
        "setActiveAlternative" => {
            next.active_alternative_id = payload.get("alternativeId").and_then(|v| v.as_str()).map(str::to_string);
        }
        "applyAppOperation" => {
            let instance_id = payload.get("instanceId").and_then(|v| v.as_str()).unwrap_or("");
            let next_source: SemiosSourceDocument =
                serde_json::from_value(payload.get("nextSource").cloned().unwrap_or(json!({}))).unwrap_or(SemiosSourceDocument {
                    format: "unknown".into(),
                    vcs_json: None,
                    inline: None,
                    payload_ref: None,
                });
            for instance in &mut next.app_instances {
                if instance.id == instance_id {
                    instance.source_document = next_source.clone();
                }
            }
        }
        "spawnAppInstance" => {
            if let Ok(instance) = serde_json::from_value::<SemiosAppInstance>(payload.get("instance").cloned().unwrap_or(json!({}))) {
                if !next.programs.contains(&instance.program_id) {
                    next.programs.push(instance.program_id.clone());
                }
                let position = payload.get("position").cloned().unwrap_or(json!({ "x": 0.0, "y": 0.0 }));
                let x = position.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let y = position.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let node = SemiosMediaGraphNode {
                    id: create_semios_id("node"),
                    instance_id: instance.id.clone(),
                    label: instance.label.clone(),
                    x,
                    y,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                };
                next.media_graph.nodes.push(node);
                next.app_instances.push(instance);
            }
        }
        "removeAppInstance" => {
            let instance_id = payload.get("instanceId").and_then(|v| v.as_str()).unwrap_or("");
            let node_id = next
                .media_graph
                .nodes
                .iter()
                .find(|node| node.instance_id == instance_id)
                .map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != instance_id);
            next.media_graph.nodes.retain(|node| node.instance_id != instance_id);
            if let Some(node_id) = node_id {
                next.media_graph
                    .edges
                    .retain(|edge| edge.source_node_id != node_id && edge.target_node_id != node_id);
            }
        }
        "connectMediaPorts" => {
            if let Ok(edge) = serde_json::from_value::<SemiosMediaGraphEdge>(payload.get("edge").cloned().unwrap_or(json!({}))) {
                next.media_graph.edges.push(edge);
            }
        }
        "disconnectMediaEdge" => {
            let edge_id = payload.get("edgeId").and_then(|v| v.as_str()).unwrap_or("");
            next.media_graph.edges.retain(|edge| edge.id != edge_id);
        }
        "moveMediaNode" => {
            let node_id = payload.get("nodeId").and_then(|v| v.as_str()).unwrap_or("");
            let x = payload.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let y = payload.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
            for node in &mut next.media_graph.nodes {
                if node.id == node_id {
                    node.x = x;
                    node.y = y;
                }
            }
        }
        "patchAppSource" => {
            let instance_id = payload.get("instanceId").and_then(|v| v.as_str()).unwrap_or("");
            let inline = payload.get("inline").and_then(|v| v.as_str()).unwrap_or("").to_string();
            for instance in &mut next.app_instances {
                if instance.id == instance_id {
                    instance.source_document.inline = Some(inline.clone());
                }
            }
        }
        _ => {}
    }
    next
}

pub struct StudioApplier;

impl ApplyDocumentOp for StudioApplier {
    fn apply(&self, projection: &Value, operation: &Value) -> Result<Value, VcsError> {
        let current = projection_from_value(projection);
        let next = apply_studio_operation(&current, operation);
        Ok(value_from_projection(&next))
    }
}

pub fn materialize_studio_projection(document: &SemiosStudioDocumentV1, applied_change_ids: &[String]) -> Result<SemiosStudioProjection, VcsError> {
    let envelope = DocumentVcsEnvelope {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document.backbone.clone(),
        active_alternative_id: None,
    };
    let value = materialize_document_projection(&envelope, applied_change_ids, &StudioApplier)?;
    Ok(projection_from_value(&value))
}
//#endregion 🔖Projection

//#region 🔖StudioStore
pub struct StudioStore {
    inner: DocumentVcsStore,
    name: String,
}

impl StudioStore {
    pub fn new(document: SemiosStudioDocumentV1) -> Self {
        let envelope = DocumentVcsEnvelope {
            schema: document.schema,
            id: document.id,
            vcs: document.vcs,
            backbone: document.backbone,
            active_alternative_id: None,
        };
        Self {
            inner: DocumentVcsStore::new(envelope, Arc::new(StudioApplier)),
            name: document.name,
        }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<SemiosStudioProjection, VcsError> {
        let value = self.inner.projection()?;
        Ok(projection_from_value(&value))
    }

    pub fn document(&self) -> SemiosStudioDocumentV1 {
        let envelope = self.inner.envelope();
        SemiosStudioDocumentV1 {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            backbone: envelope.backbone.clone(),
        }
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentVcsCommand =
            serde_json::from_str(command_json).map_err(|e| VcsError::Json(e.to_string()))?;
        self.inner.dispatch(command)
    }

    pub fn dispatch_apply(&mut self, forwards: Vec<Value>, backwards: Vec<Value>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentVcsCommand::Apply {
            forwards,
            backwards,
            description: None,
        })
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
            let document: SemiosStudioDocumentV1 =
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
        let instance = SemiosAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            source_document: SemiosSourceDocument {
                format: "draw.document/v1".into(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        store
            .dispatch_apply(
                vec![json!({
                    "op": "spawnAppInstance",
                    "payload": {
                        "instance": instance,
                        "position": { "x": 0, "y": 0 }
                    }
                })],
                vec![json!({ "op": "removeAppInstance", "payload": { "instanceId": "app-1" } })],
            )
            .expect("spawn");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.app_instances.len(), 1);
        assert_eq!(projection.media_graph.nodes.len(), 1);
    }

    #[test]
    fn undo_after_spawn() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SemiosAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            source_document: SemiosSourceDocument {
                format: "draw.document/v1".into(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        store
            .dispatch_apply(
                vec![json!({
                    "op": "spawnAppInstance",
                    "payload": { "instance": instance, "position": { "x": 0, "y": 0 } }
                })],
                vec![json!({ "op": "removeAppInstance", "payload": { "instanceId": "app-1" } })],
            )
            .expect("spawn");
        store
            .dispatch_json(r#"{"kind":"undo"}"#)
            .expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }
}
//#endregion 🧪Tests
