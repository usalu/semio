mod header {
    // 🧲Header
    // OS hub — generic VFS + document op log with REST and WebSocket streaming.
}

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use semio_framework_core::OpEnvelope;
use semio_framework_sync::OpDag;
use uuid::Uuid;

#[derive(Clone)]
struct HubState {
    nodes: Arc<DashMap<Uuid, NodeRow>>,
    documents: Arc<DashMap<String, DocumentRow>>,
    ops: Arc<DashMap<String, Vec<OpRow>>>,
    dags: Arc<DashMap<String, OpDag>>,
    bus: broadcast::Sender<HubBusEvent>,
}

#[derive(Clone, Serialize, Deserialize)]
struct NodeRow {
    id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    kind: String,
    document_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct DocumentRow {
    id: String,
    schema: String,
    snapshot: Value,
    version: i64,
}

#[derive(Clone, Serialize, Deserialize)]
struct OpRow {
    id: Uuid,
    document_id: String,
    version: i64,
    envelope: OpEnvelope,
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum HubBusEvent {
    Op {
        document_id: String,
        version: i64,
        envelope: OpEnvelope,
        insert_result: String,
    },
    Envelope {
        document_id: String,
        version: i64,
        envelope: Value,
    },
}

#[derive(Serialize)]
struct DocumentResponse {
    snapshot: Value,
    version: i64,
}

#[derive(Serialize)]
struct EnvelopeResponse {
    envelope: Value,
    version: i64,
}

#[derive(Deserialize)]
struct PutEnvelopeRequest {
    version: i64,
    envelope: Value,
}

#[derive(Serialize)]
struct PutEnvelopeResponse {
    version: i64,
}

#[derive(Clone, Serialize)]
struct HubEnvelopeEvent {
    document_id: String,
    version: i64,
    envelope: Value,
}

// kept for local clarity in put_envelope; bus uses HubBusEvent

#[derive(Deserialize)]
struct AppendOpRequest {
    version: i64,
    envelope: OpEnvelope,
}

#[derive(Serialize)]
struct AppendOpResponse {
    version: i64,
}

fn default_snapshot() -> Value {
    serde_json::json!({
        "schema": "s.studio/v1",
        "id": "default",
        "name": "Studio",
        "vcs": {
            "initialProjection": {
                "programs": [],
                "activeProgramId": null,
                "activeAlternativeId": null,
                "appInstances": [],
                "mediaGraph": { "schema": "s.media-graph", "nodes": [], "edges": [] }
            },
            "operations": [],
            "checkpoints": [],
            "alternatives": []
        }
    })
}

fn seed_state(state: &HubState) {
    if state.documents.contains_key("default") {
        return;
    }
    let folder_id = Uuid::now_v7();
    let doc_id = Uuid::now_v7();
    state.nodes.insert(
        folder_id,
        NodeRow {
            id: folder_id,
            parent_id: None,
            name: "Documents".into(),
            kind: "folder".into(),
            document_id: None,
        },
    );
    state.nodes.insert(
        doc_id,
        NodeRow {
            id: doc_id,
            parent_id: Some(folder_id),
            name: "default".into(),
            kind: "document".into(),
            document_id: Some("default".into()),
        },
    );
    state.documents.insert(
        "default".into(),
        DocumentRow {
            id: "default".into(),
            schema: "s.studio/v1".into(),
            snapshot: default_snapshot(),
            version: 0,
        },
    );
    state.ops.insert("default".into(), Vec::new());
}

async fn list_nodes(State(state): State<HubState>) -> Json<Vec<NodeRow>> {
    seed_state(&state);
    Json(state.nodes.iter().map(|entry| entry.value().clone()).collect())
}

async fn get_document(Path(document_id): Path<String>, State(state): State<HubState>) -> Result<Json<DocumentResponse>, StatusCode> {
    seed_state(&state);
    let document = state.documents.get(&document_id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(DocumentResponse {
        snapshot: document.snapshot.clone(),
        version: document.version,
    }))
}

async fn get_envelope(
    Path(document_id): Path<String>,
    State(state): State<HubState>,
) -> Result<Json<EnvelopeResponse>, StatusCode> {
    seed_state(&state);
    let document = state.documents.get(&document_id).ok_or(StatusCode::NOT_FOUND)?;
    let envelope = document
        .snapshot
        .get("vcs")
        .cloned()
        .map(|vcs| {
            serde_json::json!({
                "schema": document.schema,
                "id": document.id,
                "vcs": vcs,
                "backbone": document.snapshot.get("backbone").cloned(),
            })
        })
        .unwrap_or_else(|| document.snapshot.clone());
    Ok(Json(EnvelopeResponse {
        envelope,
        version: document.version,
    }))
}

async fn put_envelope(
    Path(document_id): Path<String>,
    State(state): State<HubState>,
    Json(body): Json<PutEnvelopeRequest>,
) -> Result<Json<PutEnvelopeResponse>, StatusCode> {
    seed_state(&state);
    let mut document = state.documents.get(&document_id).ok_or(StatusCode::NOT_FOUND)?.clone();
    if body.version != document.version {
        return Err(StatusCode::CONFLICT);
    }
    document.version += 1;
    if let Some(envelope_obj) = body.envelope.as_object() {
        if let Some(vcs) = envelope_obj.get("vcs") {
            if let Some(snapshot) = document.snapshot.as_object_mut() {
                snapshot.insert("vcs".into(), vcs.clone());
                if let Some(schema) = envelope_obj.get("schema").and_then(|value| value.as_str()) {
                    document.schema = schema.into();
                    snapshot.insert("schema".into(), Value::String(schema.into()));
                }
                if let Some(id) = envelope_obj.get("id").and_then(|value| value.as_str()) {
                    document.id = id.into();
                    snapshot.insert("id".into(), Value::String(id.into()));
                }
                if let Some(backbone) = envelope_obj.get("backbone") {
                    snapshot.insert("backbone".into(), backbone.clone());
                }
            }
        } else {
            document.snapshot = body.envelope.clone();
        }
    } else {
        document.snapshot = body.envelope.clone();
    }
    state.documents.insert(document_id.clone(), document.clone());
    let _ = state.bus.send(HubBusEvent::Envelope {
        document_id: document_id.clone(),
        version: document.version,
        envelope: body.envelope,
    });
    Ok(Json(PutEnvelopeResponse {
        version: document.version,
    }))
}

async fn append_op(
    Path(document_id): Path<String>,
    State(state): State<HubState>,
    Json(body): Json<AppendOpRequest>,
) -> Result<Json<AppendOpResponse>, StatusCode> {
    seed_state(&state);
    let mut document = state.documents.get(&document_id).ok_or(StatusCode::NOT_FOUND)?.clone();
    if body.version != document.version {
        return Err(StatusCode::CONFLICT);
    }
    document.version += 1;
    let mut dag = state.dags.entry(document_id.clone()).or_default();
    let insert_result = dag
        .insert(body.envelope.clone())
        .map_err(|_| StatusCode::CONFLICT)?;
    let op = OpRow {
        id: Uuid::now_v7(),
        document_id: document_id.clone(),
        version: document.version,
        envelope: body.envelope.clone(),
    };
    if let Some(mut snapshot) = document.snapshot.as_object_mut() {
        if let Some(vcs) = snapshot.get_mut("vcs").and_then(|value| value.as_object_mut()) {
            let mut operations = vcs
                .get("operations")
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_default();
            operations.push(serde_json::to_value(&body.envelope).unwrap_or(Value::Null));
            vcs.insert("operations".into(), Value::Array(operations));
        }
    }
    document.snapshot = document.snapshot.clone();
    state.documents.insert(document_id.clone(), document.clone());
    state.ops.entry(document_id.clone()).or_default().push(op);
    let _ = state.bus.send(HubBusEvent::Op {
        document_id: document_id.clone(),
        version: document.version,
        envelope: body.envelope,
        insert_result: format!("{insert_result:?}"),
    });
    Ok(Json(AppendOpResponse {
        version: document.version,
    }))
}

async fn document_ws(
    ws: WebSocketUpgrade,
    Path(document_id): Path<String>,
    State(state): State<HubState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, document_id, state))
}

