//! 🗄️ Generic document VCS engine — typed Change/Checkpoint/Alternative, materialize-by-replay, backbone.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
pub struct DocumentChange<Op> {
    pub id: String,
    pub forwards: Vec<Op>,
    pub backwards: Vec<Op>,
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
pub struct DocumentVcs<P, Op> {
    pub initial_projection: P,
    pub operations: Vec<DocumentChange<Op>>,
    pub checkpoints: Vec<DocumentCheckpoint>,
    pub alternatives: Vec<DocumentAlternative>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcsEnvelope<P, Op> {
    pub schema: String,
    pub id: String,
    pub vcs: DocumentVcs<P, Op>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentVcsCommand<Op> {
    Apply {
        operations: Vec<Op>,
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
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("deserialize error: {0}")]
    Deserialize(String),
    #[error("backbone error: {0}")]
    Backbone(String),
    #[error("remote sync not implemented")]
    RemoteSyncNotImplemented,
}
//#endregion 🔖Errors

//#region 🔖CollectionDiff
/// @emoji 🧩 Sparse collection patch entry (mirrors compose `XModified`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩 Sparse collection diff (mirrors compose `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self {
            removed: Vec::new(),
            modified: Vec::new(),
            added: Vec::new(),
        }
    }
}
//#endregion 🔖CollectionDiff

//#region 🔖Operation
/// @emoji 📦 Centralized projection mutation — one `apply` per technology.
pub trait OperationDiff<P>: Clone + Default + Serialize + DeserializeOwned {
    fn apply(&self, projection: &P) -> P;
    fn absorb(&mut self, other: Self);
}

/// @emoji 🔁 Stored operation: emits a diff and computes backwards from pre-state.
pub trait Operation<P>: Clone + Serialize + DeserializeOwned {
    type Diff: OperationDiff<P>;
    fn diff(&self, projection: &P) -> Self::Diff;
    fn backwards(&self, projection: &P) -> Vec<Self>;
}

pub fn apply_operation<P, Op>(projection: &P, operation: &Op) -> P
where
    Op: Operation<P>,
{
    operation.diff(projection).apply(projection)
}
//#endregion 🔖Operation

