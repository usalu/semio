// #region 🔖Header
// [👤semio📚server💻semio-session🔖domain](repo://p/u/semio/b/l/server/f/domain.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Session domain newtypes, FieldPatch, PropertyKey, ConflictPolicy.
// #endregion 🔖Header

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #region 🔖Newtype Ids
// Newtype Ids MUST wrap Uuid for each session-scoped identity.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersonId(pub Uuid);

pub type DomainVersion = i64;
pub type SemioVersion = i64;

// #endregion 🔖Newtype Ids

// #region 🔖FieldPatch
// FieldPatch MUST distinguish no-change, set-value, and clear-to-null.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "value")]
pub enum FieldPatch<T> {
    NoChange,
    Set(T),
    Clear,
}

impl<T> Default for FieldPatch<T> {
    fn default() -> Self {
        Self::NoChange
    }
}

impl<T> FieldPatch<T> {
    pub fn is_change(&self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "value")]
pub enum RequiredFieldPatch<T> {
    NoChange,
    Set(T),
}

impl<T> Default for RequiredFieldPatch<T> {
    fn default() -> Self {
        Self::NoChange
    }
}

impl<T> RequiredFieldPatch<T> {
    pub fn is_change(&self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

// #endregion 🔖FieldPatch

// #region 🔖EntityKind
// EntityKind MUST enumerate all mutable entity kinds.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Kit,
    Author,
    Location,
    Folder,
    File,
    Tag,
    Concept,
    Port,
    Quality,
    Benchmark,
    Type,
    Model,
    Connector,
    Prop,
    Attribute,
    Design,
    Layer,
    Piece,
    Group,
    Connection,
    Stat,
}

// #endregion 🔖EntityKind

// #region 🔖Lifecycle
// Lifecycle MUST track active/tombstoned state per entity.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lifecycle {
    Active,
    Tombstoned { at: DomainVersion, by: CommandId },
}

impl Lifecycle {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

// #endregion 🔖Lifecycle

// #region 🔖ConflictPolicy
// ConflictPolicy MUST define per-property merge behaviour.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictPolicy {
    RejectIfChanged,
    LastWriterWins,
    ReferenceMustExistAndBeActive,
    SemioLastWriterWins,
}

// #endregion 🔖ConflictPolicy

// #region 🔖PropertyKey
// PropertyKey MUST enumerate all mutable scalar/ref properties.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertyKey {
    // Kit
    KitName,
    KitVersion,
    KitDescription,
    KitIcon,
    KitImage,
    KitPreview,
    KitRemote,
    KitHomepage,
    KitLicense,
    // Type
    TypeName,
    TypeParent,
    TypeDescription,
    TypeIcon,
    TypeImage,
    TypeFolder,
    TypeUnit,
    TypeStock,
    TypeIsAbstract,
    TypeVirtual,
    TypeLocation,
    // Design
    DesignName,
    DesignParent,
    DesignDescription,
    DesignIcon,
    DesignImage,
    DesignFolder,
    DesignUnit,
    DesignIsAbstract,
    DesignCanScale,
    DesignCanMirror,
    DesignActiveLayer,
    DesignLocation,
    // Piece
    PieceName,
    PieceType,
    PieceDesign,
    PiecePlane,
    PieceCenter,
    PieceScale,
    PieceMirrorPlane,
    PieceIsHidden,
    PieceIsLocked,
    PieceColor,
    PieceDescription,
    // Connection
    ConnectionConnected,
    ConnectionConnecting,
    ConnectionGap,
    ConnectionShift,
    ConnectionRise,
    ConnectionRotation,
    ConnectionTurn,
    ConnectionTilt,
    ConnectionU,
    ConnectionV,
    ConnectionDescription,
    // Others - simple scalar
    AuthorName,
    AuthorEmail,
    FolderName,
    FolderParent,
    FolderDescription,
    FileName,
    FileRemote,
    FileFolder,
    FileBlob,
    TagName,
    TagDescription,
    TagIcon,
    ConceptName,
    ConceptDescription,
    ConceptIcon,
    PortName,
    PortDescription,
    PortIcon,
    QualityKey,
    QualityName,
    QualityDescription,
    LayerPath,
    LayerIsHidden,
    LayerIsLocked,
    LayerColor,
    LayerDescription,
    GroupName,
    GroupColor,
    GroupDescription,
    // Lifecycle
    EntityLifecycle,
}