async fn handle_ws(mut socket: WebSocket, document_id: String, state: HubState) {
    let mut rx = state.bus.subscribe();
    loop {
        tokio::select! {
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = socket.send(Message::Pong(payload)).await;
                    }
                    _ => {}
                }
            }
            event = rx.recv() => {
                if let Ok(event) = event {
                    let payload = match event {
                        HubBusEvent::Op { document_id: id, version, envelope, insert_result } if id == document_id => {
                            Some(serde_json::json!({
                                "kind": "op",
                                "version": version,
                                "envelope": envelope,
                                "insertResult": insert_result,
                            }))
                        }
                        HubBusEvent::Envelope { document_id: id, version, envelope } if id == document_id => {
                            Some(serde_json::json!({
                                "kind": "envelope",
                                "version": version,
                                "envelope": envelope,
                            }))
                        }
                        _ => None,
                    };
                    if let Some(payload) = payload {
                        if socket.send(Message::Text(payload.to_string().into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let port: u16 = std::env::var("OS_HUB_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(6070);
    let (bus, _) = broadcast::channel(256);
    let state = HubState {
        nodes: Arc::new(DashMap::new()),
        documents: Arc::new(DashMap::new()),
        ops: Arc::new(DashMap::new()),
        dags: Arc::new(DashMap::new()),
        bus,
    };
    seed_state(&state);
    let app = Router::new()
        .route("/nodes", get(list_nodes))
        .route("/documents/{id}", get(get_document))
        .route("/documents/{id}/envelope", get(get_envelope).put(put_envelope))
        .route("/documents/{id}/ops", post(append_op))
        .route("/documents/{id}/ws", get(document_ws))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("os-hub listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

#[cfg(test)]
mod tests {
    use super::*;

    use semio_framework_core::{
        ActorId, DocumentDiff, DocumentId, InverseOperation, OperationId, PayloadHash, SchemaId,
        SchemaVersion, UndoPolicy,
    };

    fn sample_envelope(id: &str) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            document: DocumentId("default".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: Vec::new(),
            payload_hash: PayloadHash("hash".into()),
            diff: DocumentDiff {
                schema_id: SchemaId("diff.v1".into()),
                payload: serde_json::json!({"value": id}),
            },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: DocumentDiff {
                    schema_id: SchemaId("diff.v1".into()),
                    payload: serde_json::json!({}),
                },
                base_version: semio_framework_core::DocumentVersion(0),
                dependencies: Vec::new(),
                undo_policy: UndoPolicy::ExactBaseOnly,
            },
        }
    }

    #[tokio::test]
    async fn append_op_increments_version() {
        let (bus, _) = broadcast::channel(8);
        let state = HubState {
            nodes: Arc::new(DashMap::new()),
            documents: Arc::new(DashMap::new()),
            ops: Arc::new(DashMap::new()),
            dags: Arc::new(DashMap::new()),
            bus,
        };
        seed_state(&state);
        let response = append_op(
            Path("default".into()),
            State(state.clone()),
            Json(AppendOpRequest {
                version: 0,
                envelope: sample_envelope("op-1"),
            }),
        )
        .await
        .expect("append");
        assert_eq!(response.0.version, 1);
    }

    #[tokio::test]
    async fn envelope_round_trip_updates_version() {
        let (bus, _) = broadcast::channel(8);
        let state = HubState {
            nodes: Arc::new(DashMap::new()),
            documents: Arc::new(DashMap::new()),
            ops: Arc::new(DashMap::new()),
            dags: Arc::new(DashMap::new()),
            bus,
        };
        seed_state(&state);
        let loaded = get_envelope(Path("default".into()), State(state.clone()))
            .await
            .expect("get");
        let response = put_envelope(
            Path("default".into()),
            State(state.clone()),
            Json(PutEnvelopeRequest {
                version: loaded.0.version,
                envelope: loaded.0.envelope,
            }),
        )
        .await
        .expect("put");
        assert_eq!(response.0.version, loaded.0.version + 1);
    }
}
