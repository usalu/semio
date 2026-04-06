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
    Accepted { domain_version: DomainVersion },
    Rejected { conflicts: Vec<ConflictDetail> },
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

// #region 🔖Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_command_create_type_serde_roundtrip() {
        let cmd = DomainCommand::CreateType(CreateEntity {
            entity_id: Uuid::nil(),
            fields: serde_json::json!({"name": "Wall"}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let deser: DomainCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, DomainCommand::CreateType(_)));
    }

    #[test]
    fn domain_command_batch_serde() {
        let batch = DomainCommand::Batch(DomainBatch {
            commands: vec![
                DomainCommand::PatchKit(PatchKit {
                    fields: serde_json::json!({"name": "new-name"}),
                }),
                DomainCommand::DeleteType(DeleteEntity {
                    entity_id: Uuid::nil(),
                }),
            ],
        });
        let json = serde_json::to_string(&batch).unwrap();
        let deser: DomainCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, DomainCommand::Batch(_)));
    }

    #[test]
    fn semio_command_upsert_cursor_serde() {
        let cmd = SemioCommand::UpsertCursor(UpsertCursor { u: 1.0, v: 2.0 });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("\"kind\":\"UpsertCursor\""));
        let deser: SemioCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, SemioCommand::UpsertCursor(_)));
    }

    #[test]
    fn semio_command_set_selection_serde() {
        let cmd = SemioCommand::SetSelection(SetSelection {
            piece_ids: vec![Uuid::nil()],
            design_ids: vec![],
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let deser: SemioCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, SemioCommand::SetSelection(_)));
    }

    #[test]
    fn command_result_accepted_serde() {
        let result = CommandResult::Accepted { domain_version: 42 };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"status\":\"Accepted\""));
        let deser: CommandResult = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            deser,
            CommandResult::Accepted { domain_version: 42 }
        ));
    }

    #[test]
    fn command_result_idempotent_duplicate_serde() {
        let result = CommandResult::IdempotentDuplicate;
        let json = serde_json::to_string(&result).unwrap();
        let deser: CommandResult = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, CommandResult::IdempotentDuplicate));
    }

    #[test]
    fn command_envelope_serde_roundtrip() {
        let env = CommandEnvelope {
            command_id: CommandId(Uuid::nil()),
            client_id: ClientId(Uuid::nil()),
            request_id: RequestId(Uuid::nil()),
            actor_person_id: PersonId(Uuid::nil()),
            base_domain_version: 0,
        };
        let json = serde_json::to_string(&env).unwrap();
        let deser: CommandEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(deser.base_domain_version, 0);
    }

    #[test]
    fn create_piece_serde() {
        let cmd = DomainCommand::CreatePiece(CreatePiece {
            piece_id: Uuid::nil(),
            design_id: Uuid::nil(),
            fields: serde_json::json!({"name": "p1"}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let deser: DomainCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, DomainCommand::CreatePiece(_)));
    }

    #[test]
    fn create_connection_serde() {
        let cmd = DomainCommand::CreateConnection(CreateConnection {
            connection_id: Uuid::nil(),
            design_id: Uuid::nil(),
            fields: serde_json::json!({}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let deser: DomainCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, DomainCommand::CreateConnection(_)));
    }
}

// #endregion 🔖Tests
