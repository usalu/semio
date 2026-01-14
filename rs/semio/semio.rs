#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use nalgebra::{Matrix4, Point3, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use thiserror::Error;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// #region Error Types

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SemioError {
    #[error("Entity not found: {kind} with guid {guid}")]
    NotFound { kind: String, guid: String },
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    #[error("Database error: {message}")]
    Database { message: String },
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
}

pub type Result<T> = std::result::Result<T, SemioError>;

// #endregion Error Types

// #region Utility Functions

pub type Guid = String;

pub fn guid() -> String { Uuid::now_v7().to_string() }

pub fn normalize(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

pub fn round(value: f64) -> f64 { normalize(value, 3) }

pub fn jaccard<T: Eq + std::hash::Hash>(a: &HashSet<T>, b: &HashSet<T>) -> f64 {
    if a.is_empty() && b.is_empty() { return 1.0; }
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

pub fn deep_equal<T: Serialize>(a: &T, b: &T) -> bool {
    match (serde_json::to_value(a), serde_json::to_value(b)) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

pub fn generate_unique_name(base: &str, existing: &[String]) -> String {
    let existing_set: HashSet<_> = existing.iter().collect();
    if !existing_set.contains(&base.to_string()) { return base.to_string(); }
    let mut counter = 1;
    loop {
        let candidate = format!("{} ({})", base, counter);
        if !existing_set.contains(&candidate) { return candidate; }
        counter += 1;
    }
}

// #endregion Utility Functions

// #region Model Types - Attribute

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
pub struct AttributeId { pub guid: Guid }

// #endregion Model Types - Attribute

// #region Model Types - Coord

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Coord { pub u: f64, pub v: f64 }

impl Coord { pub fn new(u: f64, v: f64) -> Self { Self { u, v } } }

// #endregion Model Types - Coord

// #region Model Types - Vector

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Vector { pub x: f64, pub y: f64, pub z: f64 }

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn unit_x() -> Self { Self::new(1.0, 0.0, 0.0) }
    pub fn unit_y() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub fn unit_z() -> Self { Self::new(0.0, 0.0, 1.0) }
    pub fn to_nalgebra(&self) -> Vector3<f64> { Vector3::new(self.x, self.y, self.z) }
    pub fn from_nalgebra(v: &Vector3<f64>) -> Self { Self::new(v.x, v.y, v.z) }
}

// #endregion Model Types - Vector

// #region Model Types - Plane

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
        Self { origin: Vector::zero(), x_axis: Vector::unit_x(), y_axis: Vector::unit_y() }
    }
}

impl Plane {
    pub fn new(origin: Vector, x_axis: Vector, y_axis: Vector) -> Self { Self { origin, x_axis, y_axis } }
    pub fn world_xy() -> Self { Self::default() }

    pub fn to_matrix(&self) -> Matrix4<f64> {
        let x = self.x_axis.to_nalgebra().normalize();
        let y = self.y_axis.to_nalgebra().normalize();
        let z = x.cross(&y).normalize();
        let o = Point3::new(self.origin.x, self.origin.y, self.origin.z);
        Matrix4::new(x.x, y.x, z.x, o.x, x.y, y.y, z.y, o.y, x.z, y.z, z.z, o.z, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn from_matrix(m: &Matrix4<f64>) -> Self {
        let origin = Vector::new(m[(0, 3)], m[(1, 3)], m[(2, 3)]);
        let x_axis = Vector::new(m[(0, 0)], m[(1, 0)], m[(2, 0)]);
        let y_axis = Vector::new(m[(0, 1)], m[(1, 1)], m[(2, 1)]);
        Self { origin, x_axis, y_axis }
    }

    pub fn round(&self) -> Self {
        Self {
            origin: Vector::new(round(self.origin.x), round(self.origin.y), round(self.origin.z)),
            x_axis: Vector::new(round(self.x_axis.x), round(self.x_axis.y), round(self.x_axis.z)),
            y_axis: Vector::new(round(self.y_axis.x), round(self.y_axis.y), round(self.y_axis.z)),
        }
    }
}

// #endregion Model Types - Plane

// #region Model Types - Camera

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
        Self { position: Vector::new(0.0, 0.0, 10.0), target: Vector::zero(), up: Vector::unit_y(), fov: 45.0, near: 0.1, far: 1000.0 }
    }
}

// #endregion Model Types - Camera

// #region Model Types - Location, Author, File, Folder

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FolderId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileId { pub guid: Guid }

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

// #endregion Model Types - Location, Author, File, Folder

