// #region 🔖Header
// [👤semio📚server💻semio-session🔖error](repo://p/u/semio/b/l/server/f/error.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Error types for the session backend service.
// #endregion 🔖Header

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

// #region 🔖SessionError
// SessionError MUST cover all service error cases.

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found: {0}")]
    SessionNotFound(String),

    #[error("entity not found: {kind} {guid}")]
    EntityNotFound { kind: String, guid: String },

    #[error("conflict on property {property}: {reason}")]
    Conflict { property: String, reason: String },

    #[error("validation error: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx_core::Error),

    #[error("actor mailbox closed")]
    ActorGone,

    #[error("idempotent duplicate: command {0} already processed")]
    IdempotentDuplicate(String),

    #[error("internal error: {0}")]
    Internal(String),
}

// #endregion 🔖SessionError

// #region 🔖ErrorResponse
// ErrorResponse MUST serialize error details for HTTP responses.

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    detail: String,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let (status, error, detail) = match &self {
            SessionError::SessionNotFound(_) => {
                (StatusCode::NOT_FOUND, "session_not_found", self.to_string())
            }
            SessionError::EntityNotFound { .. } => {
                (StatusCode::NOT_FOUND, "entity_not_found", self.to_string())
            }
            SessionError::Conflict { .. } => {
                (StatusCode::CONFLICT, "conflict", self.to_string())
            }
            SessionError::Validation(_) => {
                (StatusCode::BAD_REQUEST, "validation", self.to_string())
            }
            SessionError::IdempotentDuplicate(_) => {
                (StatusCode::OK, "idempotent_duplicate", self.to_string())
            }
            SessionError::ActorGone => {
                (StatusCode::SERVICE_UNAVAILABLE, "actor_gone", self.to_string())
            }
            SessionError::Database(_) | SessionError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal", self.to_string())
            }
        };

        let body = ErrorBody {
            error: error.to_string(),
            detail,
        };
        (status, axum::Json(body)).into_response()
    }
}

// #endregion 🔖ErrorResponse
