use thiserror::Error;

use crate::guid::Guid;

#[derive(Error, Debug)]
pub enum SemioError {
    #[error("entity not found: {kind} '{guid}'")]
    NotFound { kind: &'static str, guid: Guid },

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("invalid operation: {0}")]
    InvalidOperation(String),

    #[error("lock poisoned accessing {0}")]
    LockPoisoned(&'static str),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SemioError>;
