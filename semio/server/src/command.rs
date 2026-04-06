// #region 🔖Header
// [👤semio📚server💻semio-session🔖command](repo://p/u/semio/b/l/server/f/command.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Explicit command types for domain and semio mutations.
// #endregion 🔖Header

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::*;

// #region 🔖DomainCommand
// DomainCommand MUST model every semantic mutation as an explicit variant.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub client_id: ClientId,
    pub request_id: RequestId,
    pub actor_person_id: PersonId,
    pub base_domain_version: DomainVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum DomainCommand {
    PatchKit(PatchKit),
    CreateType(CreateEntity),
    PatchType(PatchEntity),
    DeleteType(DeleteEntity),
    CreateDesign(CreateEntity),
    PatchDesign(PatchEntity),
    DeleteDesign(DeleteEntity),
    CreatePiece(CreatePiece),
    PatchPiece(PatchEntity),
    DeletePiece(DeleteEntity),
    CreateConnection(CreateConnection),
    PatchConnection(PatchEntity),
    DeleteConnection(DeleteEntity),
    CreateLayer(CreateEntity),
    PatchLayer(PatchEntity),
    DeleteLayer(DeleteEntity),
    CreateGroup(CreateEntity),
    PatchGroup(PatchEntity),
    DeleteGroup(DeleteEntity),
    CreateAuthor(CreateEntity),
    PatchAuthor(PatchEntity),
    DeleteAuthor(DeleteEntity),
    CreateTag(CreateEntity),
    PatchTag(PatchEntity),
    DeleteTag(DeleteEntity),
    CreateConcept(CreateEntity),
    PatchConcept(PatchEntity),
    DeleteConcept(DeleteEntity),
    CreatePort(CreateEntity),
    PatchPort(PatchEntity),
    DeletePort(DeleteEntity),
    CreateQuality(CreateEntity),
    PatchQuality(PatchEntity),
    DeleteQuality(DeleteEntity),
    CreateFolder(CreateEntity),
    PatchFolder(PatchEntity),
    DeleteFolder(DeleteEntity),
    CreateFile(CreateEntity),
    PatchFile(PatchEntity),
    DeleteFile(DeleteEntity),
    Batch(DomainBatch),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBatch {
    pub commands: Vec<DomainCommand>,
}

// #endregion 🔖DomainCommand

// #region 🔖Command Payloads
// Command Payloads MUST carry the data for each mutation kind.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchKit {
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntity {
    pub entity_id: Uuid,
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEntity {
    pub entity_id: Uuid,
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEntity {
    pub entity_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePiece {
    pub piece_id: Uuid,
    pub design_id: Uuid,
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnection {
    pub connection_id: Uuid,
    pub design_id: Uuid,
    pub fields: serde_json::Value,
}

// #endregion 🔖Command Payloads

// #region 🔖SemioCommand
// SemioCommand MUST model semio/presence mutations separately.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemioEnvelope {
    pub client_id: ClientId,
    pub person_id: PersonId,
    pub frontend_id: String,
    pub base_semio_version: SemioVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum SemioCommand {
    UpsertCursor(UpsertCursor),
    UpsertLook(UpsertLook),
    SetSelection(SetSelection),
    ClearPresence(ClearPresence),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCursor {
    pub u: f64,
    pub v: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertLook {
    pub position: [f64; 3],
    pub forward: [f64; 3],
    pub up: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSelection {
    pub piece_ids: Vec<Uuid>,
    pub design_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearPresence;

// #endregion 🔖SemioCommand

// #region 🔖CommandResult
// CommandResult MUST report outcome per command.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum CommandResult {
    Accepted {
        domain_version: DomainVersion,
    },
    Rejected {
        conflicts: Vec<ConflictDetail>,
    },
    IdempotentDuplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetail {
    pub property: PropertyKey,
    pub entity_kind: EntityKind,
    pub entity_id: Uuid,
    pub reason: String,
}

// #endregion 🔖CommandResult
