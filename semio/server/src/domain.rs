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
    Tombstoned {
        at: DomainVersion,
        by: CommandId,
    },
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
