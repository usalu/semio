//! 🛟️ Backend-neutral helpers every directory backend shares: the driver-error mapper, the admin-effect preflight classification and the actor-kind column vocabulary. None of them touches a driver, so each backend imports them once from here instead of keeping a copy.
//!
//! See also `📇️directory/🪶️sqlite`, `📇️directory/🐘️postgres` and `📇️directory/🌐️neo4j`, the backends that import them.

use crate::error::{DirectoryError, DirectoryResult};
use crate::model::AdminEffectCommitV1;
use directory::os_directory::DirectoryActorKind;

/// 🚧️ Maps a driver error onto the opaque [`DirectoryError::Backend`] so no driver type crosses a backend's public API.
pub fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

/// 🧭️ Why an admin effect never reached its commit: the preflight refused it, or the backend could not tell.
pub enum AdminEffectPreflightFailure {
    RejectedBeforeCommit,
    Indeterminate,
}

impl<T> From<AdminEffectPreflightFailure> for AdminEffectCommitV1<T> {
    fn from(value: AdminEffectPreflightFailure) -> Self {
        match value {
            AdminEffectPreflightFailure::RejectedBeforeCommit => Self::RejectedBeforeCommit,
            AdminEffectPreflightFailure::Indeterminate => Self::Indeterminate,
        }
    }
}

/// 🔎️ Classifies a preflight result: a backend fault is indeterminate, every other refusal happened before the commit.
pub fn admin_effect_preflight<T>(result: DirectoryResult<T>) -> Result<T, AdminEffectPreflightFailure> {
    match result {
        Ok(value) => Ok(value),
        Err(DirectoryError::Backend(_)) => Err(AdminEffectPreflightFailure::Indeterminate),
        Err(_) => Err(AdminEffectPreflightFailure::RejectedBeforeCommit),
    }
}

/// 🏷️ The lowercase actor-kind text every backend stores in its `actor_kind` column.
pub fn actor_kind_to_str(kind: DirectoryActorKind) -> &'static str {
    match kind {
        DirectoryActorKind::User => "user",
        DirectoryActorKind::Admin => "admin",
        DirectoryActorKind::System => "system",
    }
}

/// 🔡️ Reads the stored actor-kind text back; anything unrecognised is a plain user.
pub fn actor_kind_from_str(value: &str) -> DirectoryActorKind {
    match value {
        "admin" => DirectoryActorKind::Admin,
        "system" => DirectoryActorKind::System,
        _ => DirectoryActorKind::User,
    }
}