pub fn conflict_policy(key: PropertyKey) -> ConflictPolicy {
    match key {
        PropertyKey::KitName => ConflictPolicy::RejectIfChanged,
        PropertyKey::PieceType | PropertyKey::PieceDesign => {
            ConflictPolicy::ReferenceMustExistAndBeActive
        }
        PropertyKey::TypeParent
        | PropertyKey::DesignParent
        | PropertyKey::FolderParent
        | PropertyKey::DesignActiveLayer
        | PropertyKey::TypeLocation
        | PropertyKey::DesignLocation
        | PropertyKey::FileFolder => ConflictPolicy::ReferenceMustExistAndBeActive,
        _ => ConflictPolicy::LastWriterWins,
    }
}

// #endregion 🔖PropertyKey

// #region 🔖SessionStatus

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Passivated,
    Closed,
}

// #endregion 🔖SessionStatus

// #region 🔖Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_patch_no_change_is_default() {
        let p: FieldPatch<String> = FieldPatch::default();
        assert!(!p.is_change());
    }

    #[test]
    fn field_patch_set_is_change() {
        let p = FieldPatch::Set("hello".to_string());
        assert!(p.is_change());
    }

    #[test]
    fn field_patch_clear_is_change() {
        let p: FieldPatch<String> = FieldPatch::Clear;
        assert!(p.is_change());
    }

    #[test]
    fn field_patch_serde_roundtrip() {
        let set = FieldPatch::Set(42);
        let json = serde_json::to_string(&set).unwrap();
        let deser: FieldPatch<i32> = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, FieldPatch::Set(42)));

        let clear: FieldPatch<i32> = FieldPatch::Clear;
        let json = serde_json::to_string(&clear).unwrap();
        let deser: FieldPatch<i32> = serde_json::from_str(&json).unwrap();
        assert!(matches!(deser, FieldPatch::Clear));
    }

    #[test]
    fn required_field_patch_no_change_is_default() {
        let p: RequiredFieldPatch<String> = RequiredFieldPatch::default();
        assert!(!p.is_change());
    }

    #[test]
    fn required_field_patch_set_is_change() {
        let p = RequiredFieldPatch::Set("hello".to_string());
        assert!(p.is_change());
    }

    #[test]
    fn entity_kind_serde_uses_snake_case() {
        let json = serde_json::to_string(&EntityKind::Kit).unwrap();
        assert_eq!(json, "\"kit\"");
        let json = serde_json::to_string(&EntityKind::Connection).unwrap();
        assert_eq!(json, "\"connection\"");
    }

    #[test]
    fn lifecycle_is_active() {
        assert!(Lifecycle::Active.is_active());
        let tombstoned = Lifecycle::Tombstoned {
            at: 5,
            by: CommandId(Uuid::nil()),
        };
        assert!(!tombstoned.is_active());
    }

    #[test]
    fn conflict_policy_kit_name_rejects() {
        assert_eq!(
            conflict_policy(PropertyKey::KitName),
            ConflictPolicy::RejectIfChanged
        );
    }

    #[test]
    fn conflict_policy_piece_type_requires_reference() {
        assert_eq!(
            conflict_policy(PropertyKey::PieceType),
            ConflictPolicy::ReferenceMustExistAndBeActive
        );
    }

    #[test]
    fn conflict_policy_kit_description_last_writer_wins() {
        assert_eq!(
            conflict_policy(PropertyKey::KitDescription),
            ConflictPolicy::LastWriterWins
        );
    }

    #[test]
    fn session_status_serde_snake_case() {
        let json = serde_json::to_string(&SessionStatus::Active).unwrap();
        assert_eq!(json, "\"active\"");
        let json = serde_json::to_string(&SessionStatus::Passivated).unwrap();
        assert_eq!(json, "\"passivated\"");
    }

    #[test]
    fn session_id_serde_roundtrip() {
        let id = SessionId(Uuid::nil());
        let json = serde_json::to_string(&id).unwrap();
        let deser: SessionId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deser);
    }
}

// #endregion 🔖Tests