// #region Model Types - Quality, Interface, Tag, Concept

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QualityId { pub guid: Guid }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum QualityKind { #[default] Integer = 0, Float = 1, Boolean = 2 }

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
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterfaceId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConceptId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Concept {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

// #endregion Model Types - Quality, Interface, Tag, Concept

// #region Model Types - Prop, Model, Connector

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PropId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectorId { pub guid: Guid }

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

// #endregion Model Types - Prop, Model, Connector

// #region Model Types - Type

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TypeId { pub guid: Guid }

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

// #endregion Model Types - Type

// #region Model Types - Layer, Piece, Group, Side, Connection, Stat

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PieceId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DesignId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GroupId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Side {
    pub piece: PieceId,
    #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
    pub design_piece: Option<PieceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<ConnectorId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectionId { pub guid: Guid }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StatId { pub guid: Guid }

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

// #endregion Model Types - Layer, Piece, Group, Side, Connection, Stat

// #region Model Types - Design

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

// #endregion Model Types - Design

// #region Model Types - Kit

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

// #endregion Model Types - Kit

// #region Finder Functions

pub fn find_type_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Type> {
    kit.types.as_ref()?.iter().find(|t| t.guid == guid)
}

pub fn find_type_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Type> {
    kit.types.as_mut()?.iter_mut().find(|t| t.guid == guid)
}

pub fn find_design_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Design> {
    kit.designs.as_ref()?.iter().find(|d| d.guid == guid)
}

pub fn find_design_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Design> {
    kit.designs.as_mut()?.iter_mut().find(|d| d.guid == guid)
}

pub fn find_piece_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Piece> {
    design.pieces.as_ref()?.iter().find(|p| p.guid == guid)
}

pub fn find_piece_in_design_mut<'a>(design: &'a mut Design, guid: &str) -> Option<&'a mut Piece> {
    design.pieces.as_mut()?.iter_mut().find(|p| p.guid == guid)
}

pub fn find_connection_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Connection> {
    design.connections.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_connector_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Connector> {
    t.connectors.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_model_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Model> {
    t.models.as_ref()?.iter().find(|m| m.guid == guid)
}

pub fn find_file_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a File> {
    kit.files.as_ref()?.iter().find(|f| f.guid == guid)
}

pub fn find_folder_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Folder> {
    kit.folders.as_ref()?.iter().find(|f| f.guid == guid)
}

pub fn find_author_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Author> {
    kit.authors.as_ref()?.iter().find(|a| a.guid == guid)
}

pub fn find_tag_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Tag> {
    kit.tags.as_ref()?.iter().find(|t| t.guid == guid)
}

pub fn find_concept_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Concept> {
    kit.concepts.as_ref()?.iter().find(|c| c.guid == guid)
}

pub fn find_quality_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Quality> {
    kit.qualities.as_ref()?.iter().find(|q| q.guid == guid)
}

pub fn find_interface_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Interface> {
    kit.ports.as_ref()?.iter().find(|i| i.guid == guid)
}

pub fn find_layer_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Layer> {
    design.layers.as_ref()?.iter().find(|l| l.guid == guid)
}

pub fn find_group_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Group> {
    design.groups.as_ref()?.iter().find(|g| g.guid == guid)
}

pub fn find_stat_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Stat> {
    design.stats.as_ref()?.iter().find(|s| s.guid == guid)
}

// #endregion Finder Functions

// #region Serialization

pub fn serialize_kit(kit: &Kit) -> Result<String> {
    serde_json::to_string_pretty(kit).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn deserialize_kit(json: &str) -> Result<Kit> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn serialize_design(design: &Design) -> Result<String> {
    serde_json::to_string_pretty(design).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn deserialize_design(json: &str) -> Result<Design> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn serialize_type(t: &Type) -> Result<String> {
    serde_json::to_string_pretty(t).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn deserialize_type(json: &str) -> Result<Type> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

pub fn are_kits_equal(a: &Kit, b: &Kit) -> bool { deep_equal(a, b) }
pub fn are_designs_equal(a: &Design, b: &Design) -> bool { deep_equal(a, b) }
pub fn are_types_equal(a: &Type, b: &Type) -> bool { deep_equal(a, b) }

pub const SUPPORTED_MODEL_EXTENSIONS: &[&str] = &["gltf", "glb", "fbx", "obj", "dae", "3ds", "stl", "ply", "usdz", "vrm", "ifc", "3mf"];

pub fn is_supported_model_extension(ext: &str) -> bool {
    SUPPORTED_MODEL_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// #endregion Serialization

// #region Diff Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CollectionDiff<T, D> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<Guid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<D>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttributeDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PropDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectorDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<InterfaceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CollectionDiff<Prop, PropDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<TagId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TypeDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<TypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<Option<i32>>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<Option<bool>>,
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    pub virtual_type: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<LocationId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Option<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Option<Vec<AuthorId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CollectionDiff<Prop, PropDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<CollectionDiff<Model, ModelDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<CollectionDiff<Connector, ConnectorDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SideDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piece: Option<PieceId>,
    #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
    pub design_piece: Option<Option<PieceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<Option<ConnectorId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectionDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<SideDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connecting: Option<SideDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PieceDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_ref: Option<Option<TypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design: Option<Option<DesignId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plane: Option<Option<Plane>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Option<Coord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Option<f64>>,
    #[serde(rename = "mirrorPlane", skip_serializing_if = "Option::is_none")]
    pub mirror_plane: Option<Option<Plane>>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<Option<bool>>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CollectionDiff<Prop, PropDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<Option<bool>>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GroupDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<Option<Vec<PieceId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StatDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Option<f64>>,
    #[serde(rename = "minExcluded", skip_serializing_if = "Option::is_none")]
    pub min_excluded: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Option<f64>>,
    #[serde(rename = "maxExcluded", skip_serializing_if = "Option::is_none")]
    pub max_excluded: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DesignDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<DesignId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Option<String>>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<Option<bool>>,
    #[serde(rename = "canScale", skip_serializing_if = "Option::is_none")]
    pub can_scale: Option<Option<bool>>,
    #[serde(rename = "canMirror", skip_serializing_if = "Option::is_none")]
    pub can_mirror: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Option<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Option<Vec<AuthorId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CollectionDiff<Prop, PropDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<CollectionDiff<Piece, PieceDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<CollectionDiff<Connection, ConnectionDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<CollectionDiff<Layer, LayerDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<CollectionDiff<Group, GroupDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<CollectionDiff<Stat, StatDiff>>,
    #[serde(rename = "activeLayer", skip_serializing_if = "Option::is_none")]
    pub active_layer: Option<Option<LayerId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConceptDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterfaceDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "compatibleInterfaces", skip_serializing_if = "Option::is_none")]
    pub compatible_interfaces: Option<Option<Vec<InterfaceId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QualityDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<QualityKind>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<Option<String>>,
    #[serde(rename = "defaultSiUnit", skip_serializing_if = "Option::is_none")]
    pub default_si_unit: Option<Option<String>>,
    #[serde(rename = "defaultImperialUnit", skip_serializing_if = "Option::is_none")]
    pub default_imperial_unit: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Option<f64>>,
    #[serde(rename = "isMinExcluded", skip_serializing_if = "Option::is_none")]
    pub is_min_excluded: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Option<f64>>,
    #[serde(rename = "isMaxExcluded", skip_serializing_if = "Option::is_none")]
    pub is_max_excluded: Option<Option<bool>>,
    #[serde(rename = "canScale", skip_serializing_if = "Option::is_none")]
    pub can_scale: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<FolderId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FolderDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<FolderId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct KitDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<CollectionDiff<Concept, ConceptDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<CollectionDiff<Tag, TagDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<CollectionDiff<Type, TypeDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designs: Option<CollectionDiff<Design, DesignDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<CollectionDiff<Interface, InterfaceDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<CollectionDiff<Quality, QualityDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<CollectionDiff<File, FileDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<CollectionDiff<Folder, FolderDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<CollectionDiff<Author, AuthorDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

// #endregion Diff Types

// #region HasGuid Trait

pub trait HasGuid {
    fn guid(&self) -> &str;
}

impl HasGuid for Attribute { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Prop { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Connector { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Model { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Type { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Piece { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Connection { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Layer { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Group { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Stat { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Design { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Tag { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Concept { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Interface { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Quality { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for File { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Folder { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Author { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Kit { fn guid(&self) -> &str { &self.guid } }

pub trait DiffHasGuid {
    fn guid(&self) -> &str;
}

impl DiffHasGuid for AttributeDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for PropDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for ConnectorDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for ModelDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for TypeDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for PieceDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for ConnectionDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for LayerDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for GroupDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for StatDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for DesignDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for TagDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for ConceptDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for InterfaceDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for QualityDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for FileDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for FolderDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for AuthorDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for KitDiff { fn guid(&self) -> &str { &self.guid } }

// #endregion HasGuid Trait

// #region FlattenDesign

pub struct FlattenedPiece {
    pub piece: Piece,
    pub plane: Plane,
    pub type_guid: Option<String>,
    pub design_guid: Option<String>,
}

pub fn flatten_design(kit: &Kit, design_guid: &str) -> Result<Vec<FlattenedPiece>> {
    let design = find_design_in_kit(kit, design_guid)
        .ok_or_else(|| SemioError::NotFound { kind: "Design".to_string(), guid: design_guid.to_string() })?;
    
    let mut result = Vec::new();
    let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
    let connections = design.connections.as_ref().map(|c| c.as_slice()).unwrap_or(&[]);
    
    let mut piece_planes: HashMap<String, Matrix4<f64>> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    
    for piece in pieces {
        if let Some(ref plane) = piece.plane {
            piece_planes.insert(piece.guid.clone(), plane.to_matrix());
            queue.push_back(piece.guid.clone());
            visited.insert(piece.guid.clone());
        }
    }
    
    while let Some(current_guid) = queue.pop_front() {
        let current_matrix = piece_planes.get(&current_guid).cloned().unwrap_or_else(Matrix4::identity);
        
        for conn in connections {
            let (other_guid, is_connected) = if conn.connected.piece.guid == current_guid && !visited.contains(&conn.connecting.piece.guid) {
                (conn.connecting.piece.guid.clone(), true)
            } else if conn.connecting.piece.guid == current_guid && !visited.contains(&conn.connected.piece.guid) {
                (conn.connected.piece.guid.clone(), false)
            } else {
                continue;
            };
            
            let connection_matrix = compute_connection_matrix(kit, design, conn, is_connected)?;
            let new_matrix = current_matrix * connection_matrix;
            
            piece_planes.insert(other_guid.clone(), new_matrix);
            visited.insert(other_guid.clone());
            queue.push_back(other_guid);
        }
    }
    
    for piece in pieces {
        let matrix = piece_planes.get(&piece.guid).cloned().unwrap_or_else(Matrix4::identity);
        let plane = Plane::from_matrix(&matrix).round();
        
        result.push(FlattenedPiece {
            piece: piece.clone(),
            plane,
            type_guid: piece.type_ref.as_ref().map(|t| t.guid.clone()),
            design_guid: piece.design.as_ref().map(|d| d.guid.clone()),
        });
    }
    
    Ok(result)
}

fn compute_connection_matrix(kit: &Kit, design: &Design, conn: &Connection, from_connected: bool) -> Result<Matrix4<f64>> {
    let (from_side, to_side) = if from_connected {
        (&conn.connected, &conn.connecting)
    } else {
        (&conn.connecting, &conn.connected)
    };
    
    let from_connector = get_connector_for_side(kit, design, from_side)?;
    let to_connector = get_connector_for_side(kit, design, to_side)?;
    
    let from_plane = connector_to_plane(&from_connector);
    let to_plane = connector_to_plane(&to_connector);
    
    let translation = Matrix4::new_translation(&nalgebra::Vector3::new(conn.shift, conn.gap, conn.rise));
    let rot_y = Matrix4::from_euler_angles(0.0, conn.rotation * PI / 180.0, 0.0);
    let rot_z = Matrix4::from_euler_angles(0.0, 0.0, conn.turn * PI / 180.0);
    let rot_x = Matrix4::from_euler_angles(conn.tilt * PI / 180.0, 0.0, 0.0);
    
    let from_matrix = from_plane.to_matrix();
    let to_matrix_inv = to_plane.to_matrix().try_inverse().unwrap_or_else(Matrix4::identity);
    
    Ok(from_matrix * translation * rot_y * rot_z * rot_x * to_matrix_inv)
}

fn get_connector_for_side(kit: &Kit, design: &Design, side: &Side) -> Result<Connector> {
    let piece = find_piece_in_design(design, &side.piece.guid)
        .ok_or_else(|| SemioError::NotFound { kind: "Piece".to_string(), guid: side.piece.guid.clone() })?;
    
    if let Some(ref connector_id) = side.connector {
        if let Some(ref type_id) = piece.type_ref {
            if let Some(t) = find_type_in_kit(kit, &type_id.guid) {
                if let Some(connector) = find_connector_in_type(t, &connector_id.guid) {
                    return Ok(connector.clone());
                }
            }
        }
    }
    
    Ok(Connector {
        guid: guid(),
        point: Vector::zero(),
        direction: Vector::unit_y(),
        t: 0.0,
        name: None,
        description: None,
        mandatory: None,
        port: None,
        props: None,
        attributes: None,
    })
}

fn connector_to_plane(connector: &Connector) -> Plane {
    let origin = connector.point.clone();
    let y_axis = connector.direction.clone();
    let temp_x = if (y_axis.z.abs() - 1.0).abs() < 0.001 {
        Vector::unit_x()
    } else {
        Vector::unit_z()
    };
    let y_vec = y_axis.to_nalgebra();
    let x_vec = y_vec.cross(&temp_x.to_nalgebra()).normalize();
    let x_axis = Vector::from_nalgebra(&x_vec);
    Plane::new(origin, x_axis, y_axis)
}

// #endregion FlattenDesign

// #region Validation Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationProblem {
    pub id: String,
    pub severity: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<ValidationFix>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationFix {
    pub description: String,
    pub diff: Option<KitDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ValidationResult {
    pub valid: bool,
    pub problems: Vec<ValidationProblem>,
}

pub fn validate_kit(kit: &Kit) -> ValidationResult {
    let mut problems = Vec::new();
    
    let mut guids: HashSet<String> = HashSet::new();
    check_guid_uniqueness(&kit.guid, "Kit", &mut guids, &mut problems);
    
    if let Some(ref types) = kit.types {
        for t in types {
            check_guid_uniqueness(&t.guid, "Type", &mut guids, &mut problems);
            if let Some(ref connectors) = t.connectors {
                for c in connectors { check_guid_uniqueness(&c.guid, "Connector", &mut guids, &mut problems); }
            }
            if let Some(ref models) = t.models {
                for m in models { check_guid_uniqueness(&m.guid, "Model", &mut guids, &mut problems); }
            }
        }
    }
    
    if let Some(ref designs) = kit.designs {
        for d in designs {
            check_guid_uniqueness(&d.guid, "Design", &mut guids, &mut problems);
            if let Some(ref pieces) = d.pieces {
                for p in pieces { check_guid_uniqueness(&p.guid, "Piece", &mut guids, &mut problems); }
            }
            if let Some(ref connections) = d.connections {
                for c in connections { check_guid_uniqueness(&c.guid, "Connection", &mut guids, &mut problems); }
            }
        }
    }
    
    if let Some(ref tags) = kit.tags {
        for t in tags { check_guid_uniqueness(&t.guid, "Tag", &mut guids, &mut problems); }
    }
    if let Some(ref concepts) = kit.concepts {
        for c in concepts { check_guid_uniqueness(&c.guid, "Concept", &mut guids, &mut problems); }
    }
    if let Some(ref files) = kit.files {
        for f in files { check_guid_uniqueness(&f.guid, "File", &mut guids, &mut problems); }
    }
    if let Some(ref folders) = kit.folders {
        for f in folders { check_guid_uniqueness(&f.guid, "Folder", &mut guids, &mut problems); }
    }
    if let Some(ref authors) = kit.authors {
        for a in authors { check_guid_uniqueness(&a.guid, "Author", &mut guids, &mut problems); }
    }
    
    ValidationResult { valid: problems.is_empty(), problems }
}

fn check_guid_uniqueness(guid: &str, kind: &str, guids: &mut HashSet<String>, problems: &mut Vec<ValidationProblem>) {
    if guids.contains(guid) {
        problems.push(ValidationProblem {
            id: format!("duplicate-guid-{}", guid),
            severity: "error".to_string(),
            message: format!("Duplicate GUID {} found in {}", guid, kind),
            entity_kind: Some(kind.to_string()),
            entity_guid: Some(guid.to_string()),
            fix: None,
        });
    } else {
        guids.insert(guid.to_string());
    }
}

// #endregion Validation Types

// #region SQLite Import/Export

#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite {
    use super::*;
    use rusqlite::params;
    
    pub fn export_kit_to_sqlite(kit: &Kit, path: &str) -> Result<()> {
        let conn = rusqlite::Connection::open(path)
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        conn.execute_batch(include_str!("../../sql/sqlite/semio/schema.sql"))
            .map_err(|e| SemioError::Database { message: format!("Schema creation failed: {}", e) })?;
        
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![kit.guid, kit.name, kit.version, kit.description, kit.icon, kit.image, kit.preview, kit.remote, kit.homepage, kit.license, now, now],
        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        if let Some(ref tags) = kit.tags {
            for tag in tags {
                conn.execute(
                    "INSERT INTO tag (guid, name, description, icon, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![tag.guid, tag.name, tag.description, tag.icon, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref concepts) = kit.concepts {
            for concept in concepts {
                conn.execute(
                    "INSERT INTO concept (guid, name, description, icon, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![concept.guid, concept.name, concept.description, concept.icon, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref ports) = kit.ports {
            for port in ports {
                conn.execute(
                    "INSERT INTO port (guid, name, description, icon, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![port.guid, port.name, port.description, port.icon, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref folders) = kit.folders {
            for folder in folders {
                conn.execute(
                    "INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![folder.guid, folder.name, folder.parent.as_ref().map(|p| &p.guid), now, now, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref files) = kit.files {
            for file in files {
                conn.execute(
                    "INSERT INTO file (guid, name, mime, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    params![file.guid, file.name, file.mime, file.folder.as_ref().map(|f| &f.guid), file.size, file.hash, file.remote, now, now, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref authors) = kit.authors {
            for author in authors {
                conn.execute(
                    "INSERT INTO author (guid, name, email, kit_guid) VALUES (?1, ?2, ?3, ?4)",
                    params![author.guid, author.name, author.email, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
            }
        }
        
        if let Some(ref types) = kit.types {
            for t in types {
                conn.execute(
                    "INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                    params![t.guid, t.name, t.parent.as_ref().map(|p| &p.guid), t.is_abstract.unwrap_or(false), t.folder, t.stock, t.virtual_type.unwrap_or(false), t.unit, t.description, t.icon, t.image, now, now, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                
                if let Some(ref connectors) = t.connectors {
                    for c in connectors {
                        conn.execute(
                            "INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                            params![c.guid, c.name, c.point.x, c.point.y, c.point.z, c.direction.x, c.direction.y, c.direction.z, c.t, c.mandatory.unwrap_or(false), c.port.as_ref().map(|p| &p.guid), c.description, t.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
                
                if let Some(ref models) = t.models {
                    for m in models {
                        conn.execute(
                            "INSERT INTO model (guid, file_guid, name, description, type_guid) VALUES (?1, ?2, ?3, ?4, ?5)",
                            params![m.guid, m.file.guid, m.name, m.description, t.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
            }
        }
        
        if let Some(ref designs) = kit.designs {
            for d in designs {
                conn.execute(
                    "INSERT INTO design (guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                    params![d.guid, d.name, d.parent.as_ref().map(|p| &p.guid), d.unit, d.is_abstract, d.folder, d.can_scale, d.can_mirror, d.description, d.icon, d.image, now, now, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                
                if let Some(ref pieces) = d.pieces {
                    for p in pieces {
                        let (po, px, py) = p.plane.as_ref().map(|pl| (
                            (pl.origin.x, pl.origin.y, pl.origin.z),
                            (pl.x_axis.x, pl.x_axis.y, pl.x_axis.z),
                            (pl.y_axis.x, pl.y_axis.y, pl.y_axis.z),
                        )).unwrap_or(((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)));
                        
                        conn.execute(
                            "INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, is_hidden, is_locked, color, description, design_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
                            params![p.guid, p.name, p.type_ref.as_ref().map(|t| &t.guid), p.design.as_ref().map(|d| &d.guid), po.0, po.1, po.2, px.0, px.1, px.2, py.0, py.1, py.2, p.center.as_ref().map(|c| c.u), p.center.as_ref().map(|c| c.v), p.scale, p.is_hidden.unwrap_or(false), p.is_locked.unwrap_or(false), p.color, p.description, d.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
                
                if let Some(ref connections) = d.connections {
                    for c in connections {
                        conn.execute(
                            "INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
                            params![c.guid, c.connected.piece.guid, c.connected.design_piece.as_ref().map(|p| &p.guid), c.connected.connector.as_ref().map(|cn| &cn.guid).unwrap_or(&"".to_string()), c.connecting.piece.guid, c.connecting.design_piece.as_ref().map(|p| &p.guid), c.connecting.connector.as_ref().map(|cn| &cn.guid).unwrap_or(&"".to_string()), c.gap, c.shift, c.rise, c.rotation, c.turn, c.tilt, c.u, c.v, c.description, d.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
                
                if let Some(ref layers) = d.layers {
                    for l in layers {
                        conn.execute(
                            "INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                            params![l.guid, l.path, l.is_hidden.unwrap_or(false), l.is_locked.unwrap_or(false), l.color, l.description, d.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
                
                if let Some(ref groups) = d.groups {
                    for g in groups {
                        conn.execute(
                            "INSERT INTO \"group\" (guid, name, color, description, design_guid) VALUES (?1, ?2, ?3, ?4, ?5)",
                            params![g.guid, g.name, g.color, g.description, d.guid],
                        ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn import_kit_from_sqlite(path: &str) -> Result<Kit> {
        let conn = rusqlite::Connection::open(path)
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        let mut stmt = conn.prepare("SELECT guid, name, version, description, icon, image, preview, remote, homepage, license FROM kit LIMIT 1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        let kit_row = stmt.query_row([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
            ))
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        let kit_guid = kit_row.0.clone();
        
        let tags = load_tags(&conn, &kit_guid)?;
        let concepts = load_concepts(&conn, &kit_guid)?;
        let ports = load_ports(&conn, &kit_guid)?;
        let folders = load_folders(&conn, &kit_guid)?;
        let files = load_files(&conn, &kit_guid)?;
        let authors = load_authors(&conn, &kit_guid)?;
        let types = load_types(&conn, &kit_guid)?;
        let designs = load_designs(&conn, &kit_guid)?;
        
        Ok(Kit {
            guid: kit_row.0,
            name: kit_row.1,
            version: kit_row.2,
            description: kit_row.3,
            icon: kit_row.4,
            image: kit_row.5,
            preview: kit_row.6,
            remote: kit_row.7,
            homepage: kit_row.8,
            license: kit_row.9,
            concepts: if concepts.is_empty() { None } else { Some(concepts) },
            tags: if tags.is_empty() { None } else { Some(tags) },
            types: if types.is_empty() { None } else { Some(types) },
            designs: if designs.is_empty() { None } else { Some(designs) },
            ports: if ports.is_empty() { None } else { Some(ports) },
            qualities: None,
            files: if files.is_empty() { None } else { Some(files) },
            folders: if folders.is_empty() { None } else { Some(folders) },
            authors: if authors.is_empty() { None } else { Some(authors) },
            attributes: None,
            created_at: None,
            updated_at: None,
        })
    }
    
    fn load_tags(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Tag>> {
        let mut stmt = conn.prepare("SELECT guid, name, description, icon FROM tag WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Tag { guid: row.get(0)?, name: row.get(1)?, description: row.get(2)?, icon: row.get(3)? })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_concepts(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Concept>> {
        let mut stmt = conn.prepare("SELECT guid, name, description, icon FROM concept WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Concept { guid: row.get(0)?, name: row.get(1)?, description: row.get(2)?, icon: row.get(3)? })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_ports(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Interface>> {
        let mut stmt = conn.prepare("SELECT guid, name, description, icon FROM port WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Interface { guid: row.get(0)?, name: row.get(1)?, description: row.get(2)?, icon: row.get(3)?, compatible_interfaces: None, attributes: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_folders(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Folder>> {
        let mut stmt = conn.prepare("SELECT guid, name, parent_guid FROM folder WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Folder { guid: row.get(0)?, name: row.get(1)?, parent: row.get::<_, Option<String>>(2)?.map(|g| FolderId { guid: g }), attributes: None, created_at: None, updated_at: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_files(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<File>> {
        let mut stmt = conn.prepare("SELECT guid, name, mime, folder_guid, size, hash, remote_url FROM file WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(File { guid: row.get(0)?, name: row.get(1)?, mime: row.get(2)?, folder: row.get::<_, Option<String>>(3)?.map(|g| FolderId { guid: g }), size: row.get(4)?, hash: row.get(5)?, remote: row.get(6)?, created_at: None, updated_at: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_authors(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Author>> {
        let mut stmt = conn.prepare("SELECT guid, name, email FROM author WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Author { guid: row.get(0)?, name: row.get(1)?, email: row.get(2)?, attributes: None, created_at: None, updated_at: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_types(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Type>> {
        let mut stmt = conn.prepare("SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image FROM type WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            let type_guid: String = row.get(0)?;
            Ok((type_guid, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, bool>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<i32>>(5)?, row.get::<_, bool>(6)?, row.get::<_, Option<String>>(7)?, row.get::<_, Option<String>>(8)?, row.get::<_, Option<String>>(9)?, row.get::<_, Option<String>>(10)?))
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        let type_data: Vec<_> = rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })?;
        let mut types = Vec::new();
        
        for (type_guid, name, parent, is_abstract, folder, stock, virtual_type, unit, description, icon, image) in type_data {
            let connectors = load_connectors(conn, &type_guid)?;
            let models = load_models(conn, &type_guid)?;
            
            types.push(Type {
                guid: type_guid,
                name,
                parent: parent.map(|g| TypeId { guid: g }),
                description,
                icon,
                image,
                folder,
                unit,
                stock,
                is_abstract: Some(is_abstract),
                virtual_type: Some(virtual_type),
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: if models.is_empty() { None } else { Some(models) },
                connectors: if connectors.is_empty() { None } else { Some(connectors) },
                attributes: None,
                created_at: None,
                updated_at: None,
            });
        }
        
        Ok(types)
    }
    
    fn load_connectors(conn: &rusqlite::Connection, type_guid: &str) -> Result<Vec<Connector>> {
        let mut stmt = conn.prepare("SELECT guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description FROM connector WHERE type_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([type_guid], |row| {
            Ok(Connector {
                guid: row.get(0)?,
                name: row.get(1)?,
                point: Vector::new(row.get(2)?, row.get(3)?, row.get(4)?),
                direction: Vector::new(row.get(5)?, row.get(6)?, row.get(7)?),
                t: row.get(8)?,
                mandatory: Some(row.get(9)?),
                port: row.get::<_, Option<String>>(10)?.map(|g| InterfaceId { guid: g }),
                description: row.get(11)?,
                props: None,
                attributes: None,
            })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_models(conn: &rusqlite::Connection, type_guid: &str) -> Result<Vec<Model>> {
        let mut stmt = conn.prepare("SELECT guid, file_guid, name, description FROM model WHERE type_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([type_guid], |row| {
            Ok(Model { guid: row.get(0)?, file: FileId { guid: row.get(1)? }, name: row.get(2)?, description: row.get(3)?, tags: None, attributes: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_designs(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Design>> {
        let mut stmt = conn.prepare("SELECT guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image FROM design WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, Option<String>>(3)?, row.get::<_, Option<bool>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, Option<bool>>(6)?, row.get::<_, Option<bool>>(7)?, row.get::<_, Option<String>>(8)?, row.get::<_, Option<String>>(9)?, row.get::<_, Option<String>>(10)?))
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        
        let design_data: Vec<_> = rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })?;
        let mut designs = Vec::new();
        
        for (design_guid, name, parent, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image) in design_data {
            let pieces = load_pieces(conn, &design_guid)?;
            let connections = load_connections(conn, &design_guid)?;
            let layers = load_layers(conn, &design_guid)?;
            let groups = load_groups(conn, &design_guid)?;
            
            designs.push(Design {
                guid: design_guid,
                name,
                parent: parent.map(|g| DesignId { guid: g }),
                description,
                icon,
                image,
                folder,
                unit,
                is_abstract,
                can_scale,
                can_mirror,
                concepts: None,
                authors: None,
                props: None,
                pieces: if pieces.is_empty() { None } else { Some(pieces) },
                connections: if connections.is_empty() { None } else { Some(connections) },
                layers: if layers.is_empty() { None } else { Some(layers) },
                groups: if groups.is_empty() { None } else { Some(groups) },
                stats: None,
                active_layer: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            });
        }
        
        Ok(designs)
    }
    
    fn load_pieces(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Piece>> {
        let mut stmt = conn.prepare("SELECT guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, is_hidden, is_locked, color, description FROM piece WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([design_guid], |row| {
            let plane = if let (Some(ox), Some(oy), Some(oz)) = (row.get::<_, Option<f64>>(4)?, row.get::<_, Option<f64>>(5)?, row.get::<_, Option<f64>>(6)?) {
                Some(Plane::new(
                    Vector::new(ox, oy, oz),
                    Vector::new(row.get::<_, f64>(7)?, row.get::<_, f64>(8)?, row.get::<_, f64>(9)?),
                    Vector::new(row.get::<_, f64>(10)?, row.get::<_, f64>(11)?, row.get::<_, f64>(12)?),
                ))
            } else { None };
            
            let center = if let (Some(u), Some(v)) = (row.get::<_, Option<f64>>(13)?, row.get::<_, Option<f64>>(14)?) {
                Some(Coord::new(u, v))
            } else { None };
            
            Ok(Piece {
                guid: row.get(0)?,
                name: row.get(1)?,
                type_ref: row.get::<_, Option<String>>(2)?.map(|g| TypeId { guid: g }),
                design: row.get::<_, Option<String>>(3)?.map(|g| DesignId { guid: g }),
                plane,
                center,
                scale: row.get(15)?,
                mirror_plane: None,
                is_hidden: Some(row.get(16)?),
                is_locked: Some(row.get(17)?),
                color: row.get(18)?,
                description: row.get(19)?,
                props: None,
                attributes: None,
            })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_connections(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Connection>> {
        let mut stmt = conn.prepare("SELECT guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description FROM connection WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([design_guid], |row| {
            Ok(Connection {
                guid: row.get(0)?,
                connected: Side {
                    piece: PieceId { guid: row.get(1)? },
                    design_piece: row.get::<_, Option<String>>(2)?.map(|g| PieceId { guid: g }),
                    connector: row.get::<_, Option<String>>(3)?.filter(|s| !s.is_empty()).map(|g| ConnectorId { guid: g }),
                },
                connecting: Side {
                    piece: PieceId { guid: row.get(4)? },
                    design_piece: row.get::<_, Option<String>>(5)?.map(|g| PieceId { guid: g }),
                    connector: row.get::<_, Option<String>>(6)?.filter(|s| !s.is_empty()).map(|g| ConnectorId { guid: g }),
                },
                gap: row.get(7)?,
                shift: row.get(8)?,
                rise: row.get(9)?,
                rotation: row.get(10)?,
                turn: row.get(11)?,
                tilt: row.get(12)?,
                u: row.get(13)?,
                v: row.get(14)?,
                description: row.get(15)?,
                attributes: None,
            })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_layers(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Layer>> {
        let mut stmt = conn.prepare("SELECT guid, path, is_hidden, is_locked, color, description FROM layer WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([design_guid], |row| {
            Ok(Layer { guid: row.get(0)?, path: row.get(1)?, is_hidden: Some(row.get(2)?), is_locked: Some(row.get(3)?), color: row.get(4)?, description: row.get(5)?, attributes: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
    
    fn load_groups(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Group>> {
        let mut stmt = conn.prepare("SELECT guid, name, color, description FROM \"group\" WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([design_guid], |row| {
            Ok(Group { guid: row.get(0)?, name: row.get(1)?, color: row.get(2)?, description: row.get(3)?, pieces: None, attributes: None })
        }).map_err(|e| SemioError::Database { message: e.to_string() })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| SemioError::Database { message: e.to_string() })
    }
}

// #endregion SQLite Import/Export

// #region WASM Bindings

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[derive(Serialize, Deserialize)]
    pub struct WasmResult<T> {
        pub ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<T>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
    }

    impl<T> WasmResult<T> {
        pub fn success(data: T) -> Self { Self { ok: true, data: Some(data), error: None } }
        pub fn failure(error: String) -> Self { Self { ok: false, data: None, error: Some(error) } }
    }

    fn to_js_value<T: Serialize>(result: WasmResult<T>) -> JsValue {
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(js_name = "generateGuid")]
    pub fn wasm_generate_guid() -> String { guid() }

    #[wasm_bindgen(js_name = "serializeKit")]
    pub fn wasm_serialize_kit(kit_json: &str) -> JsValue {
        match deserialize_kit(kit_json) {
            Ok(kit) => match serialize_kit(&kit) {
                Ok(json) => to_js_value(WasmResult::success(json)),
                Err(e) => to_js_value(WasmResult::<String>::failure(e.to_string())),
            },
            Err(e) => to_js_value(WasmResult::<String>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "deserializeKit")]
    pub fn wasm_deserialize_kit(json: &str) -> JsValue {
        match deserialize_kit(json) {
            Ok(kit) => to_js_value(WasmResult::success(kit)),
            Err(e) => to_js_value(WasmResult::<Kit>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "validateKit")]
    pub fn wasm_validate_kit(kit_json: &str) -> JsValue {
        match deserialize_kit(kit_json) {
            Ok(kit) => {
                let result = validate_kit(&kit);
                to_js_value(WasmResult::success(result))
            },
            Err(e) => to_js_value(WasmResult::<ValidationResult>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "areKitsEqual")]
    pub fn wasm_are_kits_equal(kit_a_json: &str, kit_b_json: &str) -> JsValue {
        let result = (|| -> Result<bool> {
            let kit_a = deserialize_kit(kit_a_json)?;
            let kit_b = deserialize_kit(kit_b_json)?;
            Ok(are_kits_equal(&kit_a, &kit_b))
        })();
        match result {
            Ok(equal) => to_js_value(WasmResult::success(equal)),
            Err(e) => to_js_value(WasmResult::<bool>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "flattenDesign")]
    pub fn wasm_flatten_design(kit_json: &str, design_guid: &str) -> JsValue {
        match deserialize_kit(kit_json) {
            Ok(kit) => match flatten_design(&kit, design_guid) {
                Ok(flattened) => {
                    let planes: Vec<_> = flattened.iter().map(|fp| {
                        serde_json::json!({
                            "pieceGuid": fp.piece.guid,
                            "plane": fp.plane,
                            "typeGuid": fp.type_guid,
                            "designGuid": fp.design_guid,
                        })
                    }).collect();
                    to_js_value(WasmResult::success(planes))
                },
                Err(e) => to_js_value(WasmResult::<Vec<serde_json::Value>>::failure(e.to_string())),
            },
            Err(e) => to_js_value(WasmResult::<Vec<serde_json::Value>>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "normalize")]
    pub fn wasm_normalize(value: f64, decimals: u32) -> f64 { normalize(value, decimals) }

    #[wasm_bindgen(js_name = "round")]
    pub fn wasm_round(value: f64) -> f64 { round(value) }

    #[wasm_bindgen(js_name = "isSupportedModelExtension")]
    pub fn wasm_is_supported_model_extension(ext: &str) -> bool { is_supported_model_extension(ext) }

    #[wasm_bindgen(js_name = "generateUniqueName")]
    pub fn wasm_generate_unique_name(base: &str, existing_json: &str) -> JsValue {
        match serde_json::from_str::<Vec<String>>(existing_json) {
            Ok(existing) => to_js_value(WasmResult::success(generate_unique_name(base, &existing))),
            Err(e) => to_js_value(WasmResult::<String>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "findTypeInKit")]
    pub fn wasm_find_type_in_kit(kit_json: &str, guid: &str) -> JsValue {
        match deserialize_kit(kit_json) {
            Ok(kit) => match find_type_in_kit(&kit, guid) {
                Some(t) => to_js_value(WasmResult::success(t.clone())),
                None => to_js_value(WasmResult::<Type>::failure(format!("Type {} not found", guid))),
            },
            Err(e) => to_js_value(WasmResult::<Type>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "findDesignInKit")]
    pub fn wasm_find_design_in_kit(kit_json: &str, guid: &str) -> JsValue {
        match deserialize_kit(kit_json) {
            Ok(kit) => match find_design_in_kit(&kit, guid) {
                Some(d) => to_js_value(WasmResult::success(d.clone())),
                None => to_js_value(WasmResult::<Design>::failure(format!("Design {} not found", guid))),
            },
            Err(e) => to_js_value(WasmResult::<Design>::failure(e.to_string())),
        }
    }

    #[wasm_bindgen(start)]
    pub fn wasm_init() {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }
}

// #endregion WASM Bindings

// #region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const ASSETS_DIR: &str = "../../assets/semio";

    fn load_kit(filename: &str) -> Kit {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize kit")
    }

    #[test]
    fn test_roundtrip_metabolism() {
        let kit = load_kit("kit_metabolism.json");
        let json = serialize_kit(&kit).unwrap();
        let restored = deserialize_kit(&json).unwrap();
        assert!(are_kits_equal(&kit, &restored));
    }

    #[test]
    fn test_validation_invalid() {
        let kit = load_kit("kit_invalid.json");
        let result = validate_kit(&kit);
        assert!(!result.problems.is_empty());
        assert!(result.problems.iter().any(|p| p.constraint_id == "guid-unique"));
        assert!(result.problems.iter().any(|p| p.constraint_id == "type-name-unique"));
    }

    #[test]
    fn test_guid_generation() {
        let g1 = guid();
        let g2 = guid();
        assert_ne!(g1, g2);
        assert!(!g1.is_empty());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(3.14159, 2), 3.14);
        assert_eq!(normalize(3.145, 2), 3.15);
        assert_eq!(round(3.1415926), 3.142);
    }

    #[test]
    fn test_jaccard() {
        let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
        assert!((jaccard(&a, &b) - 0.5).abs() < 0.001);
        
        let empty: HashSet<i32> = HashSet::new();
        assert_eq!(jaccard(&empty, &empty), 1.0);
    }

    #[test]
    fn test_generate_unique_name() {
        let existing = vec!["Test".to_string(), "Test (1)".to_string()];
        assert_eq!(generate_unique_name("Test", &existing), "Test (2)");
        assert_eq!(generate_unique_name("New", &existing), "New");
    }

    #[test]
    fn test_vector_operations() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        
        let nalg = v.to_nalgebra();
        let back = Vector::from_nalgebra(&nalg);
        assert_eq!(v, back);
    }

    #[test]
    fn test_plane_default() {
        let p = Plane::default();
        assert_eq!(p.origin, Vector::zero());
        assert_eq!(p.x_axis, Vector::unit_x());
        assert_eq!(p.y_axis, Vector::unit_y());
    }

    #[test]
    fn test_coord() {
        let c = Coord::new(1.0, 2.0);
        assert_eq!(c.u, 1.0);
        assert_eq!(c.v, 2.0);
    }

    #[test]
    fn test_kit_serialization_roundtrip() {
        let kit = Kit {
            guid: guid(),
            name: "Test Kit".to_string(),
            version: Some("1.0.0".to_string()),
            description: None,
            icon: None,
            image: None,
            preview: None,
            remote: None,
            homepage: None,
            license: None,
            concepts: None,
            tags: None,
            types: None,
            designs: None,
            ports: None,
            qualities: None,
            files: None,
            folders: None,
            authors: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        };
        
        let json = serialize_kit(&kit).unwrap();
        let restored = deserialize_kit(&json).unwrap();
        assert!(are_kits_equal(&kit, &restored));
    }

    #[test]
    fn test_type_serialization() {
        let t = Type {
            guid: guid(),
            name: "Test Type".to_string(),
            parent: None,
            description: Some("A test type".to_string()),
            icon: None,
            image: None,
            folder: None,
            unit: None,
            stock: None,
            is_abstract: None,
            virtual_type: None,
            location: None,
            concepts: None,
            authors: None,
            props: None,
            models: None,
            connectors: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        };
        
        let json = serialize_type(&t).unwrap();
        let restored = deserialize_type(&json).unwrap();
        assert!(are_types_equal(&t, &restored));
    }

    #[test]
    fn test_finder_functions() {
        let type_guid = guid();
        let design_guid = guid();
        let piece_guid = guid();
        
        let kit = Kit {
            guid: guid(),
            name: "Finder Test".to_string(),
            version: None,
            description: None,
            icon: None,
            image: None,
            preview: None,
            remote: None,
            homepage: None,
            license: None,
            concepts: None,
            tags: None,
            types: Some(vec![Type {
                guid: type_guid.clone(),
                name: "Found Type".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            }]),
            designs: Some(vec![Design {
                guid: design_guid.clone(),
                name: "Found Design".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                is_abstract: None,
                can_scale: None,
                can_mirror: None,
                concepts: None,
                authors: None,
                props: None,
                pieces: Some(vec![Piece {
                    guid: piece_guid.clone(),
                    name: Some("Found Piece".to_string()),
                    type_ref: None,
                    design: None,
                    plane: None,
                    center: None,
                    scale: None,
                    mirror_plane: None,
                    is_hidden: None,
                    is_locked: None,
                    color: None,
                    description: None,
                    props: None,
                    attributes: None,
                }]),
                connections: None,
                layers: None,
                groups: None,
                stats: None,
                active_layer: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            }]),
            ports: None,
            qualities: None,
            files: None,
            folders: None,
            authors: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        };
        
        assert!(find_type_in_kit(&kit, &type_guid).is_some());
        assert!(find_type_in_kit(&kit, "nonexistent").is_none());
        
        assert!(find_design_in_kit(&kit, &design_guid).is_some());
        
        let design = find_design_in_kit(&kit, &design_guid).unwrap();
        assert!(find_piece_in_design(design, &piece_guid).is_some());
    }

    #[test]
    fn test_validation() {
        let kit = Kit {
            guid: guid(),
            name: "Valid Kit".to_string(),
            version: None,
            description: None,
            icon: None,
            image: None,
            preview: None,
            remote: None,
            homepage: None,
            license: None,
            concepts: None,
            tags: None,
            types: None,
            designs: None,
            ports: None,
            qualities: None,
            files: None,
            folders: None,
            authors: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        };
        
        let result = validate_kit(&kit);
        assert!(result.valid);
        assert!(result.problems.is_empty());
    }

    #[test]
    fn test_supported_model_extensions() {
        assert!(is_supported_model_extension("gltf"));
        assert!(is_supported_model_extension("GLTF"));
        assert!(is_supported_model_extension("glb"));
        assert!(!is_supported_model_extension("txt"));
        assert!(!is_supported_model_extension("jpg"));
    }

    #[test]
    fn test_deep_equal() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let v3 = Vector::new(1.0, 2.0, 4.0);
        
        assert!(deep_equal(&v1, &v2));
        assert!(!deep_equal(&v1, &v3));
    }

    #[test]
    fn test_has_guid_trait() {
        let attr = Attribute {
            guid: "test-guid".to_string(),
            key: "test-key".to_string(),
            value: None,
            definition: None,
        };
        assert_eq!(attr.guid(), "test-guid");
    }
}

// #endregion Tests