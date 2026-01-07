// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

use serde::{Deserialize, Serialize};
use super::Guid;

// #region Attribute

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub guid: Guid,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttributeId {
    pub guid: Guid,
}

impl AttributeId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

// #endregion Attribute

// #region Coord

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Coord {
    pub u: f64,
    pub v: f64,
}

impl Coord {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

// #endregion Coord

// #region Vector

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

// #endregion Vector

// #region Plane

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plane {
    pub origin: Vector,
    #[serde(rename = "xAxis")]
    pub x_axis: Vector,
    #[serde(rename = "yAxis")]
    pub y_axis: Vector,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            origin: Vector::zero(),
            x_axis: Vector::unit_x(),
            y_axis: Vector::unit_y(),
        }
    }
}

impl Plane {
    pub fn new(origin: Vector, x_axis: Vector, y_axis: Vector) -> Self {
        Self { origin, x_axis, y_axis }
    }

    pub fn world_xy() -> Self {
        Self::default()
    }
}

// #endregion Plane

// #region Camera

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Camera {
    pub position: Vector,
    pub target: Vector,
    pub up: Vector,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector::new(0.0, 0.0, 10.0),
            target: Vector::zero(),
            up: Vector::unit_y(),
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

// #endregion Camera

// #region Location

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Location

// #region Author

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorId {
    pub guid: Guid,
}

impl AuthorId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// #endregion Author

// #region Folder

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FolderId {
    pub guid: Guid,
}

impl FolderId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<FolderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// #endregion Folder

// #region File

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileId {
    pub guid: Guid,
}

impl FileId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<FolderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// #endregion File

// #region Benchmark

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Benchmark {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "minExcluded", skip_serializing_if = "Option::is_none")]
    pub min_excluded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "maxExcluded", skip_serializing_if = "Option::is_none")]
    pub max_excluded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Benchmark

// #region Quality

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QualityId {
    pub guid: Guid,
}

impl QualityId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum QualityKind {
    #[default]
    Integer = 0,
    Float = 1,
    Boolean = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quality {
    pub guid: Guid,
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub kind: QualityKind,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    #[serde(rename = "defaultSiUnit", skip_serializing_if = "Option::is_none")]
    pub default_si_unit: Option<String>,
    #[serde(rename = "defaultImperialUnit", skip_serializing_if = "Option::is_none")]
    pub default_imperial_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "isMinExcluded", skip_serializing_if = "Option::is_none")]
    pub is_min_excluded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "isMaxExcluded", skip_serializing_if = "Option::is_none")]
    pub is_max_excluded: Option<bool>,
    #[serde(rename = "canScale", skip_serializing_if = "Option::is_none")]
    pub can_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmarks: Option<Vec<Benchmark>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Quality

// #region Interface (Port)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterfaceId {
    pub guid: Guid,
}

impl InterfaceId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interface {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "compatibleInterfaces", skip_serializing_if = "Option::is_none")]
    pub compatible_interfaces: Option<Vec<InterfaceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Interface

