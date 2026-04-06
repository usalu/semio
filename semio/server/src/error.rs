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
            SessionError::Conflict { .. } => (StatusCode::CONFLICT, "conflict", self.to_string()),
            SessionError::Validation(_) => {
                (StatusCode::BAD_REQUEST, "validation", self.to_string())
            }
            SessionError::IdempotentDuplicate(_) => {
                (StatusCode::OK, "idempotent_duplicate", self.to_string())
            }
            SessionError::ActorGone => (
                StatusCode::SERVICE_UNAVAILABLE,
                "actor_gone",
                self.to_string(),
            ),
            SessionError::Database(_) | SessionError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                self.to_string(),
            ),
        };

        let body = ErrorBody {
            error: error.to_string(),
            detail,
        };
        (status, axum::Json(body)).into_response()
    }
}

// #endregion 🔖ErrorResponse

// #region 🔖Tests

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    fn status_of(err: SessionError) -> StatusCode {
        err.into_response().status()
    }

    #[test]
    fn session_not_found_returns_404() {
        assert_eq!(
            status_of(SessionError::SessionNotFound("x".into())),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn entity_not_found_returns_404() {
        assert_eq!(
            status_of(SessionError::EntityNotFound {
                kind: "type".into(),
                guid: "abc".into(),
            }),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn conflict_returns_409() {
        assert_eq!(
            status_of(SessionError::Conflict {
                property: "name".into(),
                reason: "changed".into(),
            }),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn validation_returns_400() {
        assert_eq!(
            status_of(SessionError::Validation("bad".into())),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn actor_gone_returns_503() {
        assert_eq!(
            status_of(SessionError::ActorGone),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn idempotent_duplicate_returns_200() {
        assert_eq!(
            status_of(SessionError::IdempotentDuplicate("cmd".into())),
            StatusCode::OK
        );
    }

    #[test]
    fn internal_returns_500() {
        assert_eq!(
            status_of(SessionError::Internal("oops".into())),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}

// #endregion 🔖Tests