//#region 🔖Materialize
pub fn create_document_vcs_envelope<P, Op>(
    schema: &str,
    id: &str,
    initial_projection: P,
    backbone: Option<DocumentBackboneRef>,
) -> DocumentVcsEnvelope<P, Op>
where
    P: Clone,
{
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

pub fn materialize_document_projection<P, Op>(
    envelope: &DocumentVcsEnvelope<P, Op>,
    applied_change_ids: &[String],
) -> Result<P, VcsError>
where
    P: Clone,
    Op: Operation<P>,
{
    let mut projection = envelope.vcs.initial_projection.clone();
    for change_id in applied_change_ids {
        let change = envelope
            .vcs
            .operations
            .iter()
            .find(|entry| entry.id == *change_id)
            .ok_or_else(|| VcsError::UnknownChange(change_id.clone()))?;
        for operation in &change.forwards {
            projection = apply_operation(&projection, operation);
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
pub struct DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    envelope: DocumentVcsEnvelope<P, Op>,
    backbone: Option<Box<dyn Backbone>>,
    applied_change_ids: Vec<String>,
    redo_change_ids: Vec<String>,
    generation: u64,
}

impl<P, Op> DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    pub fn new(envelope: DocumentVcsEnvelope<P, Op>) -> Self {
        let backbone = envelope
            .backbone
            .as_ref()
            .and_then(|entry| resolve_backbone(&entry.uri).ok());
        Self {
            envelope,
            backbone,
            applied_change_ids: Vec::new(),
            redo_change_ids: Vec::new(),
            generation: 0,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &DocumentVcsEnvelope<P, Op> {
        &self.envelope
    }

    pub fn applied_change_ids(&self) -> &[String] {
        &self.applied_change_ids
    }

    pub fn set_envelope(&mut self, envelope: DocumentVcsEnvelope<P, Op>, applied_change_ids: Vec<String>) {
        self.backbone = envelope
            .backbone
            .as_ref()
            .and_then(|entry| resolve_backbone(&entry.uri).ok());
        self.envelope = envelope;
        self.applied_change_ids = applied_change_ids;
        self.redo_change_ids.clear();
        self.bump();
    }

    pub fn projection(&self) -> Result<P, VcsError> {
        materialize_document_projection(&self.envelope, &self.applied_change_ids)
    }

    pub fn dispatch(&mut self, command: DocumentVcsCommand<Op>) -> Result<(), VcsError> {
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
                operations,
                description,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let mut projection = self.projection()?;
                let mut forwards = Vec::with_capacity(operations.len());
                let mut backwards = Vec::new();
                for operation in operations {
                    let mut back = operation.backwards(&projection);
                    back.reverse();
                    backwards.extend(back);
                    projection = apply_operation(&projection, &operation);
                    forwards.push(operation);
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

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentVcsCommand<Op> =
            serde_json::from_str(command_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        self.dispatch(command)
    }

    pub fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(&self.envelope).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    pub fn projection_json(&self) -> Result<String, VcsError> {
        let projection = self.projection()?;
        serde_json::to_string(&projection).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    pub fn sync_backbone(&self) -> Result<(), VcsError> {
        let backbone = self
            .backbone
            .as_ref()
            .ok_or_else(|| VcsError::Backbone("no backbone attached".into()))?;
        let json = self.envelope_json()?;
        backbone.sync(&json)
    }

    pub fn load_backbone(&mut self) -> Result<(), VcsError> {
        let backbone = self
            .backbone
            .as_ref()
            .ok_or_else(|| VcsError::Backbone("no backbone attached".into()))?;
        let json = backbone.load()?;
        let loaded: DocumentVcsEnvelope<P, Op> =
            serde_json::from_str(&json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        self.set_envelope(loaded, Vec::new());
        Ok(())
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

/// @emoji 🗄️ Opaque envelope persistence — callers only pass a URI.
pub trait Backbone: Send + Sync {
    fn load(&self) -> Result<String, VcsError>;
    fn sync(&self, envelope_json: &str) -> Result<(), VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
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

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

pub struct DevJsonFileBackbone {
    uri: String,
    #[cfg(not(target_arch = "wasm32"))]
    port: Option<Arc<dyn BackbonePort>>,
}

impl DevJsonFileBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !uri.starts_with("dev://") {
            return Err(VcsError::Backbone(format!("expected dev:// uri, got {uri}")));
        }
        Ok(Self {
            uri: uri.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            port: None,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_port(mut self, port: Arc<dyn BackbonePort>) -> Self {
        self.port = Some(port);
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn file_path(&self) -> Result<std::path::PathBuf, VcsError> {
        let relative = self.uri.strip_prefix("dev://").unwrap_or(&self.uri);
        Ok(std::path::PathBuf::from(relative))
    }
}

impl Backbone for DevJsonFileBackbone {
    fn load(&self) -> Result<String, VcsError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(port) = &self.port {
                return port.read(&self.uri);
            }
            let path = self.file_path()?;
            std::fs::read_to_string(&path).map_err(|e| VcsError::Backbone(e.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err(VcsError::Backbone("dev file backbone is native-only".into()))
        }
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(port) = &self.port {
                return port.write(&self.uri, envelope_json);
            }
            let path = self.file_path()?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| VcsError::Backbone(e.to_string()))?;
            }
            std::fs::write(&path, envelope_json).map_err(|e| VcsError::Backbone(e.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = envelope_json;
            Err(VcsError::Backbone("dev file backbone is native-only".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct SqliteFolderBackbone {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl SqliteFolderBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        let folder = uri
            .strip_prefix("local://")
            .or_else(|| uri.strip_prefix("sqlite://"))
            .ok_or_else(|| VcsError::Backbone(format!("expected local:// or sqlite:// uri, got {uri}")))?;
        Ok(Self {
            folder: std::path::PathBuf::from(folder),
        })
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.folder.join("vcs.sqlite")
    }

    fn connection(&self) -> Result<rusqlite::Connection, VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| VcsError::Backbone(e.to_string()))?;
        rusqlite::Connection::open(self.db_path()).map_err(|e| VcsError::Backbone(e.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Backbone for SqliteFolderBackbone {
    fn load(&self) -> Result<String, VcsError> {
        let conn = self.connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS envelope (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL);",
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        let json: String = conn
            .query_row("SELECT json FROM envelope WHERE id = 1", [], |row| row.get(0))
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(json)
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        let conn = self.connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS envelope (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL);",
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        conn.execute(
            "INSERT INTO envelope (id, json) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET json = excluded.json",
            [envelope_json],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }
}

pub struct RemoteHttpBackbone {
    uri: String,
    last_conflict: Mutex<Option<StudioConflict>>,
}

impl RemoteHttpBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !(uri.starts_with("remote://") || uri.starts_with("http://") || uri.starts_with("https://")) {
            return Err(VcsError::Backbone(format!("expected remote/http uri, got {uri}")));
        }
        Ok(Self {
            uri: uri.to_string(),
            last_conflict: Mutex::new(None),
        })
    }

    pub fn last_conflict(&self) -> Option<StudioConflict> {
        self.last_conflict.lock().ok().and_then(|g| g.clone())
    }
}

impl Backbone for RemoteHttpBackbone {
    fn load(&self) -> Result<String, VcsError> {
        let _ = &self.uri;
        Err(VcsError::RemoteSyncNotImplemented)
    }

    fn sync(&self, _envelope_json: &str) -> Result<(), VcsError> {
        let conflict = StudioConflict {
            kind: "studio-conflict".into(),
            uri: self.uri.clone(),
            message: "remote backbone sync is not implemented".into(),
        };
        if let Ok(mut guard) = self.last_conflict.lock() {
            *guard = Some(conflict);
        }
        Err(VcsError::RemoteSyncNotImplemented)
    }
}

/// @emoji 🔌 Resolves a backbone URI to a concrete storage implementation.
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    let scheme = uri.split("://").next().unwrap_or("");
    match scheme {
        "dev" => Ok(Box::new(DevJsonFileBackbone::new(uri)?)),
        "local" | "sqlite" => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Ok(Box::new(SqliteFolderBackbone::new(uri)?))
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = uri;
                Err(VcsError::Backbone("sqlite backbone is native-only".into()))
            }
        }
        "remote" | "http" | "https" => Ok(Box::new(RemoteHttpBackbone::new(uri)?)),
        _ => Err(VcsError::Backbone(format!("unsupported backbone uri: {uri}"))),
    }
}
//#endregion 🔖Backbone

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoProjection {
        n: i32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection {
                n: self.n.unwrap_or(projection.n),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op")]
    enum DemoOp {
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOp {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOp::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOp::SetN { n: projection.n }]
        }
    }

    #[test]
    fn materialize_replays_forward_ops() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").n, 0);
        store.dispatch(DocumentVcsCommand::Redo).expect("redo");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let change = &store.envelope().vcs.operations[0];
        assert_eq!(change.backwards, vec![DemoOp::SetN { n: 0 }]);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
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
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on branch");
        store
            .dispatch(DocumentVcsCommand::SwitchAlternative {
                alternative_id: alt_id,
            })
            .expect("switch");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn dev_json_backbone_round_trip() {
        let port = Arc::new(MemoryBackbonePort::new());
        let backbone = DevJsonFileBackbone::new("dev://demo.json")
            .expect("backbone")
            .with_port(port.clone());
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 1 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        backbone.sync(&json).expect("sync");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&backbone.load().expect("load")).expect("parse");
        assert_eq!(loaded.id, "demo");
    }

    #[test]
    fn sqlite_folder_backbone_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let uri = format!("local://{}", dir.path().display());
        let backbone = SqliteFolderBackbone::new(&uri).expect("backbone");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 3 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        backbone.sync(&json).expect("sync");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&backbone.load().expect("load")).expect("parse");
        assert_eq!(loaded.vcs.initial_projection.n, 3);
    }

    #[test]
    fn remote_backbone_sync_reports_conflict() {
        let remote = RemoteHttpBackbone::new("remote://studio").expect("attach");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        assert_eq!(remote.sync(&json), Err(VcsError::RemoteSyncNotImplemented));
        assert_eq!(
            remote.last_conflict().map(|c| c.kind),
            Some("studio-conflict".into())
        );
    }

    #[test]
    fn store_syncs_through_resolved_backbone() {
        let port = Arc::new(MemoryBackbonePort::new());
        let backbone_uri = "dev://studio-store.json".to_string();
        DevJsonFileBackbone::new(&backbone_uri)
            .expect("backbone")
            .with_port(port.clone())
            .sync("{}")
            .expect("seed");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> = create_document_vcs_envelope(
            "demo/v1",
            "demo",
            DemoProjection { n: 0 },
            Some(DocumentBackboneRef {
                kind: "dev".into(),
                uri: backbone_uri,
            }),
        );
        let store = DocumentVcsStore::new(envelope);
        store.sync_backbone().expect("sync");
        assert!(port.read("dev://studio-store.json").is_ok());
    }
}
//#endregion 🧪Tests
