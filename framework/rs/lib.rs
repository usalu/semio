//! 🗄️ Generic document VCS engine — Change/Checkpoint/Alternative, materialize-by-replay, backbone, WASM bridge.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for document VCS entities.
pub fn create_document_vcs_id(prefix: &str) -> String {
    let n = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

//#region 🔖Schemas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBackboneRef {
    pub kind: String,
    pub uri: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentChange {
    pub id: String,
    pub forwards: Vec<Value>,
    pub backwards: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCheckpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub saved_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcs {
    pub initial_projection: Value,
    pub operations: Vec<DocumentChange>,
    pub checkpoints: Vec<DocumentCheckpoint>,
    pub alternatives: Vec<DocumentAlternative>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcsEnvelope {
    pub schema: String,
    pub id: String,
    pub vcs: DocumentVcs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentVcsCommand {
    Apply {
        forwards: Vec<Value>,
        backwards: Vec<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Undo,
    Redo,
    CommitCheckpoint {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
}
//#endregion 🔖Schemas

//#region 🔖Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VcsError {
    #[error("unknown change id: {0}")]
    UnknownChange(String),
    #[error("unknown alternative id: {0}")]
    UnknownAlternative(String),
    #[error("no checkpoint for alternative")]
    NoCheckpoint,
    #[error("empty apply command")]
    EmptyApply,
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("nothing to redo")]
    NothingToRedo,
    #[error("json error: {0}")]
    Json(String),
    #[error("backbone error: {0}")]
    Backbone(String),
    #[error("remote sync not implemented")]
    RemoteSyncNotImplemented,
}
//#endregion 🔖Errors

//#region 🔖ApplyOp
/// @emoji 🔁 Technology-specific operation application.
pub trait ApplyDocumentOp: Send + Sync {
    fn apply(&self, projection: &Value, operation: &Value) -> Result<Value, VcsError>;
}

/// @emoji 🧩 Whole-projection JSON replace op for technologies without fine-grained ops yet.
pub struct JsonReplaceApplier;

impl ApplyDocumentOp for JsonReplaceApplier {
    fn apply(&self, _projection: &Value, operation: &Value) -> Result<Value, VcsError> {
        let projection = operation
            .get("projection")
            .cloned()
            .ok_or_else(|| VcsError::Json("replaceProjection missing projection".into()))?;
        Ok(projection)
    }
}

pub fn apply_json_replace_op(projection: &Value, operation: &Value) -> Value {
    JsonReplaceApplier
        .apply(projection, operation)
        .unwrap_or_else(|_| projection.clone())
}

pub fn json_replace_op(projection: Value) -> Value {
    serde_json::json!({ "op": "replaceProjection", "projection": projection })
}
//#endregion 🔖ApplyOp

//#region 🔖Materialize
pub fn create_document_vcs_envelope(
    schema: &str,
    id: &str,
    initial_projection: Value,
    backbone: Option<DocumentBackboneRef>,
) -> DocumentVcsEnvelope {
    DocumentVcsEnvelope {
        schema: schema.into(),
        id: id.into(),
        vcs: DocumentVcs {
            initial_projection,
            operations: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
        },
        backbone,
        active_alternative_id: None,
    }
}

pub fn materialize_document_projection(
    envelope: &DocumentVcsEnvelope,
    applied_change_ids: &[String],
    applier: &dyn ApplyDocumentOp,
) -> Result<Value, VcsError> {
    let mut projection = envelope.vcs.initial_projection.clone();
    for change_id in applied_change_ids {
        let change = envelope
            .vcs
            .operations
            .iter()
            .find(|entry| entry.id == *change_id)
            .ok_or_else(|| VcsError::UnknownChange(change_id.clone()))?;
        for operation in &change.forwards {
            projection = applier.apply(&projection, operation)?;
        }
    }
    Ok(projection)
}

fn now_iso() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        return format!("{ms}");
    }
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new_0().to_iso_string().into()
    }
}
//#endregion 🔖Materialize

//#region 🔖DocumentVcsStore
pub struct DocumentVcsStore {
    envelope: DocumentVcsEnvelope,
    applier: Arc<dyn ApplyDocumentOp>,
    applied_change_ids: Vec<String>,
    redo_change_ids: Vec<String>,
    generation: u64,
}

impl DocumentVcsStore {
    pub fn new(envelope: DocumentVcsEnvelope, applier: Arc<dyn ApplyDocumentOp>) -> Self {
        Self {
            envelope,
            applier,
            applied_change_ids: Vec::new(),
            redo_change_ids: Vec::new(),
            generation: 0,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &DocumentVcsEnvelope {
        &self.envelope
    }

    pub fn applied_change_ids(&self) -> &[String] {
        &self.applied_change_ids
    }

    pub fn set_envelope(&mut self, envelope: DocumentVcsEnvelope, applied_change_ids: Vec<String>) {
        self.envelope = envelope;
        self.applied_change_ids = applied_change_ids;
        self.redo_change_ids.clear();
        self.bump();
    }

    pub fn projection(&self) -> Result<Value, VcsError> {
        materialize_document_projection(&self.envelope, &self.applied_change_ids, self.applier.as_ref())
    }

    pub fn dispatch(&mut self, command: DocumentVcsCommand) -> Result<(), VcsError> {
        match command {
            DocumentVcsCommand::Undo => {
                let last = self.applied_change_ids.pop().ok_or(VcsError::NothingToUndo)?;
                self.redo_change_ids.push(last);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::Redo => {
                let next = self.redo_change_ids.pop().ok_or(VcsError::NothingToRedo)?;
                self.applied_change_ids.push(next);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CommitCheckpoint { message } => {
                self.envelope.vcs.checkpoints.push(DocumentCheckpoint {
                    id: create_document_vcs_id("checkpoint"),
                    change_ids: self.applied_change_ids.clone(),
                    message,
                    saved_at: now_iso(),
                });
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    self.dispatch(DocumentVcsCommand::CommitCheckpoint { message: None })?;
                }
                let checkpoint_id = self
                    .envelope
                    .vcs
                    .checkpoints
                    .last()
                    .map(|cp| cp.id.clone())
                    .ok_or(VcsError::NoCheckpoint)?;
                let alt_id = create_document_vcs_id("alternative");
                self.envelope.vcs.alternatives.push(DocumentAlternative {
                    id: alt_id.clone(),
                    name,
                    checkpoint_ids: vec![checkpoint_id],
                });
                self.envelope.active_alternative_id = Some(alt_id);
                self.applied_change_ids.clear();
                self.redo_change_ids.clear();
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::SwitchAlternative { alternative_id } => {
                let alternative = self
                    .envelope
                    .vcs
                    .alternatives
                    .iter()
                    .find(|alt| alt.id == alternative_id)
                    .ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?
                    .clone();
                let checkpoint_id = alternative
                    .checkpoint_ids
                    .last()
                    .ok_or(VcsError::NoCheckpoint)?
                    .clone();
                let checkpoint = self
                    .envelope
                    .vcs
                    .checkpoints
                    .iter()
                    .find(|cp| cp.id == checkpoint_id)
                    .ok_or(VcsError::NoCheckpoint)?;
                self.applied_change_ids = checkpoint.change_ids.clone();
                self.redo_change_ids.clear();
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::Apply {
                forwards,
                backwards,
                description,
            } => {
                if forwards.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let change = DocumentChange {
                    id: create_document_vcs_id("change"),
                    forwards,
                    backwards,
                    description,
                    saved_at: Some(now_iso()),
                };
                self.applied_change_ids.push(change.id.clone());
                self.envelope.vcs.operations.push(change);
                self.redo_change_ids.clear();
                self.bump();
                Ok(())
            }
        }
    }

    fn bump(&mut self) {
        self.generation += 1;
    }
}
//#endregion 🔖DocumentVcsStore

//#region 🔖Backbone
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, json: &str) -> Result<(), VcsError>;
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .get(uri)
            .cloned()
            .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    fn write(&self, uri: &str, json: &str) -> Result<(), VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .insert(uri.to_string(), json.to_string());
        Ok(())
    }
}

pub struct DevJsonBackbone {
    uri: Option<String>,
    port: Arc<dyn BackbonePort>,
}

impl DevJsonBackbone {
    pub fn new(port: Arc<dyn BackbonePort>) -> Self {
        Self { uri: None, port }
    }

    pub fn attach(&mut self, uri: &str) {
        self.uri = Some(uri.to_string());
    }

    pub fn sync(&self, envelope: &DocumentVcsEnvelope) -> Result<String, VcsError> {
        let uri = self.uri.clone().ok_or_else(|| VcsError::Backbone("not attached".into()))?;
        let mut doc = envelope.clone();
        doc.backbone = Some(DocumentBackboneRef {
            kind: "dev".into(),
            uri: uri.clone(),
        });
        let json = serde_json::to_string_pretty(&doc).map_err(|e| VcsError::Json(e.to_string()))?;
        self.port.write(&uri, &json)?;
        Ok(json)
    }

    pub fn load(&self, uri: &str) -> Result<DocumentVcsEnvelope, VcsError> {
        let json = self.port.read(uri)?;
        serde_json::from_str(&json).map_err(|e| VcsError::Json(e.to_string()))
    }
}

pub struct RemoteJsonBackbone {
    uri: Option<String>,
    last_conflict: Mutex<Option<StudioConflict>>,
}

impl RemoteJsonBackbone {
    pub fn new() -> Self {
        Self {
            uri: None,
            last_conflict: Mutex::new(None),
        }
    }

    pub fn attach(&mut self, uri: &str) -> Result<(), VcsError> {
        if !uri.starts_with("remote://") {
            return Err(VcsError::Backbone(format!("expected remote:// uri, got {uri}")));
        }
        self.uri = Some(uri.to_string());
        Ok(())
    }

    pub fn sync(&self, _envelope: &DocumentVcsEnvelope) -> Result<(), VcsError> {
        let conflict = StudioConflict {
            kind: "studio-conflict".into(),
            uri: self.uri.clone().unwrap_or_else(|| "remote://unknown".into()),
            message: "remote backbone sync is not implemented".into(),
        };
        *self
            .last_conflict
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))? = Some(conflict.clone());
        Err(VcsError::RemoteSyncNotImplemented)
    }

    pub fn last_conflict(&self) -> Option<StudioConflict> {
        self.last_conflict.lock().ok().and_then(|g| g.clone())
    }
}
//#endregion 🔖Backbone

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DocumentVcsHandle {
        store: Mutex<DocumentVcsStore>,
    }

    #[wasm_bindgen]
    impl DocumentVcsHandle {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<DocumentVcsHandle, JsValue> {
            let envelope: DocumentVcsEnvelope =
                serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self {
                store: Mutex::new(DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier))),
            })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            let command: DocumentVcsCommand =
                serde_json::from_str(command_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            store.dispatch(command).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            let store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            serde_json::to_string(store.envelope()).map_err(|e| JsValue::from_str(&e.to_string()))
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

//#region 🔖TechnologyStore
/// @emoji 🧩 JSON document store for technology WASM sessions.
pub struct JsonDocumentStore {
    inner: DocumentVcsStore,
}

impl JsonDocumentStore {
    pub fn new(schema: &str, id: &str, initial_projection: Value) -> Self {
        let envelope = create_document_vcs_envelope(schema, id, initial_projection, None);
        Self {
            inner: DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier)),
        }
    }

    pub fn from_envelope_json(envelope_json: &str) -> Result<Self, VcsError> {
        let envelope: DocumentVcsEnvelope =
            serde_json::from_str(envelope_json).map_err(|e| VcsError::Json(e.to_string()))?;
        Ok(Self {
            inner: DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier)),
        })
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentVcsCommand =
            serde_json::from_str(command_json).map_err(|e| VcsError::Json(e.to_string()))?;
        self.inner.dispatch(command)
    }

    pub fn projection_json(&self) -> Result<String, VcsError> {
        let projection = self.inner.projection()?;
        serde_json::to_string(&projection).map_err(|e| VcsError::Json(e.to_string()))
    }

    pub fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(self.inner.envelope()).map_err(|e| VcsError::Json(e.to_string()))
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }
}
//#endregion 🔖TechnologyStore

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialize_replays_forward_ops() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", serde_json::json!({ "id": "base" }), None);
        let mut store = DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier));
        store
            .dispatch(DocumentVcsCommand::Apply {
                forwards: vec![json_replace_op(serde_json::json!({ "id": "patched" }))],
                backwards: vec![json_replace_op(serde_json::json!({ "id": "base" }))],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection["id"], "patched");
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", serde_json::json!({ "n": 0 }), None);
        let mut store = DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier));
        store
            .dispatch(DocumentVcsCommand::Apply {
                forwards: vec![json_replace_op(serde_json::json!({ "n": 1 }))],
                backwards: vec![json_replace_op(serde_json::json!({ "n": 0 }))],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection")["n"], 0);
        store.dispatch(DocumentVcsCommand::Redo).expect("redo");
        assert_eq!(store.projection().expect("projection")["n"], 1);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", serde_json::json!({ "n": 0 }), None);
        let mut store = DocumentVcsStore::new(envelope, Arc::new(JsonReplaceApplier));
        store
            .dispatch(DocumentVcsCommand::Apply {
                forwards: vec![json_replace_op(serde_json::json!({ "n": 1 }))],
                backwards: vec![json_replace_op(serde_json::json!({ "n": 0 }))],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CreateAlternative {
                name: "branch-a".into(),
            })
            .expect("create alternative");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        store
            .dispatch(DocumentVcsCommand::Apply {
                forwards: vec![json_replace_op(serde_json::json!({ "n": 2 }))],
                backwards: vec![json_replace_op(serde_json::json!({ "n": 1 }))],
                description: None,
            })
            .expect("apply on branch");
        store
            .dispatch(DocumentVcsCommand::SwitchAlternative {
                alternative_id: alt_id,
            })
            .expect("switch");
        assert_eq!(store.projection().expect("projection")["n"], 1);
    }

    #[test]
    fn dev_json_backbone_round_trip() {
        let port = Arc::new(MemoryBackbonePort::new());
        let mut backbone = DevJsonBackbone::new(port);
        backbone.attach("dev://demo.json");
        let envelope = create_document_vcs_envelope("demo/v1", "demo", serde_json::json!({ "ok": true }), None);
        backbone.sync(&envelope).expect("sync");
        let loaded = backbone.load("dev://demo.json").expect("load");
        assert_eq!(loaded.id, "demo");
        assert_eq!(loaded.backbone.as_ref().map(|b| b.uri.as_str()), Some("dev://demo.json"));
    }

    #[test]
    fn remote_backbone_sync_reports_conflict() {
        let mut remote = RemoteJsonBackbone::new();
        remote.attach("remote://studio").expect("attach");
        let envelope = create_document_vcs_envelope("demo/v1", "demo", serde_json::json!({}), None);
        assert_eq!(remote.sync(&envelope), Err(VcsError::RemoteSyncNotImplemented));
        assert_eq!(
            remote.last_conflict().map(|c| c.kind),
            Some("studio-conflict".into())
        );
    }
}
//#endregion 🧪Tests
