// #region 🔖Header
// [👤semio📚server💻semio-session🔖ws](repo://p/u/semio/b/l/server/f/ws.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// WebSocket handler for real-time session event streaming.
// #endregion 🔖Header

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::api::AppState;
use crate::domain::SessionId;
use crate::event::SessionEvent;

// #region 🔖WebSocket Handler
// WebSocket Handler MUST upgrade HTTP to WS and stream session events.

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, session_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, session_id: Uuid) {
    let handle = match state.directory.get_or_activate(SessionId(session_id)).await {
        Some(h) => h,
        None => {
            tracing::warn!("ws: session {} not found", session_id);
            return;
        }
    };

    let mut event_rx = handle.event_tx.subscribe();
    let (mut ws_tx, mut ws_rx) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let json = match serde_json::to_string(&event) {
                Ok(j) => j,
                Err(e) => {
                    tracing::error!("ws serialize error: {}", e);
                    continue;
                }
            };
            if ws_tx.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(data) => {
                    // pong handled automatically by axum
                }
                Message::Text(text) => {
                    tracing::debug!("ws received text: {}", text);
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    tracing::debug!("ws connection closed for session {}", session_id);
}

// #endregion 🔖WebSocket Handler
