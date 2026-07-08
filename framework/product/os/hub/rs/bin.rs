mod header {
    // 🧲Header
    // OS hub — generic VFS + document op log with REST and WebSocket streaming.
}

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
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
    bus: broadcast::Sender<HubEvent>,
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
struct HubEvent {
    document_id: String,
    version: i64,
    envelope: OpEnvelope,
    insert_result: String,
}

#[derive(Serialize)]
struct DocumentResponse {
    snapshot: Value,
    version: i64,
}

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
    let _ = state.bus.send(HubEvent {
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
                    if event.document_id != document_id {
                        continue;
                    }
                    let payload = serde_json::json!({
                        "kind": "op",
                        "version": event.version,
                        "envelope": event.envelope,
                        "insertResult": event.insert_result,
                    });
                    if socket.send(Message::Text(payload.to_string().into())).await.is_err() {
                        break;
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

    #[tokio::test]
    async fn append_op_increments_version() {
        let (bus, _) = broadcast::channel(8);
        let state = HubState {
            nodes: Arc::new(DashMap::new()),
            documents: Arc::new(DashMap::new()),
            ops: Arc::new(DashMap::new()),
            bus,
        };
        seed_state(&state);
        let response = append_op(
            Path("default".into()),
            State(state.clone()),
            Json(AppendOpRequest {
                version: 0,
                change: serde_json::json!({ "id": "change-1", "forwards": [], "backwards": [] }),
            }),
        )
        .await
        .expect("append");
        assert_eq!(response.version, 1);
    }
}