// #region Prop

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PropId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prop {
    pub guid: Guid,
    pub quality: QualityId,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Prop

// #region Tag

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagId {
    pub guid: Guid,
}

impl TagId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

// #endregion Tag

// #region Concept

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConceptId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Concept {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

// #endregion Concept

// #region Model

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub guid: Guid,
    pub file: FileId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Model

// #region Connector

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectorId {
    pub guid: Guid,
}

impl ConnectorId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connector {
    pub guid: Guid,
    pub point: Vector,
    pub direction: Vector,
    pub t: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<InterfaceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Connector

// #region Type

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TypeId {
    pub guid: Guid,
}

impl TypeId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

pub fn create_type_id(guid: Guid) -> TypeId {
    TypeId::new(guid)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<TypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i32>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    pub virtual_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<Model>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<Connector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

pub fn are_same_type_id(a: &TypeId, b: &TypeId) -> bool {
    a.guid == b.guid
}

// #endregion Type

// #region Layer

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layer {
    pub guid: Guid,
    pub path: String,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Layer

// #region Piece

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PieceId {
    pub guid: Guid,
}

impl PieceId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

pub fn create_piece_id(guid: Guid) -> PieceId {
    PieceId::new(guid)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DesignId {
    pub guid: Guid,
}

impl DesignId {
    pub fn new(guid: Guid) -> Self {
        Self { guid }
    }
}

pub fn create_design_id(guid: Guid) -> DesignId {
    DesignId::new(guid)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Piece {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_ref: Option<TypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design: Option<DesignId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plane: Option<Plane>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Coord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(rename = "mirrorPlane", skip_serializing_if = "Option::is_none")]
    pub mirror_plane: Option<Plane>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Piece

// #region Group

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GroupId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<Vec<PieceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Group

// #region Side

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Side {
    pub piece: PieceId,
    #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
    pub design_piece: Option<PieceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<ConnectorId>,
}

// #endregion Side

// #region Connection

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectionId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub guid: Guid,
    pub connected: Side,
    pub connecting: Side,
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion Connection

// #region Stat

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StatId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stat {
    pub guid: Guid,
    pub quality: QualityId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "minExcluded", skip_serializing_if = "Option::is_none")]
    pub min_excluded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "maxExcluded", skip_serializing_if = "Option::is_none")]
    pub max_excluded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

// #endregion Stat

// #region Design

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Design {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<DesignId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "canScale", skip_serializing_if = "Option::is_none")]
    pub can_scale: Option<bool>,
    #[serde(rename = "canMirror", skip_serializing_if = "Option::is_none")]
    pub can_mirror: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<Vec<Piece>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Vec<Stat>>,
    #[serde(rename = "activeLayer", skip_serializing_if = "Option::is_none")]
    pub active_layer: Option<LayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

pub fn are_same_design_id(a: &DesignId, b: &DesignId) -> bool {
    a.guid == b.guid
}

// #endregion Design

// #region Kit

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Kit {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<Concept>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designs: Option<Vec<Design>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Interface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<Vec<Quality>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<Folder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// #endregion Kit

// #region Finder Functions

pub fn find_type_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Type> {
    kit.types.as_ref()?.iter().find(|t| t.guid == guid)
}

pub fn find_design_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Design> {
    kit.designs.as_ref()?.iter().find(|d| d.guid == guid)
}

pub fn find_piece_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Piece> {
    design.pieces.as_ref()?.iter().find(|p| p.guid == guid)
}

pub fn find_connection_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Connection> {
    design.connections.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_connector_in_type<'a>(type_: &'a Type, guid: &str) -> Option<&'a Connector> {
    type_.connectors.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_interface_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Interface> {
    kit.ports.as_ref()?.iter().find(|p| p.guid == guid)
}

pub fn find_quality_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Quality> {
    kit.qualities.as_ref()?.iter().find(|q| q.guid == guid)
}

pub fn find_file_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a File> {
    kit.files.as_ref()?.iter().find(|f| f.guid == guid)
}

pub fn find_folder_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Folder> {
    kit.folders.as_ref()?.iter().find(|f| f.guid == guid)
}

pub fn find_tag_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Tag> {
    kit.tags.as_ref()?.iter().find(|t| t.guid == guid)
}

pub fn find_concept_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Concept> {
    kit.concepts.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_author_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Author> {
    kit.authors.as_ref()?.iter().find(|a| a.guid == guid)
}

pub fn find_piece_connections_in_design<'a>(design: &'a Design, piece_guid: &str) -> Vec<&'a Connection> {
    design
        .connections
        .as_ref()
        .map(|conns| {
            conns
                .iter()
                .filter(|c| c.connected.piece.guid == piece_guid || c.connecting.piece.guid == piece_guid)
                .collect()
        })
        .unwrap_or_default()
}

pub fn is_fixed_piece(piece: &Piece) -> bool {
    piece.plane.is_some()
}

// #endregion Finder Functions
