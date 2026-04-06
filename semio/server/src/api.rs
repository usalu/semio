// #region 🔖Header
// [👤semio📚server💻semio-session🔖api](repo://p/u/semio/b/l/server/f/api.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// HTTP API routes for session management and command submission.
// #endregion 🔖Header

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx_postgres::PgPool;
use std::sync::Arc;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::actor::ActorMessage;
use crate::command::*;
use crate::directory::{SessionDirectory, SessionHandle};
use crate::domain::*;
use crate::error::SessionError;
use crate::ws;

// #region 🔖AppState
// AppState MUST hold shared resources for all routes.

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub directory: SessionDirectory,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let directory = SessionDirectory::new(pool.clone());
        Self { pool, directory }
    }
}

// #endregion 🔖AppState

// #region 🔖Router
// Router MUST define all HTTP endpoints.

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/sessions", post(create_session))
        .route("/sessions/{session_id}/snapshot", get(get_snapshot))
        .route("/sessions/{session_id}/commands/domain", post(post_domain_command))
        .route("/sessions/{session_id}/commands/semio", post(post_semio_command))
        .route("/sessions/{session_id}/ws", get(ws::ws_handler))
        .with_state(state)
}

// #endregion 🔖Router

// #region 🔖Health

async fn health() -> &'static str {
    "ok"
}

// #endregion 🔖Health

// #region 🔖Create Session

#[derive(Deserialize)]
struct CreateSessionRequest {
    kit_name: String,
}

#[derive(Serialize)]
struct CreateSessionResponse {
    session_id: Uuid,
    kit_id: Uuid,
}

async fn create_session(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, SessionError> {
    let session_id = Uuid::now_v7();
    let kit_id = Uuid::now_v7();

    crate::persistence::create_session(
        &state.pool, session_id, kit_id, &req.kit_name,
    ).await?;

    Ok(Json(CreateSessionResponse { session_id, kit_id }))
}

// #endregion 🔖Create Session

// #region 🔖Get Snapshot

async fn get_snapshot(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<crate::actor::SessionSnapshot>, SessionError> {
    let handle = state.directory
        .get_or_activate(SessionId(session_id))
        .await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;

    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::GetSnapshot { reply: tx })
        .await
        .map_err(|_| SessionError::ActorGone)?;

    let snapshot = rx.await.map_err(|_| SessionError::ActorGone)?;
    Ok(Json(snapshot))
}

// #endregion 🔖Get Snapshot

// #region 🔖Domain Command

#[derive(Deserialize)]
struct DomainCommandRequest {
    #[serde(flatten)]
    envelope: CommandEnvelope,
    #[serde(flatten)]
    command: DomainCommand,
}

async fn post_domain_command(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<DomainCommandRequest>,
) -> Result<Json<CommandResult>, SessionError> {
    let handle = state.directory
        .get_or_activate(SessionId(session_id))
        .await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;

    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::DomainCommand {
        envelope: req.envelope,
        command: req.command,
        reply: tx,
    })
    .await
    .map_err(|_| SessionError::ActorGone)?;

    let result = rx.await.map_err(|_| SessionError::ActorGone)??;
    Ok(Json(result))
}

// #endregion 🔖Domain Command

// #region 🔖Semio Command

#[derive(Deserialize)]
struct SemioCommandRequest {
    #[serde(flatten)]
    envelope: SemioEnvelope,
    #[serde(flatten)]
    command: SemioCommand,
}

async fn post_semio_command(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<SemioCommandRequest>,
) -> Result<Json<serde_json::Value>, SessionError> {
    let handle = state.directory
        .get_or_activate(SessionId(session_id))
        .await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;

    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::SemioCommand {
        envelope: req.envelope,
        command: req.command,
        reply: tx,
    })
    .await
    .map_err(|_| SessionError::ActorGone)?;

    rx.await.map_err(|_| SessionError::ActorGone)??;
    Ok(Json(serde_json::json!({"status": "ok"})))
}

// #endregion 🔖Semio Command
