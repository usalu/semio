// #region 🔖Header
// [👤semio📚server💻semio-session🔖event](repo://p/u/semio/b/l/server/f/event.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Broadcast event types for domain and semio state changes.
// #endregion 🔖Header

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::command::{CommandResult, ConflictDetail};
use crate::domain::*;

// #region 🔖SessionEvent
// SessionEvent MUST enumerate all broadcastable events.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum SessionEvent {
    DomainCommandAccepted {
        command_id: CommandId,
        domain_version: DomainVersion,
        changes: Vec<EntityChange>,
    },
    DomainCommandRejected {
        command_id: CommandId,
        conflicts: Vec<ConflictDetail>,
    },
    SemioUpdated {
        semio_version: SemioVersion,
        person_id: PersonId,
        frontend_id: String,
        update: SemioUpdate,
    },
    SessionClosed,
}

// #endregion 🔖SessionEvent

// #region 🔖EntityChange
// EntityChange MUST describe what changed in a domain command.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum EntityChange {
    Created {
        entity_kind: EntityKind,
        entity_id: Uuid,
        snapshot: serde_json::Value,
    },
    Updated {
        entity_kind: EntityKind,
        entity_id: Uuid,
        changed_fields: serde_json::Value,
    },
    Deleted {
        entity_kind: EntityKind,
        entity_id: Uuid,
    },
}

// #endregion 🔖EntityChange

// #region 🔖SemioUpdate
// SemioUpdate MUST describe semio state changes.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SemioUpdate {
    CursorMoved { u: f64, v: f64 },
    LookChanged {
        position: [f64; 3],
        forward: [f64; 3],
        up: [f64; 3],
    },
    SelectionChanged {
        piece_ids: Vec<Uuid>,
        design_ids: Vec<Uuid>,
    },
    PresenceCleared,
}

// #endregion 🔖SemioUpdate
