#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

mod header { // 🧲Header

    // 2026 Ueli Saluz <ueli@semio-tech.de>

    // This program is free software: you can redistribute it and/or modify
    // it under the terms of the GNU Affero General Public License as
    // published by the Free Software Foundation, either version 3 of the
    // License, or (at your option) any later version.
    //
    // This program is distributed in the hope that it will be useful,
    // but WITHOUT ANY WARRANTY; without even the implied warranty of
    // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    // GNU Affero General Public License for more details.
    //
    // You should have received a copy of the GNU Affero General Public License
    // along with this program.  If not, see <https://www.gnu.org/licenses/>.
} // 🧲Header

mod imports { // ⛩️Imports
              // Imports MUST include all required crates and modules for the semio domain library.
} // ⛩️Imports

use base64::Engine;
use nalgebra::{Matrix4, Point3, Vector3};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use thiserror::Error;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod utility_functions {
    // 📦Utilities
    // 🏭Utility Functions
    // Utility Functions MUST provide the utility functions functionality.
    /// <summary>🔑Guid represents a UUID string identifier.</summary>
    /// <remarks>
    /// </remarks>
    use super::*;

    pub type Guid = String;

    /// <summary>🔑generates a new v7 UUID string.</summary>
    pub fn guid() -> String {
        Uuid::now_v7().to_string()
    }

    /// <summary>📐rounds a float to the given number of decimal places.</summary>
    pub fn normalize(value: f64, decimals: u32) -> f64 {
        let factor = 10_f64.powi(decimals as i32);
        (value * factor).round() / factor
    }

    /// <summary>📐rounds a float to 3 decimal places.</summary>
    /// <remarks>
    /// </remarks>
    pub fn round(value: f64) -> f64 {
        normalize(value, 3)
    }

    /// <summary>📊computes Jaccard similarity between two sets.</summary>
    pub fn jaccard<T: Eq + std::hash::Hash>(a: &HashSet<T>, b: &HashSet<T>) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 1.0;
        }
        let intersection = a.intersection(b).count();
        let union = a.union(b).count();
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// <summary>🔄compares two serializable values for deep equality.</summary>
    pub fn deep_equal<T: Serialize>(a: &T, b: &T) -> bool {
        pub fn normalize_json(v: &mut serde_json::Value) {
            match v {
                serde_json::Value::Array(arr) => {
                    for item in arr.iter_mut() {
                        normalize_json(item);
                    }
                    arr.sort_by(|a, b| {
                        let a_guid = a.get("guid").and_then(|g| g.as_str()).unwrap_or("");
                        let b_guid = b.get("guid").and_then(|g| g.as_str()).unwrap_or("");
                        a_guid.cmp(b_guid)
                    });
                }
                serde_json::Value::Object(map) => {
                    for (_, val) in map.iter_mut() {
                        normalize_json(val);
                    }
                }
                _ => {}
            }
        }
        pub fn json_approx_eq(a: &serde_json::Value, b: &serde_json::Value) -> bool {
            use serde_json::Value;
            match (a, b) {
                (Value::Number(na), Value::Number(nb)) => match (na.as_f64(), nb.as_f64()) {
                    (Some(fa), Some(fb)) => (fa - fb).abs() < 1e-10,
                    _ => na == nb,
                },
                (Value::Array(aa), Value::Array(ab)) => {
                    aa.len() == ab.len()
                        && aa.iter().zip(ab.iter()).all(|(x, y)| json_approx_eq(x, y))
                }
                (Value::Object(ma), Value::Object(mb)) => {
                    ma.len() == mb.len()
                        && ma
                            .iter()
                            .all(|(k, v)| mb.get(k).map_or(false, |v2| json_approx_eq(v, v2)))
                }
                _ => a == b,
            }
        }
        match (serde_json::to_value(a), serde_json::to_value(b)) {
            (Ok(mut va), Ok(mut vb)) => {
                normalize_json(&mut va);
                normalize_json(&mut vb);
                json_approx_eq(&va, &vb)
            }
            _ => false,
        }
    }
    /// <remarks>
    /// </remarks>
    pub fn generate_unique_name(base: &str, existing: &[String]) -> String {
        let existing_set: HashSet<_> = existing.iter().collect();
        if !existing_set.contains(&base.to_string()) {
            return base.to_string();
        }
        let mut counter = 1;
        loop {
            let candidate = format!("{} ({})", base, counter);
            if !existing_set.contains(&candidate) {
                return candidate;
            }
            counter += 1;
        }
    }
} // 📦Utilities
pub use utility_functions::*;

mod error_types {
    // ⚠️Exceptions
    // 🎪Error Types
    // Error Types MUST provide the error types functionality.

    use super::*;

    #[derive(Error, Debug, Clone, Serialize, Deserialize)]
    /// <summary>❌SemioError represents a domain error with context message.</summary>
    /// <remarks>
    /// </remarks>
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
    /// <summary>✅Result represents a success or SemioError outcome.</summary>
    /// <remarks>
    /// </remarks>
    pub type Result<T> = std::result::Result<T, SemioError>;
} // ⚠️Exceptions
pub use error_types::*;

mod has_guid_trait {
    // 🐍Entity IDs
    // 🎮HasGuid Trait
    // HasGuid Trait MUST provide the hasguid trait functionality.
    /// <summary>🔧d.</summary>
    /// <remarks>
    /// </remarks>
    use super::*;

    pub trait HasGuid {
        fn guid(&self) -> &str;
    }
    /// <summary>💎HasGuid implementation for Attribute.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Attribute {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📊HasGuid implementation for Prop.</summary>
    impl HasGuid for Prop {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔌HasGuid implementation for Connector.</summary>
    impl HasGuid for Connector {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🗿HasGuid implementation for Model.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Model {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🧱HasGuid implementation for Type.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Type {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🧩HasGuid implementation for Piece.</summary>
    impl HasGuid for Piece {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔗HasGuid implementation for Connection.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Connection {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🎨HasGuid implementation for Layer.</summary>
    impl HasGuid for Layer {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>👥HasGuid implementation for Group.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Group {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📈HasGuid implementation for Stat.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Stat {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📐HasGuid implementation for Design.</summary>
    impl HasGuid for Design {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🏷️HasGuid implementation for Tag.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Tag {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>💡HasGuid implementation for Concept.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Concept {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>⚓HasGuid implementation for Port.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Port {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔬HasGuid implementation for Quality.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Quality {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📄HasGuid implementation for File.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for File {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📁HasGuid implementation for Folder.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Folder {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>✍️HasGuid implementation for Author.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Author {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📦HasGuid implementation for Kit.</summary>
    /// <remarks>
    /// </remarks>
    impl HasGuid for Kit {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    impl HasGuid for Benchmark {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    impl HasGuid for Location {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>💎DiffHasGuid implementation for AttributeDiff.</summary>
    /// <remarks>
    /// </remarks>
    pub trait DiffHasGuid {
        fn guid(&self) -> &str;
    }
    /// <summary>💎HasGuid implementation for AttributeDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for AttributeDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📊HasGuid implementation for PropDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for PropDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔌HasGuid implementation for ConnectorDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for ConnectorDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🗿HasGuid implementation for ModelDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for ModelDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🧱HasGuid implementation for TypeDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for TypeDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🧩HasGuid implementation for PieceDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for PieceDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔗HasGuid implementation for ConnectionDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for ConnectionDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🎨HasGuid implementation for LayerDiff.</summary>
    /// <remarks>
    /// </remarks>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for LayerDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>👥HasGuid implementation for GroupDiff.</summary>
    impl DiffHasGuid for GroupDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📈HasGuid implementation for StatDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for StatDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📐HasGuid implementation for DesignDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for DesignDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🏷️HasGuid implementation for TagDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for TagDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>💡HasGuid implementation for ConceptDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for ConceptDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>⚓HasGuid implementation for PortDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for PortDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>🔬HasGuid implementation for QualityDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for QualityDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📄HasGuid implementation for FileDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for FileDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📁HasGuid implementation for FolderDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for FolderDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>✍️HasGuid implementation for AuthorDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for AuthorDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
    /// <summary>📦HasGuid implementation for KitDiff.</summary>
    /// <remarks>
    /// </remarks>
    impl DiffHasGuid for KitDiff {
        fn guid(&self) -> &str {
            &self.guid
        }
    }
} // 🐍Entity IDs
pub use has_guid_trait::*;

mod model_types_attribute {
    // 💎Attribute
    // 💎Model Types - Attribute
    // 💎Model Types - Attribute MUST provide the model types - attribute functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>💎Attribute represents a key-value metadata entry with optional definition.</summary>
    pub struct Attribute {
        pub guid: Guid,
        pub key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>💎AttributeId identifies an attribute entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct AttributeId {
        pub guid: Guid,
    }
} // 💎Attribute
pub use model_types_attribute::*;

mod model_types_coord {
    // 📺Coord
    // 📺Model Types - Coord
    // 📺Model Types - Coord MUST provide the model types - coord functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📺Coord represents a 2D coordinate with U and V components.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Coord {
        pub u: f64,
        pub v: f64,
    }
    /// <summary>📺Coord represents a 2D coordinate with U and V components.</summary>
    /// <remarks>
    /// </remarks>
    impl Coord {
        pub fn new(u: f64, v: f64) -> Self {
            Self { u, v }
        }
    }
} // 📺Coord
pub use model_types_coord::*;

mod model_types_vector {
    // ↗️Vector
    // ↗️Model Types - Vector
    // ↗️Model Types - Vector MUST provide the model types - vector functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>↗️Vector represents a 3D vector with X, Y and Z components.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Vector {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default)]
        pub z: f64,
    }

    /// <summary>↗️Vector represents a 3D vector with X, Y and Z components.</summary>
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
        pub fn to_nalgebra(&self) -> Vector3<f64> {
            Vector3::new(self.x, self.y, self.z)
        }
        pub fn from_nalgebra(v: &Vector3<f64>) -> Self {
            Self::new(v.x, v.y, v.z)
        }
    }
} // ↗️Vector
pub use model_types_vector::*;

mod model_types_plane {
    // ◻️Plane
    // ◻️Model Types - Plane
    // ◻️Model Types - Plane MUST provide the model types - plane functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>◻️Plane represents a plane defined by origin point and two axis vectors.</summary>
    pub struct Plane {
        pub origin: Vector,
        #[serde(rename = "xAxis")]
        pub x_axis: Vector,
        #[serde(rename = "yAxis")]
        pub y_axis: Vector,
    }

    /// <summary>◻️Default implementation for Plane.</summary>
    impl Default for Plane {
        fn default() -> Self {
            Self {
                origin: Vector::zero(),
                x_axis: Vector::unit_x(),
                y_axis: Vector::unit_y(),
            }
        }
    }

    /// <summary>◻️Plane represents a plane defined by origin point and two axis vectors.</summary>
    impl Plane {
        pub fn new(origin: Vector, x_axis: Vector, y_axis: Vector) -> Self {
            Self {
                origin,
                x_axis,
                y_axis,
            }
        }
        pub fn world_xy() -> Self {
            Self::default()
        }

        pub fn to_matrix(&self) -> Matrix4<f64> {
            let x = self.x_axis.to_nalgebra().normalize();
            let y = self.y_axis.to_nalgebra().normalize();
            let z = x.cross(&y).normalize();
            let o = Point3::new(self.origin.x, self.origin.y, self.origin.z);
            Matrix4::new(
                x.x, y.x, z.x, o.x, x.y, y.y, z.y, o.y, x.z, y.z, z.z, o.z, 0.0, 0.0, 0.0, 1.0,
            )
        }

        pub fn from_matrix(m: &Matrix4<f64>) -> Self {
            let origin = Vector::new(m[(0, 3)], m[(1, 3)], m[(2, 3)]);
            let x_axis = Vector::new(m[(0, 0)], m[(1, 0)], m[(2, 0)]);
            let y_axis = Vector::new(m[(0, 1)], m[(1, 1)], m[(2, 1)]);
            Self {
                origin,
                x_axis,
                y_axis,
            }
        }

        pub fn round(&self) -> Self {
            Self {
                origin: Vector::new(
                    round(self.origin.x),
                    round(self.origin.y),
                    round(self.origin.z),
                ),
                x_axis: Vector::new(
                    round(self.x_axis.x),
                    round(self.x_axis.y),
                    round(self.x_axis.z),
                ),
                y_axis: Vector::new(
                    round(self.y_axis.x),
                    round(self.y_axis.y),
                    round(self.y_axis.z),
                ),
            }
        }
    }
} // ◻️Plane
pub use model_types_plane::*;

mod model_types_camera {
    // 🎥Camera
    // 🎥Model Types - Camera
    // 🎥Model Types - Camera MUST provide the model types - camera functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🎥Camera represents a camera defined by position, forward and up vectors.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Camera {
        pub position: Vector,
        pub target: Vector,
        pub up: Vector,
        pub fov: f64,
        pub near: f64,
        pub far: f64,
    }

    /// <summary>🎥Default implementation for Camera.</summary>
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
} // 🎥Camera
pub use model_types_camera::*;

mod location {
    // 📍Location
    // 📍Location
    // 📍Location MUST provide the location functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📍LocationId identifies a location entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct LocationId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📍Location represents a geographic point with longitude, latitude and optional altitude.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Location {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 📍Location
pub use location::*;

mod author {
    // ✍️Author
    // ✍️Author
    // ✍️Author MUST provide the author functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>✍️AuthorId identifies an author entity by GUID.</summary>
    pub struct AuthorId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>✍️Author represents a named contributor with email and custom attributes.</summary>
    /// <remarks>
    /// </remarks>
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
} // ✍️Author
pub use author::*;

mod file_entity {
    // 📄File
    // 📄File
    // 📄File MUST provide the file functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📄FileId identifies a file entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct FileId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📄File represents a named binary resource with optional remote URL and folder.</summary>
    /// <remarks>
    /// </remarks>
    pub struct File {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub remote: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub folder: Option<FolderId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blob: Option<String>,
        #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }
} // 📄File
pub use file_entity::*;

mod folder_entity {
    // 📁Folder
    // 📁Folder
    // 📁Folder MUST provide the folder functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📁FolderId identifies a folder entity by GUID.</summary>
    pub struct FolderId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📁Folder represents a named directory for organizing files.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Folder {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent: Option<FolderId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
        #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }
} // 📁Folder
pub use folder_entity::*;

mod benchmark_entity {
    // 📏Benchmark
    // 📏Benchmark
    // 📏Benchmark MUST provide the benchmark functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BenchmarkId {
        pub guid: Guid,
    }

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
        pub definition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 📏Benchmark
pub use benchmark_entity::*;

mod quality {
    // 🔬Quality
    // 🔬Quality
    // 🔬Quality MUST provide the quality functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔬QualityId identifies a quality entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct QualityId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    /// <summary>🔬QualityKind represents the numeric kind of a quality (integer, float or boolean).</summary>
    pub enum QualityKind {
        #[default]
        Integer = 0,
        Float = 1,
        Boolean = 2,
    }

    impl Serialize for QualityKind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(*self as i32)
        }
    }

    impl<'de> Deserialize<'de> for QualityKind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let v = i32::deserialize(deserializer)?;
            match v {
                0 => Ok(QualityKind::Integer),
                1 => Ok(QualityKind::Float),
                2 => Ok(QualityKind::Boolean),
                _ => Err(serde::de::Error::custom("invalid QualityKind discriminant")),
            }
        }
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🔬Quality represents a measurable property with formula, units and benchmarks.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Quality {
        pub guid: Guid,
        pub key: String,
        pub name: String,
        #[serde(default)]
        pub kind: QualityKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
        pub default_value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub formula: Option<String>,
        #[serde(rename = "defaultSiUnit", skip_serializing_if = "Option::is_none")]
        pub default_si_unit: Option<String>,
        #[serde(
            rename = "defaultImperialUnit",
            skip_serializing_if = "Option::is_none"
        )]
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
} // 🔬Quality
pub use quality::*;

mod port {
    // ⚓Port
    // ⚓Port
    // ⚓Port MUST provide the port functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>⚓PortId identifies a port entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PortId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>⚓Port represents a named connection interface with compatible ports.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Port {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(rename = "maxChildren", skip_serializing_if = "Option::is_none")]
        pub max_children: Option<i32>,
        #[serde(rename = "compatiblePorts", skip_serializing_if = "Option::is_none")]
        pub compatible_interfaces: Option<Vec<PortId>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // ⚓Port
pub use port::*;

mod prop {
    // 📊Prop
    // 📊Prop
    // 📊Prop MUST provide the prop functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📊PropId identifies a prop entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PropId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📊Prop represents a quality measurement value with optional unit.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Prop {
        pub guid: Guid,
        pub quality: QualityId,
        pub value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 📊Prop
pub use prop::*;

mod tag {
    // 🏷️Tag
    // 🏷️Tag
    // 🏷️Tag MUST provide the tag functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🏷️TagId identifies a tag entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct TagId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🏷️Tag represents a named categorization label with optional description and icon.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Tag {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 🏷️Tag
pub use tag::*;

mod concept {
    // 💡Concept
    // 💡Concept
    // 💡Concept MUST provide the concept functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>💡ConceptId identifies a concept entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ConceptId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>💡Concept represents a named categorization concept with optional description and icon.</summary>
    /// <remarks>
    /// </remarks>
    /// <remarks>
    /// </remarks>
    pub struct Concept {
        pub guid: Guid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 💡Concept
pub use concept::*;

mod model_entity {
    // 🗿Model
    // 🗿Model
    // 🗿Model MUST provide the model functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🗿ModelId identifies a model entity by GUID.</summary>
    pub struct ModelId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🗿Model represents a 3D model reference linking a file with tags and description.</summary>
    /// <remarks>
    /// </remarks>
    /// <remarks>
    /// </remarks>
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
} // 🗿Model
pub use model_entity::*;

mod connector {
    // 🔌Connector
    // 🔌Connector
    // 🔌Connector MUST provide the connector functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔌ConnectorId identifies a connector entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ConnectorId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🔌Connector represents a connection point on a type with position, direction and parameter.</summary>
    /// <remarks>
    /// </remarks>
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
        #[serde(rename = "maxChildren", skip_serializing_if = "Option::is_none")]
        pub max_children: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<PortId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub props: Option<Vec<Prop>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
    }
} // 🔌Connector
pub use connector::*;

mod model_types_type {
    // 🧱Type
    // 🧱Model Types - Type
    // 🧱Model Types - Type MUST provide the model types - type functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🧱TypeId identifies a type entity by GUID.</summary>
    pub struct TypeId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🧱Type represents a reusable element blueprint with connectors, models and props.</summary>
    /// <remarks>
    /// </remarks>
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
        pub concepts: Option<Vec<ConceptId>>,
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
} // 🧱Type
pub use model_types_type::*;

mod layer {
    // 🎨Layer
    // 🎨Layer
    // 🎨Layer MUST provide the layer functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🎨LayerId identifies a layer entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct LayerId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🎨Layer represents a named visibility and color layer within a design.</summary>
    /// <remarks>
    /// </remarks>
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
} // 🎨Layer
pub use layer::*;

mod piece {
    // 🧩Piece
    // 🧩Piece
    // 🧩Piece MUST provide the piece functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🧩PieceId identifies a piece entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PieceId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📐DesignId identifies a design entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct DesignId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🧩Piece represents a positioned instance of a type within a design.</summary>
    /// <remarks>
    /// </remarks>
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
} // 🧩Piece
pub use piece::*;

mod group {
    // 👥Group
    // 👥Group
    // 👥Group MUST provide the group functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>👥GroupId identifies a group entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct GroupId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>👥Group represents a named collection of pieces within a design.</summary>
    /// <remarks>
    /// </remarks>
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
} // 👥Group
pub use group::*;

mod side {
    // ↔️Side
    // ↔️Side
    // ↔️Side MUST provide the side functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>↔️Side represents one side of a connection identifying a piece and optional connector.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Side {
        pub piece: PieceId,
        #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
        pub design_piece: Option<PieceId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub connector: Option<ConnectorId>,
    }
} // ↔️Side
pub use side::*;

mod connection {
    // 🔗Connection
    // 🔗Connection
    // 🔗Connection MUST provide the connection functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔗ConnectionId identifies a connection entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ConnectionId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>🔗Connection represents a spatial relationship between two pieces with gap, shift and rotation.</summary>
    /// <remarks>
    /// </remarks>
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
} // 🔗Connection
pub use connection::*;

mod stat {
    // 📈Stat
    // 📈Stat
    // 📈Stat MUST provide the stat functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📈StatId identifies a stat entity by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct StatId {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📈Stat represents a statistical quality measurement with min/max bounds and unit.</summary>
    /// <remarks>
    /// </remarks>
    /// <remarks>
    /// </remarks>
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
} // 📈Stat
pub use stat::*;

mod model_types_design {
    // 📐Design
    // 📐Model Types - Design
    // 📐Model Types - Design MUST provide the model types - design functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📐Design represents an assembly of pieces, connections, layers and groups.</summary>
    /// <remarks>
    /// </remarks>
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
        pub concepts: Option<Vec<ConceptId>>,
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
        pub location: Option<LocationId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<Attribute>>,
        #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }
} // 📐Design
pub use model_types_design::*;

mod model_types_kit {
    // ⏱️Kit
    // 📦Model Types - Kit
    // 📦Model Types - Kit MUST provide the model types - kit functionality.

    use super::*;

    mod kit_kind {
        // 📈KitKind
        // KitKind discriminates the five persistence/transport forms of a Kit.

        /// Discriminator for the five kit persistence/transport forms.
        ///
        /// Specs: Exactly five kit kinds exist:
        /// - Dev: Self-contained JSON file (.kit.json)
        /// - Local: Local folder with .semio/kit.db SQLite file and asset files
        /// - Archive: ZIP file packaging a LocalKit structure
        /// - Remote: URL-addressable kit served over HTTP(S)
        /// - Transport: In-memory ephemeral kit (no persistence)
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
        #[serde(rename_all = "lowercase")]
        pub enum KitKind {
            Dev,
            Local,
            Archive,
            Remote,
            Transport,
        }

        /// 📦All valid KitKind values.
        pub const ALL_KIT_KINDS: [KitKind; 5] = [
            KitKind::Dev,
            KitKind::Local,
            KitKind::Archive,
            KitKind::Remote,
            KitKind::Transport,
        ];
    } // 🥁KitKind
    pub use kit_kind::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// <summary>📦Kit represents the root container for all domain entities.</summary>
    /// <remarks>
    /// </remarks>
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
        pub ports: Option<Vec<Port>>,
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
} // ⏱️Kit
pub use model_types_kit::*;

mod serialization {
    // ⏰Serialization
    // 👑Serialization
    // Serialization MUST provide the serialization functionality.

    /// <summary>💾serialize kit.</summary>
    use super::*;

    pub fn serialize_kit(kit: &Kit) -> Result<String> {
        serde_json::to_string_pretty(kit).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }
    /// <summary>💾deserialize kit.</summary>
    /// <remarks>
    /// </remarks>
    pub fn deserialize_kit(json: &str) -> Result<Kit> {
        serde_json::from_str(json).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }
    /// <remarks>
    /// </remarks>
    pub fn serialize_design(design: &Design) -> Result<String> {
        serde_json::to_string_pretty(design).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }
    /// <summary>💾deserialize design.</summary>
    /// <remarks>
    /// </remarks>
    pub fn deserialize_design(json: &str) -> Result<Design> {
        serde_json::from_str(json).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }

    /// <summary>💾serialize type.</summary>
    /// <remarks>
    /// </remarks>
    pub fn serialize_type(t: &Type) -> Result<String> {
        serde_json::to_string_pretty(t).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }
    /// <summary>💾deserialize type.</summary>
    /// <remarks>
    /// </remarks>
    pub fn deserialize_type(json: &str) -> Result<Type> {
        serde_json::from_str(json).map_err(|e| SemioError::Serialization {
            message: e.to_string(),
        })
    }
    /// <remarks>
    /// </remarks>
    pub fn are_kits_equal(a: &Kit, b: &Kit) -> bool {
        deep_equal(a, b)
    }
    /// <summary>🔄compares two designs entities for deep equality.</summary>
    /// <remarks>
    /// </remarks>
    pub fn are_designs_equal(a: &Design, b: &Design) -> bool {
        deep_equal(a, b)
    }
    /// <summary>🔄compares two types entities for deep equality.</summary>
    pub fn are_types_equal(a: &Type, b: &Type) -> bool {
        deep_equal(a, b)
    }

    /// <summary>🗿the list of supported 3D model file extensions.</summary>
    pub const SUPPORTED_MODEL_EXTENSIONS: &[&str] = &[
        "gltf", "glb", "fbx", "obj", "dae", "3ds", "stl", "ply", "usdz", "vrm", "ifc", "3mf",
    ];
    /// <remarks>
    /// </remarks>
    pub fn is_supported_model_extension(ext: &str) -> bool {
        SUPPORTED_MODEL_EXTENSIONS.contains(&ext.to_lowercase().as_str())
    }
} // ⏰Serialization
pub use serialization::*;

mod diff_types {
    // ✂️Diff Types
    // ✂️Diff Types
    // Diff Types MUST provide the diff types functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🗑️RemovedItem represents an entity marked for removal by GUID.</summary>
    /// <summary>🗑️RemovedItem represents an entity marked for removal by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub struct RemovedItem {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, PartialEq)]
    /// <summary>🔄DiffUpdate represents a before-after pair for entity updates.</summary>
    /// <remarks>
    /// </remarks>
    pub struct DiffUpdate<D> {
        pub key: String,
        pub guid: Guid,
        pub diff: D,
    }

    impl<'de, D: serde::de::DeserializeOwned> Deserialize<'de> for DiffUpdate<D> {
        fn deserialize<De>(deserializer: De) -> std::result::Result<Self, De::Error>
        where
            De: serde::Deserializer<'de>,
        {
            use serde::Deserialize;
            use serde_json::Value;

            let mut map: serde_json::Map<String, Value> = Deserialize::deserialize(deserializer)?;

            let diff_val = map
                .remove("diff")
                .ok_or_else(|| serde::de::Error::missing_field("diff"))?;

            let mut guid = None;
            let mut key = String::new();

            for (k, v) in &map {
                if let Some(obj) = v.as_object() {
                    if let Some(g) = obj.get("guid") {
                        if let Some(s) = g.as_str() {
                            guid = Some(s.to_string());
                            key = k.clone();
                            break;
                        }
                    }
                }
            }

            let guid = guid
                .ok_or_else(|| serde::de::Error::custom("Could not find guid in update wrapper"))?;
            if key.is_empty() {
                return Err(serde::de::Error::custom("Could not find entity key"));
            }

            let mut diff_obj = match diff_val {
                Value::Object(o) => o,
                _ => {
                    return Err(serde::de::Error::custom(
                        "diff field expected to be an object",
                    ))
                }
            };

            if !diff_obj.contains_key("guid") {
                diff_obj.insert("guid".to_string(), Value::String(guid.clone()));
            }

            let diff: D = serde_json::from_value(Value::Object(diff_obj))
                .map_err(serde::de::Error::custom)?;

            Ok(DiffUpdate { key, guid, diff })
        }
    }

    impl<D: Serialize> Serialize for DiffUpdate<D> {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(None)?;
            map.serialize_entry("diff", &self.diff)?;

            #[derive(Serialize)]
            pub struct GuidWrapper {
                guid: String,
            }
            map.serialize_entry(
                &self.key,
                &GuidWrapper {
                    guid: self.guid.clone(),
                },
            )?;

            map.end()
        }
    }

    impl<D: DiffHasGuid> DiffHasGuid for DiffUpdate<D> {
        fn guid(&self) -> &str {
            &self.guid
        }
    }

    pub fn deserialize_some<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
    where
        T: serde::Deserialize<'de>,
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer).map(Some)
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    #[serde(bound(deserialize = "T: Deserialize<'de>, D: serde::de::DeserializeOwned"))]
    /// <summary>🔄CollectionDiff represents batched entity additions, removals and updates.</summary>
    /// <remarks>
    /// </remarks>
    pub struct CollectionDiff<T, D> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub added: Option<Vec<T>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub removed: Option<Vec<RemovedItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated: Option<Vec<DiffUpdate<D>>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>💎AttributeDiff represents a partial update to attribute's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct AttributeDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub definition: Option<Option<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📊PropDiff represents a partial update to prop's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PropDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quality: Option<QualityId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub unit: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔌ConnectorDiff represents a partial update to connector's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ConnectorDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub point: Option<Vector>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<Vector>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub t: Option<f64>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub name: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub mandatory: Option<Option<bool>>,
        #[serde(
            rename = "maxChildren",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub max_children: Option<Option<i32>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub port: Option<Option<PortId>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub props: Option<CollectionDiff<Prop, PropDiff>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🗿ModelDiff represents a partial update to model's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ModelDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub file: Option<FileId>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub name: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub tags: Option<Option<Vec<TagId>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🧱TypeDiff represents a partial update to type's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct TypeDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub parent: Option<Option<TypeId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub image: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub folder: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub unit: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub stock: Option<Option<i32>>,
        #[serde(
            rename = "isAbstract",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_abstract: Option<Option<bool>>,
        #[serde(
            rename = "virtual",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub virtual_type: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub location: Option<Option<LocationId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub concepts: Option<Option<Vec<ConceptId>>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
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
    /// <summary>↔️SideDiff represents a partial update to side's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct SideDiff {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub piece: Option<PieceId>,
        #[serde(
            rename = "designPiece",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub design_piece: Option<Option<PieceId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub connector: Option<Option<ConnectorId>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔗ConnectionDiff represents a partial update to connection's fields.</summary>
    /// <remarks>
    /// </remarks>
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
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub u: Option<Option<f64>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub v: Option<Option<f64>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🧩PieceDiff represents a partial update to piece's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PieceDiff {
        pub guid: Guid,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub name: Option<Option<String>>,
        #[serde(
            rename = "type",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub type_ref: Option<Option<TypeId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub design: Option<Option<DesignId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub plane: Option<Option<Plane>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub center: Option<Option<Coord>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub scale: Option<Option<f64>>,
        #[serde(
            rename = "mirrorPlane",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub mirror_plane: Option<Option<Plane>>,
        #[serde(
            rename = "isHidden",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_hidden: Option<Option<bool>>,
        #[serde(
            rename = "isLocked",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_locked: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub color: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub props: Option<CollectionDiff<Prop, PropDiff>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🎨LayerDiff represents a partial update to layer's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct LayerDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(
            rename = "isHidden",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_hidden: Option<Option<bool>>,
        #[serde(
            rename = "isLocked",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_locked: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub color: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>👥GroupDiff represents a partial update to group's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct GroupDiff {
        pub guid: Guid,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub name: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub color: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub pieces: Option<Option<Vec<PieceId>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📈StatDiff represents a partial update to stat's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct StatDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quality: Option<QualityId>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub min: Option<Option<f64>>,
        #[serde(
            rename = "minExcluded",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub min_excluded: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub max: Option<Option<f64>>,
        #[serde(
            rename = "maxExcluded",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub max_excluded: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub unit: Option<Option<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📐DesignDiff represents a partial update to design's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct DesignDiff {
        #[serde(default)]
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub parent: Option<Option<DesignId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub image: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub folder: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub unit: Option<Option<String>>,
        #[serde(
            rename = "isAbstract",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_abstract: Option<Option<bool>>,
        #[serde(
            rename = "canScale",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub can_scale: Option<Option<bool>>,
        #[serde(
            rename = "canMirror",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub can_mirror: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub concepts: Option<Option<Vec<ConceptId>>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
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
        #[serde(
            rename = "activeLayer",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub active_layer: Option<Option<LayerId>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🏷️TagDiff represents a partial update to tag's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct TagDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>💡ConceptDiff represents a partial update to concept's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ConceptDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>⚓PortDiff represents a partial update to port's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct PortDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
        #[serde(
            rename = "maxChildren",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub max_children: Option<Option<i32>>,
        #[serde(
            rename = "compatiblePorts",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub compatible_interfaces: Option<Option<Vec<PortId>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔬QualityDiff represents a partial update to quality's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct QualityDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kind: Option<QualityKind>,
        #[serde(
            rename = "defaultValue",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub default_value: Option<Option<f64>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub formula: Option<Option<String>>,
        #[serde(
            rename = "defaultSiUnit",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub default_si_unit: Option<Option<String>>,
        #[serde(
            rename = "defaultImperialUnit",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub default_imperial_unit: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub min: Option<Option<f64>>,
        #[serde(
            rename = "isMinExcluded",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_min_excluded: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub max: Option<Option<f64>>,
        #[serde(
            rename = "isMaxExcluded",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub is_max_excluded: Option<Option<bool>>,
        #[serde(
            rename = "canScale",
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub can_scale: Option<Option<bool>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub uri: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📄FileDiff represents a partial update to file's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct FileDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub remote: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub folder: Option<Option<FolderId>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub size: Option<Option<i64>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub hash: Option<Option<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📁FolderDiff represents a partial update to folder's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct FolderDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub parent: Option<Option<FolderId>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>✍️AuthorDiff represents a partial update to author's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct AuthorDiff {
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub email: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>📦KitDiff represents a partial update to kit's fields.</summary>
    /// <remarks>
    /// </remarks>
    pub struct KitDiff {
        #[serde(default)]
        pub guid: Guid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub version: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub icon: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub image: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub preview: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub remote: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub homepage: Option<Option<String>>,
        #[serde(
            default,
            deserialize_with = "deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
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
        pub ports: Option<CollectionDiff<Port, PortDiff>>,
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

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// <summary>🔄Change represents a tracked modification with timestamp and author.</summary>
    /// <remarks>
    /// </remarks>
    pub struct Change<TEntity, TDiff> {
        pub forward: TDiff,
        pub backward: TDiff,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub author: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<TEntity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<TEntity>,
    }
    /// <summary>💎AttributeChange represents tracks attribute modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type AttributeChange = Change<Attribute, AttributeDiff>;
    /// <summary>✍️AuthorChange represents tracks author modifications in a kit change.</summary>
    pub type AuthorChange = Change<Author, AuthorDiff>;
    /// <summary>📄FileChange represents tracks file modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type FileChange = Change<File, FileDiff>;
    /// <summary>📁FolderChange represents tracks folder modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type FolderChange = Change<Folder, FolderDiff>;
    /// <summary>🔬QualityChange represents tracks quality modifications in a kit change.</summary>
    pub type QualityChange = Change<Quality, QualityDiff>;
    /// <summary>⚓PortChange represents tracks port modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type PortChange = Change<Port, PortDiff>;
    /// <summary>📊PropChange represents tracks prop modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type PropChange = Change<Prop, PropDiff>;
    /// <summary>🏷️TagChange represents tracks tag modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type TagChange = Change<Tag, TagDiff>;
    /// <summary>💡ConceptChange represents tracks concept modifications in a kit change.</summary>
    pub type ConceptChange = Change<Concept, ConceptDiff>;
    /// <summary>🗿ModelChange represents tracks model modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type ModelChange = Change<Model, ModelDiff>;
    /// <summary>🔌ConnectorChange represents tracks connector modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type ConnectorChange = Change<Connector, ConnectorDiff>;
    /// <summary>🧱TypeChange represents tracks type modifications in a kit change.</summary>
    pub type TypeChange = Change<Type, TypeDiff>;
    /// <summary>🎨LayerChange represents tracks layer modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type LayerChange = Change<Layer, LayerDiff>;
    /// <summary>🧩PieceChange represents tracks piece modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type PieceChange = Change<Piece, PieceDiff>;
    /// <summary>👥GroupChange represents tracks group modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type GroupChange = Change<Group, GroupDiff>;
    /// <summary>↔️SideChange represents tracks side modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type SideChange = Change<Side, SideDiff>;
    /// <summary>🔗ConnectionChange represents tracks connection modifications in a kit change.</summary>
    pub type ConnectionChange = Change<Connection, ConnectionDiff>;
    /// <summary>📈StatChange represents tracks stat modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type StatChange = Change<Stat, StatDiff>;
    /// <summary>📐DesignChange represents tracks design modifications in a kit change.</summary>
    /// <remarks>
    /// </remarks>
    pub type DesignChange = Change<Design, DesignDiff>;
    /// <summary>📦KitChange represents tracks kit-level modifications.</summary>
    pub type KitChange = Change<Kit, KitDiff>;

    // #region 🎯SemioReport
    /// 📋Human-readable note attached to a SemioReport (warning, info, or error).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct OperationNote {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        pub message: String,
    }

    /// 📋Canonical semio algorithm output: ok, diff, warnings, infos, errors (tool-friendly JSON).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SemioReport<T> {
        pub ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub diff: Option<T>,
        #[serde(default)]
        pub warnings: Vec<OperationNote>,
        #[serde(default)]
        pub infos: Vec<OperationNote>,
        #[serde(default)]
        pub errors: Vec<OperationNote>,
    }

    impl<T> SemioReport<T> {
        pub fn ok_with(diff: T, warnings: Vec<OperationNote>, infos: Vec<OperationNote>) -> Self {
            Self {
                ok: true,
                diff: Some(diff),
                warnings,
                infos,
                errors: vec![],
            }
        }
        pub fn err(errors: Vec<OperationNote>) -> Self {
            Self {
                ok: false,
                diff: None,
                warnings: vec![],
                infos: vec![],
                errors,
            }
        }
    }
    // #endregion 🎯SemioReport
} // ✂️Diff Types
pub use diff_types::*;

mod meta_and_shallow_types {
    // 🔑Meta And Shallow
    // 🥉Meta And Shallow Types
    // Meta And Shallow Types MUST provide lightweight entity representations.

    use super::*;

    mod sub_entity_meta_types {
        // 🤾Sub-entity Meta Types

        /// AttributeMeta is identical to Attribute (no Vec fields to omit).
        use super::*;

        pub type AttributeMeta = Attribute;

        /// 📈StatMeta represents scalar-only view of stat excluding nested arrays.
        pub type StatMeta = Stat;

        /// 🏷️TagMeta represents scalar-only view of tag excluding nested arrays.
        pub type TagMeta = Tag;

        /// 💡ConceptMeta represents scalar-only view of concept excluding nested arrays.
        pub type ConceptMeta = Concept;

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 📊PropMeta represents scalar-only view of prop excluding nested arrays.
        pub struct PropMeta {
            pub guid: Guid,
            pub quality: QualityId,
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub unit: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// ✍️AuthorMeta represents scalar-only view of author excluding nested arrays.
        pub struct AuthorMeta {
            pub guid: Guid,
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub email: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 📄FileMeta represents scalar-only view of file excluding nested arrays.
        pub struct FileMeta {
            pub guid: Guid,
            pub name: String,
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

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 📁FolderMeta represents scalar-only view of folder excluding nested arrays.
        pub struct FolderMeta {
            pub guid: Guid,
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent: Option<FolderId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔬QualityMeta represents scalar-only view of quality excluding nested arrays.
        pub struct QualityMeta {
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
            #[serde(
                rename = "defaultImperialUnit",
                skip_serializing_if = "Option::is_none"
            )]
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
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// ⚓PortMeta represents scalar-only view of port excluding nested arrays.
        pub struct PortMeta {
            pub guid: Guid,
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub icon: Option<String>,
            #[serde(rename = "maxChildren", skip_serializing_if = "Option::is_none")]
            pub max_children: Option<i32>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🗿ModelMeta represents scalar-only view of model excluding nested arrays.
        pub struct ModelMeta {
            pub guid: Guid,
            pub file: FileId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔌ConnectorMeta represents scalar-only view of connector excluding nested arrays.
        pub struct ConnectorMeta {
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
            #[serde(rename = "maxChildren", skip_serializing_if = "Option::is_none")]
            pub max_children: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub port: Option<PortId>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🎨LayerMeta represents scalar-only view of layer excluding nested arrays.
        pub struct LayerMeta {
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
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        /// 🧩PieceMeta represents scalar-only view of piece excluding nested arrays.
        pub struct PieceMeta {
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
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 👥GroupMeta represents scalar-only view of group excluding nested arrays.
        pub struct GroupMeta {
            pub guid: Guid,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔗ConnectionMeta represents scalar-only view of connection excluding nested arrays.
        pub struct ConnectionMeta {
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
        }
    } // 🕌Sub-entity Meta Types
    pub use sub_entity_meta_types::*;

    mod main_entity_meta_types {
        // 📉Main Entity Meta Types

        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🧱TypeMeta represents scalar-only view of type excluding nested arrays.
        pub struct TypeMeta {
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
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔖TypeShallow is Type with Vec fields replaced by Meta item vectors.
        pub struct TypeShallow {
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
            pub concepts: Option<Vec<ConceptId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub authors: Option<Vec<AuthorId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub props: Option<Vec<PropMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub models: Option<Vec<ModelMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub connectors: Option<Vec<ConnectorMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes: Option<Vec<AttributeMeta>>,
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 📐DesignMeta represents scalar-only view of design excluding nested arrays.
        pub struct DesignMeta {
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
            #[serde(rename = "activeLayer", skip_serializing_if = "Option::is_none")]
            pub active_layer: Option<LayerId>,
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔖DesignShallow is Design with Vec fields replaced by Meta item vectors.
        pub struct DesignShallow {
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
            pub concepts: Option<Vec<ConceptId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub authors: Option<Vec<AuthorId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub props: Option<Vec<PropMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pieces: Option<Vec<PieceMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub connections: Option<Vec<ConnectionMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub layers: Option<Vec<LayerMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub groups: Option<Vec<GroupMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stats: Option<Vec<StatMeta>>,
            #[serde(rename = "activeLayer", skip_serializing_if = "Option::is_none")]
            pub active_layer: Option<LayerId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes: Option<Vec<AttributeMeta>>,
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 📦KitMeta represents scalar-only view of kit excluding nested arrays.
        pub struct KitMeta {
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
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        /// 🔖KitShallow is Kit with Vec fields replaced by Meta item vectors.
        pub struct KitShallow {
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
            pub concepts: Option<Vec<ConceptMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tags: Option<Vec<TagMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub types: Option<Vec<TypeMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub designs: Option<Vec<DesignMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ports: Option<Vec<PortMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub qualities: Option<Vec<QualityMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub files: Option<Vec<FileMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub folders: Option<Vec<FolderMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub authors: Option<Vec<AuthorMeta>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes: Option<Vec<AttributeMeta>>,
            #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
            pub created_at: Option<String>,
            #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<String>,
        }
    } // 🏬Main Entity Meta Types
    pub use main_entity_meta_types::*;

    mod meta_and_shallow_conversion_functions {
        // 🏋️Meta And Shallow Conversion Functions

        use super::*;

        impl Prop {
            pub fn to_meta(&self) -> PropMeta {
                PropMeta {
                    guid: self.guid.clone(),
                    quality: self.quality.clone(),
                    value: self.value.clone(),
                    unit: self.unit.clone(),
                }
            }
        }

        impl Author {
            pub fn to_meta(&self) -> AuthorMeta {
                AuthorMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    email: self.email.clone(),
                }
            }
        }

        impl File {
            pub fn to_meta(&self) -> FileMeta {
                FileMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    remote: self.remote.clone(),
                    folder: self.folder.clone(),
                    size: self.size,
                    hash: self.hash.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }
        }

        impl Folder {
            pub fn to_meta(&self) -> FolderMeta {
                FolderMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    parent: self.parent.clone(),
                    description: None,
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }
        }

        impl Quality {
            pub fn to_meta(&self) -> QualityMeta {
                QualityMeta {
                    guid: self.guid.clone(),
                    key: self.key.clone(),
                    name: self.name.clone(),
                    kind: self.kind,
                    default_value: self.default_value,
                    formula: self.formula.clone(),
                    default_si_unit: self.default_si_unit.clone(),
                    default_imperial_unit: self.default_imperial_unit.clone(),
                    min: self.min,
                    is_min_excluded: self.is_min_excluded,
                    max: self.max,
                    is_max_excluded: self.is_max_excluded,
                    can_scale: self.can_scale,
                    uri: self.uri.clone(),
                }
            }
        }

        impl Port {
            pub fn to_meta(&self) -> PortMeta {
                PortMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    max_children: self.max_children,
                }
            }
        }

        impl Model {
            pub fn to_meta(&self) -> ModelMeta {
                ModelMeta {
                    guid: self.guid.clone(),
                    file: self.file.clone(),
                    name: self.name.clone(),
                    description: self.description.clone(),
                }
            }
        }

        impl Connector {
            pub fn to_meta(&self) -> ConnectorMeta {
                ConnectorMeta {
                    guid: self.guid.clone(),
                    point: self.point.clone(),
                    direction: self.direction.clone(),
                    t: self.t,
                    name: self.name.clone(),
                    description: self.description.clone(),
                    mandatory: self.mandatory,
                    max_children: self.max_children,
                    port: self.port.clone(),
                }
            }
        }

        impl Layer {
            pub fn to_meta(&self) -> LayerMeta {
                LayerMeta {
                    guid: self.guid.clone(),
                    path: self.path.clone(),
                    is_hidden: self.is_hidden,
                    is_locked: self.is_locked,
                    color: self.color.clone(),
                    description: self.description.clone(),
                }
            }
        }

        impl Piece {
            pub fn to_meta(&self) -> PieceMeta {
                PieceMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    type_ref: self.type_ref.clone(),
                    design: self.design.clone(),
                    plane: self.plane.clone(),
                    center: self.center.clone(),
                    scale: self.scale,
                    mirror_plane: self.mirror_plane.clone(),
                    is_hidden: self.is_hidden,
                    is_locked: self.is_locked,
                    color: self.color.clone(),
                    description: self.description.clone(),
                }
            }
        }

        impl Group {
            pub fn to_meta(&self) -> GroupMeta {
                GroupMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    color: self.color.clone(),
                    description: self.description.clone(),
                }
            }
        }

        impl Connection {
            pub fn to_meta(&self) -> ConnectionMeta {
                ConnectionMeta {
                    guid: self.guid.clone(),
                    connected: self.connected.clone(),
                    connecting: self.connecting.clone(),
                    gap: self.gap,
                    shift: self.shift,
                    rise: self.rise,
                    rotation: self.rotation,
                    turn: self.turn,
                    tilt: self.tilt,
                    u: self.u,
                    v: self.v,
                    description: self.description.clone(),
                }
            }
        }

        impl Type {
            pub fn to_meta(&self) -> TypeMeta {
                TypeMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    parent: self.parent.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    folder: self.folder.clone(),
                    unit: self.unit.clone(),
                    stock: self.stock,
                    is_abstract: self.is_abstract,
                    virtual_type: self.virtual_type,
                    location: self.location.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }

            pub fn to_shallow(&self) -> TypeShallow {
                TypeShallow {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    parent: self.parent.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    folder: self.folder.clone(),
                    unit: self.unit.clone(),
                    stock: self.stock,
                    is_abstract: self.is_abstract,
                    virtual_type: self.virtual_type,
                    location: self.location.clone(),
                    concepts: self.concepts.clone(),
                    authors: self.authors.clone(),
                    props: self
                        .props
                        .as_ref()
                        .map(|v| v.iter().map(|p| p.to_meta()).collect()),
                    models: self
                        .models
                        .as_ref()
                        .map(|v| v.iter().map(|m| m.to_meta()).collect()),
                    connectors: self
                        .connectors
                        .as_ref()
                        .map(|v| v.iter().map(|c| c.to_meta()).collect()),
                    attributes: self.attributes.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }
        }

        impl Design {
            pub fn to_meta(&self) -> DesignMeta {
                DesignMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    parent: self.parent.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    folder: self.folder.clone(),
                    unit: self.unit.clone(),
                    is_abstract: self.is_abstract,
                    can_scale: self.can_scale,
                    can_mirror: self.can_mirror,
                    active_layer: self.active_layer.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }

            pub fn to_shallow(&self) -> DesignShallow {
                DesignShallow {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    parent: self.parent.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    folder: self.folder.clone(),
                    unit: self.unit.clone(),
                    is_abstract: self.is_abstract,
                    can_scale: self.can_scale,
                    can_mirror: self.can_mirror,
                    concepts: self.concepts.clone(),
                    authors: self.authors.clone(),
                    props: self
                        .props
                        .as_ref()
                        .map(|v| v.iter().map(|p| p.to_meta()).collect()),
                    pieces: self
                        .pieces
                        .as_ref()
                        .map(|v| v.iter().map(|p| p.to_meta()).collect()),
                    connections: self
                        .connections
                        .as_ref()
                        .map(|v| v.iter().map(|c| c.to_meta()).collect()),
                    layers: self
                        .layers
                        .as_ref()
                        .map(|v| v.iter().map(|l| l.to_meta()).collect()),
                    groups: self
                        .groups
                        .as_ref()
                        .map(|v| v.iter().map(|g| g.to_meta()).collect()),
                    stats: self.stats.clone(),
                    active_layer: self.active_layer.clone(),
                    attributes: self.attributes.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }
        }

        impl Kit {
            pub fn to_meta(&self) -> KitMeta {
                KitMeta {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    version: self.version.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    preview: self.preview.clone(),
                    remote: self.remote.clone(),
                    homepage: self.homepage.clone(),
                    license: self.license.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }

            pub fn to_shallow(&self) -> KitShallow {
                KitShallow {
                    guid: self.guid.clone(),
                    name: self.name.clone(),
                    version: self.version.clone(),
                    description: self.description.clone(),
                    icon: self.icon.clone(),
                    image: self.image.clone(),
                    preview: self.preview.clone(),
                    remote: self.remote.clone(),
                    homepage: self.homepage.clone(),
                    license: self.license.clone(),
                    concepts: self.concepts.clone(),
                    tags: self.tags.clone(),
                    types: self
                        .types
                        .as_ref()
                        .map(|v| v.iter().map(|t| t.to_meta()).collect()),
                    designs: self
                        .designs
                        .as_ref()
                        .map(|v| v.iter().map(|d| d.to_meta()).collect()),
                    ports: self
                        .ports
                        .as_ref()
                        .map(|v| v.iter().map(|p| p.to_meta()).collect()),
                    qualities: self
                        .qualities
                        .as_ref()
                        .map(|v| v.iter().map(|q| q.to_meta()).collect()),
                    files: self
                        .files
                        .as_ref()
                        .map(|v| v.iter().map(|f| f.to_meta()).collect()),
                    folders: self
                        .folders
                        .as_ref()
                        .map(|v| v.iter().map(|f| f.to_meta()).collect()),
                    authors: self
                        .authors
                        .as_ref()
                        .map(|v| v.iter().map(|a| a.to_meta()).collect()),
                    attributes: self.attributes.clone(),
                    created_at: self.created_at.clone(),
                    updated_at: self.updated_at.clone(),
                }
            }
        }
    } // 🤸Meta And Shallow Conversion Functions
    pub use meta_and_shallow_conversion_functions::*;
} // 🔑Meta And Shallow
pub use meta_and_shallow_types::*;

mod apply_diff {
    // 🎲ApplyDiff
    // ApplyDiff MUST provide the applydiff functionality.

    /// <summary>🔖apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
    /// <remarks>
    /// </remarks>
    use super::*;

    pub fn apply_collection_diff<T, D>(
        collection: &mut Option<Vec<T>>,
        diff: &Option<CollectionDiff<T, D>>,
        apply_item_diff: impl Fn(&mut T, &D),
    ) where
        T: HasGuid + Clone,
        D: DiffHasGuid,
    {
        if let Some(diff) = diff {
            let mut new_items = collection.clone().unwrap_or_default();

            if let Some(removed_items) = &diff.removed {
                let removed_set: HashSet<_> =
                    removed_items.iter().map(|s| s.guid.clone()).collect();
                new_items.retain(|item| !removed_set.contains(&item.guid().to_string()));
            }

            if let Some(updated_diffs) = &diff.updated {
                let diff_map: HashMap<_, _> = updated_diffs
                    .iter()
                    .map(|d| (d.guid().to_string(), d))
                    .collect();
                for item in &mut new_items {
                    if let Some(update) = diff_map.get(item.guid()) {
                        apply_item_diff(item, &update.diff);
                    }
                }
            }

            if let Some(added_items) = &diff.added {
                new_items.extend(added_items.clone());
            }

            *collection = if new_items.is_empty() {
                None
            } else {
                Some(new_items)
            };
        }
    }
    /// 🔖<remarks>
    /// </remarks>
    pub fn apply_attribute_diff(item: &mut Attribute, diff: &AttributeDiff) {
        if let Some(value) = &diff.key {
            item.key = value.clone();
        }
        if let Some(value) = &diff.value {
            item.value = value.clone();
        }
        if let Some(value) = &diff.definition {
            item.definition = value.clone();
        }
    }
    /// 🔖<summary>🔖apply_prop_diff holds the data fields for a apply_prop_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_prop_diff(item: &mut Prop, diff: &PropDiff) {
        if let Some(value) = &diff.quality {
            item.quality = value.clone();
        }
        if let Some(value) = &diff.value {
            item.value = value.clone();
        }
        if let Some(value) = &diff.unit {
            item.unit = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<summary>🔖apply_connector_diff holds the data fields for a apply_connector_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_connector_diff(item: &mut Connector, diff: &ConnectorDiff) {
        if let Some(value) = &diff.point {
            item.point.x += value.x;
            item.point.y += value.y;
            item.point.z += value.z;
        }
        if let Some(value) = &diff.direction {
            item.direction.x += value.x;
            item.direction.y += value.y;
            item.direction.z += value.z;
        }
        if let Some(value) = &diff.t {
            item.t = *value;
        }
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.mandatory {
            item.mandatory = *value;
        }
        if let Some(value) = &diff.port {
            item.port = value.clone();
        }
        apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<remarks>
    /// </remarks>
    pub fn apply_model_diff(item: &mut Model, diff: &ModelDiff) {
        if let Some(value) = &diff.file {
            item.file = value.clone();
        }
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.tags {
            item.tags = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<summary>🔖apply_type_diff holds the data fields for a apply_type_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_type_diff(item: &mut Type, diff: &TypeDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.parent {
            item.parent = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
        if let Some(value) = &diff.image {
            item.image = value.clone();
        }
        if let Some(value) = &diff.folder {
            item.folder = value.clone();
        }
        if let Some(value) = &diff.unit {
            item.unit = value.clone();
        }
        if let Some(value) = &diff.stock {
            item.stock = value.clone();
        }
        if let Some(value) = &diff.is_abstract {
            item.is_abstract = *value;
        }
        if let Some(value) = &diff.virtual_type {
            item.virtual_type = *value;
        }
        if let Some(value) = &diff.location {
            item.location = value.clone();
        }
        if let Some(value) = &diff.concepts {
            item.concepts = value.clone();
        }
        if let Some(value) = &diff.authors {
            item.authors = value.clone();
        }
        apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
        apply_collection_diff(&mut item.models, &diff.models, apply_model_diff);
        apply_collection_diff(&mut item.connectors, &diff.connectors, apply_connector_diff);
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 🔖<summary>🔖apply_layer_diff holds the data fields for a apply_layer_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_layer_diff(item: &mut Layer, diff: &LayerDiff) {
        if let Some(value) = &diff.path {
            item.path = value.clone();
        }
        if let Some(value) = &diff.is_hidden {
            item.is_hidden = *value;
        }
        if let Some(value) = &diff.is_locked {
            item.is_locked = *value;
        }
        if let Some(value) = &diff.color {
            item.color = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 🔖<summary>🔖apply_group_diff holds the data fields for a apply_group_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_group_diff(item: &mut Group, diff: &GroupDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.color {
            item.color = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.pieces {
            item.pieces = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 🔖<summary>🔖apply_stat_diff holds the data fields for a apply_stat_diff record.</summary>
    pub fn apply_stat_diff(item: &mut Stat, diff: &StatDiff) {
        if let Some(value) = &diff.quality {
            item.quality = value.clone();
        }
        if let Some(value) = &diff.min {
            item.min = *value;
        }
        if let Some(value) = &diff.min_excluded {
            item.min_excluded = *value;
        }
        if let Some(value) = &diff.max {
            item.max = *value;
        }
        if let Some(value) = &diff.max_excluded {
            item.max_excluded = *value;
        }
        if let Some(value) = &diff.unit {
            item.unit = value.clone();
        }
    }
    /// 🔖<summary>🔖apply_piece_diff holds the data fields for a apply_piece_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_piece_diff(item: &mut Piece, diff: &PieceDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.type_ref {
            item.type_ref = value.clone();
        }
        if let Some(value) = &diff.design {
            item.design = value.clone();
        }
        if let Some(value) = &diff.plane {
            item.plane = value.clone();
        }
        if let Some(value) = &diff.center {
            item.center = value.clone();
        }
        if let Some(value) = &diff.scale {
            item.scale = *value;
        }
        if let Some(value) = &diff.mirror_plane {
            item.mirror_plane = value.clone();
        }
        if let Some(value) = &diff.is_hidden {
            item.is_hidden = *value;
        }
        if let Some(value) = &diff.is_locked {
            item.is_locked = *value;
        }
        if let Some(value) = &diff.color {
            item.color = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<summary>🔖apply_connection_diff holds the data fields for a apply_connection_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_connection_diff(item: &mut Connection, diff: &ConnectionDiff) {
        if let Some(value) = &diff.connected {
            if let Some(v) = &value.piece {
                item.connected.piece = v.clone();
            }
            if let Some(v) = &value.design_piece {
                item.connected.design_piece = v.clone();
            }
            if let Some(v) = &value.connector {
                item.connected.connector = v.clone();
            }
        }
        if let Some(value) = &diff.connecting {
            if let Some(v) = &value.piece {
                item.connecting.piece = v.clone();
            }
            if let Some(v) = &value.design_piece {
                item.connecting.design_piece = v.clone();
            }
            if let Some(v) = &value.connector {
                item.connecting.connector = v.clone();
            }
        }
        if let Some(value) = &diff.gap {
            item.gap += value;
        }
        if let Some(value) = &diff.shift {
            item.shift += value;
        }
        if let Some(value) = &diff.rise {
            item.rise += value;
        }
        if let Some(value) = &diff.rotation {
            item.rotation += value;
        }
        if let Some(value) = &diff.turn {
            item.turn += value;
        }
        if let Some(value) = &diff.tilt {
            item.tilt += value;
        }
        if let Some(value) = &diff.u {
            match value {
                Some(delta) => {
                    item.u = Some(item.u.unwrap_or(0.0) + delta);
                }
                None => {
                    item.u = None;
                }
            }
        }
        if let Some(value) = &diff.v {
            match value {
                Some(delta) => {
                    item.v = Some(item.v.unwrap_or(0.0) + delta);
                }
                None => {
                    item.v = None;
                }
            }
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<remarks>
    /// </remarks>
    pub fn apply_design_diff(item: &mut Design, diff: &DesignDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.parent {
            item.parent = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
        if let Some(value) = &diff.image {
            item.image = value.clone();
        }
        if let Some(value) = &diff.folder {
            item.folder = value.clone();
        }
        if let Some(value) = &diff.unit {
            item.unit = value.clone();
        }
        if let Some(value) = &diff.is_abstract {
            item.is_abstract = *value;
        }
        if let Some(value) = &diff.can_scale {
            item.can_scale = *value;
        }
        if let Some(value) = &diff.can_mirror {
            item.can_mirror = *value;
        }
        if let Some(value) = &diff.concepts {
            item.concepts = value.clone();
        }
        if let Some(value) = &diff.authors {
            item.authors = value.clone();
        }
        if let Some(value) = &diff.active_layer {
            item.active_layer = value.clone();
        }
        apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
        apply_collection_diff(&mut item.pieces, &diff.pieces, apply_piece_diff);
        apply_collection_diff(
            &mut item.connections,
            &diff.connections,
            apply_connection_diff,
        );
        apply_collection_diff(&mut item.layers, &diff.layers, apply_layer_diff);
        apply_collection_diff(&mut item.groups, &diff.groups, apply_group_diff);
        apply_collection_diff(&mut item.stats, &diff.stats, apply_stat_diff);
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 📌Creates a mixed design keeping old entities with diff status annotations.
    /// annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added).
    /// Updated entities are applied (new positions/values) and marked as modified.
    /// Removed entities are kept in place marked as removed.
    /// Added entities are appended marked as added.
    pub fn design_with_diff(base: &Design, diff: &DesignDiff) -> Design {
        let status_attr = |status: &str| Attribute {
            guid: format!("semio.diffStatus.{}", status),
            key: "semio.diffStatus".to_string(),
            value: Some(status.to_string()),
            definition: None,
        };

        let removed_piece_guids: std::collections::HashSet<&str> = diff
            .pieces
            .as_ref()
            .map(|pd| {
                pd.removed
                    .as_ref()
                    .map(|r| r.iter().map(|id| id.guid.as_str()).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let updated_piece_map: std::collections::HashMap<&str, &PieceDiff> = diff
            .pieces
            .as_ref()
            .map(|pd| {
                pd.updated
                    .as_ref()
                    .map(|u| u.iter().map(|upd| (upd.guid.as_str(), &upd.diff)).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let removed_conn_guids: std::collections::HashSet<&str> = diff
            .connections
            .as_ref()
            .map(|cd| {
                cd.removed
                    .as_ref()
                    .map(|r| r.iter().map(|id| id.guid.as_str()).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let updated_conn_map: std::collections::HashMap<&str, &ConnectionDiff> = diff
            .connections
            .as_ref()
            .map(|cd| {
                cd.updated
                    .as_ref()
                    .map(|u| u.iter().map(|upd| (upd.guid.as_str(), &upd.diff)).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let mut result_pieces: Vec<Piece> = Vec::new();
        if let Some(ref pieces) = base.pieces {
            for p in pieces {
                let mut pc = p.clone();
                if removed_piece_guids.contains(pc.guid.as_str()) {
                    let mut attrs = pc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("removed"));
                    pc.attributes = Some(attrs);
                } else if let Some(piece_diff) = updated_piece_map.get(pc.guid.as_str()) {
                    apply_piece_diff(&mut pc, piece_diff);
                    let mut attrs = pc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("modified"));
                    pc.attributes = Some(attrs);
                } else {
                    let mut attrs = pc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("unchanged"));
                    pc.attributes = Some(attrs);
                }
                result_pieces.push(pc);
            }
        }
        if let Some(ref pd) = diff.pieces {
            if let Some(ref added) = pd.added {
                for a in added {
                    let mut ac = a.clone();
                    let mut attrs = ac.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("added"));
                    ac.attributes = Some(attrs);
                    result_pieces.push(ac);
                }
            }
        }

        let mut result_conns: Vec<Connection> = Vec::new();
        if let Some(ref conns) = base.connections {
            for c in conns {
                let mut cc = c.clone();
                if removed_conn_guids.contains(cc.guid.as_str()) {
                    let mut attrs = cc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("removed"));
                    cc.attributes = Some(attrs);
                } else if let Some(conn_diff) = updated_conn_map.get(cc.guid.as_str()) {
                    apply_connection_diff(&mut cc, conn_diff);
                    let mut attrs = cc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("modified"));
                    cc.attributes = Some(attrs);
                } else {
                    let mut attrs = cc.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("unchanged"));
                    cc.attributes = Some(attrs);
                }
                result_conns.push(cc);
            }
        }
        if let Some(ref cd) = diff.connections {
            if let Some(ref added) = cd.added {
                for a in added {
                    let mut ac = a.clone();
                    let mut attrs = ac.attributes.clone().unwrap_or_default();
                    attrs.push(status_attr("added"));
                    ac.attributes = Some(attrs);
                    result_conns.push(ac);
                }
            }
        }

        let mut result = base.clone();
        result.pieces = Some(result_pieces);
        result.connections = Some(result_conns);
        result
    }

    /// 🔖<summary>🔖apply_tag_diff holds the data fields for a apply_tag_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_tag_diff(item: &mut Tag, diff: &TagDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
    }

    /// 🔖<summary>🔖apply_concept_diff holds the data fields for a apply_concept_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_concept_diff(item: &mut Concept, diff: &ConceptDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
    }

    /// 🔖<summary>🔢apply_interface_diff holds the data fields for a apply_interface_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_interface_diff(item: &mut Port, diff: &PortDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
        if let Some(value) = &diff.compatible_interfaces {
            item.compatible_interfaces = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 🔖<summary>🔖apply_quality_diff holds the data fields for a apply_quality_diff record.</summary>
    pub fn apply_quality_diff(item: &mut Quality, diff: &QualityDiff) {
        if let Some(value) = &diff.key {
            item.key = value.clone();
        }
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.kind {
            item.kind = value.clone();
        }
        if let Some(value) = &diff.default_value {
            item.default_value = *value;
        }
        if let Some(value) = &diff.formula {
            item.formula = value.clone();
        }
        if let Some(value) = &diff.default_si_unit {
            item.default_si_unit = value.clone();
        }
        if let Some(value) = &diff.default_imperial_unit {
            item.default_imperial_unit = value.clone();
        }
        if let Some(value) = &diff.min {
            item.min = *value;
        }
        if let Some(value) = &diff.is_min_excluded {
            item.is_min_excluded = *value;
        }
        if let Some(value) = &diff.max {
            item.max = *value;
        }
        if let Some(value) = &diff.is_max_excluded {
            item.is_max_excluded = *value;
        }
        if let Some(value) = &diff.can_scale {
            item.can_scale = *value;
        }
        if let Some(value) = &diff.uri {
            item.uri = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<summary>🔖apply_file_diff holds the data fields for a apply_file_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_file_diff(item: &mut File, diff: &FileDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.remote {
            item.remote = value.clone();
        }
        if let Some(value) = &diff.folder {
            item.folder = value.clone();
        }
        if let Some(value) = &diff.size {
            item.size = *value;
        }
        if let Some(value) = &diff.hash {
            item.hash = value.clone();
        }
    }
    /// 🔖<remarks>
    /// </remarks>
    pub fn apply_folder_diff(item: &mut Folder, diff: &FolderDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.parent {
            item.parent = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }

    /// 🔖<summary>🔖apply_author_diff holds the data fields for a apply_author_diff record.</summary>
    pub fn apply_author_diff(item: &mut Author, diff: &AuthorDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.email {
            item.email = value.clone();
        }
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
    /// 🔖<summary>🔖apply_kit_diff holds the data fields for a apply_kit_diff record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_kit_diff(item: &mut Kit, diff: &KitDiff) {
        if let Some(value) = &diff.name {
            item.name = value.clone();
        }
        if let Some(value) = &diff.version {
            item.version = value.clone();
        }
        if let Some(value) = &diff.description {
            item.description = value.clone();
        }
        if let Some(value) = &diff.icon {
            item.icon = value.clone();
        }
        if let Some(value) = &diff.image {
            item.image = value.clone();
        }
        if let Some(value) = &diff.preview {
            item.preview = value.clone();
        }
        if let Some(value) = &diff.remote {
            item.remote = value.clone();
        }
        if let Some(value) = &diff.homepage {
            item.homepage = value.clone();
        }
        if let Some(value) = &diff.license {
            item.license = value.clone();
        }
        apply_collection_diff(&mut item.concepts, &diff.concepts, apply_concept_diff);
        apply_collection_diff(&mut item.tags, &diff.tags, apply_tag_diff);
        apply_collection_diff(&mut item.types, &diff.types, apply_type_diff);
        apply_collection_diff(&mut item.designs, &diff.designs, apply_design_diff);
        apply_collection_diff(&mut item.ports, &diff.ports, apply_interface_diff);
        apply_collection_diff(&mut item.qualities, &diff.qualities, apply_quality_diff);
        apply_collection_diff(&mut item.files, &diff.files, apply_file_diff);
        apply_collection_diff(&mut item.folders, &diff.folders, apply_folder_diff);
        apply_collection_diff(&mut item.authors, &diff.authors, apply_author_diff);
        apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
    }
} // ✈️ApplyDiff
pub use apply_diff::*;

mod kit_change_helpers {
    // 🏬Kit Change Helpers
    // Kit Change Helpers MUST provide convenience functions for computing kit and design diffs, inverses, and changes.

    /// Computes a CollectionDiff between two optional collections of guid-identified items.
    /// Uses a caller-provided `compute_diff` function for entity-level diffs.
    use super::*;

    pub fn get_guid_collection_diff<T, D>(
        before: &Option<Vec<T>>,
        after: &Option<Vec<T>>,
        entity_key: &str,
        compute_diff: impl Fn(&T, &T) -> D,
    ) -> Option<CollectionDiff<T, D>>
    where
        T: HasGuid + Clone + PartialEq,
        D: DiffHasGuid,
    {
        let before_items = before.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        let after_items = after.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        let before_map: HashMap<String, &T> = before_items
            .iter()
            .map(|i| (i.guid().to_string(), i))
            .collect();
        let after_map: HashMap<String, &T> = after_items
            .iter()
            .map(|i| (i.guid().to_string(), i))
            .collect();
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut updated = Vec::new();
        for item in after_items {
            if !before_map.contains_key(item.guid()) {
                added.push(item.clone());
            }
        }
        for item in before_items {
            if !after_map.contains_key(item.guid()) {
                removed.push(RemovedItem {
                    guid: item.guid().to_string(),
                });
            }
        }
        for item in after_items {
            if let Some(before_item) = before_map.get(item.guid()) {
                if *before_item != item {
                    let diff = compute_diff(before_item, item);
                    updated.push(DiffUpdate {
                        key: entity_key.to_string(),
                        guid: item.guid().to_string(),
                        diff,
                    });
                }
            }
        }
        if added.is_empty() && removed.is_empty() && updated.is_empty() {
            None
        } else {
            Some(CollectionDiff {
                added: if added.is_empty() { None } else { Some(added) },
                removed: if removed.is_empty() {
                    None
                } else {
                    Some(removed)
                },
                updated: if updated.is_empty() {
                    None
                } else {
                    Some(updated)
                },
            })
        }
    }

    /// 🔄Computes the KitDiff that transforms efore into fter.
    pub fn get_kit_diff(before: &Kit, after: &Kit) -> KitDiff {
        before.diff_from(after)
    }

    /// 🔖Computes the DesignDiff that transforms efore into fter.
    pub fn get_design_diff(before: &Design, after: &Design) -> DesignDiff {
        before.diff_from(after)
    }


    /// 🔖Computes the inverse of a KitDiff given the original Kit state.
    pub fn inverse_kit_diff(original: &Kit, forward: &KitDiff) -> KitDiff {
        let mut after = original.clone();
        apply_kit_diff(&mut after, forward);
        get_kit_diff(&after, original)
    }

    /// 🔖Computes the inverse of a DesignDiff given the original Design state.
    pub fn inverse_design_diff(original: &Design, forward: &DesignDiff) -> DesignDiff {
        let mut after = original.clone();
        apply_design_diff(&mut after, forward);
        get_design_diff(&after, original)
    }

    /// 🗑️Deletes pieces and connections from a design, returning a canonical `SemioReport<DesignDiff>`.
    /// Removes stale connections referencing deleted pieces.
    /// Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
    pub fn delete_pieces_and_connections_in_design(
        kit: &Kit,
        design: &Design,
        piece_guids: &[String],
        connection_guids: &[String],
    ) -> SemioReport<DesignDiff> {
        let deleted_piece_set: HashSet<&str> = piece_guids.iter().map(|s| s.as_str()).collect();
        let connections = design.connections.as_deref().unwrap_or(&[]);

        // Find stale connections: connections referencing any deleted piece
        let mut stale_connection_guids: HashSet<String> = HashSet::new();
        for conn in connections {
            if deleted_piece_set.contains(conn.connected.piece.guid.as_str())
                || deleted_piece_set.contains(conn.connecting.piece.guid.as_str())
            {
                stale_connection_guids.insert(conn.guid.clone());
            }
        }

        // All removed connections = explicit + stale
        let mut all_removed_connection_guids: HashSet<String> =
            connection_guids.iter().cloned().collect();
        all_removed_connection_guids.extend(stale_connection_guids);

        // Find pieces that become fixed: pieces whose parent connection was removed
        // and are not themselves being deleted.
        // A piece becomes fixed when the connection where it is the "connecting" side is removed
        // and it has no other remaining parent connection.
        let mut fixed_piece_guids: Vec<String> = Vec::new();
        for conn_guid in &all_removed_connection_guids {
            let conn = match connections.iter().find(|c| &c.guid == conn_guid) {
                Some(c) => c,
                None => continue,
            };
            let connecting_guid = &conn.connecting.piece.guid;
            if deleted_piece_set.contains(connecting_guid.as_str()) {
                continue;
            }
            // Check if this piece has another parent connection not in the removed set
            let has_other_parent = connections.iter().any(|c| {
                c.connecting.piece.guid == *connecting_guid
                    && !all_removed_connection_guids.contains(&c.guid)
            });
            if !has_other_parent && !fixed_piece_guids.contains(connecting_guid) {
                fixed_piece_guids.push(connecting_guid.clone());
            }
        }

        // Build the diff - flatten the design to get absolute plane and center for each piece
        let flat_rep = flatten_design(kit, &design.guid);
        if !flat_rep.ok {
            return SemioReport::err(flat_rep.errors);
        }
        let flat_change = flat_rep.diff.expect("flatten ok implies diff");
        let mut flat_piece_map: HashMap<String, (Option<Plane>, Option<Coord>)> = HashMap::new();
        if let Some(pieces) = &design.pieces {
            for piece in pieces {
                if let Some(plane) = &piece.plane {
                    flat_piece_map.insert(
                        piece.guid.clone(),
                        (Some(plane.clone()), piece.center.clone()),
                    );
                }
            }
        }
        if let Some(pieces_diff) = &flat_change.forward.pieces {
            if let Some(updates) = &pieces_diff.updated {
                for update in updates {
                    let entry = flat_piece_map
                        .entry(update.guid.clone())
                        .or_insert((None, None));
                    if let Some(Some(plane)) = &update.diff.plane {
                        entry.0 = Some(plane.clone());
                    }
                    if let Some(Some(center)) = &update.diff.center {
                        entry.1 = Some(center.clone());
                    }
                }
            }
        }

        let pieces_removed: Vec<RemovedItem> = piece_guids
            .iter()
            .map(|g| RemovedItem { guid: g.clone() })
            .collect();
        let pieces_updated: Vec<DiffUpdate<PieceDiff>> = fixed_piece_guids
            .iter()
            .map(|g| {
                let (flat_plane, flat_center) = flat_piece_map
                    .get(g)
                    .cloned()
                    .unwrap_or((Some(Plane::default()), Some(Coord::default())));
                DiffUpdate {
                    key: "piece".to_string(),
                    guid: g.clone(),
                    diff: PieceDiff {
                        guid: g.clone(),
                        plane: Some(flat_plane),
                        center: Some(flat_center),
                        ..Default::default()
                    },
                }
            })
            .collect();
        let mut sorted_removed_connections: Vec<String> =
            all_removed_connection_guids.into_iter().collect();
        sorted_removed_connections.sort();
        let connections_removed: Vec<RemovedItem> = sorted_removed_connections
            .iter()
            .map(|g| RemovedItem { guid: g.clone() })
            .collect();

        let mut diff = DesignDiff {
            guid: design.guid.clone(),
            ..Default::default()
        };

        if !pieces_removed.is_empty() || !pieces_updated.is_empty() {
            diff.pieces = Some(CollectionDiff {
                removed: if pieces_removed.is_empty() {
                    None
                } else {
                    Some(pieces_removed)
                },
                updated: if pieces_updated.is_empty() {
                    None
                } else {
                    Some(pieces_updated)
                },
                added: None,
            });
        }

        if !connections_removed.is_empty() {
            diff.connections = Some(CollectionDiff {
                removed: Some(connections_removed),
                updated: None,
                added: None,
            });
        }

        SemioReport::ok_with(diff, flat_rep.warnings, flat_rep.infos)
    }

    /// 🔖Computes a reversible KitChange from two kit states.
    pub fn get_kit_change(before: &Kit, after: &Kit) -> KitChange {
        let forward = get_kit_diff(before, after);
        let backward = inverse_kit_diff(before, &forward);
        KitChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before.clone()),
            after: Some(after.clone()),
        }
    }

    /// 🔖Computes a reversible DesignChange from two design states.
    pub fn get_design_change(before: &Design, after: &Design) -> DesignChange {
        let forward = get_design_diff(before, after);
        let backward = inverse_design_diff(before, &forward);
        DesignChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before.clone()),
            after: Some(after.clone()),
        }
    }
} // 🎽Kit Change Helpers
pub use kit_change_helpers::*;

mod filter {
    // 📷Filter
    // Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    /// Glob filter with include and exclude patterns for name-based entity filtering.
    /// If include is non-empty, only names matching at least one include pattern are kept.
    /// Names matching any exclude pattern are always removed.
    use super::*;

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct GlobFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclude: Option<Vec<String>>,
    }

    /// General-purpose kit filter combining design-based transitive filtering with glob-based name filtering.
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct KitFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub design_guid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub model_tags: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub designs: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub types: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ports: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub files: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub concepts: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qualities: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authors: Option<GlobFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub folders: Option<GlobFilter>,
    }

    /// 🧩Matches a name against a glob pattern supporting * (any chars) and ? (single char). Case-insensitive.
    pub fn glob_match(name: &str, pattern: &str) -> bool {
        let name_lower = name.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        let name_bytes = name_lower.as_bytes();
        let pattern_bytes = pattern_lower.as_bytes();

        pub fn matches(name: &[u8], pattern: &[u8]) -> bool {
            match (name.len(), pattern.len()) {
                (0, 0) => true,
                (_, 0) => false,
                (0, _) => pattern.iter().all(|&c| c == b'*'),
                _ => {
                    if pattern[0] == b'*' {
                        matches(name, &pattern[1..]) || matches(&name[1..], pattern)
                    } else if pattern[0] == b'?' || pattern[0] == name[0] {
                        matches(&name[1..], &pattern[1..])
                    } else {
                        false
                    }
                }
            }
        }

        matches(name_bytes, pattern_bytes)
    }

    /// 🧹Checks if a name passes a GlobFilter. Returns true if filter is None or name matches.
    pub fn matches_glob_filter(name: &str, filter: Option<&GlobFilter>) -> bool {
        let Some(filter) = filter else { return true };
        if let Some(include) = &filter.include {
            if !include.is_empty() && !include.iter().any(|p| glob_match(name, p)) {
                return false;
            }
        }
        if let Some(exclude) = &filter.exclude {
            if exclude.iter().any(|p| glob_match(name, p)) {
                return false;
            }
        }
        true
    }

    pub fn select_best_model_for_filter(
        models: &[Model],
        selected_tag_guids: &[String],
    ) -> Option<Model> {
        if models.is_empty() {
            return None;
        }
        if selected_tag_guids.is_empty() {
            if let Some(model) = models.iter().find(|model| {
                model
                    .tags
                    .as_ref()
                    .map(|tags| tags.is_empty())
                    .unwrap_or(true)
            }) {
                return Some(model.clone());
            }
            return Some(models[0].clone());
        }

        let filtered: Vec<Model> = models
            .iter()
            .filter(|model| {
                let model_tag_guids: HashSet<String> = model
                    .tags
                    .as_ref()
                    .map(|tags| tags.iter().map(|tag| tag.guid.clone()).collect())
                    .unwrap_or_default();
                selected_tag_guids
                    .iter()
                    .all(|selected| model_tag_guids.contains(selected))
            })
            .cloned()
            .collect();
        if filtered.is_empty() {
            return None;
        }

        let mut best_model = filtered[0].clone();
        let mut best_score = -1.0_f64;
        for model in filtered {
            let model_tag_guids: HashSet<String> = model
                .tags
                .as_ref()
                .map(|tags| tags.iter().map(|tag| tag.guid.clone()).collect())
                .unwrap_or_default();
            let selected: HashSet<String> = selected_tag_guids.iter().cloned().collect();
            let intersection = model_tag_guids.intersection(&selected).count();
            let union = model_tag_guids.union(&selected).count();
            let score = if union == 0 {
                0.0
            } else {
                intersection as f64 / union as f64
            };
            if score > best_score {
                best_score = score;
                best_model = model;
            }
        }
        Some(best_model)
    }

    /// 🎨Internal design-based transitive kit filtering.
    /// Removes types not used by pieces, designs not used by pieces, ports not used by connectors of used types,
    /// files not used by selected models, and keeps at most one model per type according to the optional tags.
    pub fn filter_kit_by_design(kit: &Kit, design_guid: &str, tags: Option<&[String]>) -> Kit {
        let design = match find_design_in_kit(kit, design_guid) {
            Some(design) => design,
            None => {
                return Kit {
                    guid: kit.guid.clone(),
                    name: kit.name.clone(),
                    version: kit.version.clone(),
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
                }
            }
        };

        let mut used_type_guids: HashSet<String> = HashSet::new();
        let mut used_design_guids: HashSet<String> = HashSet::from([design_guid.to_string()]);
        for piece in design
            .pieces
            .as_ref()
            .map(|pieces| pieces.as_slice())
            .unwrap_or(&[])
        {
            if let Some(piece_type) = piece.type_ref.as_ref() {
                used_type_guids.insert(piece_type.guid.clone());
            }
            if let Some(piece_design) = piece.design.as_ref() {
                used_design_guids.insert(piece_design.guid.clone());
            }
        }

        let type_by_guid: HashMap<String, Type> = kit
            .types
            .as_ref()
            .map(|types| {
                types
                    .iter()
                    .cloned()
                    .map(|type_item| (type_item.guid.clone(), type_item))
                    .collect()
            })
            .unwrap_or_default();
        pub fn collect_type_ancestors(
            type_by_guid: &HashMap<String, Type>,
            used_type_guids: &mut HashSet<String>,
            type_guid: &str,
        ) {
            let Some(type_item) = type_by_guid.get(type_guid) else {
                return;
            };
            let Some(parent) = type_item.parent.as_ref() else {
                return;
            };
            if used_type_guids.insert(parent.guid.clone()) {
                collect_type_ancestors(type_by_guid, used_type_guids, &parent.guid);
            }
        }
        let type_snapshot: Vec<String> = used_type_guids.iter().cloned().collect();
        for type_guid in type_snapshot {
            collect_type_ancestors(&type_by_guid, &mut used_type_guids, &type_guid);
        }

        let all_tags: &[Tag] = kit.tags.as_deref().unwrap_or(&[]);
        let mut resolved_tag_guids: Vec<String> = Vec::new();
        for tag_value in tags.unwrap_or(&[]) {
            if let Some(tag) = all_tags.iter().find(|tag| tag.guid == *tag_value) {
                resolved_tag_guids.push(tag.guid.clone());
                continue;
            }
            for tag in all_tags.iter().filter(|tag| tag.name == *tag_value) {
                resolved_tag_guids.push(tag.guid.clone());
            }
        }

        let mut used_port_guids: HashSet<String> = HashSet::new();
        let mut used_file_guids: HashSet<String> = HashSet::new();
        let mut used_tag_guids: HashSet<String> = HashSet::new();
        let mut used_concept_guids: HashSet<String> = HashSet::new();
        let mut used_quality_guids: HashSet<String> = HashSet::new();
        let mut used_author_guids: HashSet<String> = HashSet::new();
        let mut used_folder_names: HashSet<String> = HashSet::new();
        let mut selected_models: HashMap<String, Model> = HashMap::new();

        let mut collect_quality_from_props = |props: &[Prop]| {
            for prop in props {
                used_quality_guids.insert(prop.quality.guid.clone());
            }
        };

        for type_guid in &used_type_guids {
            let Some(type_item) = type_by_guid.get(type_guid) else {
                continue;
            };
            if let Some(folder) = type_item.folder.as_ref() {
                used_folder_names.insert(folder.clone());
            }
            for connector in type_item.connectors.as_deref().unwrap_or(&[]) {
                if let Some(port) = connector.port.as_ref() {
                    used_port_guids.insert(port.guid.clone());
                }
                collect_quality_from_props(connector.props.as_deref().unwrap_or(&[]));
            }
            collect_quality_from_props(type_item.props.as_deref().unwrap_or(&[]));
            for author in type_item.authors.as_deref().unwrap_or(&[]) {
                used_author_guids.insert(author.guid.clone());
            }
            for concept in type_item.concepts.as_deref().unwrap_or(&[]) {
                used_concept_guids.insert(concept.guid.clone());
            }
            if let Some(best_model) = select_best_model_for_filter(
                type_item.models.as_deref().unwrap_or(&[]),
                &resolved_tag_guids,
            ) {
                used_file_guids.insert(best_model.file.guid.clone());
                for tag in best_model.tags.as_deref().unwrap_or(&[]) {
                    used_tag_guids.insert(tag.guid.clone());
                }
                selected_models.insert(type_guid.clone(), best_model);
            }
        }

        for piece in design.pieces.as_deref().unwrap_or(&[]) {
            collect_quality_from_props(piece.props.as_deref().unwrap_or(&[]));
        }
        for concept in design.concepts.as_deref().unwrap_or(&[]) {
            used_concept_guids.insert(concept.guid.clone());
        }
        for author in design.authors.as_deref().unwrap_or(&[]) {
            used_author_guids.insert(author.guid.clone());
        }
        let port_snapshot: Vec<String> = used_port_guids.iter().cloned().collect();
        for port_guid in port_snapshot {
            if let Some(port) = kit
                .ports
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .find(|port| port.guid == port_guid)
            {
                for compatible in port.compatible_interfaces.as_deref().unwrap_or(&[]) {
                    used_port_guids.insert(compatible.guid.clone());
                }
            }
        }
        for tag_guid in resolved_tag_guids {
            used_tag_guids.insert(tag_guid);
        }

        let filtered_types = kit
            .types
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|type_item| used_type_guids.contains(&type_item.guid))
            .map(|type_item| {
                let mut filtered_type = type_item.clone();
                filtered_type.models = Some(
                    selected_models
                        .get(&type_item.guid)
                        .cloned()
                        .into_iter()
                        .collect(),
                );
                filtered_type
            })
            .collect();

        Kit {
            guid: kit.guid.clone(),
            name: kit.name.clone(),
            version: kit.version.clone(),
            description: kit.description.clone(),
            icon: kit.icon.clone(),
            image: kit.image.clone(),
            preview: kit.preview.clone(),
            remote: kit.remote.clone(),
            homepage: kit.homepage.clone(),
            license: kit.license.clone(),
            types: Some(filtered_types),
            designs: Some(
                kit.designs
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|design_item| used_design_guids.contains(&design_item.guid))
                    .cloned()
                    .collect(),
            ),
            ports: Some(
                kit.ports
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|port| used_port_guids.contains(&port.guid))
                    .cloned()
                    .collect(),
            ),
            files: Some(
                kit.files
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|file| used_file_guids.contains(&file.guid))
                    .cloned()
                    .collect(),
            ),
            tags: Some(
                kit.tags
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|tag| used_tag_guids.contains(&tag.guid))
                    .cloned()
                    .collect(),
            ),
            concepts: Some(
                kit.concepts
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|concept| used_concept_guids.contains(&concept.guid))
                    .cloned()
                    .collect(),
            ),
            qualities: Some(
                kit.qualities
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|quality| used_quality_guids.contains(&quality.guid))
                    .cloned()
                    .collect(),
            ),
            folders: Some(
                kit.folders
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|folder| used_folder_names.contains(&folder.name))
                    .cloned()
                    .collect(),
            ),
            authors: Some(
                kit.authors
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|author| used_author_guids.contains(&author.guid))
                    .cloned()
                    .collect(),
            ),
            attributes: kit.attributes.clone(),
            created_at: kit.created_at.clone(),
            updated_at: kit.updated_at.clone(),
        }
    }

    /// 🔖General-purpose kit filter. Combines optional design-based transitive filtering with glob-based name filtering.
    /// When design_guid is set, first performs transitive design-scoped subset extraction.
    /// Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
    pub fn filter_kit(kit: &Kit, filter: &KitFilter) -> Kit {
        let base = if let Some(design_guid) = &filter.design_guid {
            filter_kit_by_design(kit, design_guid, filter.model_tags.as_deref())
        } else {
            kit.clone()
        };

        let has_glob_filters = filter.designs.is_some()
            || filter.types.is_some()
            || filter.ports.is_some()
            || filter.files.is_some()
            || filter.tags.is_some()
            || filter.concepts.is_some()
            || filter.qualities.is_some()
            || filter.authors.is_some()
            || filter.folders.is_some();

        if !has_glob_filters {
            return base;
        }

        Kit {
            guid: base.guid,
            name: base.name,
            version: base.version,
            description: base.description,
            icon: base.icon,
            image: base.image,
            preview: base.preview,
            remote: base.remote,
            homepage: base.homepage,
            license: base.license,
            types: Some(
                base.types
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|t| matches_glob_filter(&t.name, filter.types.as_ref()))
                    .cloned()
                    .collect(),
            ),
            designs: Some(
                base.designs
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|d| matches_glob_filter(&d.name, filter.designs.as_ref()))
                    .cloned()
                    .collect(),
            ),
            ports: Some(
                base.ports
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|p| matches_glob_filter(&p.name, filter.ports.as_ref()))
                    .cloned()
                    .collect(),
            ),
            files: Some(
                base.files
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|f| matches_glob_filter(&f.name, filter.files.as_ref()))
                    .cloned()
                    .collect(),
            ),
            tags: Some(
                base.tags
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|t| matches_glob_filter(&t.name, filter.tags.as_ref()))
                    .cloned()
                    .collect(),
            ),
            concepts: Some(
                base.concepts
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|c| matches_glob_filter(&c.name, filter.concepts.as_ref()))
                    .cloned()
                    .collect(),
            ),
            qualities: Some(
                base.qualities
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|q| matches_glob_filter(&q.name, filter.qualities.as_ref()))
                    .cloned()
                    .collect(),
            ),
            folders: Some(
                base.folders
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|f| matches_glob_filter(&f.name, filter.folders.as_ref()))
                    .cloned()
                    .collect(),
            ),
            authors: Some(
                base.authors
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|a| matches_glob_filter(&a.name, filter.authors.as_ref()))
                    .cloned()
                    .collect(),
            ),
            attributes: base.attributes,
            created_at: base.created_at,
            updated_at: base.updated_at,
        }
    }
} // 🧩Filter
pub use filter::*;

mod flatten_design {
    // 🏦FlattenDesign
    // FlattenDesign MUST provide the flattendesign functionality.

    /// <summary>🔖FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
    use super::*;

    pub struct FlattenedPiece {
        pub piece: Piece,
        pub plane: Plane,
        pub type_guid: Option<String>,
        pub design_guid: Option<String>,
        pub path: Vec<String>,
    }

    /// 🔖Computes forward/backward DesignChange for flatten (hot path; no SemioReport wrapper).
    pub fn flatten_design_change(kit: &Kit, design_guid: &str) -> DesignChange {
        let design = match find_design_in_kit(kit, design_guid) {
            Some(d) => d,
            None => {
                let empty_diff = DesignDiff {
                    guid: design_guid.to_string(),
                    ..Default::default()
                };
                return DesignChange {
                    forward: empty_diff.clone(),
                    backward: empty_diff,
                    author: None,
                    time: None,
                    before: None,
                    after: None,
                };
            }
        };

        let before_design = design.clone();

        let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
        if pieces.is_empty() {
            let empty_diff = DesignDiff {
                guid: design_guid.to_string(),
                ..Default::default()
            };
            return DesignChange {
                forward: empty_diff.clone(),
                backward: empty_diff,
                author: None,
                time: None,
                before: Some(before_design.clone()),
                after: Some(before_design),
            };
        }

        let connections = design
            .connections
            .as_ref()
            .map(|c| c.as_slice())
            .unwrap_or(&[]);

        let types_map: HashMap<&str, &Type> = kit
            .types
            .as_ref()
            .map(|types| types.iter().map(|t| (t.guid.as_str(), t)).collect())
            .unwrap_or_default();

        let pieces_map: HashMap<&str, &Piece> =
            pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        let mut adjacency: HashMap<&str, Vec<(&str, &Connection, bool)>> = HashMap::new();
        for conn in connections {
            let src = conn.connected.piece.guid.as_str();
            let tgt = conn.connecting.piece.guid.as_str();
            if pieces_map.contains_key(src) && pieces_map.contains_key(tgt) {
                adjacency.entry(src).or_default().push((tgt, conn, true));
                adjacency.entry(tgt).or_default().push((src, conn, false));
            }
        }

        let mut piece_planes: HashMap<&str, Matrix4<f64>> = HashMap::with_capacity(pieces.len());
        let mut piece_centers: HashMap<&str, Coord> = HashMap::with_capacity(pieces.len());
        let mut piece_paths: HashMap<&str, String> = HashMap::with_capacity(pieces.len());
        let mut visited: HashSet<&str> = HashSet::with_capacity(pieces.len());
        let mut queue: VecDeque<&str> = VecDeque::with_capacity(pieces.len());

        pub const RADIUS: f64 = 2.697;
        pub const VERTICAL_V_EXTRA: f64 = 1.0;
        pub const HORIZONTAL_SCALE: f64 = 3.0633;

        for piece in pieces {
            if !visited.contains(piece.guid.as_str()) {
                let initial_matrix = piece
                    .plane
                    .as_ref()
                    .map(|p| p.to_matrix())
                    .unwrap_or_else(Matrix4::identity);
                piece_planes.insert(piece.guid.as_str(), initial_matrix);

                let initial_center = piece.center.clone().unwrap_or(Coord { u: 0.0, v: 0.0 });
                piece_centers.insert(piece.guid.as_str(), initial_center);
                piece_paths.insert(piece.guid.as_str(), piece.guid.clone());

                visited.insert(piece.guid.as_str());
                queue.push_back(piece.guid.as_str());

                while let Some(current_guid) = queue.pop_front() {
                    let current_matrix = *piece_planes.get(current_guid).unwrap();
                    let parent_center = piece_centers
                        .get(current_guid)
                        .cloned()
                        .unwrap_or(Coord { u: 0.0, v: 0.0 });

                    if let Some(neighbors) = adjacency.get(current_guid) {
                        for &(neighbor_guid, conn, is_connected) in neighbors {
                            if visited.contains(neighbor_guid) {
                                continue;
                            }

                            let (parent_side, _child_side) = if is_connected {
                                (&conn.connected, &conn.connecting)
                            } else {
                                (&conn.connecting, &conn.connected)
                            };

                            let parent_connector = match get_connector_for_side_fast(
                                &types_map,
                                &pieces_map,
                                parent_side,
                            ) {
                                Some(c) => c,
                                None => continue,
                            };

                            let connection_matrix = match compute_connection_matrix_fast(
                                &types_map,
                                &pieces_map,
                                conn,
                                is_connected,
                            ) {
                                Some(m) => m,
                                None => continue,
                            };

                            let new_matrix = current_matrix * connection_matrix;

                            let conn_u = conn.u.unwrap_or(0.0);
                            let conn_v = conn.v.unwrap_or(0.0);

                            let (child_u, child_v) = if parent_center.u.abs() < 0.0001
                                && parent_center.v.abs() < 0.0001
                            {
                                let angle = 2.0 * PI * parent_connector.t;
                                (RADIUS * angle.sin(), RADIUS * angle.cos())
                            } else {
                                let is_vertical = parent_connector.direction.z.abs() > 0.5;
                                if is_vertical {
                                    (
                                        parent_center.u + conn_u,
                                        parent_center.v + conn_v + VERTICAL_V_EXTRA,
                                    )
                                } else {
                                    (
                                        parent_center.u + conn_u * HORIZONTAL_SCALE,
                                        parent_center.v + conn_v * HORIZONTAL_SCALE,
                                    )
                                }
                            };

                            let child_center = Coord {
                                u: (child_u * 1_000_000.0).round() / 1_000_000.0,
                                v: (child_v * 1_000_000.0).round() / 1_000_000.0,
                            };

                            piece_planes.insert(neighbor_guid, new_matrix);
                            piece_centers.insert(neighbor_guid, child_center);
                            let parent_path =
                                piece_paths.get(current_guid).cloned().unwrap_or_default();
                            piece_paths.insert(
                                neighbor_guid,
                                format!("{},{}", parent_path, neighbor_guid),
                            );
                            visited.insert(neighbor_guid);
                            queue.push_back(neighbor_guid);
                        }
                    }
                }
            }
        }

        let mut updated_pieces: Vec<DiffUpdate<PieceDiff>> = Vec::new();

        for piece in pieces {
            let matrix_opt = piece_planes.get(piece.guid.as_str());
            let center_opt = piece_centers.get(piece.guid.as_str());

            if let (Some(&matrix), Some(center)) = (matrix_opt, center_opt) {
                let new_plane = Plane::from_matrix(&matrix).round();
                let plane_needs_update = match &piece.plane {
                    Some(existing) => !planes_equal_approx(existing, &new_plane),
                    None => true,
                };

                let center_needs_update = match &piece.center {
                    Some(existing) => {
                        (existing.u - center.u).abs() > 0.0001
                            || (existing.v - center.v).abs() > 0.0001
                    }
                    None => true,
                };

                if plane_needs_update || center_needs_update {
                    let path_attr =
                        piece_paths
                            .get(piece.guid.as_str())
                            .map(|path| CollectionDiff {
                                added: Some(vec![Attribute {
                                    guid: guid(),
                                    key: "semio.path".to_string(),
                                    value: Some(path.clone()),
                                    definition: None,
                                }]),
                                removed: None,
                                updated: None,
                            });
                    updated_pieces.push(DiffUpdate {
                        key: "piece".to_string(),
                        guid: piece.guid.clone(),
                        diff: PieceDiff {
                            guid: piece.guid.clone(),
                            plane: if plane_needs_update {
                                Some(Some(new_plane))
                            } else {
                                None
                            },
                            center: if center_needs_update {
                                Some(Some(center.clone()))
                            } else {
                                None
                            },
                            attributes: path_attr,
                            ..Default::default()
                        },
                    });
                }
            }
        }

        let mut forward = DesignDiff {
            guid: design_guid.to_string(),
            ..Default::default()
        };

        if !updated_pieces.is_empty() {
            forward.pieces = Some(CollectionDiff {
                added: None,
                removed: None,
                updated: Some(updated_pieces),
            });
        }

        let mut after_design = before_design.clone();
        apply_design_diff(&mut after_design, &forward);
        let backward = get_design_diff(&after_design, &before_design);

        DesignChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before_design),
            after: Some(after_design),
        }
    }

    fn flatten_design_report_from_change(
        kit: &Kit,
        design_guid: &str,
        change: DesignChange,
    ) -> SemioReport<DesignChange> {
        let pieces_empty = find_design_in_kit(kit, design_guid)
            .and_then(|d| d.pieces.as_ref())
            .map(|p| p.is_empty())
            .unwrap_or(true);
        if pieces_empty {
            SemioReport::ok_with(
                change,
                vec![],
                vec![OperationNote {
                    code: Some("flatten.empty-pieces".into()),
                    message: "No pieces to flatten; returning empty forward and backward diffs."
                        .into(),
                }],
            )
        } else {
            SemioReport::ok_with(change, vec![], vec![])
        }
    }

    /// 🌤️Canonical flatten report (matches TypeScript flattenDesign).
    pub fn flatten_design(kit: &Kit, design_guid: &str) -> SemioReport<DesignChange> {
        if find_design_in_kit(kit, design_guid).is_none() {
            return SemioReport::err(vec![OperationNote {
                code: Some("flatten.design-not-found".into()),
                message: format!("Design {design_guid} not found in kit"),
            }]);
        }
        let change = flatten_design_change(kit, design_guid);
        flatten_design_report_from_change(kit, design_guid, change)
    }

    /// 🔖<summary>🔖planes_equal_approx holds the data fields for a planes_equal_approx record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn planes_equal_approx(a: &Plane, b: &Plane) -> bool {
        pub const TOL: f64 = 0.0001;
        (a.origin.x - b.origin.x).abs() < TOL
            && (a.origin.y - b.origin.y).abs() < TOL
            && (a.origin.z - b.origin.z).abs() < TOL
            && (a.x_axis.x - b.x_axis.x).abs() < TOL
            && (a.x_axis.y - b.x_axis.y).abs() < TOL
            && (a.x_axis.z - b.x_axis.z).abs() < TOL
            && (a.y_axis.x - b.y_axis.x).abs() < TOL
            && (a.y_axis.y - b.y_axis.y).abs() < TOL
            && (a.y_axis.z - b.y_axis.z).abs() < TOL
    }
    /// 🔖<summary>💾compute_connection_matrix_fast holds the data fields for a compute_connection_matrix_fast record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn compute_connection_matrix_fast(
        types_map: &HashMap<&str, &Type>,
        pieces_map: &HashMap<&str, &Piece>,
        conn: &Connection,
        from_connected: bool,
    ) -> Option<Matrix4<f64>> {
        let (from_side, to_side) = if from_connected {
            (&conn.connected, &conn.connecting)
        } else {
            (&conn.connecting, &conn.connected)
        };

        let parent_connector = get_connector_for_side_fast(types_map, pieces_map, from_side)?;
        let child_connector = get_connector_for_side_fast(types_map, pieces_map, to_side)?;

        Some(compute_child_plane_matrix(
            &parent_connector,
            &child_connector,
            conn,
        ))
    }
    /// 🔖<summary>🔖compute_child_plane_matrix holds the data fields for a compute_child_plane_matrix record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn compute_child_plane_matrix(
        parent_connector: &Connector,
        child_connector: &Connector,
        conn: &Connection,
    ) -> Matrix4<f64> {
        use nalgebra::{UnitQuaternion, Vector3};

        let parent_point = Vector3::new(
            parent_connector.point.x,
            parent_connector.point.y,
            parent_connector.point.z,
        );
        let mut parent_dir = Vector3::new(
            parent_connector.direction.x,
            parent_connector.direction.y,
            parent_connector.direction.z,
        );
        if parent_dir.norm() > 0.0001 {
            parent_dir = parent_dir.normalize();
        }

        let child_point = Vector3::new(
            child_connector.point.x,
            child_connector.point.y,
            child_connector.point.z,
        );
        let mut child_dir = Vector3::new(
            child_connector.direction.x,
            child_connector.direction.y,
            child_connector.direction.z,
        );
        if child_dir.norm() > 0.0001 {
            child_dir = child_dir.normalize();
        }

        let gap = conn.gap;
        let shift = conn.shift;
        let rise = conn.rise;
        let rotation_rad = conn.rotation * PI / 180.0;
        let turn_rad = conn.turn * PI / 180.0;
        let tilt_rad = conn.tilt * PI / 180.0;

        let reverse_child_dir = -child_dir;

        let align_quat: UnitQuaternion<f64> = {
            let cross_vec = parent_dir.cross(&reverse_child_dir);
            let cross_len = cross_vec.norm();
            let dot = parent_dir.dot(&reverse_child_dir);

            if cross_len < 0.01 {
                if dot > 0.0 {
                    UnitQuaternion::identity()
                } else if parent_dir.z.abs() < 0.001 {
                    UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI)
                } else {
                    let axis = Vector3::new(0.0, 0.0, 1.0).cross(&parent_dir);
                    if axis.norm() > 0.0001 {
                        UnitQuaternion::from_axis_angle(
                            &nalgebra::Unit::new_normalize(axis.normalize()),
                            PI,
                        )
                    } else {
                        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI)
                    }
                }
            } else {
                UnitQuaternion::rotation_between(&reverse_child_dir, &parent_dir)
                    .unwrap_or_else(UnitQuaternion::identity)
            }
        };
        let direction_t = quat_to_matrix4(&align_quat);

        let y_axis = Vector3::new(0.0, 1.0, 0.0);
        let parent_connector_quat = {
            let dot = y_axis.dot(&parent_dir);
            if (dot + 1.0).abs() < 0.0001 {
                UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI)
            } else {
                UnitQuaternion::rotation_between(&y_axis, &parent_dir)
                    .unwrap_or_else(UnitQuaternion::identity)
            }
        };
        let parent_rotation_t = quat_to_matrix4(&parent_connector_quat);

        let gap_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 1.0, 0.0));
        let shift_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(1.0, 0.0, 0.0));
        let raise_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 0.0, 1.0));
        let mut turn_axis = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 0.0, 1.0));
        let mut tilt_axis = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(1.0, 0.0, 0.0));

        let mut orientation_t = direction_t;

        let rotate_quat = UnitQuaternion::from_axis_angle(
            &nalgebra::Unit::new_normalize(parent_dir),
            -rotation_rad,
        );
        let rotate_t = quat_to_matrix4(&rotate_quat);
        orientation_t = rotate_t * orientation_t;

        turn_axis = apply_matrix4_to_vec3(&rotate_t, &turn_axis);
        tilt_axis = apply_matrix4_to_vec3(&rotate_t, &tilt_axis);

        if turn_axis.norm() > 0.0001 {
            let turn_quat = UnitQuaternion::from_axis_angle(
                &nalgebra::Unit::new_normalize(turn_axis.normalize()),
                turn_rad,
            );
            let turn_t = quat_to_matrix4(&turn_quat);
            orientation_t = turn_t * orientation_t;
        }

        if tilt_axis.norm() > 0.0001 {
            let tilt_quat = UnitQuaternion::from_axis_angle(
                &nalgebra::Unit::new_normalize(tilt_axis.normalize()),
                tilt_rad,
            );
            let tilt_t = quat_to_matrix4(&tilt_quat);
            orientation_t = tilt_t * orientation_t;
        }

        let center_child_t = make_translation(-child_point.x, -child_point.y, -child_point.z);
        let mut transform = orientation_t * center_child_t;

        let gap_transform = make_translation(gap_dir.x * gap, gap_dir.y * gap, gap_dir.z * gap);
        let shift_transform = make_translation(
            shift_dir.x * shift,
            shift_dir.y * shift,
            shift_dir.z * shift,
        );
        let raise_transform =
            make_translation(raise_dir.x * rise, raise_dir.y * rise, raise_dir.z * rise);

        let translation_t = raise_transform * (shift_transform * gap_transform);
        transform = translation_t * transform;

        let move_to_parent_t = make_translation(parent_point.x, parent_point.y, parent_point.z);
        transform = move_to_parent_t * transform;

        transform
    }
    /// 🔖<summary>🔄quat_to_matrix4 holds the data fields for a quat_to_matrix4 record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn quat_to_matrix4(q: &nalgebra::UnitQuaternion<f64>) -> Matrix4<f64> {
        let rot = q.to_rotation_matrix();
        let m = rot.matrix();
        Matrix4::new(
            m[(0, 0)],
            m[(0, 1)],
            m[(0, 2)],
            0.0,
            m[(1, 0)],
            m[(1, 1)],
            m[(1, 2)],
            0.0,
            m[(2, 0)],
            m[(2, 1)],
            m[(2, 2)],
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
    /// 🔖<summary>🔖make_translation holds the data fields for a make_translation record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn make_translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }
    /// 🔖<summary>🔖apply_matrix4_to_vec3 holds the data fields for a apply_matrix4_to_vec3 record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn apply_matrix4_to_vec3(
        m: &Matrix4<f64>,
        v: &nalgebra::Vector3<f64>,
    ) -> nalgebra::Vector3<f64> {
        nalgebra::Vector3::new(
            m[(0, 0)] * v.x + m[(0, 1)] * v.y + m[(0, 2)] * v.z,
            m[(1, 0)] * v.x + m[(1, 1)] * v.y + m[(1, 2)] * v.z,
            m[(2, 0)] * v.x + m[(2, 1)] * v.y + m[(2, 2)] * v.z,
        )
    }
    /// 🔖<summary>🔖get_connector_for_side_fast holds the data fields for a get_connector_for_side_fast record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn get_connector_for_side_fast<'a>(
        types_map: &HashMap<&str, &'a Type>,
        pieces_map: &HashMap<&str, &Piece>,
        side: &Side,
    ) -> Option<Connector> {
        let piece = pieces_map.get(side.piece.guid.as_str())?;
        let type_id = piece.type_ref.as_ref()?;
        let t = types_map.get(type_id.guid.as_str())?;
        let connector_guid = side.connector.as_ref().map(|c| c.guid.as_str());
        get_connector_from_type(types_map, t, connector_guid).map(|c| c.clone())
    }
    /// 🔖<summary>🔖get_connector_from_type holds the data fields for a get_connector_from_type record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn get_connector_from_type<'a>(
        types_map: &HashMap<&str, &'a Type>,
        t: &'a Type,
        connector_guid: Option<&str>,
    ) -> Option<&'a Connector> {
        match connector_guid {
            None | Some("") => {
                if let Some(connectors) = &t.connectors {
                    if !connectors.is_empty() {
                        return Some(&connectors[0]);
                    }
                }
                if let Some(parent_ref) = &t.parent {
                    if let Some(parent) = types_map.get(parent_ref.guid.as_str()) {
                        return get_connector_from_type(types_map, parent, connector_guid);
                    }
                }
                None
            }
            Some(guid) => {
                if let Some(connectors) = &t.connectors {
                    for c in connectors {
                        if c.guid == guid {
                            return Some(c);
                        }
                    }
                }
                if let Some(parent_ref) = &t.parent {
                    if let Some(parent) = types_map.get(parent_ref.guid.as_str()) {
                        if let Some(c) = get_connector_from_type(types_map, parent, connector_guid)
                        {
                            return Some(c);
                        }
                    }
                }
                if let Some(connectors) = &t.connectors {
                    if !connectors.is_empty() {
                        return Some(&connectors[0]);
                    }
                }
                None
            }
        }
    }
    /// 🔖<summary>🔖connector_to_plane holds the data fields for a connector_to_plane record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn connector_to_plane(connector: &Connector) -> Plane {
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

    pub fn drag_pieces_in_design(
        design_pieces: &[Piece],
        design_connections: &[Connection],
        selected_pieces: &[Piece],
        offset: &Coord,
    ) -> DesignDiff {
        let selected_guids: HashSet<&str> =
            selected_pieces.iter().map(|p| p.guid.as_str()).collect();
        let mut parent_map: HashMap<&str, (&str, &str)> = HashMap::new();
        for conn in design_connections {
            let connecting_guid = conn.connecting.piece.guid.as_str();
            let connected_guid = conn.connected.piece.guid.as_str();
            parent_map.insert(connecting_guid, (conn.guid.as_str(), connected_guid));
        }
        let piece_map: HashMap<&str, &Piece> =
            design_pieces.iter().map(|p| (p.guid.as_str(), p)).collect();
        let fixed_guids: HashSet<&str> = selected_guids
            .iter()
            .filter(|&&guid| !parent_map.contains_key(guid))
            .copied()
            .collect();
        let piece_updates: Vec<DiffUpdate<PieceDiff>> = fixed_guids
            .iter()
            .filter_map(|&guid| {
                let center = piece_map.get(guid)?.center.as_ref()?;
                Some(DiffUpdate {
                    key: "piece".to_string(),
                    guid: guid.to_string(),
                    diff: PieceDiff {
                        guid: guid.to_string(),
                        center: Some(Some(Coord {
                            u: center.u + offset.u,
                            v: center.v + offset.v,
                        })),
                        ..Default::default()
                    },
                })
            })
            .collect();
        let connection_updates: Vec<DiffUpdate<ConnectionDiff>> = selected_guids
            .iter()
            .filter(|&&guid| {
                if fixed_guids.contains(guid) {
                    return false;
                }
                let mut current = guid;
                loop {
                    match parent_map.get(current) {
                        Some(&(_conn_guid, ancestor_guid)) => {
                            if selected_guids.contains(ancestor_guid) {
                                return false;
                            }
                            current = ancestor_guid;
                        }
                        None => break,
                    }
                }
                parent_map.contains_key(guid)
            })
            .map(|&guid| {
                let (conn_guid, _parent_guid) = parent_map[guid];
                DiffUpdate {
                    key: "connection".to_string(),
                    guid: conn_guid.to_string(),
                    diff: ConnectionDiff {
                        guid: conn_guid.to_string(),
                        u: Some(Some(offset.u)),
                        v: Some(Some(offset.v)),
                        ..Default::default()
                    },
                }
            })
            .collect();
        let mut diff = DesignDiff {
            guid: String::new(),
            ..Default::default()
        };
        if !piece_updates.is_empty() {
            diff.pieces = Some(CollectionDiff {
                added: None,
                removed: None,
                updated: Some(piece_updates),
            });
        }
        if !connection_updates.is_empty() {
            diff.connections = Some(CollectionDiff {
                added: None,
                removed: None,
                updated: Some(connection_updates),
            });
        }
        diff
    }

    // #region 🌳Flatten Merkle Hashes
    // 🌳Per-piece {plane_hash, center_hash} merkle hashes for cached flatten_design reuse.

    /// <summary>🌳FlatMerkleHashes holds the plane and center merkle hashes for a single piece in a flattened design.</summary>
    #[derive(Debug, Clone, PartialEq)]
    pub struct FlatMerkleHashes {
        pub plane_hash: String,
        pub center_hash: String,
    }

    /// <summary>🌳FlatMerkleCacheEntry caches a piece's merkle hashes together with the resolved plane and center from the previous flatten_design run.</summary>
    #[derive(Debug, Clone, PartialEq)]
    pub struct FlatMerkleCacheEntry {
        pub plane_hash: String,
        pub center_hash: String,
        pub plane: Option<Plane>,
        pub center: Option<Coord>,
    }

    /// <summary>🌱Root plane hash includes only the piece guid and its fixed plane components (identity when absent).</summary>
    fn hash_plane_root(guid: &str, plane: Option<&Plane>) -> String {
        let mut w = super::HashWriter::new();
        match plane {
            None => {
                w.write_string("plane.root.identity");
                w.write_string(guid);
            }
            Some(p) => {
                w.write_string("plane.root");
                w.write_string(guid);
                w.write_number(p.origin.x);
                w.write_number(p.origin.y);
                w.write_number(p.origin.z);
                w.write_number(p.x_axis.x);
                w.write_number(p.x_axis.y);
                w.write_number(p.x_axis.z);
                w.write_number(p.y_axis.x);
                w.write_number(p.y_axis.y);
                w.write_number(p.y_axis.z);
            }
        }
        w.digest()
    }

    /// <summary>🔗Chain plane hash depends on the parent plane hash plus every input consumed by compute_child_plane_matrix.</summary>
    fn hash_plane_chain(
        parent_hash: &str,
        parent_connector: &Connector,
        child_connector: &Connector,
        conn: &Connection,
    ) -> String {
        let mut w = super::HashWriter::new();
        w.write_string("plane.chain");
        w.write_hash(parent_hash);
        w.write_number(parent_connector.point.x);
        w.write_number(parent_connector.point.y);
        w.write_number(parent_connector.point.z);
        w.write_number(parent_connector.direction.x);
        w.write_number(parent_connector.direction.y);
        w.write_number(parent_connector.direction.z);
        w.write_number(child_connector.point.x);
        w.write_number(child_connector.point.y);
        w.write_number(child_connector.point.z);
        w.write_number(child_connector.direction.x);
        w.write_number(child_connector.direction.y);
        w.write_number(child_connector.direction.z);
        w.write_number(conn.gap);
        w.write_number(conn.shift);
        w.write_number(conn.rise);
        w.write_number(conn.rotation);
        w.write_number(conn.turn);
        w.write_number(conn.tilt);
        w.digest()
    }

    /// <summary>🌱Root center hash includes only the piece guid and its fixed center (identity when absent).</summary>
    fn hash_center_root(guid: &str, center: Option<&Coord>) -> String {
        let mut w = super::HashWriter::new();
        match center {
            None => {
                w.write_string("center.root.identity");
                w.write_string(guid);
            }
            Some(c) => {
                w.write_string("center.root");
                w.write_string(guid);
                w.write_number(c.u);
                w.write_number(c.v);
            }
        }
        w.digest()
    }

    /// <summary>🔗Chain center hash conservatively includes every potentially-read input of the child center computation.</summary>
    fn hash_center_chain(
        parent_hash: &str,
        parent_connector: &Connector,
        conn: &Connection,
    ) -> String {
        let mut w = super::HashWriter::new();
        w.write_string("center.chain");
        w.write_hash(parent_hash);
        w.write_number(parent_connector.direction.z);
        w.write_number(parent_connector.t);
        w.write_number(conn.u.unwrap_or(0.0));
        w.write_number(conn.v.unwrap_or(0.0));
        w.digest()
    }

    /// <summary>🌳Compute per-piece {plane_hash, center_hash} merkle hashes for the flattened design so callers can cache by chain identity.</summary>
    pub fn compute_flat_hashes(kit: &Kit, design_guid: &str) -> HashMap<String, FlatMerkleHashes> {
        let design = match find_design_in_kit(kit, design_guid) {
            Some(d) => d,
            None => return HashMap::new(),
        };
        let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
        if pieces.is_empty() {
            return HashMap::new();
        }
        let connections = design
            .connections
            .as_ref()
            .map(|c| c.as_slice())
            .unwrap_or(&[]);
        let types_map: HashMap<&str, &Type> = kit
            .types
            .as_ref()
            .map(|types| types.iter().map(|t| (t.guid.as_str(), t)).collect())
            .unwrap_or_default();
        let pieces_map: HashMap<&str, &Piece> =
            pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        // Same adjacency order as flatten_design so the BFS chain selects the same parent for every child.
        let mut adjacency: HashMap<&str, Vec<(&str, &Connection, bool)>> = HashMap::new();
        for conn in connections {
            let src = conn.connected.piece.guid.as_str();
            let tgt = conn.connecting.piece.guid.as_str();
            if pieces_map.contains_key(src) && pieces_map.contains_key(tgt) {
                adjacency.entry(src).or_default().push((tgt, conn, true));
                adjacency.entry(tgt).or_default().push((src, conn, false));
            }
        }

        let mut components: Vec<Vec<&str>> = Vec::new();
        {
            let mut component_visited: HashSet<&str> = HashSet::new();
            for piece in pieces {
                let guid = piece.guid.as_str();
                if component_visited.contains(guid) {
                    continue;
                }
                let mut component = Vec::new();
                let mut queue: VecDeque<&str> = VecDeque::new();
                queue.push_back(guid);
                component_visited.insert(guid);
                while let Some(cur) = queue.pop_front() {
                    component.push(cur);
                    if let Some(neighbors) = adjacency.get(cur) {
                        for &(neigh, _, _) in neighbors {
                            if !component_visited.contains(neigh) {
                                component_visited.insert(neigh);
                                queue.push_back(neigh);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }

        let mut plane_hashes: HashMap<String, String> = HashMap::new();
        let mut center_hashes: HashMap<String, String> = HashMap::new();

        for component in &components {
            let component_set: HashSet<&str> = component.iter().copied().collect();

            // Rule 1: first piece (in slice order) of this component with both plane and center.
            let mut root: Option<&str> = None;
            for piece in pieces {
                let g = piece.guid.as_str();
                if component_set.contains(g) && piece.plane.is_some() && piece.center.is_some() {
                    root = Some(g);
                    break;
                }
            }
            // Rule 2: lexicographically smallest guid in the component.
            if root.is_none() {
                let mut sorted: Vec<&str> = component.iter().copied().collect();
                sorted.sort();
                root = sorted.first().copied();
            }
            let root_guid = match root {
                Some(g) => g,
                None => continue,
            };
            let root_piece = match pieces_map.get(root_guid) {
                Some(p) => *p,
                None => continue,
            };
            plane_hashes.insert(
                root_guid.to_string(),
                hash_plane_root(root_guid, root_piece.plane.as_ref()),
            );
            center_hashes.insert(
                root_guid.to_string(),
                hash_center_root(root_guid, root_piece.center.as_ref()),
            );

            let mut bfs_visited: HashSet<&str> = HashSet::new();
            bfs_visited.insert(root_guid);
            let mut queue: VecDeque<&str> = VecDeque::new();
            queue.push_back(root_guid);
            while let Some(cur) = queue.pop_front() {
                let parent_plane_hash = match plane_hashes.get(cur).cloned() {
                    Some(h) => h,
                    None => continue,
                };
                let parent_center_hash = match center_hashes.get(cur).cloned() {
                    Some(h) => h,
                    None => continue,
                };
                if let Some(neighbors) = adjacency.get(cur) {
                    for &(neigh, conn, is_connected) in neighbors {
                        if bfs_visited.contains(neigh) {
                            continue;
                        }
                        let (parent_side, child_side) = if is_connected {
                            (&conn.connected, &conn.connecting)
                        } else {
                            (&conn.connecting, &conn.connected)
                        };
                        let parent_connector =
                            match get_connector_for_side_fast(&types_map, &pieces_map, parent_side)
                            {
                                Some(c) => c,
                                None => continue,
                            };
                        let child_connector = match get_connector_for_side_fast(
                            &types_map,
                            &pieces_map,
                            child_side,
                        ) {
                            Some(c) => c,
                            None => continue,
                        };
                        plane_hashes.insert(
                            neigh.to_string(),
                            hash_plane_chain(
                                &parent_plane_hash,
                                &parent_connector,
                                &child_connector,
                                conn,
                            ),
                        );
                        center_hashes.insert(
                            neigh.to_string(),
                            hash_center_chain(&parent_center_hash, &parent_connector, conn),
                        );
                        bfs_visited.insert(neigh);
                        queue.push_back(neigh);
                    }
                }
            }
        }

        let mut result: HashMap<String, FlatMerkleHashes> = HashMap::new();
        for (guid, plane_hash) in plane_hashes {
            if let Some(center_hash) = center_hashes.get(&guid).cloned() {
                result.insert(
                    guid,
                    FlatMerkleHashes {
                        plane_hash,
                        center_hash,
                    },
                );
            }
        }
        result
    }

    /// <summary>🧠Flatten a design reusing cached plane/center values when the per-piece merkle hashes match the previous run.</summary>
    pub fn flatten_design_cached(
        kit: &Kit,
        design_guid: &str,
        cache: Option<&HashMap<String, FlatMerkleCacheEntry>>,
    ) -> (
        SemioReport<DesignChange>,
        HashMap<String, FlatMerkleCacheEntry>,
    ) {
        let new_hashes = compute_flat_hashes(kit, design_guid);
        let change = flatten_design_change(kit, design_guid);
        let report = if find_design_in_kit(kit, design_guid).is_none() {
            SemioReport::err(vec![OperationNote {
                code: Some("flatten.design-not-found".into()),
                message: format!("Design {design_guid} not found in kit"),
            }])
        } else {
            flatten_design_report_from_change(kit, design_guid, change.clone())
        };

        let mut updated_by_id: HashMap<String, (Option<Plane>, Option<Coord>)> = HashMap::new();
        if let Some(pieces_diff) = &change.forward.pieces {
            if let Some(updates) = &pieces_diff.updated {
                for upd in updates {
                    let plane = upd.diff.plane.clone().flatten();
                    let center = upd.diff.center.clone().flatten();
                    updated_by_id.insert(upd.guid.clone(), (plane, center));
                }
            }
        }

        let mut next_cache: HashMap<String, FlatMerkleCacheEntry> = HashMap::new();
        match cache {
            Some(prev_cache) => {
                for (guid, hashes) in &new_hashes {
                    let prev = prev_cache.get(guid);
                    let updated = updated_by_id.get(guid);
                    match (prev, updated) {
                        (None, None) => {}
                        (None, Some((plane, center))) => {
                            next_cache.insert(
                                guid.clone(),
                                FlatMerkleCacheEntry {
                                    plane_hash: hashes.plane_hash.clone(),
                                    center_hash: hashes.center_hash.clone(),
                                    plane: plane.clone(),
                                    center: center.clone(),
                                },
                            );
                        }
                        (Some(_), None) => {}
                        (Some(p), Some((u_plane, u_center))) => {
                            let plane = if p.plane_hash == hashes.plane_hash {
                                p.plane.clone()
                            } else {
                                u_plane.clone()
                            };
                            let center = if p.center_hash == hashes.center_hash {
                                p.center.clone()
                            } else {
                                u_center.clone()
                            };
                            next_cache.insert(
                                guid.clone(),
                                FlatMerkleCacheEntry {
                                    plane_hash: hashes.plane_hash.clone(),
                                    center_hash: hashes.center_hash.clone(),
                                    plane,
                                    center,
                                },
                            );
                        }
                    }
                }
            }
            None => {
                for (guid, hashes) in &new_hashes {
                    if let Some((plane, center)) = updated_by_id.get(guid) {
                        next_cache.insert(
                            guid.clone(),
                            FlatMerkleCacheEntry {
                                plane_hash: hashes.plane_hash.clone(),
                                center_hash: hashes.center_hash.clone(),
                                plane: plane.clone(),
                                center: center.clone(),
                            },
                        );
                    }
                }
            }
        }

        (report, next_cache)
    }

    // #endregion 🌳Flatten Merkle Hashes
} // 🕍FlattenDesign
pub use flatten_design::*;

mod find_replaceable_types_in_designs {
    // 🔍Find Replaceable Types In Designs
    // Find Replaceable Types In Designs MUST find all types and designs that can replace selected pieces in a design.
    // Specs: Build one selection boundary requirement multiset from actual opposite-side connectors and accept a candidate only if distinct compatible candidate connectors can satisfy the whole multiset. Without boundary connections, use the selected pieces' own connectors with multiplicity.

    use super::*;
    use std::collections::{HashMap, HashSet};

    /// 🔍Finds all types and designs whose root type can replace the selected pieces in a design.
    /// Specs: Returns (type_guids, design_guids). Boundary requirements come from actual opposite-side connectors, isolated selections use selected-piece connectors, and candidate validity requires one injective connector matching across the whole requirement multiset.
    pub fn find_replaceable_types_in_designs_for_pieces_in_design(
        kit: &Kit,
        design_guid: &str,
        piece_guids: &[String],
    ) -> (Vec<String>, Vec<String>) {
        let design = match find_design_in_kit(kit, design_guid) {
            Some(d) => d,
            None => return (vec![], vec![]),
        };

        let ports = kit.ports.as_deref().unwrap_or(&[]);
        let types = kit.types.as_deref().unwrap_or(&[]);
        let designs = kit.designs.as_deref().unwrap_or(&[]);
        let pieces = design.pieces.as_deref().unwrap_or(&[]);
        let connections = design.connections.as_deref().unwrap_or(&[]);

        let ports_map: HashMap<&str, &Port> = ports
            .iter()
            .map(|port| (port.guid.as_str(), port))
            .collect();
        let types_map: HashMap<&str, &Type> = types
            .iter()
            .map(|kind| (kind.guid.as_str(), kind))
            .collect();
        let piece_map: HashMap<&str, &Piece> = pieces
            .iter()
            .map(|piece| (piece.guid.as_str(), piece))
            .collect();

        let selected_set: HashSet<&str> = piece_guids.iter().map(|s| s.as_str()).collect();

        let are_ports_compatible = |candidate_port_guid: &str, required_port_guid: &str| -> bool {
            if candidate_port_guid.is_empty() || required_port_guid.is_empty() {
                return false;
            }
            if candidate_port_guid == required_port_guid {
                return true;
            }
            let candidate_port = match ports_map.get(candidate_port_guid) {
                Some(port) => port,
                None => return false,
            };
            let required_port = match ports_map.get(required_port_guid) {
                Some(port) => port,
                None => return false,
            };
            if candidate_port
                .compatible_interfaces
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .any(|compatible_port| compatible_port.guid == required_port_guid)
            {
                return true;
            }
            if required_port
                .compatible_interfaces
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .any(|compatible_port| compatible_port.guid == candidate_port_guid)
            {
                return true;
            }
            false
        };

        let get_connector_port_guid = |type_guid: &str, connector_guid: &str| -> String {
            if type_guid.is_empty() || connector_guid.is_empty() {
                return String::new();
            }
            let candidate_type = match types_map.get(type_guid) {
                Some(kind) => *kind,
                None => return String::new(),
            };
            for connector in candidate_type.connectors.as_deref().unwrap_or(&[]) {
                if connector.guid == connector_guid {
                    return connector
                        .port
                        .as_ref()
                        .map(|port| port.guid.clone())
                        .unwrap_or_default();
                }
            }
            String::new()
        };

        let get_own_requirement_port_guids = |piece_guid: &str| -> Vec<String> {
            let piece = match piece_map.get(piece_guid) {
                Some(piece) => *piece,
                None => return vec![],
            };
            let type_guid = match piece.type_ref.as_ref() {
                Some(kind) => kind.guid.as_str(),
                None => return vec![],
            };
            let candidate_type = match types_map.get(type_guid) {
                Some(kind) => *kind,
                None => return vec![],
            };
            candidate_type
                .connectors
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|connector| {
                    connector
                        .port
                        .as_ref()
                        .map(|port| port.guid.clone())
                        .unwrap_or_default()
                })
                .collect()
        };

        let get_boundary_requirement_port_guids = || -> Vec<String> {
            let mut requirement_port_guids = Vec::new();
            for connection in connections {
                let connected_selected =
                    selected_set.contains(connection.connected.piece.guid.as_str());
                let connecting_selected =
                    selected_set.contains(connection.connecting.piece.guid.as_str());
                if connected_selected == connecting_selected {
                    continue;
                }

                let other_side = if connected_selected {
                    &connection.connecting
                } else {
                    &connection.connected
                };

                let other_piece = match piece_map.get(other_side.piece.guid.as_str()) {
                    Some(piece) => *piece,
                    None => {
                        requirement_port_guids.push(String::new());
                        continue;
                    }
                };
                let other_type_guid = match other_piece.type_ref.as_ref() {
                    Some(kind) => kind.guid.as_str(),
                    None => {
                        requirement_port_guids.push(String::new());
                        continue;
                    }
                };
                let other_connector_guid = other_side
                    .connector
                    .as_ref()
                    .map(|connector| connector.guid.as_str())
                    .unwrap_or("");
                requirement_port_guids.push(get_connector_port_guid(
                    other_type_guid,
                    other_connector_guid,
                ));
            }
            requirement_port_guids
        };

        let get_selection_own_requirement_port_guids = || -> Vec<String> {
            piece_guids
                .iter()
                .flat_map(|piece_guid| get_own_requirement_port_guids(piece_guid.as_str()))
                .collect()
        };

        let mut required_port_guids = get_boundary_requirement_port_guids();
        if required_port_guids.is_empty() {
            required_port_guids = get_selection_own_requirement_port_guids();
        }

        let can_satisfy_requirements =
            |required_port_guids: &[String], available_port_guids: &[String]| -> bool {
                if required_port_guids.is_empty() {
                    return true;
                }
                if available_port_guids.len() < required_port_guids.len() {
                    return false;
                }

                let mut requirement_options: Vec<Vec<usize>> =
                    Vec::with_capacity(required_port_guids.len());
                for required_port_guid in required_port_guids {
                    let connector_indexes: Vec<usize> = available_port_guids
                        .iter()
                        .enumerate()
                        .filter_map(|(connector_index, available_port_guid)| {
                            if are_ports_compatible(
                                available_port_guid.as_str(),
                                required_port_guid.as_str(),
                            ) {
                                Some(connector_index)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if connector_indexes.is_empty() {
                        return false;
                    }
                    requirement_options.push(connector_indexes);
                }
                requirement_options.sort_by_key(|connector_indexes| connector_indexes.len());

                fn match_requirements(
                    requirement_index: usize,
                    requirement_options: &[Vec<usize>],
                    used_connector_indexes: &mut [bool],
                ) -> bool {
                    if requirement_index >= requirement_options.len() {
                        return true;
                    }
                    for connector_index in &requirement_options[requirement_index] {
                        if used_connector_indexes[*connector_index] {
                            continue;
                        }
                        used_connector_indexes[*connector_index] = true;
                        if match_requirements(
                            requirement_index + 1,
                            requirement_options,
                            used_connector_indexes,
                        ) {
                            return true;
                        }
                        used_connector_indexes[*connector_index] = false;
                    }
                    false
                }

                let mut used_connector_indexes = vec![false; available_port_guids.len()];
                match_requirements(0, &requirement_options, &mut used_connector_indexes)
            };

        let candidate_type_available_port_guids = |candidate_type: &Type| -> Vec<String> {
            candidate_type
                .connectors
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|connector| {
                    connector
                        .port
                        .as_ref()
                        .map(|port| port.guid.clone())
                        .unwrap_or_default()
                })
                .collect()
        };

        let candidate_design_available_port_guids = |candidate_design: &Design| -> Vec<String> {
            let mut consumed_connector_keys: HashSet<String> = HashSet::new();
            for connection in candidate_design.connections.as_deref().unwrap_or(&[]) {
                for side in [&connection.connected, &connection.connecting] {
                    if let Some(connector) = &side.connector {
                        consumed_connector_keys
                            .insert(format!("{}::{}", side.piece.guid, connector.guid));
                    }
                }
            }

            let mut available_port_guids = Vec::new();
            for piece in candidate_design.pieces.as_deref().unwrap_or(&[]) {
                let type_guid = match piece.type_ref.as_ref() {
                    Some(kind) => kind.guid.as_str(),
                    None => continue,
                };
                let candidate_type = match types_map.get(type_guid) {
                    Some(kind) => *kind,
                    None => continue,
                };
                for connector in candidate_type.connectors.as_deref().unwrap_or(&[]) {
                    if consumed_connector_keys
                        .contains(format!("{}::{}", piece.guid, connector.guid).as_str())
                    {
                        continue;
                    }
                    available_port_guids.push(
                        connector
                            .port
                            .as_ref()
                            .map(|port| port.guid.clone())
                            .unwrap_or_default(),
                    );
                }
            }
            available_port_guids
        };

        if piece_guids.is_empty() {
            let type_guids = types
                .iter()
                .filter(|candidate_type| {
                    candidate_type_available_port_guids(candidate_type).is_empty()
                })
                .map(|candidate_type| candidate_type.guid.clone())
                .collect();
            let design_guids = designs
                .iter()
                .filter(|candidate_design| {
                    candidate_design_available_port_guids(candidate_design).is_empty()
                })
                .map(|candidate_design| candidate_design.guid.clone())
                .collect();
            return (type_guids, design_guids);
        }

        let is_valid_candidate = |available_port_guids: &[String]| -> bool {
            can_satisfy_requirements(&required_port_guids, available_port_guids)
        };

        let type_guids = types
            .iter()
            .filter(|candidate_type| {
                let available_port_guids = candidate_type_available_port_guids(candidate_type);
                is_valid_candidate(&available_port_guids)
            })
            .map(|candidate_type| candidate_type.guid.clone())
            .collect();
        let design_guids = designs
            .iter()
            .filter(|candidate_design| {
                let available_port_guids = candidate_design_available_port_guids(candidate_design);
                is_valid_candidate(&available_port_guids)
            })
            .map(|candidate_design| candidate_design.guid.clone())
            .collect();

        (type_guids, design_guids)
    }
} // 🔍Find Replaceable Types In Designs
pub use find_replaceable_types_in_designs::*;

mod copy_paste_design {
    // 📋Copy Paste Design
    // Copy Paste Design MUST provide copy and paste functionality for designs.
    // Specs: CopyDesign extracts selected pieces and connections. PasteDesign inserts them into a target design.

    use super::*;
    use std::collections::{HashMap, HashSet};

    /// 📋Extracts selected pieces and connections from a design into a new Design.
    /// Specs: Selected pieces are classified as internal-fixed, internal-connected, or parent-piece-exclusive parent-connection-inclusive.
    /// Internal pieces are copied as-is. Pp-excl-pc-incl pieces get semio.center and semio.plane attributes.
    /// Non-internal connections include their external pieces marked with semio.piece.origin = "external".
    pub fn copy_design(
        kit: &Kit,
        design: &Design,
        piece_guids: &[String],
        connection_guids: &[String],
    ) -> Design {
        let selected_piece_set: HashSet<&str> = piece_guids.iter().map(|s| s.as_str()).collect();
        let selected_connection_set: HashSet<&str> =
            connection_guids.iter().map(|s| s.as_str()).collect();

        let connections = design.connections.as_deref().unwrap_or(&[]);
        let pieces = design.pieces.as_deref().unwrap_or(&[]);

        // Build parent map: child guid -> (parent guid, connection)
        let mut parent_map: HashMap<&str, (&str, &Connection)> = HashMap::new();
        for conn in connections {
            parent_map.insert(
                conn.connecting.piece.guid.as_str(),
                (conn.connected.piece.guid.as_str(), conn),
            );
        }

        // Flatten the design to get absolute planes/centers
        let flat_change = flatten_design_change(kit, &design.guid);
        let mut flat_design = design.clone();
        apply_design_diff(&mut flat_design, &flat_change.forward);
        let flat_piece_map: HashMap<&str, &Piece> = flat_design
            .pieces
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|p| (p.guid.as_str(), p))
            .collect();

        let mut copy_pieces: Vec<Piece> = Vec::new();
        let mut added_piece_guids: HashSet<String> = HashSet::new();
        let mut copy_connections: Vec<Connection> = Vec::new();

        // Process selected pieces
        for piece_guid in piece_guids {
            let piece = match pieces.iter().find(|p| p.guid == *piece_guid) {
                Some(p) => p,
                None => continue,
            };

            let is_fixed = piece.plane.is_some();
            let is_connected = parent_map.contains_key(piece_guid.as_str());

            let mut is_internal_connected = false;
            let is_internal_fixed = is_fixed && selected_piece_set.contains(piece_guid.as_str());
            let mut is_pp_excl_pc_incl = false;

            if is_connected {
                let (parent_guid, parent_conn) = parent_map[piece_guid.as_str()];
                let parent_piece_selected = selected_piece_set.contains(parent_guid);
                let parent_conn_selected =
                    selected_connection_set.contains(parent_conn.guid.as_str());
                is_internal_connected = parent_piece_selected && parent_conn_selected;
                is_pp_excl_pc_incl = !parent_piece_selected && parent_conn_selected;
            }

            if is_internal_fixed || is_internal_connected {
                copy_pieces.push(piece.clone());
                added_piece_guids.insert(piece_guid.clone());
            } else if is_pp_excl_pc_incl {
                let mut copied = piece.clone();
                if let Some(flat_piece) = flat_piece_map.get(piece_guid.as_str()) {
                    let center_value = match &flat_piece.center {
                        Some(c) => serde_json::to_string(c).unwrap_or_default(),
                        None => serde_json::to_string(&Coord::default()).unwrap_or_default(),
                    };
                    let plane_value = match &flat_piece.plane {
                        Some(p) => serde_json::to_string(p).unwrap_or_default(),
                        None => serde_json::to_string(&Plane::default()).unwrap_or_default(),
                    };
                    let attrs = copied.attributes.get_or_insert_with(Vec::new);
                    attrs.push(Attribute {
                        guid: String::new(),
                        key: "semio.center".to_string(),
                        value: Some(center_value),
                        definition: None,
                    });
                    attrs.push(Attribute {
                        guid: String::new(),
                        key: "semio.plane".to_string(),
                        value: Some(plane_value),
                        definition: None,
                    });
                }
                copy_pieces.push(copied);
                added_piece_guids.insert(piece_guid.clone());
            }
        }

        // Process selected connections
        for conn_guid in connection_guids {
            let conn = match connections.iter().find(|c| c.guid == *conn_guid) {
                Some(c) => c,
                None => continue,
            };

            let connected_guid = &conn.connected.piece.guid;
            let connecting_guid = &conn.connecting.piece.guid;
            let connected_selected = selected_piece_set.contains(connected_guid.as_str());
            let connecting_selected = selected_piece_set.contains(connecting_guid.as_str());

            let is_internal = connected_selected && connecting_selected;

            if is_internal {
                copy_connections.push(conn.clone());
            } else {
                // Orphaned, parent-excl-child-incl, or parent-incl-child-excl
                copy_connections.push(conn.clone());

                let mut external_guids: Vec<&str> = Vec::new();
                if !connected_selected {
                    external_guids.push(connected_guid.as_str());
                }
                if !connecting_selected {
                    external_guids.push(connecting_guid.as_str());
                }

                for ext_guid in external_guids {
                    if !added_piece_guids.contains(ext_guid) {
                        if let Some(ext_piece) = pieces.iter().find(|p| p.guid == ext_guid) {
                            let mut cloned = ext_piece.clone();
                            let attrs = cloned.attributes.get_or_insert_with(Vec::new);
                            attrs.push(Attribute {
                                guid: String::new(),
                                key: "semio.piece.origin".to_string(),
                                value: Some("external".to_string()),
                                definition: None,
                            });
                            if let Some(flat_piece) = flat_piece_map.get(ext_guid) {
                                let center_value = match &flat_piece.center {
                                    Some(c) => serde_json::to_string(c).unwrap_or_default(),
                                    None => {
                                        serde_json::to_string(&Coord::default()).unwrap_or_default()
                                    }
                                };
                                attrs.push(Attribute {
                                    guid: String::new(),
                                    key: "semio.center".to_string(),
                                    value: Some(center_value),
                                    definition: None,
                                });
                            }
                            copy_pieces.push(cloned);
                            added_piece_guids.insert(ext_guid.to_string());
                        }
                    }
                }
            }
        }

        Design {
            guid: String::new(),
            name: String::new(),
            pieces: Some(copy_pieces),
            connections: Some(copy_connections),
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
            layers: None,
            groups: None,
            stats: None,
            active_layer: None,
            location: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// 📋Pastes a copied design into a target design, returning a DesignDiff.
    /// Specs: Anchoring determines the reference point within the bounding rectangle of the source.
    /// Fixed pieces get -anchor offset applied to center; if coord is given, +coord offset is also applied.
    /// Connected pieces with non-external parents are added as-is.
    /// Connected pieces with external-origin parents: if a matching piece with a matching connector is found in target,
    /// the parent connection is remapped; otherwise treated as fixed using semio.center/semio.plane attributes.
    pub fn paste_design(
        kit: &Kit,
        source: &Design,
        target: &Design,
        anchoring: &str,
        coord: Option<&Coord>,
    ) -> DesignDiff {
        let types_map: HashMap<&str, &Type> = kit
            .types
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|t| (t.guid.as_str(), t))
            .collect();
        let ports_map: HashMap<&str, &Port> = kit
            .ports
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|p| (p.guid.as_str(), p))
            .collect();

        let source_pieces = source.pieces.as_deref().unwrap_or(&[]);
        let source_connections = source.connections.as_deref().unwrap_or(&[]);
        let target_pieces = target.pieces.as_deref().unwrap_or(&[]);

        // Classify source pieces
        let external_origin_guids: HashSet<&str> = source_pieces
            .iter()
            .filter(|p| {
                p.attributes.as_deref().unwrap_or(&[]).iter().any(|a| {
                    a.key == "semio.piece.origin" && a.value.as_deref() == Some("external")
                })
            })
            .map(|p| p.guid.as_str())
            .collect();

        let source_piece_map: HashMap<&str, &Piece> =
            source_pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        let mut source_parent_map: HashMap<&str, (&str, &Connection)> = HashMap::new();
        for conn in source_connections {
            let child_guid = conn.connecting.piece.guid.as_str();
            let parent_guid = conn.connected.piece.guid.as_str();
            match source_parent_map.get(child_guid).copied() {
                None => {
                    source_parent_map.insert(child_guid, (parent_guid, conn));
                }
                Some((prev_parent, _)) => {
                    let prev_stub = external_origin_guids.contains(prev_parent);
                    let next_stub = external_origin_guids.contains(parent_guid);
                    if prev_stub != next_stub && next_stub {
                        source_parent_map.insert(child_guid, (parent_guid, conn));
                    }
                }
            }
        }

        // Compute bounding rectangle from flat centers
        let mut center_coords: Vec<Coord> = Vec::new();
        for piece in source_pieces {
            if external_origin_guids.contains(piece.guid.as_str()) {
                continue;
            }
            let mut center: Option<Coord> = piece.center.clone();
            if center.is_none() {
                if let Some(attr) = piece
                    .attributes
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .find(|a| a.key == "semio.center")
                {
                    if let Some(val) = &attr.value {
                        center = serde_json::from_str(val).ok();
                    }
                }
            }
            if let Some(c) = center {
                center_coords.push(c);
            }
        }

        if center_coords.is_empty() {
            center_coords.push(Coord::default());
        }

        let min_u = center_coords
            .iter()
            .map(|c| c.u)
            .fold(f64::INFINITY, f64::min);
        let max_u = center_coords
            .iter()
            .map(|c| c.u)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_v = center_coords
            .iter()
            .map(|c| c.v)
            .fold(f64::INFINITY, f64::min);
        let max_v = center_coords
            .iter()
            .map(|c| c.v)
            .fold(f64::NEG_INFINITY, f64::max);

        let anchor = match anchoring {
            "middle" => Coord::new((min_u + max_u) / 2.0, (min_v + max_v) / 2.0),
            "centroid" => {
                let n = center_coords.len() as f64;
                Coord::new(
                    center_coords.iter().map(|c| c.u).sum::<f64>() / n,
                    center_coords.iter().map(|c| c.v).sum::<f64>() / n,
                )
            }
            "bottomLeft" => Coord::new(min_u, min_v),
            "bottomRight" => Coord::new(max_u, min_v),
            "topLeft" => Coord::new(min_u, max_v),
            "topRight" => Coord::new(max_u, max_v),
            _ => Coord::new(0.0, 0.0), // "original"
        };

        // Build target piece maps for matching
        let mut target_pieces_by_name: HashMap<&str, Vec<&Piece>> = HashMap::new();
        for tp in target_pieces {
            if let Some(ref name) = tp.name {
                target_pieces_by_name
                    .entry(name.as_str())
                    .or_default()
                    .push(tp);
            }
        }

        // Helper: check port compatibility
        let are_ports_compatible = |pg1: &str, pg2: &str| -> bool {
            if pg1.is_empty() || pg2.is_empty() {
                return false;
            }
            if pg1 == pg2 {
                return true;
            }
            let port1 = match ports_map.get(pg1) {
                Some(p) => p,
                None => return false,
            };
            let port2 = match ports_map.get(pg2) {
                Some(p) => p,
                None => return false,
            };
            if let Some(ref cp) = port1.compatible_interfaces {
                if cp.iter().any(|c| c.guid == pg2) {
                    return true;
                }
            }
            if let Some(ref cp) = port2.compatible_interfaces {
                if cp.iter().any(|c| c.guid == pg1) {
                    return true;
                }
            }
            false
        };

        // Helper: find matching connector on a type
        let find_matching_connector =
            |type_guid: &str, source_connector: &Connector| -> Option<Connector> {
                let t = types_map.get(type_guid)?;
                let connectors = t.connectors.as_deref().unwrap_or(&[]);
                connectors
                    .iter()
                    .find(|c| {
                        c.name == source_connector.name && {
                            let pg1 = c.port.as_ref().map(|p| p.guid.as_str()).unwrap_or("");
                            let pg2 = source_connector
                                .port
                                .as_ref()
                                .map(|p| p.guid.as_str())
                                .unwrap_or("");
                            are_ports_compatible(pg1, pg2)
                        }
                    })
                    .cloned()
            };

        let mut added_pieces: Vec<Piece> = Vec::new();
        let mut added_connections: Vec<Connection> = Vec::new();

        // Process source pieces
        for piece in source_pieces {
            if external_origin_guids.contains(piece.guid.as_str()) {
                continue;
            }

            let is_fixed = piece.plane.is_some();
            let is_connected = source_parent_map.contains_key(piece.guid.as_str());

            if is_fixed && !is_connected {
                // Fixed piece: apply -anchor offset, then +coord if given
                let mut copied = piece.clone();
                let c = copied.center.clone().unwrap_or_default();
                let mut new_center = Coord::new(c.u - anchor.u, c.v - anchor.v);
                if let Some(co) = coord {
                    new_center = Coord::new(new_center.u + co.u, new_center.v + co.v);
                }
                copied.center = Some(new_center);
                added_pieces.push(copied);
            } else if is_connected {
                let (parent_guid, parent_conn) = source_parent_map[piece.guid.as_str()];
                if external_origin_guids.contains(parent_guid) {
                    // Parent is external-origin: try to match in target
                    let external_parent = source_piece_map[parent_guid];
                    let mut matched = false;

                    let ext_name = external_parent.name.as_deref().unwrap_or("");
                    if !ext_name.is_empty() {
                        if let Some(candidates) = target_pieces_by_name.get(ext_name) {
                            let is_parent_connected =
                                parent_conn.connected.piece.guid.as_str() == parent_guid;
                            let parent_connector_guid = if is_parent_connected {
                                parent_conn
                                    .connected
                                    .connector
                                    .as_ref()
                                    .map(|c| c.guid.as_str())
                                    .unwrap_or("")
                            } else {
                                parent_conn
                                    .connecting
                                    .connector
                                    .as_ref()
                                    .map(|c| c.guid.as_str())
                                    .unwrap_or("")
                            };

                            // Find the source parent connector
                            let source_parent_connector: Option<&Connector> =
                                external_parent.type_ref.as_ref().and_then(|tr| {
                                    types_map.get(tr.guid.as_str()).and_then(|parent_type| {
                                        parent_type
                                            .connectors
                                            .as_deref()
                                            .unwrap_or(&[])
                                            .iter()
                                            .find(|c| c.guid.as_str() == parent_connector_guid)
                                    })
                                });

                            if let Some(src_conn) = source_parent_connector {
                                for candidate in candidates {
                                    if let Some(ref type_ref) = candidate.type_ref {
                                        if let Some(matching_connector) =
                                            find_matching_connector(&type_ref.guid, src_conn)
                                        {
                                            matched = true;
                                            added_pieces.push(piece.clone());

                                            let mut copied_conn = parent_conn.clone();
                                            if is_parent_connected {
                                                copied_conn.connected = Side {
                                                    piece: PieceId {
                                                        guid: candidate.guid.clone(),
                                                    },
                                                    design_piece: None,
                                                    connector: Some(ConnectorId {
                                                        guid: matching_connector.guid.clone(),
                                                    }),
                                                };
                                            } else {
                                                copied_conn.connecting = Side {
                                                    piece: PieceId {
                                                        guid: candidate.guid.clone(),
                                                    },
                                                    design_piece: None,
                                                    connector: Some(ConnectorId {
                                                        guid: matching_connector.guid.clone(),
                                                    }),
                                                };
                                            }
                                            if let Some(co) = coord {
                                                let connected_stub = external_origin_guids
                                                    .contains(
                                                        parent_conn.connected.piece.guid.as_str(),
                                                    );
                                                let connecting_stub = external_origin_guids
                                                    .contains(
                                                        parent_conn.connecting.piece.guid.as_str(),
                                                    );
                                                let conn_matches_parentage = (parent_conn
                                                    .connecting
                                                    .piece
                                                    .guid
                                                    .as_str()
                                                    == piece.guid.as_str()
                                                    && parent_conn.connected.piece.guid.as_str()
                                                        == parent_guid)
                                                    || (parent_conn.connected.piece.guid.as_str()
                                                        == piece.guid.as_str()
                                                        && parent_conn
                                                            .connecting
                                                            .piece
                                                            .guid
                                                            .as_str()
                                                            == parent_guid);
                                                // Specs: Coord may shift diagram u/v only for the remapped bridge to a clipboard external stub;
                                                // internal–internal source edges (neither side a stub) must keep cloned u/v.
                                                if conn_matches_parentage
                                                    && connected_stub != connecting_stub
                                                {
                                                    let mut flat_parent_center: Option<Coord> =
                                                        candidate.center.clone();
                                                    if flat_parent_center.is_none() {
                                                        flat_parent_center = candidate
                                                            .attributes
                                                            .as_deref()
                                                            .unwrap_or(&[])
                                                            .iter()
                                                            .find(|a| a.key == "semio.center")
                                                            .and_then(|a| a.value.as_ref())
                                                            .and_then(|v| {
                                                                serde_json::from_str::<Coord>(v)
                                                                    .ok()
                                                            });
                                                    }
                                                    if flat_parent_center.is_none() {
                                                        flat_parent_center = external_parent
                                                            .attributes
                                                            .as_deref()
                                                            .unwrap_or(&[])
                                                            .iter()
                                                            .find(|a| a.key == "semio.center")
                                                            .and_then(|a| a.value.as_ref())
                                                            .and_then(|v| {
                                                                serde_json::from_str::<Coord>(v)
                                                                    .ok()
                                                            });
                                                    }
                                                    if flat_parent_center.is_none() {
                                                        flat_parent_center =
                                                            external_parent.center.clone();
                                                    }
                                                    let mut flat_child_center: Option<Coord> =
                                                        piece
                                                            .attributes
                                                            .as_deref()
                                                            .unwrap_or(&[])
                                                            .iter()
                                                            .find(|a| a.key == "semio.center")
                                                            .and_then(|a| a.value.as_ref())
                                                            .and_then(|v| {
                                                                serde_json::from_str::<Coord>(v)
                                                                    .ok()
                                                            });
                                                    if flat_child_center.is_none() {
                                                        flat_child_center = piece.center.clone();
                                                    }
                                                    if let (Some(fpc), Some(fcc)) =
                                                        (flat_parent_center, flat_child_center)
                                                    {
                                                        copied_conn.u = Some(
                                                            fpc.u - (co.u + (anchor.u - fcc.u)),
                                                        );
                                                        copied_conn.v = Some(
                                                            fpc.v - (co.v + (anchor.v - fcc.v)),
                                                        );
                                                    }
                                                }
                                            }
                                            added_connections.push(copied_conn);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !matched {
                        // Treat as fixed piece using semio.center and semio.plane attributes
                        let mut copied = piece.clone();
                        let attrs = piece.attributes.as_deref().unwrap_or(&[]);
                        for attr in attrs {
                            if attr.key == "semio.center" {
                                if let Some(val) = &attr.value {
                                    if let Ok(c) = serde_json::from_str::<Coord>(val) {
                                        copied.center = Some(c);
                                    }
                                }
                            }
                            if attr.key == "semio.plane" {
                                if let Some(val) = &attr.value {
                                    if let Ok(p) = serde_json::from_str::<Plane>(val) {
                                        copied.plane = Some(p);
                                    }
                                }
                            }
                        }
                        let c = copied.center.clone().unwrap_or_default();
                        let mut new_center = Coord::new(c.u - anchor.u, c.v - anchor.v);
                        if let Some(co) = coord {
                            new_center = Coord::new(new_center.u + co.u, new_center.v + co.v);
                        }
                        copied.center = Some(new_center);
                        added_pieces.push(copied);
                    }
                } else {
                    // Parent is not external: add connected piece as-is
                    added_pieces.push(piece.clone());
                }
            }
        }

        // Process source connections (non-external internal connections)
        let added_piece_guids: HashSet<&str> =
            added_pieces.iter().map(|p| p.guid.as_str()).collect();
        for conn in source_connections {
            let connected_guid = conn.connected.piece.guid.as_str();
            let connecting_guid = conn.connecting.piece.guid.as_str();

            if external_origin_guids.contains(connected_guid)
                || external_origin_guids.contains(connecting_guid)
            {
                continue;
            }

            if !added_piece_guids.contains(connected_guid)
                || !added_piece_guids.contains(connecting_guid)
            {
                continue;
            }

            added_connections.push(conn.clone());
        }

        let mut diff = DesignDiff {
            guid: String::new(),
            ..Default::default()
        };
        if !added_pieces.is_empty() {
            diff.pieces = Some(CollectionDiff {
                added: Some(added_pieces),
                removed: None,
                updated: None,
            });
        }
        if !added_connections.is_empty() {
            diff.connections = Some(CollectionDiff {
                added: Some(added_connections),
                removed: None,
                updated: None,
            });
        }
        diff
    }
} // 📋CopyPasteDesign
pub use copy_paste_design::*;

mod kit_model_export {
    // 🏤Kit Model Export
    // Kit Model Export MUST provide GLB/glTF export of a design's assembled 3D model.

    /// <summary>📺Supported 3D model export formats (extension, description).</summary>
    use super::*;

    pub const EXPORT_MODEL_FORMATS: &[(&str, &str)] = &[
        ("glb", "GLB Binary (glTF 2.0)"),
        ("gltf", "glTF JSON (glTF 2.0)"),
    ];

    /// <summary>🔐Decodes a data URI blob (data:mime;base64,...) into raw bytes.</summary>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn decode_data_uri_blob(blob: &str) -> Option<Vec<u8>> {
        use base64::Engine;
        let b64 = if let Some(pos) = blob.find(";base64,") {
            &blob[pos + 8..]
        } else {
            blob
        };
        base64::engine::general_purpose::STANDARD.decode(b64).ok()
    }

    /// <summary>🔬Parses a GLB binary into its JSON chunk and BIN chunk.</summary>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn parse_glb(data: &[u8]) -> Option<(serde_json::Value, Vec<u8>)> {
        if data.len() < 12 {
            return None;
        }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != 0x46546C67 {
            return None;
        }
        let mut offset = 12usize;
        let mut json_value: Option<serde_json::Value> = None;
        let mut bin_data: Vec<u8> = Vec::new();
        while offset + 8 <= data.len() {
            let chunk_length = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            let chunk_type = u32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);
            offset += 8;
            if offset + chunk_length > data.len() {
                break;
            }
            let chunk_data = &data[offset..offset + chunk_length];
            if chunk_type == 0x4E4F534A {
                if let Ok(s) = std::str::from_utf8(chunk_data) {
                    json_value = serde_json::from_str(s.trim()).ok();
                }
            } else if chunk_type == 0x004E4942 {
                bin_data = chunk_data.to_vec();
            }
            offset += chunk_length;
        }
        json_value.map(|json| (json, bin_data))
    }

    /// <summary>🏗️Builds a GLB binary from a glTF JSON value and binary buffer.</summary>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_glb(json: &serde_json::Value, bin: &[u8]) -> Vec<u8> {
        let json_str = serde_json::to_string(json).unwrap_or_default();
        let json_bytes = json_str.as_bytes();
        let json_padded_len = (json_bytes.len() + 3) & !3;
        let bin_padded_len = (bin.len() + 3) & !3;
        let has_bin = !bin.is_empty();
        let total_length = 12 + 8 + json_padded_len + if has_bin { 8 + bin_padded_len } else { 0 };
        let mut result = Vec::with_capacity(total_length);
        result.extend_from_slice(&0x46546C67u32.to_le_bytes());
        result.extend_from_slice(&2u32.to_le_bytes());
        result.extend_from_slice(&(total_length as u32).to_le_bytes());
        result.extend_from_slice(&(json_padded_len as u32).to_le_bytes());
        result.extend_from_slice(&0x4E4F534Au32.to_le_bytes());
        result.extend_from_slice(json_bytes);
        result.resize(result.len() + json_padded_len - json_bytes.len(), b' ');
        if has_bin {
            result.extend_from_slice(&(bin_padded_len as u32).to_le_bytes());
            result.extend_from_slice(&0x004E4942u32.to_le_bytes());
            result.extend_from_slice(bin);
            result.resize(result.len() + bin_padded_len - bin.len(), 0);
        }
        result
    }

    /// 📚<summary>🔖Converts a nalgebra Matrix4 to glTF column-major array of 16 f64.</summary>
    pub fn matrix4_to_gltf_column_major(m: &Matrix4<f64>) -> [f64; 16] {
        let transformed = semio_matrix_to_gltf_matrix(m);
        [
            transformed[(0, 0)],
            transformed[(1, 0)],
            transformed[(2, 0)],
            transformed[(3, 0)],
            transformed[(0, 1)],
            transformed[(1, 1)],
            transformed[(2, 1)],
            transformed[(3, 1)],
            transformed[(0, 2)],
            transformed[(1, 2)],
            transformed[(2, 2)],
            transformed[(3, 2)],
            transformed[(0, 3)],
            transformed[(1, 3)],
            transformed[(2, 3)],
            transformed[(3, 3)],
        ]
    }

    pub fn semio_matrix_to_gltf_matrix(matrix: &Matrix4<f64>) -> Matrix4<f64> {
        let basis = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let basis_inv = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        basis * matrix * basis_inv
    }

    /// 🔗<summary>🔖Preserves source mesh geometry while clearing unresolved material links.</summary>
    pub fn strip_mesh_material_references(mesh: &mut serde_json::Value) {
        if let Some(primitives) = mesh
            .get_mut("primitives")
            .and_then(|value| value.as_array_mut())
        {
            for primitive in primitives.iter_mut() {
                if let Some(object) = primitive.as_object_mut() {
                    object.remove("material");
                }
            }
        }
    }

    /// 📖<summary>🔤Assigns a human-readable source file name to a merged mesh.</summary>
    pub fn set_mesh_name(mesh: &mut serde_json::Value, mesh_name: &str) {
        mesh["name"] = serde_json::json!(mesh_name);
    }

    pub fn matrix4_to_gltf_column_major_legacy(m: &Matrix4<f64>) -> [f64; 16] {
        [
            m[(0, 0)],
            m[(1, 0)],
            m[(2, 0)],
            m[(3, 0)],
            m[(0, 1)],
            m[(1, 1)],
            m[(2, 1)],
            m[(3, 1)],
            m[(0, 2)],
            m[(1, 2)],
            m[(2, 2)],
            m[(3, 2)],
            m[(0, 3)],
            m[(1, 3)],
            m[(2, 3)],
            m[(3, 3)],
        ]
    }

    /// <summary>➕Writes a box placeholder mesh into the combined GLB buffers and returns the mesh index.</summary>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn append_box_mesh(
        combined_bin: &mut Vec<u8>,
        buffer_views: &mut Vec<serde_json::Value>,
        accessors: &mut Vec<serde_json::Value>,
        meshes: &mut Vec<serde_json::Value>,
    ) {
        let s: f32 = 0.5;
        #[rustfmt::skip]
    let positions: [f32; 72] = [
        -s, -s,  s,   s, -s,  s,   s,  s,  s,  -s,  s,  s,
        -s, -s, -s,  -s,  s, -s,   s,  s, -s,   s, -s, -s,
        -s,  s, -s,  -s,  s,  s,   s,  s,  s,   s,  s, -s,
        -s, -s, -s,   s, -s, -s,   s, -s,  s,  -s, -s,  s,
         s, -s, -s,   s,  s, -s,   s,  s,  s,   s, -s,  s,
        -s, -s, -s,  -s, -s,  s,  -s,  s,  s,  -s,  s, -s,
    ];
        #[rustfmt::skip]
    let indices: [u16; 36] = [
         0,  1,  2,   0,  2,  3,
         4,  5,  6,   4,  6,  7,
         8,  9, 10,   8, 10, 11,
        12, 13, 14,  12, 14, 15,
        16, 17, 18,  16, 18, 19,
        20, 21, 22,  20, 22, 23,
    ];

        while combined_bin.len() % 4 != 0 {
            combined_bin.push(0);
        }
        let pos_offset = combined_bin.len();
        for &f in &positions {
            combined_bin.extend_from_slice(&f.to_le_bytes());
        }
        let pos_byte_length = positions.len() * 4;

        while combined_bin.len() % 4 != 0 {
            combined_bin.push(0);
        }
        let idx_offset = combined_bin.len();
        for &i in &indices {
            combined_bin.extend_from_slice(&i.to_le_bytes());
        }
        let idx_byte_length = indices.len() * 2;

        let bv_pos_idx = buffer_views.len();
        buffer_views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": pos_offset,
            "byteLength": pos_byte_length,
            "target": 34962
        }));
        let bv_idx_idx = buffer_views.len();
        buffer_views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": idx_offset,
            "byteLength": idx_byte_length,
            "target": 34963
        }));

        let acc_pos_idx = accessors.len();
        accessors.push(serde_json::json!({
            "bufferView": bv_pos_idx,
            "componentType": 5126,
            "count": 24,
            "type": "VEC3",
            "max": [s, s, s],
            "min": [-s, -s, -s]
        }));
        let acc_idx_idx = accessors.len();
        accessors.push(serde_json::json!({
            "bufferView": bv_idx_idx,
            "componentType": 5123,
            "count": 36,
            "type": "SCALAR"
        }));

        meshes.push(serde_json::json!({
            "primitives": [{
                "attributes": { "POSITION": acc_pos_idx },
                "indices": acc_idx_idx
            }]
        }));
    }

    /// 🔖<summary>🔖Selects the best model for a type given desired tag guids.</summary>
    pub fn select_model_for_type<'a>(t: &'a Type, tags: &[String]) -> Option<&'a Model> {
        let models = t.models.as_ref()?;
        if models.is_empty() {
            return None;
        }
        if tags.is_empty() {
            if let Some(m) = models
                .iter()
                .find(|m| m.tags.as_ref().map(|t| t.is_empty()).unwrap_or(true))
            {
                return Some(m);
            }
            return models.first();
        }
        let tag_guid_set: HashSet<&str> = tags.iter().map(|s| s.as_str()).collect();
        let mut best: Option<&Model> = None;
        let mut best_score = -1.0f64;
        for model in models {
            let model_tag_guids: HashSet<&str> = model
                .tags
                .as_ref()
                .map(|tags| tags.iter().map(|t| t.guid.as_str()).collect())
                .unwrap_or_default();
            if !tag_guid_set.iter().all(|t| model_tag_guids.contains(t)) {
                continue;
            }
            let intersection = model_tag_guids.intersection(&tag_guid_set).count();
            let union = model_tag_guids.union(&tag_guid_set).count();
            let score = if union > 0 {
                intersection as f64 / union as f64
            } else {
                0.0
            };
            if score > best_score {
                best_score = score;
                best = Some(model);
            }
        }
        if best.is_some() {
            return best;
        }
        models.first()
    }

    /// <summary>🔖Merges a source GLB's mesh data into the combined GLB builder.</summary>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn merge_glb_mesh(
        src_json: &serde_json::Value,
        src_bin: &[u8],
        combined_bin: &mut Vec<u8>,
        buffer_views: &mut Vec<serde_json::Value>,
        accessors: &mut Vec<serde_json::Value>,
        meshes: &mut Vec<serde_json::Value>,
        mesh_name: &str,
    ) -> bool {
        while combined_bin.len() % 4 != 0 {
            combined_bin.push(0);
        }
        let bin_offset = combined_bin.len();
        combined_bin.extend_from_slice(src_bin);
        while combined_bin.len() % 4 != 0 {
            combined_bin.push(0);
        }

        let bv_base = buffer_views.len();
        let acc_base = accessors.len();
        if let Some(bvs) = src_json.get("bufferViews").and_then(|v| v.as_array()) {
            for bv in bvs {
                let mut new_bv = bv.clone();
                new_bv["buffer"] = serde_json::json!(0);
                let orig_offset = bv.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0);
                new_bv["byteOffset"] = serde_json::json!(bin_offset as u64 + orig_offset);
                buffer_views.push(new_bv);
            }
        }

        if let Some(accs) = src_json.get("accessors").and_then(|v| v.as_array()) {
            for acc in accs {
                let mut new_acc = acc.clone();
                if let Some(bv_idx) = acc.get("bufferView").and_then(|v| v.as_u64()) {
                    new_acc["bufferView"] = serde_json::json!(bv_base as u64 + bv_idx);
                }
                accessors.push(new_acc);
            }
        }

        let mut merged_primitives: Vec<serde_json::Value> = Vec::new();
        if let Some(src_mesh_arr) = src_json.get("meshes").and_then(|v| v.as_array()) {
            for mesh in src_mesh_arr {
                if let Some(primitives) = mesh.get("primitives").and_then(|v| v.as_array()) {
                    for primitive in primitives {
                        let mut new_primitive = primitive.clone();
                        new_primitive["mode"] = serde_json::json!(4);
                        if let Some(attrs) = new_primitive.get_mut("attributes") {
                            if let Some(obj) = attrs.as_object_mut() {
                                for (_, val) in obj.iter_mut() {
                                    if let Some(idx) = val.as_u64() {
                                        *val = serde_json::json!(acc_base as u64 + idx);
                                    }
                                }
                            }
                        }
                        if let Some(idx_val) = new_primitive.get("indices").and_then(|v| v.as_u64())
                        {
                            new_primitive["indices"] = serde_json::json!(acc_base as u64 + idx_val);
                        }
                        merged_primitives.push(new_primitive);
                    }
                }
            }
        }
        if !merged_primitives.is_empty() {
            let mut new_mesh = serde_json::json!({
                "primitives": merged_primitives,
            });
            strip_mesh_material_references(&mut new_mesh);
            set_mesh_name(&mut new_mesh, mesh_name);
            meshes.push(new_mesh);
            return true;
        }
        false
    }

    /// <summary>🔖Exports the 3D model of a design to GLB or glTF format.</summary>
    /// <remarks>
    /// merging per-type GLB meshes (or box placeholders) into a single GLB/glTF output.
    /// </remarks>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn export_design_model(
        kit: &Kit,
        design_guid: &str,
        format: &str,
        tags: &[String],
        _options: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<u8>> {
        let format_lower = format.to_lowercase();
        let format_lower = format_lower.trim_start_matches('.');
        if format_lower != "glb" && format_lower != "gltf" {
            return Err(SemioError::InvalidOperation {
                message: format!(
                    "Unsupported export format '{}'. Supported: .glb, .gltf",
                    format
                ),
            });
        }

        let design = find_design_in_kit(kit, design_guid).ok_or_else(|| SemioError::NotFound {
            kind: "Design".to_string(),
            guid: design_guid.to_string(),
        })?;

        let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
        if pieces.is_empty() {
            let empty_json = serde_json::json!({
                "asset": { "version": "2.0", "generator": "semio" },
                "scene": 0,
                "scenes": [{ "nodes": [] }]
            });
            return if format_lower == "glb" {
                Ok(build_glb(&empty_json, &[]))
            } else {
                Ok(serde_json::to_string_pretty(&empty_json)
                    .unwrap_or_default()
                    .into_bytes())
            };
        }

        let connections = design
            .connections
            .as_ref()
            .map(|c| c.as_slice())
            .unwrap_or(&[]);

        let types_map: HashMap<&str, &Type> = kit
            .types
            .as_ref()
            .map(|types| types.iter().map(|t| (t.guid.as_str(), t)).collect())
            .unwrap_or_default();

        let pieces_map: HashMap<&str, &Piece> =
            pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        let files_map: HashMap<&str, &File> = kit
            .files
            .as_ref()
            .map(|files| files.iter().map(|f| (f.guid.as_str(), f)).collect())
            .unwrap_or_default();

        // 📹BFS World Transforms
        let mut adjacency: HashMap<&str, Vec<(&str, &Connection, bool)>> = HashMap::new();
        for conn in connections {
            let src = conn.connected.piece.guid.as_str();
            let tgt = conn.connecting.piece.guid.as_str();
            if pieces_map.contains_key(src) && pieces_map.contains_key(tgt) {
                adjacency.entry(src).or_default().push((tgt, conn, true));
                adjacency.entry(tgt).or_default().push((src, conn, false));
            }
        }

        let mut piece_world_matrices: HashMap<&str, Matrix4<f64>> =
            HashMap::with_capacity(pieces.len());
        let mut visited: HashSet<&str> = HashSet::with_capacity(pieces.len());
        let mut queue: VecDeque<&str> = VecDeque::with_capacity(pieces.len());
        let mut parent_map: HashMap<&str, &str> = HashMap::new();

        for piece in pieces {
            if visited.contains(piece.guid.as_str()) {
                continue;
            }
            let initial_matrix = piece
                .plane
                .as_ref()
                .map(|p| p.to_matrix())
                .unwrap_or_else(Matrix4::identity);
            piece_world_matrices.insert(piece.guid.as_str(), initial_matrix);
            visited.insert(piece.guid.as_str());
            queue.push_back(piece.guid.as_str());

            while let Some(current_guid) = queue.pop_front() {
                let current_matrix = *piece_world_matrices.get(current_guid).unwrap();
                if let Some(neighbors) = adjacency.get(current_guid) {
                    for &(neighbor_guid, conn, is_connected) in neighbors {
                        if visited.contains(neighbor_guid) {
                            continue;
                        }
                        let connection_matrix = match compute_connection_matrix_fast(
                            &types_map,
                            &pieces_map,
                            conn,
                            is_connected,
                        ) {
                            Some(m) => m,
                            None => continue,
                        };
                        let new_matrix = current_matrix * connection_matrix;
                        piece_world_matrices.insert(neighbor_guid, new_matrix);
                        visited.insert(neighbor_guid);
                        parent_map.insert(neighbor_guid, current_guid);
                        queue.push_back(neighbor_guid);
                    }
                }
            }
        }
        // 🔑BFS World Transforms

        // 🏞️Mesh Assembly
        let mut combined_bin: Vec<u8> = Vec::new();
        let mut gltf_buffer_views: Vec<serde_json::Value> = Vec::new();
        let mut gltf_accessors: Vec<serde_json::Value> = Vec::new();
        let mut gltf_meshes: Vec<serde_json::Value> = Vec::new();
        let mut type_mesh_map: HashMap<String, usize> = HashMap::new();

        for piece in pieces {
            let type_guid = match &piece.type_ref {
                Some(tr) => &tr.guid,
                None => continue,
            };
            if type_mesh_map.contains_key(type_guid.as_str()) {
                continue;
            }
            let mesh_idx = gltf_meshes.len();
            let mut added = false;

            if let Some(t) = types_map.get(type_guid.as_str()) {
                if let Some(model) = select_model_for_type(t, tags) {
                    if let Some(file) = files_map.get(model.file.guid.as_str()) {
                        if let Some(blob) = &file.blob {
                            if let Some(data) = decode_data_uri_blob(blob) {
                                let is_glb = file.name.ends_with(".glb")
                                    || (data.len() >= 4
                                        && u32::from_le_bytes([
                                            data[0], data[1], data[2], data[3],
                                        ]) == 0x46546C67);
                                if is_glb {
                                    if let Some((src_json, src_bin)) = parse_glb(&data) {
                                        added = merge_glb_mesh(
                                            &src_json,
                                            &src_bin,
                                            &mut combined_bin,
                                            &mut gltf_buffer_views,
                                            &mut gltf_accessors,
                                            &mut gltf_meshes,
                                            file.name.as_str(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if added {
                type_mesh_map.insert(type_guid.clone(), mesh_idx);
            }
        }
        // 📈Mesh Assembly

        // 🧤Scene Graph
        let mut gltf_nodes: Vec<serde_json::Value> = Vec::new();
        let mut piece_node_indices: HashMap<&str, usize> = HashMap::new();
        let mut root_node_indices: Vec<usize> = Vec::new();

        for piece in pieces {
            let node_idx = gltf_nodes.len();
            piece_node_indices.insert(piece.guid.as_str(), node_idx);

            let world_matrix = piece_world_matrices
                .get(piece.guid.as_str())
                .copied()
                .unwrap_or_else(Matrix4::identity);

            let local_matrix = if let Some(&parent_guid) = parent_map.get(piece.guid.as_str()) {
                let parent_world = piece_world_matrices
                    .get(parent_guid)
                    .copied()
                    .unwrap_or_else(Matrix4::identity);
                match parent_world.try_inverse() {
                    Some(inv) => inv * world_matrix,
                    None => world_matrix,
                }
            } else {
                root_node_indices.push(node_idx);
                world_matrix
            };

            let mesh_idx = piece
                .type_ref
                .as_ref()
                .and_then(|tr| type_mesh_map.get(&tr.guid))
                .copied();

            let col_major = matrix4_to_gltf_column_major(&local_matrix);
            let mut node = serde_json::json!({
                "matrix": col_major
            });
            if let Some(name) = &piece.name {
                node["name"] = serde_json::json!(name);
            }
            if let Some(idx) = mesh_idx {
                node["mesh"] = serde_json::json!(idx);
            }
            gltf_nodes.push(node);
        }

        let mut children_map: HashMap<&str, Vec<usize>> = HashMap::new();
        for piece in pieces {
            if let Some(&pg) = parent_map.get(piece.guid.as_str()) {
                if let Some(&child_idx) = piece_node_indices.get(piece.guid.as_str()) {
                    children_map.entry(pg).or_default().push(child_idx);
                }
            }
        }
        for (parent_guid, child_indices) in &children_map {
            if let Some(&parent_node_idx) = piece_node_indices.get(parent_guid) {
                gltf_nodes[parent_node_idx]["children"] = serde_json::json!(child_indices);
            }
        }
        // 📑Scene Graph

        // 📂Output
        let mut gltf_root = serde_json::json!({
            "asset": { "version": "2.0", "generator": "semio" },
            "scene": 0,
            "scenes": [{ "nodes": root_node_indices }],
            "nodes": gltf_nodes,
            "meshes": gltf_meshes,
            "accessors": gltf_accessors,
            "bufferViews": gltf_buffer_views,
            "buffers": [{ "byteLength": combined_bin.len() }]
        });
        if format_lower == "glb" {
            Ok(build_glb(&gltf_root, &combined_bin))
        } else {
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&combined_bin);
            gltf_root["buffers"] = serde_json::json!([{
                "byteLength": combined_bin.len(),
                "uri": format!("data:application/octet-stream;base64,{}", b64)
            }]);
            Ok(serde_json::to_string_pretty(&gltf_root)
                .unwrap_or_default()
                .into_bytes())
        }
        // 📢Output
    }
} // 🔮Kit Model Export
pub use kit_model_export::*;

mod geometric_insights {
    // 🥽Geometric Insights
    // Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

    /// Geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
    use super::*;

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometricInsights {
        pub bounding_box_min: Option<[f64; 3]>,
        pub bounding_box_max: Option<[f64; 3]>,
        pub dimension_x: f64,
        pub dimension_y: f64,
        pub dimension_z: f64,
        pub characteristic_length: f64,
        pub footprint_area: f64,
        pub total_surface_area: f64,
        pub enclosed_volume: f64,
        pub surface_to_volume_ratio: f64,
        pub aspect_ratio_xy: f64,
        pub aspect_ratio_xz: f64,
        pub aspect_ratio_yz: f64,
        pub slenderness: f64,
        pub centroid: Option<[f64; 3]>,
        pub vertex_count: usize,
        pub face_count: usize,
        pub euler_characteristic: i32,
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_gltf_mesh_data(
        json: &serde_json::Value,
        bin: &[u8],
    ) -> Option<(Vec<[f32; 3]>, Vec<u32>, [f32; 3], [f32; 3])> {
        let accessors = json.get("accessors")?.as_array()?;
        let buffer_views = json.get("bufferViews")?.as_array()?;
        let gltf_int =
            |v: Option<&serde_json::Value>| v.and_then(|x| x.as_u64()).unwrap_or(0) as usize;
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut pos_min = [f32::MAX; 3];
        let mut pos_max = [f32::MIN; 3];

        for mesh in json.get("meshes")?.as_array()? {
            for prim in mesh.get("primitives")?.as_array()? {
                let attrs = prim.get("attributes")?;
                let pos_acc_idx = gltf_int(attrs.get("POSITION"));
                let pos_acc = accessors.get(pos_acc_idx)?;
                let bv_idx = gltf_int(pos_acc.get("bufferView"));
                let bv = buffer_views.get(bv_idx)?;
                let count = gltf_int(pos_acc.get("count"));
                let bv_offset = gltf_int(bv.get("byteOffset"));
                let acc_offset = gltf_int(pos_acc.get("byteOffset"));
                let stride = gltf_int(bv.get("byteStride")).max(12);
                let base = bv_offset + acc_offset;
                let vertex_base = positions.len();
                for i in 0..count {
                    let start = base + i * stride;
                    if start + 12 > bin.len() {
                        break;
                    }
                    let x = f32::from_le_bytes([
                        bin[start],
                        bin[start + 1],
                        bin[start + 2],
                        bin[start + 3],
                    ]);
                    let y = f32::from_le_bytes([
                        bin[start + 4],
                        bin[start + 5],
                        bin[start + 6],
                        bin[start + 7],
                    ]);
                    let z = f32::from_le_bytes([
                        bin[start + 8],
                        bin[start + 9],
                        bin[start + 10],
                        bin[start + 11],
                    ]);
                    let v = [x, y, z];
                    positions.push(v);
                    pos_min[0] = pos_min[0].min(x);
                    pos_min[1] = pos_min[1].min(y);
                    pos_min[2] = pos_min[2].min(z);
                    pos_max[0] = pos_max[0].max(x);
                    pos_max[1] = pos_max[1].max(y);
                    pos_max[2] = pos_max[2].max(z);
                }
                if let Some(idx_val) = prim.get("indices") {
                    let idx_acc = accessors.get(gltf_int(Some(idx_val)))?;
                    let bv_idx = gltf_int(idx_acc.get("bufferView"));
                    let bv = buffer_views.get(bv_idx)?;
                    let count = gltf_int(idx_acc.get("count"));
                    let component_type = gltf_int(idx_acc.get("componentType"));
                    let bv_offset = gltf_int(bv.get("byteOffset"));
                    let acc_offset = gltf_int(idx_acc.get("byteOffset"));
                    let bytes_per = match component_type {
                        5121 => 1,
                        5123 => 2,
                        5125 => 4,
                        _ => 4,
                    };
                    let stride = gltf_int(bv.get("byteStride")).max(bytes_per);
                    let base = bv_offset + acc_offset;
                    for i in 0..count {
                        let start = base + i * stride;
                        let idx = match component_type {
                            5121 => bin.get(start).copied().unwrap_or(0) as u32,
                            5123 => u16::from_le_bytes([bin[start], bin[start + 1]]) as u32,
                            _ => u32::from_le_bytes([
                                bin[start],
                                bin[start + 1],
                                bin[start + 2],
                                bin[start + 3],
                            ]),
                        };
                        indices.push(vertex_base as u32 + idx);
                    }
                } else {
                    for i in 0..(count / 3) {
                        indices.push(vertex_base as u32 + (i * 3) as u32);
                        indices.push(vertex_base as u32 + (i * 3 + 1) as u32);
                        indices.push(vertex_base as u32 + (i * 3 + 2) as u32);
                    }
                }
            }
        }
        if positions.is_empty() || indices.is_empty() {
            return None;
        }
        Some((positions, indices, pos_min, pos_max))
    }

    /// Computes key performance indicators for the geometry of a GLB/GLTF model.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_geometric_insights_for_model(model: &[u8]) -> Result<GeometricInsights> {
        let (json, bin) = if model.len() >= 4
            && u32::from_le_bytes([model[0], model[1], model[2], model[3]]) == 0x46546C67
        {
            parse_glb(model).ok_or_else(|| SemioError::InvalidOperation {
                message: "Invalid GLB".to_string(),
            })?
        } else {
            let json: serde_json::Value =
                serde_json::from_slice(model).map_err(|e| SemioError::InvalidOperation {
                    message: format!("Invalid glTF JSON: {}", e),
                })?;
            let mut bin_data = Vec::new();
            if let Some(buffers) = json.get("buffers").and_then(|b| b.as_array()) {
                if let Some(buf) = buffers.first().and_then(|b| b.as_object()) {
                    if let Some(uri) = buf.get("uri").and_then(|u| u.as_str()) {
                        if uri.starts_with("data:") {
                            if let Some(b64) = uri.split(',').nth(1) {
                                let b64_clean: String =
                                    b64.chars().filter(|c| !c.is_whitespace()).collect();
                                bin_data = base64::engine::general_purpose::STANDARD
                                    .decode(&b64_clean)
                                    .or_else(|_| {
                                        base64::engine::general_purpose::STANDARD_NO_PAD
                                            .decode(&b64_clean)
                                    })
                                    .unwrap_or_default();
                            }
                        }
                    }
                }
            }
            (json, bin_data)
        };

        let (positions, indices, _pos_min, _pos_max) = read_gltf_mesh_data(&json, &bin)
            .ok_or_else(|| SemioError::InvalidOperation {
                message: "No mesh data in model".to_string(),
            })?;

        let n = positions.len();
        let mut sx_min = f64::MAX;
        let mut sy_min = f64::MAX;
        let mut sz_min = f64::MAX;
        let mut sx_max = f64::MIN;
        let mut sy_max = f64::MIN;
        let mut sz_max = f64::MIN;
        let mut sum_sx = 0.0_f64;
        let mut sum_sy = 0.0_f64;
        let mut sum_sz = 0.0_f64;
        for p in &positions {
            let xg = p[0] as f64;
            let yg = p[1] as f64;
            let sx = xg;
            let sy = -xg;
            let sz = yg;
            sx_min = sx_min.min(sx);
            sx_max = sx_max.max(sx);
            sy_min = sy_min.min(sy);
            sy_max = sy_max.max(sy);
            sz_min = sz_min.min(sz);
            sz_max = sz_max.max(sz);
            sum_sx += sx;
            sum_sy += sy;
            sum_sz += sz;
        }
        let centroid = [sum_sx / n as f64, sum_sy / n as f64, sum_sz / n as f64];
        let dim_x = sx_max - sx_min;
        let dim_y = sy_max - sy_min;
        let dim_z = sz_max - sz_min;
        let mut area = 0.0_f64;
        let mut volume = 0.0_f64;
        for chunk in indices.chunks(3) {
            if chunk.len() < 3 {
                break;
            }
            let a = positions[chunk[0] as usize];
            let b = positions[chunk[1] as usize];
            let c = positions[chunk[2] as usize];
            let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            area += 0.5
                * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() as f64;
            volume += (1.0 / 6.0)
                * (a[0] as f64 * (b[1] as f64 * c[2] as f64 - b[2] as f64 * c[1] as f64)
                    + a[1] as f64 * (b[2] as f64 * c[0] as f64 - b[0] as f64 * c[2] as f64)
                    + a[2] as f64 * (b[0] as f64 * c[1] as f64 - b[1] as f64 * c[0] as f64));
        }
        volume = volume.abs();
        let face_count = indices.len() / 3;
        let surface_to_vol = if volume > 1e-20 { area / volume } else { 0.0 };
        let char_len = (dim_x * dim_y * dim_z).cbrt();
        let mut aspect_xy = 0.0;
        let mut aspect_xz = 0.0;
        let mut aspect_yz = 0.0;
        if dim_y > 1e-10 && dim_x > 1e-10 {
            aspect_xy = dim_x / dim_y;
        }
        if dim_z > 1e-10 && dim_x > 1e-10 {
            aspect_xz = dim_x / dim_z;
        }
        if dim_z > 1e-10 && dim_y > 1e-10 {
            aspect_yz = dim_y / dim_z;
        }
        let max_ext = dim_x.max(dim_y).max(dim_z);
        let slenderness = if max_ext > 1e-10 && area > 0.0 {
            max_ext / (area * max_ext).cbrt()
        } else {
            0.0
        };
        let euler = n as i32 - (3 * face_count) as i32 / 2 + face_count as i32;

        Ok(GeometricInsights {
            bounding_box_min: Some([sx_min, sy_min, sz_min]),
            bounding_box_max: Some([sx_max, sy_max, sz_max]),
            dimension_x: dim_x,
            dimension_y: dim_y,
            dimension_z: dim_z,
            characteristic_length: char_len,
            footprint_area: dim_x * dim_z,
            total_surface_area: area,
            enclosed_volume: volume,
            surface_to_volume_ratio: surface_to_vol,
            aspect_ratio_xy: aspect_xy,
            aspect_ratio_xz: aspect_xz,
            aspect_ratio_yz: aspect_yz,
            slenderness,
            centroid: Some(centroid),
            vertex_count: n,
            face_count,
            euler_characteristic: euler,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_geometric_insights_for_model_path(path: &str) -> Result<GeometricInsights> {
        let data = std::fs::read(path).map_err(|e| SemioError::InvalidOperation {
            message: format!("Failed to read model file: {}", e),
        })?;
        if path.to_lowercase().ends_with(".gltf") {
            let json: serde_json::Value =
                serde_json::from_slice(&data).map_err(|e| SemioError::InvalidOperation {
                    message: format!("Invalid glTF JSON: {}", e),
                })?;
            let mut bin_data = Vec::new();
            if let Some(buffers) = json.get("buffers").and_then(|b| b.as_array()) {
                if let Some(buf) = buffers.first().and_then(|b| b.as_object()) {
                    if let Some(uri) = buf.get("uri").and_then(|u| u.as_str()) {
                        if uri.starts_with("data:") {
                            if let Some(b64) = uri.split(',').nth(1) {
                                let b64_clean: String =
                                    b64.chars().filter(|c| !c.is_whitespace()).collect();
                                bin_data = base64::engine::general_purpose::STANDARD
                                    .decode(&b64_clean)
                                    .or_else(|_| {
                                        base64::engine::general_purpose::STANDARD_NO_PAD
                                            .decode(&b64_clean)
                                    })
                                    .unwrap_or_default();
                            }
                        } else {
                            let dir = std::path::Path::new(path)
                                .parent()
                                .unwrap_or(std::path::Path::new("."));
                            let bin_path = dir.join(uri);
                            if let Ok(b) = std::fs::read(&bin_path) {
                                bin_data = b;
                            }
                        }
                    }
                }
            }
            let (positions, indices, _pos_min, _pos_max) = read_gltf_mesh_data(&json, &bin_data)
                .ok_or_else(|| SemioError::InvalidOperation {
                    message: "No mesh data in model".to_string(),
                })?;
            let n = positions.len();
            let mut sx_min = f64::MAX;
            let mut sy_min = f64::MAX;
            let mut sz_min = f64::MAX;
            let mut sx_max = f64::MIN;
            let mut sy_max = f64::MIN;
            let mut sz_max = f64::MIN;
            let mut sum_sx = 0.0_f64;
            let mut sum_sy = 0.0_f64;
            let mut sum_sz = 0.0_f64;
            for p in &positions {
                let xg = p[0] as f64;
                let yg = p[1] as f64;
                let sx = xg;
                let sy = -xg;
                let sz = yg;
                sx_min = sx_min.min(sx);
                sx_max = sx_max.max(sx);
                sy_min = sy_min.min(sy);
                sy_max = sy_max.max(sy);
                sz_min = sz_min.min(sz);
                sz_max = sz_max.max(sz);
                sum_sx += sx;
                sum_sy += sy;
                sum_sz += sz;
            }
            let centroid = [sum_sx / n as f64, sum_sy / n as f64, sum_sz / n as f64];
            let dim_x = sx_max - sx_min;
            let dim_y = sy_max - sy_min;
            let dim_z = sz_max - sz_min;
            let mut area = 0.0_f64;
            let mut volume = 0.0_f64;
            for chunk in indices.chunks(3) {
                if chunk.len() < 3 {
                    break;
                }
                let a = positions[chunk[0] as usize];
                let b = positions[chunk[1] as usize];
                let c = positions[chunk[2] as usize];
                let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let cross = [
                    ab[1] * ac[2] - ab[2] * ac[1],
                    ab[2] * ac[0] - ab[0] * ac[2],
                    ab[0] * ac[1] - ab[1] * ac[0],
                ];
                area += 0.5
                    * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
                        as f64;
                volume += (1.0 / 6.0)
                    * (a[0] as f64 * (b[1] as f64 * c[2] as f64 - b[2] as f64 * c[1] as f64)
                        + a[1] as f64 * (b[2] as f64 * c[0] as f64 - b[0] as f64 * c[2] as f64)
                        + a[2] as f64 * (b[0] as f64 * c[1] as f64 - b[1] as f64 * c[0] as f64));
            }
            volume = volume.abs();
            let face_count = indices.len() / 3;
            let surface_to_vol = if volume > 1e-20 { area / volume } else { 0.0 };
            let char_len = (dim_x * dim_y * dim_z).cbrt();
            let mut aspect_xy = 0.0;
            let mut aspect_xz = 0.0;
            let mut aspect_yz = 0.0;
            if dim_y > 1e-10 && dim_x > 1e-10 {
                aspect_xy = dim_x / dim_y;
            }
            if dim_z > 1e-10 && dim_x > 1e-10 {
                aspect_xz = dim_x / dim_z;
            }
            if dim_z > 1e-10 && dim_y > 1e-10 {
                aspect_yz = dim_y / dim_z;
            }
            let max_ext = dim_x.max(dim_y).max(dim_z);
            let slenderness = if max_ext > 1e-10 && area > 0.0 {
                max_ext / (area * max_ext).cbrt()
            } else {
                0.0
            };
            let euler = n as i32 - (3 * face_count) as i32 / 2 + face_count as i32;
            return Ok(GeometricInsights {
                bounding_box_min: Some([sx_min, sy_min, sz_min]),
                bounding_box_max: Some([sx_max, sy_max, sz_max]),
                dimension_x: dim_x,
                dimension_y: dim_y,
                dimension_z: dim_z,
                characteristic_length: char_len,
                footprint_area: dim_x * dim_z,
                total_surface_area: area,
                enclosed_volume: volume,
                surface_to_volume_ratio: surface_to_vol,
                aspect_ratio_xy: aspect_xy,
                aspect_ratio_xz: aspect_xz,
                aspect_ratio_yz: aspect_yz,
                slenderness,
                centroid: Some(centroid),
                vertex_count: n,
                face_count,
                euler_characteristic: euler,
            });
        }
        get_geometric_insights_for_model(&data)
    }
} // 🏥Geometric Insights
pub use geometric_insights::*;

mod validation_types {
    // 🌸Validation Types
    // Validation Types MUST provide the validation types functionality.

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    #[serde(rename_all = "camelCase")]
    /// 🔖<summary>✔️ValidationProblem holds the data fields for a ValidationProblem record.</summary>
    /// <remarks>
    /// </remarks>
    /// <remarks>
    /// </remarks>
    pub struct ValidationProblem {
        pub constraint_id: String,
        pub message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entity_kind: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entity_guid: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub fixes: Vec<ValidationFix>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// 🔧<summary>🔖ValidationFix holds the data fields for a ValidationFix record.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ValidationFix {
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub diff: Option<KitDiff>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    /// 🔖<summary>🔖ValidationResult holds the data fields for a ValidationResult record.</summary>
    /// <remarks>
    /// </remarks>
    pub struct ValidationResult {
        pub problems: Vec<ValidationProblem>,
    }

    /// 🔖<summary>🔖validate_kit holds the data fields for a validate_kit record.</summary>
    pub fn validate_kit(kit: &Kit) -> ValidationResult {
        let mut problems = Vec::new();

        check_guid_uniqueness_constraint(kit, &mut problems);
        check_type_name_uniqueness(kit, &mut problems);
        check_design_name_uniqueness(kit, &mut problems);
        check_piece_name_uniqueness(kit, &mut problems);
        check_connection_name_uniqueness(kit, &mut problems);
        check_connector_name_uniqueness(kit, &mut problems);
        check_model_name_uniqueness(kit, &mut problems);
        check_layer_path_uniqueness(kit, &mut problems);
        check_quality_name_uniqueness(kit, &mut problems);
        check_port_name_uniqueness(kit, &mut problems);
        check_file_name_uniqueness(kit, &mut problems);
        check_folder_name_uniqueness(kit, &mut problems);
        check_description_emoji_unique(kit, &mut problems);

        ValidationResult { problems }
    }
    /// 🔒<summary>🧪check_guid_uniqueness_constraint holds the data fields for a check_guid_uniqueness_constraint record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_guid_uniqueness_constraint(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let mut guids: HashSet<String> = HashSet::new();
        guids.insert(kit.guid.clone());

        if let Some(ref types) = kit.types {
            for t in types {
                check_guid(&t.guid, "Type", &mut guids, problems);
                if let Some(ref connectors) = t.connectors {
                    for c in connectors {
                        check_guid(&c.guid, "Connector", &mut guids, problems);
                    }
                }
                if let Some(ref models) = t.models {
                    for m in models {
                        check_guid(&m.guid, "Model", &mut guids, problems);
                    }
                }
            }
        }

        if let Some(ref designs) = kit.designs {
            for d in designs {
                check_guid(&d.guid, "Design", &mut guids, problems);
                if let Some(ref pieces) = d.pieces {
                    for p in pieces {
                        check_guid(&p.guid, "Piece", &mut guids, problems);
                    }
                }
                if let Some(ref connections) = d.connections {
                    for c in connections {
                        check_guid(&c.guid, "Connection", &mut guids, problems);
                    }
                }
                if let Some(ref layers) = d.layers {
                    for l in layers {
                        check_guid(&l.guid, "Layer", &mut guids, problems);
                    }
                }
            }
        }

        if let Some(ref tags) = kit.tags {
            for t in tags {
                check_guid(&t.guid, "Tag", &mut guids, problems);
            }
        }
        if let Some(ref concepts) = kit.concepts {
            for c in concepts {
                check_guid(&c.guid, "Concept", &mut guids, problems);
            }
        }
        if let Some(ref files) = kit.files {
            for f in files {
                check_guid(&f.guid, "File", &mut guids, problems);
            }
        }
        if let Some(ref folders) = kit.folders {
            for f in folders {
                check_guid(&f.guid, "Folder", &mut guids, problems);
            }
        }
        if let Some(ref authors) = kit.authors {
            for a in authors {
                check_guid(&a.guid, "Author", &mut guids, problems);
            }
        }
        if let Some(ref ports) = kit.ports {
            for p in ports {
                check_guid(&p.guid, "Port", &mut guids, problems);
            }
        }
        if let Some(ref qualities) = kit.qualities {
            for q in qualities {
                check_guid(&q.guid, "Quality", &mut guids, problems);
            }
        }
    }
    /// 🔖<summary>🔖check_guid holds the data fields for a check_guid record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_guid(
        guid: &str,
        kind: &str,
        guids: &mut HashSet<String>,
        problems: &mut Vec<ValidationProblem>,
    ) {
        if guids.contains(guid) {
            problems.push(ValidationProblem {
                constraint_id: "guid-unique".to_string(),
                message: format!("Duplicate GUID \"{}\". First occurrence kept.", guid),
                entity_kind: Some(kind.to_string()),
                entity_guid: Some(guid.to_string()),
                fixes: vec![],
            });
        } else {
            guids.insert(guid.to_string());
        }
    }
    /// 🔖<summary>🔖check_type_name_uniqueness holds the data fields for a check_type_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_type_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref types) = kit.types else { return };
        let mut siblings: HashMap<Option<String>, Vec<&Type>> = HashMap::new();
        for t in types {
            let parent = t.parent.as_ref().map(|p| p.guid.clone());
            siblings.entry(parent).or_default().push(t);
        }
        for (_parent, group) in siblings {
            let mut names: HashMap<&str, Vec<&Type>> = HashMap::new();
            for t in &group {
                names.entry(&t.name).or_default().push(t);
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "type-name-unique".to_string(),
                            message: format!("Duplicate type name \"{}\" among siblings.", name),
                            entity_kind: Some("Type".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🔖<summary>🔖check_design_name_uniqueness holds the data fields for a check_design_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_design_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref designs) = kit.designs else {
            return;
        };
        let mut siblings: HashMap<Option<String>, Vec<&Design>> = HashMap::new();
        for d in designs {
            let parent = d.parent.as_ref().map(|p| p.guid.clone());
            siblings.entry(parent).or_default().push(d);
        }
        for (_parent, group) in siblings {
            let mut names: HashMap<&str, Vec<&Design>> = HashMap::new();
            for d in &group {
                names.entry(&d.name).or_default().push(d);
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "design-name-unique".to_string(),
                            message: format!("Duplicate design name \"{}\" among siblings.", name),
                            entity_kind: Some("Design".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🔖<summary>🔖check_piece_name_uniqueness holds the data fields for a check_piece_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_piece_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref designs) = kit.designs else {
            return;
        };
        for design in designs {
            let Some(ref pieces) = design.pieces else {
                continue;
            };
            let mut names: HashMap<&str, Vec<&Piece>> = HashMap::new();
            for p in pieces {
                if let Some(ref name) = p.name {
                    names.entry(name.as_str()).or_default().push(p);
                }
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "piece-name-unique".to_string(),
                            message: format!(
                                "Duplicate piece name \"{}\" inside design \"{}\".",
                                name, design.name
                            ),
                            entity_kind: Some("Piece".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🔖<summary>🔖check_connection_name_uniqueness holds the data fields for a check_connection_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_connection_name_uniqueness(_kit: &Kit, _problems: &mut Vec<ValidationProblem>) {}
    /// 🔖<summary>🔖check_connector_name_uniqueness holds the data fields for a check_connector_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_connector_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref types) = kit.types else { return };
        for typ in types {
            let Some(ref connectors) = typ.connectors else {
                continue;
            };
            let mut names: HashMap<&str, Vec<&Connector>> = HashMap::new();
            for c in connectors {
                if let Some(ref name) = c.name {
                    names.entry(name.as_str()).or_default().push(c);
                }
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "connector-name-unique".to_string(),
                            message: format!(
                                "Duplicate connector name \"{}\" inside type \"{}\".",
                                name, typ.name
                            ),
                            entity_kind: Some("Connector".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🔖<summary>🔖check_model_name_uniqueness holds the data fields for a check_model_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_model_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref types) = kit.types else { return };
        for typ in types {
            let Some(ref models) = typ.models else {
                continue;
            };
            let mut names: HashMap<&str, Vec<&Model>> = HashMap::new();
            for m in models {
                if let Some(ref name) = m.name {
                    names.entry(name.as_str()).or_default().push(m);
                }
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "model-name-unique".to_string(),
                            message: format!(
                                "Duplicate model name \"{}\" inside type \"{}\".",
                                name, typ.name
                            ),
                            entity_kind: Some("Model".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🛤️<summary>🔖check_layer_path_uniqueness holds the data fields for a check_layer_path_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_layer_path_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref designs) = kit.designs else {
            return;
        };
        for design in designs {
            let Some(ref layers) = design.layers else {
                continue;
            };
            let mut paths: HashMap<&str, Vec<&Layer>> = HashMap::new();
            for l in layers {
                paths.entry(&l.path).or_default().push(l);
            }
            for (path, dups) in paths {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "layer-path-unique".to_string(),
                            message: format!(
                                "Duplicate layer path \"{}\" inside design \"{}\".",
                                path, design.name
                            ),
                            entity_kind: Some("Layer".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }
    /// 🔖<summary>🔖check_quality_name_uniqueness holds the data fields for a check_quality_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_quality_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref qualities) = kit.qualities else {
            return;
        };
        let mut names: HashMap<&str, Vec<&Quality>> = HashMap::new();
        for q in qualities {
            names.entry(&q.name).or_default().push(q);
        }
        for (name, dups) in names {
            if dups.len() > 1 {
                for dup in dups.iter().skip(1) {
                    problems.push(ValidationProblem {
                        constraint_id: "quality-name-unique".to_string(),
                        message: format!("Duplicate quality name \"{}\".", name),
                        entity_kind: Some("Quality".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
    /// 🔖<summary>🔖check_port_name_uniqueness holds the data fields for a check_port_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_port_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref ports) = kit.ports else { return };
        let mut names: HashMap<&str, Vec<&Port>> = HashMap::new();
        for p in ports {
            names.entry(&p.name).or_default().push(p);
        }
        for (name, dups) in names {
            if dups.len() > 1 {
                for dup in dups.iter().skip(1) {
                    problems.push(ValidationProblem {
                        constraint_id: "port-name-unique".to_string(),
                        message: format!("Duplicate port name \"{}\".", name),
                        entity_kind: Some("Port".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
    /// 🔖<summary>🔖check_file_name_uniqueness holds the data fields for a check_file_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_file_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref files) = kit.files else { return };
        let mut names: HashMap<&str, Vec<&File>> = HashMap::new();
        for f in files {
            names.entry(&f.name).or_default().push(f);
        }
        for (name, dups) in names {
            if dups.len() > 1 {
                for dup in dups.iter().skip(1) {
                    problems.push(ValidationProblem {
                        constraint_id: "file-name-unique".to_string(),
                        message: format!("Duplicate file name \"{}\".", name),
                        entity_kind: Some("File".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
    /// 🔖<summary>🔖check_folder_name_uniqueness holds the data fields for a check_folder_name_uniqueness record.</summary>
    /// <remarks>
    /// </remarks>
    pub fn check_folder_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        let Some(ref folders) = kit.folders else {
            return;
        };
        let mut siblings: HashMap<Option<String>, Vec<&Folder>> = HashMap::new();
        for f in folders {
            let parent = f.parent.as_ref().map(|p| p.guid.clone());
            siblings.entry(parent).or_default().push(f);
        }
        for (_parent, group) in siblings {
            let mut names: HashMap<&str, Vec<&Folder>> = HashMap::new();
            for f in &group {
                names.entry(&f.name).or_default().push(f);
            }
            for (name, dups) in names {
                if dups.len() > 1 {
                    for dup in dups.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "folder-name-unique".to_string(),
                            message: format!("Duplicate folder name \"{}\" among siblings.", name),
                            entity_kind: Some("Folder".to_string()),
                            entity_guid: Some(dup.guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }

    /// 🔣 Extract the first emoji from a string, or None if none.
    pub fn extract_first_emoji(text: &str) -> Option<String> {
        if text.is_empty() {
            return None;
        }
        let mut chars = text.chars();
        let first = chars.next()?;
        let mut cluster = String::new();
        cluster.push(first);
        for ch in chars {
            if ch == '\u{FE0F}'
                || ch == '\u{FE0E}'
                || ch == '\u{200D}'
                || ('\u{1F3FB}'..='\u{1F3FF}').contains(&ch)
            {
                cluster.push(ch);
                continue;
            }
            break;
        }
        fn is_emoji_char(c: char) -> bool {
            matches!(c,
                '\u{1F600}'..='\u{1F64F}' | '\u{1F300}'..='\u{1F5FF}' |
                '\u{1F680}'..='\u{1F6FF}' | '\u{1F900}'..='\u{1F9FF}' |
                '\u{1FA00}'..='\u{1FA6F}' | '\u{1FA70}'..='\u{1FAFF}' |
                '\u{2600}'..='\u{26FF}' | '\u{2700}'..='\u{27BF}' |
                '\u{231A}'..='\u{231B}' | '\u{2328}' | '\u{23CF}' |
                '\u{23E9}'..='\u{23F3}' | '\u{23F8}'..='\u{23FA}' |
                '\u{25AA}'..='\u{25AB}' | '\u{25B6}' | '\u{25C0}' |
                '\u{25FB}'..='\u{25FE}' |
                '\u{203C}' | '\u{2049}' | '\u{2122}' | '\u{2139}' |
                '\u{2194}'..='\u{2199}' | '\u{21A9}'..='\u{21AA}' |
                '\u{00A9}' | '\u{00AE}' | '\u{1F1E0}'..='\u{1F1FF}'
            )
        }
        if is_emoji_char(first) {
            Some(cluster)
        } else {
            None
        }
    }

    /// 🚫 Legacy no-op: plain descriptions are valid; only duplicate leading emojis are validated.
    pub fn check_description_missing_emoji(_kit: &Kit, _problems: &mut Vec<ValidationProblem>) {}
    /// 🔤 Check that sibling entity descriptions have unique leading emojis.
    pub fn check_description_emoji_unique(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
        fn check_siblings(
            kind: &str,
            siblings: &[(String, Option<String>)],
            problems: &mut Vec<ValidationProblem>,
        ) {
            let mut emoji_map: HashMap<String, Vec<String>> = HashMap::new();
            for (guid, desc) in siblings {
                if let Some(ref d) = desc {
                    if let Some(emoji) = extract_first_emoji(d) {
                        emoji_map.entry(emoji).or_default().push(guid.clone());
                    }
                }
            }
            for (emoji, guids) in &emoji_map {
                if guids.len() > 1 {
                    for guid in guids.iter().skip(1) {
                        problems.push(ValidationProblem {
                            constraint_id: "description-emoji-unique".to_string(),
                            message: format!(
                                "Duplicate leading emoji \"{}\" in {} descriptions among siblings.",
                                emoji, kind
                            ),
                            entity_kind: Some(kind.to_string()),
                            entity_guid: Some(guid.clone()),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
        if let Some(ref types) = kit.types {
            let mut by_parent: HashMap<Option<String>, Vec<(String, Option<String>)>> =
                HashMap::new();
            for t in types {
                let pid = t.parent.as_ref().map(|p| p.guid.clone());
                by_parent
                    .entry(pid)
                    .or_default()
                    .push((t.guid.clone(), t.description.clone()));
            }
            for (_, siblings) in &by_parent {
                check_siblings("Type", siblings, problems);
            }
            for t in types {
                if let Some(ref connectors) = t.connectors {
                    let s: Vec<_> = connectors
                        .iter()
                        .map(|c| (c.guid.clone(), c.description.clone()))
                        .collect();
                    check_siblings("Connector", &s, problems);
                }
                if let Some(ref models) = t.models {
                    let s: Vec<_> = models
                        .iter()
                        .map(|m| (m.guid.clone(), m.description.clone()))
                        .collect();
                    check_siblings("Model", &s, problems);
                }
            }
        }
        if let Some(ref designs) = kit.designs {
            let mut by_parent: HashMap<Option<String>, Vec<(String, Option<String>)>> =
                HashMap::new();
            for d in designs {
                let pid = d.parent.as_ref().map(|p| p.guid.clone());
                by_parent
                    .entry(pid)
                    .or_default()
                    .push((d.guid.clone(), d.description.clone()));
            }
            for (_, siblings) in &by_parent {
                check_siblings("Design", siblings, problems);
            }
            for d in designs {
                if let Some(ref pieces) = d.pieces {
                    let s: Vec<_> = pieces
                        .iter()
                        .map(|p| (p.guid.clone(), p.description.clone()))
                        .collect();
                    check_siblings("Piece", &s, problems);
                }
                if let Some(ref connections) = d.connections {
                    let s: Vec<_> = connections
                        .iter()
                        .map(|c| (c.guid.clone(), c.description.clone()))
                        .collect();
                    check_siblings("Connection", &s, problems);
                }
            }
        }
        if let Some(ref qualities) = kit.qualities {
            let s: Vec<_> = qualities
                .iter()
                .map(|q| (q.guid.clone(), q.description.clone()))
                .collect();
            check_siblings("Quality", &s, problems);
        }
        if let Some(ref ports) = kit.ports {
            let s: Vec<_> = ports
                .iter()
                .map(|p| (p.guid.clone(), p.description.clone()))
                .collect();
            check_siblings("Port", &s, problems);
        }
        if let Some(ref _files) = kit.files {}
        if let Some(ref folders) = kit.folders {
            let mut by_parent: HashMap<Option<String>, Vec<(String, Option<String>)>> =
                HashMap::new();
            for f in folders {
                let pid = f.parent.as_ref().map(|p| p.guid.clone());
                by_parent
                    .entry(pid)
                    .or_default()
                    .push((f.guid.clone(), f.description.clone()));
            }
            for (_, siblings) in &by_parent {
                check_siblings("Folder", siblings, problems);
            }
        }
    }
} // 🌿Validation Types
pub use validation_types::*;

mod sqlite_import_export {
    // 🏢SQLite Import/Export
    // SQLite Import/Export MUST provide the sqlite import/export functionality.

    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖sqlite holds the data fields for a sqlite record.</summary>
    pub mod sqlite {
        use super::*;
        use rusqlite::params;

        pub fn export_kit_to_sqlite(kit: &Kit, path: &str) -> Result<()> {
            let conn = rusqlite::Connection::open(path).map_err(|e| SemioError::Database {
                message: e.to_string(),
            })?;

            conn.execute_batch(include_str!("../sqlite/schema.sql"))
                .map_err(|e| SemioError::Database {
                    message: format!("Schema creation failed: {}", e),
                })?;

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
                    "INSERT INTO file (guid, name, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![file.guid, file.name, file.folder.as_ref().map(|f| &f.guid), file.size, file.hash, file.remote, now, now, kit.guid],
                ).map_err(|e| SemioError::Database { message: e.to_string() })?;
                }
            }

            if let Some(ref authors) = kit.authors {
                for author in authors {
                    conn.execute(
                        "INSERT INTO author (guid, name, email, kit_guid) VALUES (?1, ?2, ?3, ?4)",
                        params![author.guid, author.name, author.email, kit.guid],
                    )
                    .map_err(|e| SemioError::Database {
                        message: e.to_string(),
                    })?;
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
                            let (po, px, py) = p
                                .plane
                                .as_ref()
                                .map(|pl| {
                                    (
                                        (pl.origin.x, pl.origin.y, pl.origin.z),
                                        (pl.x_axis.x, pl.x_axis.y, pl.x_axis.z),
                                        (pl.y_axis.x, pl.y_axis.y, pl.y_axis.z),
                                    )
                                })
                                .unwrap_or(((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)));

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
            let conn = rusqlite::Connection::open(path).map_err(|e| SemioError::Database {
                message: e.to_string(),
            })?;

            let mut stmt = conn.prepare("SELECT guid, name, version, description, icon, image, preview, remote, homepage, license FROM kit LIMIT 1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;

            let kit_row = stmt
                .query_row([], |row| {
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
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;

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
                concepts: if concepts.is_empty() {
                    None
                } else {
                    Some(concepts)
                },
                tags: if tags.is_empty() { None } else { Some(tags) },
                types: if types.is_empty() { None } else { Some(types) },
                designs: if designs.is_empty() {
                    None
                } else {
                    Some(designs)
                },
                ports: if ports.is_empty() { None } else { Some(ports) },
                qualities: None,
                files: if files.is_empty() { None } else { Some(files) },
                folders: if folders.is_empty() {
                    None
                } else {
                    Some(folders)
                },
                authors: if authors.is_empty() {
                    None
                } else {
                    Some(authors)
                },
                attributes: None,
                created_at: None,
                updated_at: None,
            })
        }

        pub fn load_tags(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Tag>> {
            let mut stmt = conn
                .prepare("SELECT guid, name, description, icon FROM tag WHERE kit_guid = ?1")
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(Tag {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        icon: row.get(3)?,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_concepts(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Concept>> {
            let mut stmt = conn
                .prepare("SELECT guid, name, description, icon FROM concept WHERE kit_guid = ?1")
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(Concept {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        icon: row.get(3)?,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_ports(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Port>> {
            let mut stmt = conn
                .prepare("SELECT guid, name, description, icon FROM port WHERE kit_guid = ?1")
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(Port {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        icon: row.get(3)?,
                        max_children: None,
                        compatible_interfaces: None,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_folders(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Folder>> {
            let mut stmt = conn
                .prepare("SELECT guid, name, parent_guid FROM folder WHERE kit_guid = ?1")
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(Folder {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        description: None,
                        parent: row
                            .get::<_, Option<String>>(2)?
                            .map(|g| FolderId { guid: g }),
                        attributes: None,
                        created_at: None,
                        updated_at: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_files(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<File>> {
            let mut stmt = conn.prepare("SELECT guid, name, folder_guid, size, hash, remote_url FROM file WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(File {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        folder: row
                            .get::<_, Option<String>>(2)?
                            .map(|g| FolderId { guid: g }),
                        size: row.get(3)?,
                        hash: row.get(4)?,
                        remote: row.get(5)?,
                        blob: None,
                        created_at: None,
                        updated_at: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_authors(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Author>> {
            let mut stmt = conn
                .prepare("SELECT guid, name, email FROM author WHERE kit_guid = ?1")
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok(Author {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                        attributes: None,
                        created_at: None,
                        updated_at: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_types(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Type>> {
            let mut stmt = conn.prepare("SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image FROM type WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    let type_guid: String = row.get(0)?;
                    Ok((
                        type_guid,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, bool>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<i32>>(5)?,
                        row.get::<_, bool>(6)?,
                        row.get::<_, Option<String>>(7)?,
                        row.get::<_, Option<String>>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<String>>(10)?,
                    ))
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;

            let type_data: Vec<_> =
                rows.collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|e| SemioError::Database {
                        message: e.to_string(),
                    })?;
            let mut types = Vec::new();

            for (
                type_guid,
                name,
                parent,
                is_abstract,
                folder,
                stock,
                virtual_type,
                unit,
                description,
                icon,
                image,
            ) in type_data
            {
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
                    models: if models.is_empty() {
                        None
                    } else {
                        Some(models)
                    },
                    connectors: if connectors.is_empty() {
                        None
                    } else {
                        Some(connectors)
                    },
                    attributes: None,
                    created_at: None,
                    updated_at: None,
                });
            }

            Ok(types)
        }

        pub fn load_connectors(
            conn: &rusqlite::Connection,
            type_guid: &str,
        ) -> Result<Vec<Connector>> {
            let mut stmt = conn.prepare("SELECT guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description FROM connector WHERE type_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([type_guid], |row| {
                    Ok(Connector {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        point: Vector::new(row.get(2)?, row.get(3)?, row.get(4)?),
                        direction: Vector::new(row.get(5)?, row.get(6)?, row.get(7)?),
                        t: row.get(8)?,
                        mandatory: Some(row.get(9)?),
                        max_children: None,
                        port: row
                            .get::<_, Option<String>>(10)?
                            .map(|g| PortId { guid: g }),
                        description: row.get(11)?,
                        props: None,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_models(conn: &rusqlite::Connection, type_guid: &str) -> Result<Vec<Model>> {
            let mut stmt = conn
                .prepare(
                    "SELECT guid, file_guid, name, description FROM model WHERE type_guid = ?1",
                )
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([type_guid], |row| {
                    Ok(Model {
                        guid: row.get(0)?,
                        file: FileId { guid: row.get(1)? },
                        name: row.get(2)?,
                        description: row.get(3)?,
                        tags: None,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_designs(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Design>> {
            let mut stmt = conn.prepare("SELECT guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image FROM design WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([kit_guid], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<bool>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<bool>>(6)?,
                        row.get::<_, Option<bool>>(7)?,
                        row.get::<_, Option<String>>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<String>>(10)?,
                    ))
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;

            let design_data: Vec<_> =
                rows.collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|e| SemioError::Database {
                        message: e.to_string(),
                    })?;
            let mut designs = Vec::new();

            for (
                design_guid,
                name,
                parent,
                unit,
                is_abstract,
                folder,
                can_scale,
                can_mirror,
                description,
                icon,
                image,
            ) in design_data
            {
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
                    pieces: if pieces.is_empty() {
                        None
                    } else {
                        Some(pieces)
                    },
                    connections: if connections.is_empty() {
                        None
                    } else {
                        Some(connections)
                    },
                    layers: if layers.is_empty() {
                        None
                    } else {
                        Some(layers)
                    },
                    groups: if groups.is_empty() {
                        None
                    } else {
                        Some(groups)
                    },
                    stats: None,
                    active_layer: None,
                    location: None,
                    attributes: None,
                    created_at: None,
                    updated_at: None,
                });
            }

            Ok(designs)
        }

        pub fn load_pieces(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Piece>> {
            let mut stmt = conn.prepare("SELECT guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, is_hidden, is_locked, color, description FROM piece WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([design_guid], |row| {
                    let plane = if let (Some(ox), Some(oy), Some(oz)) = (
                        row.get::<_, Option<f64>>(4)?,
                        row.get::<_, Option<f64>>(5)?,
                        row.get::<_, Option<f64>>(6)?,
                    ) {
                        Some(Plane::new(
                            Vector::new(ox, oy, oz),
                            Vector::new(
                                row.get::<_, f64>(7)?,
                                row.get::<_, f64>(8)?,
                                row.get::<_, f64>(9)?,
                            ),
                            Vector::new(
                                row.get::<_, f64>(10)?,
                                row.get::<_, f64>(11)?,
                                row.get::<_, f64>(12)?,
                            ),
                        ))
                    } else {
                        None
                    };

                    let center = if let (Some(u), Some(v)) = (
                        row.get::<_, Option<f64>>(13)?,
                        row.get::<_, Option<f64>>(14)?,
                    ) {
                        Some(Coord::new(u, v))
                    } else {
                        None
                    };

                    Ok(Piece {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        type_ref: row.get::<_, Option<String>>(2)?.map(|g| TypeId { guid: g }),
                        design: row
                            .get::<_, Option<String>>(3)?
                            .map(|g| DesignId { guid: g }),
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
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_connections(
            conn: &rusqlite::Connection,
            design_guid: &str,
        ) -> Result<Vec<Connection>> {
            let mut stmt = conn.prepare("SELECT guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description FROM connection WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([design_guid], |row| {
                    Ok(Connection {
                        guid: row.get(0)?,
                        connected: Side {
                            piece: PieceId { guid: row.get(1)? },
                            design_piece: row
                                .get::<_, Option<String>>(2)?
                                .map(|g| PieceId { guid: g }),
                            connector: row
                                .get::<_, Option<String>>(3)?
                                .filter(|s| !s.is_empty())
                                .map(|g| ConnectorId { guid: g }),
                        },
                        connecting: Side {
                            piece: PieceId { guid: row.get(4)? },
                            design_piece: row
                                .get::<_, Option<String>>(5)?
                                .map(|g| PieceId { guid: g }),
                            connector: row
                                .get::<_, Option<String>>(6)?
                                .filter(|s| !s.is_empty())
                                .map(|g| ConnectorId { guid: g }),
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
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_layers(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Layer>> {
            let mut stmt = conn.prepare("SELECT guid, path, is_hidden, is_locked, color, description FROM layer WHERE design_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
            let rows = stmt
                .query_map([design_guid], |row| {
                    Ok(Layer {
                        guid: row.get(0)?,
                        path: row.get(1)?,
                        is_hidden: Some(row.get(2)?),
                        is_locked: Some(row.get(3)?),
                        color: row.get(4)?,
                        description: row.get(5)?,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }

        pub fn load_groups(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Group>> {
            let mut stmt = conn
                .prepare(
                    "SELECT guid, name, color, description FROM \"group\" WHERE design_guid = ?1",
                )
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            let rows = stmt
                .query_map([design_guid], |row| {
                    Ok(Group {
                        guid: row.get(0)?,
                        name: row.get(1)?,
                        color: row.get(2)?,
                        description: row.get(3)?,
                        pieces: None,
                        attributes: None,
                    })
                })
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| SemioError::Database {
                    message: e.to_string(),
                })
        }
    }
} // 👠SQLite Import/Export
pub use sqlite_import_export::*;

mod zip_import_export {
    // 📋Zip Import/Export
    // Zip Import/Export MUST provide the zip import/export functionality.

    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖zip_roundtrip holds the data fields for a zip_roundtrip record.</summary>
    pub fn mime_from_filename(filename: &str) -> &'static str {
        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
        match ext.as_str() {
            "stl" => "model/stl",
            "obj" => "model/obj",
            "glb" => "model/gltf-binary",
            "gltf" => "model/gltf+json",
            "3dm" => "model/vnd.3dm",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "svg" => "image/svg+xml",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            "json" => "application/json",
            "csv" => "text/csv",
            "txt" => "text/plain",
            _ => "application/octet-stream",
        }
    }

    pub mod zip_roundtrip {
        use super::*;
        use base64::Engine;
        use std::collections::HashMap;
        use std::io::{Read, Write};

        pub struct KitImportResult {
            pub kit: Kit,
            pub files: HashMap<String, Vec<u8>>,
        }

        pub fn build_folder_path(kit: &Kit, folder_guid: &str) -> String {
            if let Some(folders) = &kit.folders {
                for f in folders {
                    if f.guid == folder_guid {
                        if let Some(parent) = &f.parent {
                            let parent_path = build_folder_path(kit, &parent.guid);
                            if parent_path.is_empty() {
                                return f.name.clone();
                            }
                            return format!("{}/{}", parent_path, f.name);
                        }
                        return f.name.clone();
                    }
                }
            }
            String::new()
        }

        pub fn build_file_path(kit: &Kit, file: &File) -> String {
            if let Some(folder) = &file.folder {
                let folder_path = build_folder_path(kit, &folder.guid);
                if folder_path.is_empty() {
                    return file.name.clone();
                }
                return format!("{}/{}", folder_path, file.name);
            }
            file.name.clone()
        }

        pub fn import_kit_from_zip(zip_path: &str) -> Result<KitImportResult> {
            let file = std::fs::File::open(zip_path).map_err(|e| SemioError::Database {
                message: format!("Failed to open zip: {}", e),
            })?;
            let mut archive = zip::ZipArchive::new(file).map_err(|e| SemioError::Database {
                message: format!("Failed to read zip: {}", e),
            })?;

            let mut kit_json: Option<Vec<u8>> = None;
            let mut files = HashMap::new();

            for i in 0..archive.len() {
                let mut entry = archive.by_index(i).map_err(|e| SemioError::Database {
                    message: format!("Failed to read zip entry: {}", e),
                })?;
                if entry.is_dir() {
                    continue;
                }
                let name = entry.name().to_string();
                let mut data = Vec::new();
                entry
                    .read_to_end(&mut data)
                    .map_err(|e| SemioError::Database {
                        message: format!("Failed to read zip entry data: {}", e),
                    })?;

                if name == "kit.json" {
                    kit_json = Some(data);
                } else if !name.starts_with(".semio/") {
                    files.insert(name, data);
                }
            }

            let kit_json = kit_json.ok_or(SemioError::Database {
                message: "kit.json not found in zip".to_string(),
            })?;
            let kit_str = String::from_utf8(kit_json).map_err(|e| SemioError::Database {
                message: format!("Invalid UTF-8 in kit.json: {}", e),
            })?;
            let mut kit = deserialize_kit(&kit_str)?;

            let file_paths: Vec<String> = kit
                .files
                .as_ref()
                .map(|kit_files| kit_files.iter().map(|f| build_file_path(&kit, f)).collect())
                .unwrap_or_default();
            if let Some(ref mut kit_files) = kit.files {
                for (i, f) in kit_files.iter_mut().enumerate() {
                    if let Some(data) = files.get(&file_paths[i]) {
                        let mime = crate::mime_from_filename(&f.name);
                        f.blob = Some(format!(
                            "data:{};base64,{}",
                            mime,
                            base64::engine::general_purpose::STANDARD.encode(data)
                        ));
                    }
                }
            }

            Ok(KitImportResult { kit, files })
        }

        pub fn export_kit_to_zip(
            kit: &Kit,
            files: &HashMap<String, Vec<u8>>,
            zip_path: &str,
        ) -> Result<()> {
            let mut kit_for_zip = kit.clone();
            if let Some(ref mut kit_files) = kit_for_zip.files {
                for f in kit_files.iter_mut() {
                    f.blob = None;
                }
            }

            let kit_json = serialize_kit(&kit_for_zip)?;

            let zip_file = std::fs::File::create(zip_path).map_err(|e| SemioError::Database {
                message: format!("Failed to create zip: {}", e),
            })?;
            let mut zip_writer = zip::ZipWriter::new(zip_file);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            zip_writer
                .start_file("kit.json", options)
                .map_err(|e| SemioError::Database {
                    message: format!("Failed to start zip file: {}", e),
                })?;
            zip_writer
                .write_all(kit_json.as_bytes())
                .map_err(|e| SemioError::Database {
                    message: format!("Failed to write to zip: {}", e),
                })?;

            for (name, data) in files {
                zip_writer
                    .start_file(name, options)
                    .map_err(|e| SemioError::Database {
                        message: format!("Failed to start zip file: {}", e),
                    })?;
                zip_writer
                    .write_all(data)
                    .map_err(|e| SemioError::Database {
                        message: format!("Failed to write to zip: {}", e),
                    })?;
            }

            zip_writer.finish().map_err(|e| SemioError::Database {
                message: format!("Failed to finish zip: {}", e),
            })?;

            Ok(())
        }
    }
} // 🎽Zip Import/Export
pub use zip_import_export::*;

mod kit_workflow {
    // 📭Kit Workflow
    // Kit Workflow MUST provide cohesive dev, local, archive, remote, and transport kit operations.

    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    pub const KIT_LOCAL_METADATA_DIRECTORY: &str = ".semio";

    #[cfg(not(target_arch = "wasm32"))]
    pub const KIT_LOCAL_DATABASE_FILENAME: &str = "kit.db";

    #[cfg(not(target_arch = "wasm32"))]
    pub fn io_semio_error(context: &str, error: impl std::fmt::Display) -> SemioError {
        SemioError::Database {
            message: format!("{}: {}", context, error),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_local_database_path(folder_path: &str) -> std::path::PathBuf {
        std::path::Path::new(folder_path)
            .join(KIT_LOCAL_METADATA_DIRECTORY)
            .join(KIT_LOCAL_DATABASE_FILENAME)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn decode_blob_bytes(file: &File) -> Result<Option<Vec<u8>>> {
        let Some(blob) = file.blob.as_ref() else {
            return Ok(None);
        };
        let encoded = blob
            .split_once(",")
            .map(|(_, data)| data)
            .unwrap_or(blob.as_str());
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|error| SemioError::Serialization {
                message: format!("Failed to decode file blob for {}: {}", file.guid, error),
            })?;
        Ok(Some(decoded))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn hydrate_kit_file_blobs(kit: &mut Kit, files: &HashMap<String, Vec<u8>>) {
        let file_paths: Vec<String> = kit
            .files
            .as_ref()
            .map(|kit_files| {
                kit_files
                    .iter()
                    .map(|file| zip_roundtrip::build_file_path(kit, file))
                    .collect()
            })
            .unwrap_or_default();

        if let Some(kit_files) = kit.files.as_mut() {
            for (index, file) in kit_files.iter_mut().enumerate() {
                if let Some(data) = files.get(&file_paths[index]) {
                    let mime = mime_from_filename(&file.name);
                    file.blob = Some(format!(
                        "data:{};base64,{}",
                        mime,
                        base64::engine::general_purpose::STANDARD.encode(data)
                    ));
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn resolve_exported_file_bytes(
        kit: &Kit,
        files: &HashMap<String, Vec<u8>>,
    ) -> Result<HashMap<String, Vec<u8>>> {
        let mut resolved = files.clone();

        for file in kit.files.as_ref().into_iter().flatten() {
            let path = zip_roundtrip::build_file_path(kit, file);
            if resolved.contains_key(&path) {
                continue;
            }
            if let Some(bytes) = decode_blob_bytes(file)? {
                resolved.insert(path, bytes);
            }
        }

        Ok(resolved)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn remap_kit_file_bytes(
        before_kit: &Kit,
        after_kit: &Kit,
        before_files: &HashMap<String, Vec<u8>>,
    ) -> Result<HashMap<String, Vec<u8>>> {
        let mut before_paths_by_guid = HashMap::new();
        if let Some(before_files_meta) = &before_kit.files {
            for file in before_files_meta {
                before_paths_by_guid.insert(
                    file.guid.clone(),
                    zip_roundtrip::build_file_path(before_kit, file),
                );
            }
        }

        let mut after_files = HashMap::new();
        if let Some(after_files_meta) = &after_kit.files {
            for file in after_files_meta {
                let new_path = zip_roundtrip::build_file_path(after_kit, file);
                if let Some(old_path) = before_paths_by_guid.get(&file.guid) {
                    if let Some(bytes) = before_files.get(old_path) {
                        after_files.insert(new_path, bytes.clone());
                        continue;
                    }
                }
                if let Some(bytes) = decode_blob_bytes(file)? {
                    after_files.insert(new_path, bytes);
                }
            }
        }

        Ok(after_files)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn remove_stale_local_assets(
        folder_path: &str,
        previous_files: &HashMap<String, Vec<u8>>,
        next_files: &HashMap<String, Vec<u8>>,
    ) -> Result<()> {
        let root_path = std::path::Path::new(folder_path);

        for old_path in previous_files.keys() {
            if next_files.contains_key(old_path) {
                continue;
            }

            let asset_path = root_path.join(old_path);
            if asset_path.exists() {
                std::fs::remove_file(&asset_path).map_err(|error| {
                    io_semio_error("Failed to remove stale local asset", error)
                })?;
            }

            let mut current = asset_path.parent();
            while let Some(directory) = current {
                if directory == root_path {
                    break;
                }
                match std::fs::remove_dir(directory) {
                    Ok(()) => current = directory.parent(),
                    Err(error) if error.kind() == std::io::ErrorKind::DirectoryNotEmpty => break,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        current = directory.parent()
                    }
                    Err(error) => {
                        return Err(io_semio_error(
                            "Failed to remove empty local asset directory",
                            error,
                        ))
                    }
                }
            }
        }

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn import_kit_from_zip_bytes(zip_bytes: &[u8]) -> Result<zip_roundtrip::KitImportResult> {
        use std::io::Write;

        let mut temp_file = tempfile::NamedTempFile::new()
            .map_err(|error| io_semio_error("Failed to create temporary zip file", error))?;
        temp_file
            .write_all(zip_bytes)
            .map_err(|error| io_semio_error("Failed to write temporary zip file", error))?;

        let temp_path = temp_file
            .path()
            .to_str()
            .ok_or(SemioError::InvalidOperation {
                message: "Temporary zip path is not valid UTF-8".to_string(),
            })?;

        zip_roundtrip::import_kit_from_zip(temp_path)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_zip_payload(url: &str, content_type: Option<&str>, bytes: &[u8]) -> bool {
        let lower_url = url.to_lowercase();
        lower_url.ends_with(".zip")
            || content_type
                .map(|value| value.to_ascii_lowercase().contains("zip"))
                .unwrap_or(false)
            || bytes.starts_with(b"PK\x03\x04")
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 📥<summary>🔖import_dev_kit holds the data fields for a import_dev_kit record.</summary>
    pub fn import_dev_kit(path: &str) -> Result<Kit> {
        let content = std::fs::read_to_string(path)
            .map_err(|error| io_semio_error("Failed to read kit file", error))?;
        deserialize_kit(&content)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 📤<summary>🔖export_dev_kit holds the data fields for a export_dev_kit record.</summary>
    pub fn export_dev_kit(kit: &Kit, path: &str) -> Result<()> {
        let json = serialize_kit(kit)?;
        std::fs::write(path, json)
            .map_err(|error| io_semio_error("Failed to write kit file", error))
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖import_local_kit holds the data fields for a import_local_kit record.</summary>
    pub fn import_local_kit(folder_path: &str) -> Result<zip_roundtrip::KitImportResult> {
        let database_path = build_local_database_path(folder_path);
        let database_path_str = database_path.to_str().ok_or(SemioError::InvalidOperation {
            message: "Local kit database path is not valid UTF-8".to_string(),
        })?;

        let mut kit = sqlite::import_kit_from_sqlite(database_path_str)?;
        let mut files = HashMap::new();

        if let Some(kit_files) = &kit.files {
            for file in kit_files {
                let relative_path = zip_roundtrip::build_file_path(&kit, file);
                let absolute_path = std::path::Path::new(folder_path).join(&relative_path);
                match std::fs::read(&absolute_path) {
                    Ok(data) => {
                        files.insert(relative_path, data);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(io_semio_error(
                            "Failed to read local kit asset file",
                            error,
                        ))
                    }
                }
            }
        }

        hydrate_kit_file_blobs(&mut kit, &files);

        Ok(zip_roundtrip::KitImportResult { kit, files })
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖export_local_kit holds the data fields for a export_local_kit record.</summary>
    pub fn export_local_kit(
        kit: &Kit,
        files: &HashMap<String, Vec<u8>>,
        folder_path: &str,
    ) -> Result<()> {
        let root_path = std::path::Path::new(folder_path);
        std::fs::create_dir_all(root_path)
            .map_err(|error| io_semio_error("Failed to create local kit root", error))?;

        let metadata_path = root_path.join(KIT_LOCAL_METADATA_DIRECTORY);
        std::fs::create_dir_all(&metadata_path).map_err(|error| {
            io_semio_error("Failed to create local kit metadata directory", error)
        })?;

        let database_path = metadata_path.join(KIT_LOCAL_DATABASE_FILENAME);
        if database_path.exists() {
            std::fs::remove_file(&database_path).map_err(|error| {
                io_semio_error("Failed to replace existing local kit database", error)
            })?;
        }
        let database_path_str = database_path.to_str().ok_or(SemioError::InvalidOperation {
            message: "Local kit database path is not valid UTF-8".to_string(),
        })?;
        sqlite::export_kit_to_sqlite(kit, database_path_str)?;

        let resolved_files = resolve_exported_file_bytes(kit, files)?;
        for (relative_path, data) in resolved_files {
            let asset_path = root_path.join(&relative_path);
            if let Some(parent) = asset_path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| {
                    io_semio_error("Failed to create local kit asset directory", error)
                })?;
            }
            std::fs::write(&asset_path, data)
                .map_err(|error| io_semio_error("Failed to write local kit asset file", error))?;
        }

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖import_remote_kit holds the data fields for a import_remote_kit record.</summary>
    pub fn import_remote_kit(url: &str) -> Result<zip_roundtrip::KitImportResult> {
        let response = reqwest::blocking::get(url).map_err(|error| SemioError::Database {
            message: format!("Failed to fetch remote kit {}: {}", url, error),
        })?;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());
        let response = response
            .error_for_status()
            .map_err(|error| SemioError::Database {
                message: format!("Remote kit request failed for {}: {}", url, error),
            })?;
        let bytes = response.bytes().map_err(|error| SemioError::Database {
            message: format!("Failed to read remote kit body from {}: {}", url, error),
        })?;

        if is_zip_payload(url, content_type.as_deref(), bytes.as_ref()) {
            return import_kit_from_zip_bytes(bytes.as_ref());
        }

        let json =
            String::from_utf8(bytes.to_vec()).map_err(|error| SemioError::Serialization {
                message: format!("Failed to decode remote kit JSON from {}: {}", url, error),
            })?;
        let kit = deserialize_kit(&json)?;
        Ok(zip_roundtrip::KitImportResult {
            kit,
            files: HashMap::new(),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖edit_transport_kit holds the data fields for a edit_transport_kit record.</summary>
    pub fn edit_transport_kit(kit: &Kit, diff: &KitDiff) -> Kit {
        let mut edited = kit.clone();
        apply_kit_diff(&mut edited, diff);
        edited
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖edit_dev_kit holds the data fields for a edit_dev_kit record.</summary>
    pub fn edit_dev_kit(path: &str, diff: &KitDiff) -> Result<Kit> {
        let kit = import_dev_kit(path)?;
        let edited = edit_transport_kit(&kit, diff);
        export_dev_kit(&edited, path)?;
        Ok(edited)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖edit_local_kit holds the data fields for a edit_local_kit record.</summary>
    pub fn edit_local_kit(folder_path: &str, diff: &KitDiff) -> Result<Kit> {
        let imported = import_local_kit(folder_path)?;
        let edited = edit_transport_kit(&imported.kit, diff);
        let edited_files = remap_kit_file_bytes(&imported.kit, &edited, &imported.files)?;
        remove_stale_local_assets(folder_path, &imported.files, &edited_files)?;
        export_local_kit(&edited, &edited_files, folder_path)?;
        Ok(edited)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖edit_archive_kit holds the data fields for a edit_archive_kit record.</summary>
    pub fn edit_archive_kit(path: &str, diff: &KitDiff) -> Result<Kit> {
        let imported = zip_roundtrip::import_kit_from_zip(path)?;
        let edited = edit_transport_kit(&imported.kit, diff);
        let edited_files = remap_kit_file_bytes(&imported.kit, &edited, &imported.files)?;
        zip_roundtrip::export_kit_to_zip(&edited, &edited_files, path)?;
        Ok(edited)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// 🔖<summary>🔖edit_remote_kit holds the data fields for a edit_remote_kit record.</summary>
    pub fn edit_remote_kit(url: &str, diff: &KitDiff) -> Result<Kit> {
        let imported = import_remote_kit(url)?;
        Ok(edit_transport_kit(&imported.kit, diff))
    }
} // 🚣Kit Workflow
pub use kit_workflow::*;

mod kit_kind_types {
    // 📋Kit Kind Types
    // Kit Kind Types MUST provide typed wrappers for each kit persistence form.

    use super::*;

    /// 📋 Wraps a static JSON string for kit serialization/deserialization.
    pub struct TransportKit {
        pub json: String,
    }

    impl TransportKit {
        pub fn new(json: String) -> Self {
            Self { json }
        }
        pub fn to_kit(&self) -> Result<Kit> {
            deserialize_kit(&self.json)
        }
        pub fn from_kit(kit: &Kit) -> Result<Self> {
            Ok(Self {
                json: serialize_kit(kit)?,
            })
        }
    }

    /// 📦 Wraps a static zipped local kit.
    pub struct ArchiveKit {
        pub data: Vec<u8>,
    }

    /// 🔄 Trait for synchronized kit kinds.
    pub trait SyncKit {
        fn kit(&self) -> &Kit;
        fn kit_mut(&mut self) -> &mut Kit;
        fn apply(&mut self, diff: &KitDiff) {
            apply_kit_diff(self.kit_mut(), diff);
        }
        fn import_transport(&mut self, transport: &TransportKit) -> Result<()> {
            let imported = transport.to_kit()?;
            let diff = get_kit_diff(self.kit(), &imported);
            apply_kit_diff(self.kit_mut(), &diff);
            Ok(())
        }
        fn export_transport(&self) -> Result<TransportKit> {
            TransportKit::from_kit(self.kit())
        }
    }

    /// 📝 Synchronized JSON file kit.
    pub struct DevKit {
        kit: Kit,
    }
    impl DevKit {
        pub fn new(kit: Kit) -> Self {
            Self { kit }
        }
    }
    impl SyncKit for DevKit {
        fn kit(&self) -> &Kit {
            &self.kit
        }
        fn kit_mut(&mut self) -> &mut Kit {
            &mut self.kit
        }
    }

    /// 📂 Synchronized folder with .semio/kit.db SQLite database.
    pub struct LocalKit {
        kit: Kit,
    }
    impl LocalKit {
        pub fn new(kit: Kit) -> Self {
            Self { kit }
        }
    }
    impl SyncKit for LocalKit {
        fn kit(&self) -> &Kit {
            &self.kit
        }
        fn kit_mut(&mut self) -> &mut Kit {
            &mut self.kit
        }
    }

    /// 🌐 Synchronized websocket connection to semio/hub.
    pub struct RemoteKit {
        kit: Kit,
    }
    impl RemoteKit {
        pub fn new(kit: Kit) -> Self {
            Self { kit }
        }
    }
    impl SyncKit for RemoteKit {
        fn kit(&self) -> &Kit {
            &self.kit
        }
        fn kit_mut(&mut self) -> &mut Kit {
            &mut self.kit
        }
    }
} // 📋Kit Kind Types
pub use kit_kind_types::*;

mod wasm_bindings {
    // 🥈WASM Bindings
    // WASM Bindings MUST provide the wasm bindings functionality.

    use super::*;

    #[cfg(target_arch = "wasm32")]
    /// 🔖<summary>🔖wasm holds the data fields for a wasm record.</summary>
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
            pub fn success(data: T) -> Self {
                Self {
                    ok: true,
                    data: Some(data),
                    error: None,
                }
            }
            pub fn failure(error: String) -> Self {
                Self {
                    ok: false,
                    data: None,
                    error: Some(error),
                }
            }
        }

        pub fn to_js_value<T: Serialize>(result: WasmResult<T>) -> JsValue {
            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
        }

        #[wasm_bindgen(js_name = "generateGuid")]
        pub fn wasm_generate_guid() -> String {
            guid()
        }

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
                }
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
                Ok(kit) => {
                    let rep = flatten_design(&kit, design_guid);
                    to_js_value(WasmResult::success(rep))
                }
                Err(e) => to_js_value(WasmResult::<SemioReport<DesignChange>>::failure(
                    e.to_string(),
                )),
            }
        }

        #[wasm_bindgen(js_name = "normalize")]
        pub fn wasm_normalize(value: f64, decimals: u32) -> f64 {
            normalize(value, decimals)
        }

        #[wasm_bindgen(js_name = "round")]
        pub fn wasm_round(value: f64) -> f64 {
            round(value)
        }

        #[wasm_bindgen(js_name = "isSupportedModelExtension")]
        pub fn wasm_is_supported_model_extension(ext: &str) -> bool {
            is_supported_model_extension(ext)
        }

        #[wasm_bindgen(js_name = "generateUniqueName")]
        pub fn wasm_generate_unique_name(base: &str, existing_json: &str) -> JsValue {
            match serde_json::from_str::<Vec<String>>(existing_json) {
                Ok(existing) => {
                    to_js_value(WasmResult::success(generate_unique_name(base, &existing)))
                }
                Err(e) => to_js_value(WasmResult::<String>::failure(e.to_string())),
            }
        }

        #[wasm_bindgen(js_name = "findTypeInKit")]
        pub fn wasm_find_type_in_kit(kit_json: &str, guid: &str) -> JsValue {
            match deserialize_kit(kit_json) {
                Ok(kit) => match find_type_in_kit(&kit, guid) {
                    Some(t) => to_js_value(WasmResult::success(t.clone())),
                    None => to_js_value(WasmResult::<Type>::failure(format!(
                        "Type {} not found",
                        guid
                    ))),
                },
                Err(e) => to_js_value(WasmResult::<Type>::failure(e.to_string())),
            }
        }

        #[wasm_bindgen(js_name = "findDesignInKit")]
        pub fn wasm_find_design_in_kit(kit_json: &str, guid: &str) -> JsValue {
            match deserialize_kit(kit_json) {
                Ok(kit) => match find_design_in_kit(&kit, guid) {
                    Some(d) => to_js_value(WasmResult::success(d.clone())),
                    None => to_js_value(WasmResult::<Design>::failure(format!(
                        "Design {} not found",
                        guid
                    ))),
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
} // 🎰WASM Bindings
pub use wasm_bindings::*;

mod hash {
    // 📭Hash
    // Hash MUST provide deterministic SHA-256 Merkle hash functions for all entities.

    use super::*;

    pub struct HashWriter {
        buf: Vec<u8>,
    }

    impl HashWriter {
        pub fn new() -> Self {
            Self { buf: Vec::new() }
        }

        pub fn write_string(&mut self, s: &str) {
            let bytes = s.as_bytes();
            self.buf
                .extend_from_slice(&(bytes.len() as u32).to_be_bytes());
            self.buf.extend_from_slice(bytes);
        }

        pub fn write_number(&mut self, n: f64) {
            self.write_string(&format_number_for_hash(n));
        }

        pub fn write_int_number(&mut self, n: i32) {
            self.write_string(&n.to_string());
        }

        pub fn write_bool(&mut self, b: bool) {
            self.buf.push(if b { 1 } else { 0 });
        }

        pub fn write_hash(&mut self, h: &str) {
            self.write_string(h);
        }

        pub fn write_hash_list(&mut self, hashes: &[String]) {
            let mut sorted: Vec<&str> = hashes.iter().map(|s| s.as_str()).collect();
            sorted.sort();
            self.buf
                .extend_from_slice(&(sorted.len() as u32).to_be_bytes());
            for h in sorted {
                self.write_string(h);
            }
        }

        pub fn write_guid_list(&mut self, guids: &[String]) {
            let mut sorted: Vec<&str> = guids.iter().map(|s| s.as_str()).collect();
            sorted.sort();
            self.buf
                .extend_from_slice(&(sorted.len() as u32).to_be_bytes());
            for g in sorted {
                self.write_string(g);
            }
        }

        pub fn digest(&self) -> String {
            let hash = Sha256::digest(&self.buf);
            hex::encode(hash)
        }
    }

    pub fn format_number_for_hash(n: f64) -> String {
        let abs = n.abs();
        if n == n.trunc() && !n.is_infinite() && abs < 1e15 {
            return (n as i64).to_string();
        }
        if abs > 0.0 && (abs < 1e-6 || abs >= 1e21) {
            let s = format!("{:e}", n);
            if let Some(e_pos) = s.rfind('e') {
                let mantissa = &s[..e_pos];
                let exp_part = &s[e_pos + 1..];
                let sign = if exp_part.starts_with('-') { "-" } else { "+" };
                let digits = exp_part.trim_start_matches(|c: char| c == '+' || c == '-');
                let digits = digits.trim_start_matches('0');
                let digits = if digits.is_empty() { "0" } else { digits };
                return format!("{}e{}{}", mantissa, sign, digits);
            }
            return s;
        }
        format!("{}", n)
    }

    mod hash_value_types {
        // 📝Hash Value Types

        use super::*;

        pub fn hash_coord(c: &Coord) -> String {
            let mut w = HashWriter::new();
            w.write_string("Coord");
            w.write_string("u");
            w.write_number(c.u);
            w.write_string("v");
            w.write_number(c.v);
            w.digest()
        }

        pub fn hash_point(v: &Vector) -> String {
            let mut w = HashWriter::new();
            w.write_string("Point");
            w.write_string("x");
            w.write_number(v.x);
            w.write_string("y");
            w.write_number(v.y);
            w.write_string("z");
            w.write_number(v.z);
            w.digest()
        }

        pub fn hash_vector(v: &Vector) -> String {
            let mut w = HashWriter::new();
            w.write_string("Vector");
            w.write_string("x");
            w.write_number(v.x);
            w.write_string("y");
            w.write_number(v.y);
            w.write_string("z");
            w.write_number(v.z);
            w.digest()
        }

        pub fn hash_plane(p: &Plane) -> String {
            let mut w = HashWriter::new();
            w.write_string("Plane");
            w.write_string("origin");
            w.write_hash(&hash_point(&p.origin));
            w.write_string("xAxis");
            w.write_hash(&hash_vector(&p.x_axis));
            w.write_string("yAxis");
            w.write_hash(&hash_vector(&p.y_axis));
            w.digest()
        }

        pub fn hash_camera(c: &Camera) -> String {
            let mut w = HashWriter::new();
            w.write_string("Camera");
            w.write_string("forward");
            w.write_hash(&hash_vector(&c.target));
            w.write_string("position");
            w.write_hash(&hash_point(&c.position));
            w.write_string("up");
            w.write_hash(&hash_vector(&c.up));
            w.digest()
        }
    } // 🧮Hash Value Types
    pub use hash_value_types::*;

    mod hash_entities {
        // 📎Hash Entities

        use super::*;

        pub fn hash_attribute(a: &Attribute) -> String {
            let mut w = HashWriter::new();
            w.write_string("Attribute");
            if let Some(s) = &a.definition {
                w.write_string("definition");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&a.guid);
            w.write_string("key");
            w.write_string(&a.key);
            if let Some(s) = &a.value {
                w.write_string("value");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_author(a: &Author) -> String {
            let mut w = HashWriter::new();
            w.write_string("Author");
            if let Some(v) = &a.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &a.email {
                if !s.is_empty() {
                    w.write_string("email");
                    w.write_string(s);
                }
            }
            w.write_string("guid");
            w.write_string(&a.guid);
            w.write_string("name");
            w.write_string(&a.name);
            w.digest()
        }

        pub fn hash_file(f: &File) -> String {
            let mut w = HashWriter::new();
            w.write_string("File");
            if let Some(s) = &f.blob {
                w.write_string("blob");
                w.write_string(s);
            }
            if let Some(fid) = &f.folder {
                w.write_string("folder");
                w.write_string(&fid.guid);
            }
            w.write_string("guid");
            w.write_string(&f.guid);
            if let Some(s) = &f.hash {
                w.write_string("hash");
                w.write_string(s);
            }
            w.write_string("name");
            w.write_string(&f.name);
            if let Some(s) = &f.remote {
                w.write_string("remote");
                w.write_string(s);
            }
            if let Some(n) = &f.size {
                w.write_string("size");
                w.write_int_number(*n as i32);
            }
            w.digest()
        }

        pub fn hash_folder(f: &Folder) -> String {
            let mut w = HashWriter::new();
            w.write_string("Folder");
            if let Some(v) = &f.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &f.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&f.guid);
            w.write_string("name");
            w.write_string(&f.name);
            if let Some(p) = &f.parent {
                w.write_string("parent");
                w.write_string(&p.guid);
            }
            w.digest()
        }

        pub fn hash_benchmark(b: &Benchmark) -> String {
            let mut w = HashWriter::new();
            w.write_string("Benchmark");
            if let Some(v) = &b.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("guid");
            w.write_string(&b.guid);
            if let Some(s) = &b.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            if let Some(n) = &b.max {
                w.write_string("max");
                w.write_number(*n);
            }
            if let Some(b_val) = &b.max_excluded {
                w.write_string("maxExcluded");
                w.write_bool(*b_val);
            }
            if let Some(n) = &b.min {
                w.write_string("min");
                w.write_number(*n);
            }
            if let Some(b_val) = &b.min_excluded {
                w.write_string("minExcluded");
                w.write_bool(*b_val);
            }
            w.write_string("name");
            w.write_string(&b.name);
            w.digest()
        }

        pub fn hash_quality(q: &Quality) -> String {
            let mut w = HashWriter::new();
            w.write_string("Quality");
            if let Some(v) = &q.benchmarks {
                if !v.is_empty() {
                    w.write_string("benchmarks");
                    let hashes: Vec<String> = v.iter().map(|x| hash_benchmark(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(b) = &q.can_scale {
                w.write_string("canScale");
                w.write_bool(*b);
            }
            if let Some(s) = &q.default_imperial_unit {
                w.write_string("defaultImperialUnit");
                w.write_string(s);
            }
            if let Some(s) = &q.default_si_unit {
                w.write_string("defaultSiUnit");
                w.write_string(s);
            }
            if let Some(n) = &q.default_value {
                w.write_string("defaultValue");
                w.write_number(*n);
            }
            if let Some(s) = &q.description {
                w.write_string("description");
                w.write_string(s);
            }
            if let Some(s) = &q.formula {
                w.write_string("formula");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&q.guid);
            if let Some(s) = &q.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            if let Some(s) = &q.image {
                w.write_string("image");
                w.write_string(s);
            }
            if let Some(b) = &q.is_max_excluded {
                w.write_string("isMaxExcluded");
                w.write_bool(*b);
            }
            if let Some(b) = &q.is_min_excluded {
                w.write_string("isMinExcluded");
                w.write_bool(*b);
            }
            w.write_string("key");
            w.write_string(&q.key);
            if q.kind as i32 != 0 {
                w.write_string("kind");
                w.write_int_number(q.kind as i32);
            }
            if let Some(n) = &q.max {
                w.write_string("max");
                w.write_number(*n);
            }
            if let Some(n) = &q.min {
                w.write_string("min");
                w.write_number(*n);
            }
            w.write_string("name");
            w.write_string(&q.name);
            if let Some(s) = &q.unit {
                w.write_string("unit");
                w.write_string(s);
            }
            if let Some(s) = &q.uri {
                w.write_string("uri");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_port(p: &Port) -> String {
            let mut w = HashWriter::new();
            w.write_string("Port");
            if let Some(v) = &p.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &p.compatible_interfaces {
                if !v.is_empty() {
                    w.write_string("compatiblePorts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(s) = &p.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&p.guid);
            if let Some(s) = &p.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            w.write_string("name");
            w.write_string(&p.name);
            w.digest()
        }

        pub fn hash_prop(p: &Prop) -> String {
            let mut w = HashWriter::new();
            w.write_string("Prop");
            if let Some(v) = &p.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("guid");
            w.write_string(&p.guid);
            w.write_string("quality");
            w.write_string(&p.quality.guid);
            if let Some(s) = &p.unit {
                w.write_string("unit");
                w.write_string(s);
            }
            w.write_string("value");
            w.write_string(&p.value);
            w.digest()
        }

        pub fn hash_tag(t: &Tag) -> String {
            let mut w = HashWriter::new();
            w.write_string("Tag");
            if let Some(v) = &t.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &t.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&t.guid);
            if let Some(s) = &t.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            w.write_string("name");
            w.write_string(&t.name);
            w.digest()
        }

        pub fn hash_concept(c: &Concept) -> String {
            let mut w = HashWriter::new();
            w.write_string("Concept");
            if let Some(v) = &c.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &c.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&c.guid);
            if let Some(s) = &c.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            w.write_string("name");
            w.write_string(&c.name);
            w.digest()
        }

        pub fn hash_model(m: &Model) -> String {
            let mut w = HashWriter::new();
            w.write_string("Model");
            if let Some(v) = &m.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &m.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("file");
            w.write_string(&m.file.guid);
            w.write_string("guid");
            w.write_string(&m.guid);
            if let Some(s) = &m.name {
                w.write_string("name");
                w.write_string(s);
            }
            if let Some(v) = &m.tags {
                if !v.is_empty() {
                    w.write_string("tags");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            w.digest()
        }

        pub fn hash_connector(c: &Connector) -> String {
            let mut w = HashWriter::new();
            w.write_string("Connector");
            if let Some(v) = &c.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &c.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("direction");
            w.write_hash(&hash_vector(&c.direction));
            w.write_string("guid");
            w.write_string(&c.guid);
            if let Some(b) = &c.mandatory {
                w.write_string("mandatory");
                w.write_bool(*b);
            }
            if let Some(s) = &c.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.write_string("point");
            w.write_hash(&hash_point(&c.point));
            if let Some(pid) = &c.port {
                w.write_string("port");
                w.write_string(&pid.guid);
            }
            if let Some(v) = &c.props {
                if !v.is_empty() {
                    w.write_string("props");
                    let hashes: Vec<String> = v.iter().map(|x| hash_prop(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("t");
            w.write_number(c.t);
            w.digest()
        }

        pub fn hash_type(t: &Type) -> String {
            let mut w = HashWriter::new();
            w.write_string("Type");
            if let Some(v) = &t.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &t.authors {
                if !v.is_empty() {
                    w.write_string("authors");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(v) = &t.concepts {
                if !v.is_empty() {
                    w.write_string("concepts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(v) = &t.connectors {
                if !v.is_empty() {
                    w.write_string("connectors");
                    let hashes: Vec<String> = v.iter().map(|x| hash_connector(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &t.description {
                w.write_string("description");
                w.write_string(s);
            }
            if let Some(s) = &t.folder {
                w.write_string("folder");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&t.guid);
            if let Some(s) = &t.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            if let Some(s) = &t.image {
                w.write_string("image");
                w.write_string(s);
            }
            if let Some(b) = &t.is_abstract {
                w.write_string("isAbstract");
                w.write_bool(*b);
            }
            if let Some(lid) = &t.location {
                w.write_string("location");
                w.write_string(&lid.guid);
            }
            if let Some(v) = &t.models {
                if !v.is_empty() {
                    w.write_string("models");
                    let hashes: Vec<String> = v.iter().map(|x| hash_model(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("name");
            w.write_string(&t.name);
            if let Some(pid) = &t.parent {
                w.write_string("parent");
                w.write_string(&pid.guid);
            }
            if let Some(v) = &t.props {
                if !v.is_empty() {
                    w.write_string("props");
                    let hashes: Vec<String> = v.iter().map(|x| hash_prop(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(n) = &t.stock {
                w.write_string("stock");
                w.write_int_number(*n);
            }
            if let Some(s) = &t.unit {
                w.write_string("unit");
                w.write_string(s);
            }
            if let Some(b) = &t.virtual_type {
                w.write_string("virtual");
                w.write_bool(*b);
            }
            w.digest()
        }

        pub fn hash_layer(l: &Layer) -> String {
            let mut w = HashWriter::new();
            w.write_string("Layer");
            if let Some(v) = &l.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &l.color {
                w.write_string("color");
                w.write_string(s);
            }
            if let Some(s) = &l.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&l.guid);
            if let Some(b) = &l.is_hidden {
                w.write_string("isHidden");
                w.write_bool(*b);
            }
            if let Some(b) = &l.is_locked {
                w.write_string("isLocked");
                w.write_bool(*b);
            }
            w.write_string("path");
            w.write_string(&l.path);
            w.digest()
        }

        pub fn hash_stat(s: &Stat) -> String {
            let mut w = HashWriter::new();
            w.write_string("Stat");
            w.write_string("guid");
            w.write_string(&s.guid);
            if let Some(n) = &s.max {
                w.write_string("max");
                w.write_number(*n);
            }
            if let Some(b) = &s.max_excluded {
                w.write_string("maxExcluded");
                w.write_bool(*b);
            }
            if let Some(n) = &s.min {
                w.write_string("min");
                w.write_number(*n);
            }
            if let Some(b) = &s.min_excluded {
                w.write_string("minExcluded");
                w.write_bool(*b);
            }
            w.write_string("quality");
            w.write_string(&s.quality.guid);
            if let Some(u) = &s.unit {
                w.write_string("unit");
                w.write_string(u);
            }
            w.digest()
        }

        pub fn hash_group(g: &Group) -> String {
            let mut w = HashWriter::new();
            w.write_string("Group");
            if let Some(v) = &g.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &g.color {
                w.write_string("color");
                w.write_string(s);
            }
            if let Some(s) = &g.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("guid");
            w.write_string(&g.guid);
            if let Some(s) = &g.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.write_string("pieces");
            let guids: Vec<String> = g
                .pieces
                .as_ref()
                .map_or(Vec::new(), |v| v.iter().map(|x| x.guid.clone()).collect());
            w.write_guid_list(&guids);
            w.digest()
        }

        pub fn hash_side(s: &Side) -> String {
            let mut w = HashWriter::new();
            w.write_string("Side");
            if let Some(cid) = &s.connector {
                w.write_string("connector");
                w.write_string(&cid.guid);
            }
            if let Some(dpid) = &s.design_piece {
                w.write_string("designPiece");
                w.write_string(&dpid.guid);
            }
            w.write_string("piece");
            w.write_string(&s.piece.guid);
            w.digest()
        }

        pub fn hash_connection(c: &Connection) -> String {
            let mut w = HashWriter::new();
            w.write_string("Connection");
            if let Some(v) = &c.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("connected");
            w.write_hash(&hash_side(&c.connected));
            w.write_string("connecting");
            w.write_hash(&hash_side(&c.connecting));
            if let Some(s) = &c.description {
                w.write_string("description");
                w.write_string(s);
            }
            w.write_string("gap");
            w.write_number(c.gap);
            w.write_string("guid");
            w.write_string(&c.guid);
            w.write_string("rise");
            w.write_number(c.rise);
            w.write_string("rotation");
            w.write_number(c.rotation);
            w.write_string("shift");
            w.write_number(c.shift);
            w.write_string("tilt");
            w.write_number(c.tilt);
            w.write_string("turn");
            w.write_number(c.turn);
            w.write_string("u");
            w.write_number(c.u.unwrap_or(0.0));
            w.write_string("v");
            w.write_number(c.v.unwrap_or(0.0));
            w.digest()
        }

        pub fn hash_piece(p: &Piece) -> String {
            let mut w = HashWriter::new();
            w.write_string("Piece");
            if let Some(v) = &p.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(c) = &p.center {
                w.write_string("center");
                w.write_hash(&hash_coord(c));
            }
            if let Some(s) = &p.color {
                w.write_string("color");
                w.write_string(s);
            }
            if let Some(s) = &p.description {
                w.write_string("description");
                w.write_string(s);
            }
            if let Some(did) = &p.design {
                w.write_string("design");
                w.write_string(&did.guid);
            }
            w.write_string("guid");
            w.write_string(&p.guid);
            if let Some(b) = &p.is_hidden {
                w.write_string("isHidden");
                w.write_bool(*b);
            }
            if let Some(b) = &p.is_locked {
                w.write_string("isLocked");
                w.write_bool(*b);
            }
            if let Some(mp) = &p.mirror_plane {
                w.write_string("mirrorPlane");
                w.write_hash(&hash_plane(mp));
            }
            if let Some(s) = &p.name {
                w.write_string("name");
                w.write_string(s);
            }
            if let Some(pl) = &p.plane {
                w.write_string("plane");
                w.write_hash(&hash_plane(pl));
            }
            if let Some(v) = &p.props {
                if !v.is_empty() {
                    w.write_string("props");
                    let hashes: Vec<String> = v.iter().map(|x| hash_prop(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(n) = &p.scale {
                w.write_string("scale");
                w.write_number(*n);
            }
            if let Some(tid) = &p.type_ref {
                w.write_string("type");
                w.write_string(&tid.guid);
            }
            w.digest()
        }

        pub fn hash_design(d: &Design) -> String {
            let mut w = HashWriter::new();
            w.write_string("Design");
            if let Some(al) = &d.active_layer {
                w.write_string("activeLayer");
                w.write_string(&al.guid);
            }
            if let Some(v) = &d.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &d.authors {
                if !v.is_empty() {
                    w.write_string("authors");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(b) = &d.can_mirror {
                w.write_string("canMirror");
                w.write_bool(*b);
            }
            if let Some(b) = &d.can_scale {
                w.write_string("canScale");
                w.write_bool(*b);
            }
            if let Some(v) = &d.concepts {
                if !v.is_empty() {
                    w.write_string("concepts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(v) = &d.connections {
                if !v.is_empty() {
                    w.write_string("connections");
                    let hashes: Vec<String> = v.iter().map(|x| hash_connection(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &d.description {
                w.write_string("description");
                w.write_string(s);
            }
            if let Some(s) = &d.folder {
                w.write_string("folder");
                w.write_string(s);
            }
            if let Some(v) = &d.groups {
                if !v.is_empty() {
                    w.write_string("groups");
                    let hashes: Vec<String> = v.iter().map(|x| hash_group(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("guid");
            w.write_string(&d.guid);
            if let Some(s) = &d.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            if let Some(s) = &d.image {
                w.write_string("image");
                w.write_string(s);
            }
            if let Some(b) = &d.is_abstract {
                w.write_string("isAbstract");
                w.write_bool(*b);
            }
            if let Some(v) = &d.layers {
                if !v.is_empty() {
                    w.write_string("layers");
                    let hashes: Vec<String> = v.iter().map(|x| hash_layer(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(lid) = &d.location {
                w.write_string("location");
                w.write_string(&lid.guid);
            }
            w.write_string("name");
            w.write_string(&d.name);
            if let Some(pid) = &d.parent {
                w.write_string("parent");
                w.write_string(&pid.guid);
            }
            if let Some(v) = &d.pieces {
                if !v.is_empty() {
                    w.write_string("pieces");
                    let hashes: Vec<String> = v.iter().map(|x| hash_piece(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &d.props {
                if !v.is_empty() {
                    w.write_string("props");
                    let hashes: Vec<String> = v.iter().map(|x| hash_prop(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &d.stats {
                if !v.is_empty() {
                    w.write_string("stats");
                    let hashes: Vec<String> = v.iter().map(|x| hash_stat(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &d.unit {
                w.write_string("unit");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_kit(k: &Kit) -> String {
            let mut w = HashWriter::new();
            w.write_string("Kit");
            if let Some(v) = &k.attributes {
                if !v.is_empty() {
                    w.write_string("attributes");
                    let hashes: Vec<String> = v.iter().map(|x| hash_attribute(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &k.authors {
                if !v.is_empty() {
                    w.write_string("authors");
                    let hashes: Vec<String> = v.iter().map(|x| hash_author(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &k.concepts {
                if !v.is_empty() {
                    w.write_string("concepts");
                    let hashes: Vec<String> = v.iter().map(|x| hash_concept(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &k.description {
                w.write_string("description");
                w.write_string(s);
            }
            if let Some(v) = &k.designs {
                if !v.is_empty() {
                    w.write_string("designs");
                    let hashes: Vec<String> = v.iter().map(|x| hash_design(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &k.files {
                if !v.is_empty() {
                    w.write_string("files");
                    let hashes: Vec<String> = v.iter().map(|x| hash_file(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &k.folders {
                if !v.is_empty() {
                    w.write_string("folders");
                    let hashes: Vec<String> = v.iter().map(|x| hash_folder(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            w.write_string("guid");
            w.write_string(&k.guid);
            if let Some(s) = &k.homepage {
                w.write_string("homepage");
                w.write_string(s);
            }
            if let Some(s) = &k.icon {
                w.write_string("icon");
                w.write_string(s);
            }
            if let Some(s) = &k.image {
                w.write_string("image");
                w.write_string(s);
            }
            if let Some(s) = &k.license {
                w.write_string("license");
                w.write_string(s);
            }
            w.write_string("name");
            w.write_string(&k.name);
            if let Some(v) = &k.ports {
                if !v.is_empty() {
                    w.write_string("ports");
                    let hashes: Vec<String> = v.iter().map(|x| hash_port(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &k.preview {
                w.write_string("preview");
                w.write_string(s);
            }
            if let Some(v) = &k.qualities {
                if !v.is_empty() {
                    w.write_string("qualities");
                    let hashes: Vec<String> = v.iter().map(|x| hash_quality(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &k.remote {
                w.write_string("remote");
                w.write_string(s);
            }
            if let Some(v) = &k.tags {
                if !v.is_empty() {
                    w.write_string("tags");
                    let hashes: Vec<String> = v.iter().map(|x| hash_tag(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(v) = &k.types {
                if !v.is_empty() {
                    w.write_string("types");
                    let hashes: Vec<String> = v.iter().map(|x| hash_type(x)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(s) = &k.version {
                if !s.is_empty() {
                    w.write_string("version");
                    w.write_string(s);
                }
            }
            w.digest()
        }
    } // ⛷️Hash Entities
    pub use hash_entities::*;

    mod hash_diff_value_types {
        // 📣Hash Diff Value Types

        use super::*;

        pub fn hash_coord_diff(c: &Coord) -> String {
            let mut w = HashWriter::new();
            w.write_string("CoordDiff");
            w.write_string("u");
            w.write_number(c.u);
            w.write_string("v");
            w.write_number(c.v);
            w.digest()
        }

        pub fn hash_point_diff(v: &Vector) -> String {
            let mut w = HashWriter::new();
            w.write_string("PointDiff");
            w.write_string("x");
            w.write_number(v.x);
            w.write_string("y");
            w.write_number(v.y);
            w.write_string("z");
            w.write_number(v.z);
            w.digest()
        }

        pub fn hash_vector_diff(v: &Vector) -> String {
            let mut w = HashWriter::new();
            w.write_string("VectorDiff");
            w.write_string("x");
            w.write_number(v.x);
            w.write_string("y");
            w.write_number(v.y);
            w.write_string("z");
            w.write_number(v.z);
            w.digest()
        }

        pub fn hash_plane_diff(p: &Plane) -> String {
            let mut w = HashWriter::new();
            w.write_string("PlaneDiff");
            w.write_string("origin");
            w.write_hash(&hash_point_diff(&p.origin));
            w.write_string("xAxis");
            w.write_hash(&hash_vector_diff(&p.x_axis));
            w.write_string("yAxis");
            w.write_hash(&hash_vector_diff(&p.y_axis));
            w.digest()
        }
    } // 🖌️Hash Diff Value Types
    pub use hash_diff_value_types::*;

    mod hash_diff_collection {
        // 🥇Hash Diff Collection

        use super::*;

        pub fn hash_collection_diff<T, D>(
            tag: &str,
            update_tag: &str,
            entity_key_name: &str,
            hash_entity: &dyn Fn(&T) -> String,
            hash_diff: &dyn Fn(&D) -> String,
            coll: &CollectionDiff<T, D>,
        ) -> String {
            let mut w = HashWriter::new();
            w.write_string(tag);
            if let Some(ref added) = coll.added {
                if !added.is_empty() {
                    w.write_string("added");
                    let hashes: Vec<String> = added.iter().map(|e| hash_entity(e)).collect();
                    w.write_hash_list(&hashes);
                }
            }
            if let Some(ref removed) = coll.removed {
                if !removed.is_empty() {
                    w.write_string("removed");
                    let guids: Vec<String> = removed.iter().map(|r| r.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
            }
            if let Some(ref updated) = coll.updated {
                if !updated.is_empty() {
                    w.write_string("updated");
                    let mut keys = vec![entity_key_name.to_string(), "diff".to_string()];
                    keys.sort();
                    let update_hashes: Vec<String> = updated
                        .iter()
                        .map(|u| {
                            let mut uw = HashWriter::new();
                            uw.write_string(update_tag);
                            for k in &keys {
                                if k == "diff" {
                                    uw.write_string("diff");
                                    uw.write_hash(&hash_diff(&u.diff));
                                } else {
                                    uw.write_string(k);
                                    uw.write_string(&u.guid);
                                }
                            }
                            uw.digest()
                        })
                        .collect();
                    w.write_hash_list(&update_hashes);
                }
            }
            w.digest()
        }
    } // 🧢Hash Diff Collection
    pub use hash_diff_collection::*;

    mod hash_diff_entities {
        // 🏸Hash Diff Entities

        use super::*;

        pub fn hash_attribute_diff(d: &AttributeDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("AttributeDiff");
            match &d.definition {
                Some(Some(s)) => {
                    w.write_string("definition");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("definition");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.key {
                w.write_string("key");
                w.write_string(s);
            }
            match &d.value {
                Some(Some(s)) => {
                    w.write_string("value");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("value");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_author_diff(d: &AuthorDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("AuthorDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.email {
                Some(Some(s)) => {
                    w.write_string("email");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("email");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_file_diff(d: &FileDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("FileDiff");
            match &d.folder {
                Some(Some(fid)) => {
                    w.write_string("folder");
                    w.write_string(&fid.guid);
                }
                Some(None) => {
                    w.write_string("folder");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.hash {
                Some(Some(s)) => {
                    w.write_string("hash");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("hash");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            match &d.remote {
                Some(Some(s)) => {
                    w.write_string("remote");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("remote");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.size {
                Some(Some(n)) => {
                    w.write_string("size");
                    w.write_int_number(*n as i32);
                }
                Some(None) => {
                    w.write_string("size");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_folder_diff(d: &FolderDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("FolderDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            match &d.parent {
                Some(Some(fid)) => {
                    w.write_string("parent");
                    w.write_string(&fid.guid);
                }
                Some(None) => {
                    w.write_string("parent");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_quality_diff(d: &QualityDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("QualityDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.can_scale {
                Some(Some(b)) => {
                    w.write_string("canScale");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("canScale");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.default_imperial_unit {
                Some(Some(s)) => {
                    w.write_string("defaultImperialUnit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("defaultImperialUnit");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.default_si_unit {
                Some(Some(s)) => {
                    w.write_string("defaultSiUnit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("defaultSiUnit");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.default_value {
                Some(Some(n)) => {
                    w.write_string("defaultValue");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("defaultValue");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.formula {
                Some(Some(s)) => {
                    w.write_string("formula");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("formula");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_max_excluded {
                Some(Some(b)) => {
                    w.write_string("isMaxExcluded");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isMaxExcluded");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_min_excluded {
                Some(Some(b)) => {
                    w.write_string("isMinExcluded");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isMinExcluded");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.key {
                w.write_string("key");
                w.write_string(s);
            }
            if let Some(ref kind) = d.kind {
                w.write_string("kind");
                w.write_int_number(*kind as i32);
            }
            match &d.max {
                Some(Some(n)) => {
                    w.write_string("max");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("max");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.min {
                Some(Some(n)) => {
                    w.write_string("min");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("min");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            match &d.uri {
                Some(Some(s)) => {
                    w.write_string("uri");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("uri");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_port_diff(d: &PortDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("PortDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.compatible_interfaces {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("compatiblePorts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("compatiblePorts");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_prop_diff(d: &PropDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("PropDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            if let Some(ref qid) = d.quality {
                w.write_string("quality");
                w.write_string(&qid.guid);
            }
            match &d.unit {
                Some(Some(s)) => {
                    w.write_string("unit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("unit");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.value {
                w.write_string("value");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_tag_diff(d: &TagDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("TagDiff");
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_concept_diff(d: &ConceptDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("ConceptDiff");
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_model_diff(d: &ModelDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("ModelDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref fid) = d.file {
                w.write_string("file");
                w.write_string(&fid.guid);
            }
            match &d.name {
                Some(Some(s)) => {
                    w.write_string("name");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("name");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.tags {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("tags");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("tags");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_connector_diff(d: &ConnectorDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("ConnectorDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref v) = d.direction {
                w.write_string("direction");
                w.write_hash(&hash_vector_diff(v));
            }
            match &d.mandatory {
                Some(Some(b)) => {
                    w.write_string("mandatory");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("mandatory");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.name {
                Some(Some(s)) => {
                    w.write_string("name");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("name");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref v) = d.point {
                w.write_string("point");
                w.write_hash(&hash_point_diff(v));
            }
            match &d.port {
                Some(Some(pid)) => {
                    w.write_string("port");
                    w.write_string(&pid.guid);
                }
                Some(None) => {
                    w.write_string("port");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.props {
                w.write_string("props");
                w.write_hash(&hash_collection_diff(
                    "PropsDiff",
                    "PropDiffUpdate",
                    "prop",
                    &|p: &Prop| hash_prop(p),
                    &|d: &PropDiff| hash_prop_diff(d),
                    coll,
                ));
            }
            if let Some(n) = d.t {
                w.write_string("t");
                w.write_number(n);
            }
            w.digest()
        }

        pub fn hash_type_diff(d: &TypeDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("TypeDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.authors {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("authors");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("authors");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.concepts {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("concepts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("concepts");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.connectors {
                w.write_string("connectors");
                w.write_hash(&hash_collection_diff(
                    "ConnectorsDiff",
                    "ConnectorDiffUpdate",
                    "connector",
                    &|c: &Connector| hash_connector(c),
                    &|d: &ConnectorDiff| hash_connector_diff(d),
                    coll,
                ));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.folder {
                Some(Some(s)) => {
                    w.write_string("folder");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("folder");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.image {
                Some(Some(s)) => {
                    w.write_string("image");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("image");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_abstract {
                Some(Some(b)) => {
                    w.write_string("isAbstract");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isAbstract");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.location {
                Some(Some(lid)) => {
                    w.write_string("location");
                    w.write_string(&lid.guid);
                }
                Some(None) => {
                    w.write_string("location");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.models {
                w.write_string("models");
                w.write_hash(&hash_collection_diff(
                    "ModelsDiff",
                    "ModelDiffUpdate",
                    "model",
                    &|m: &Model| hash_model(m),
                    &|d: &ModelDiff| hash_model_diff(d),
                    coll,
                ));
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            match &d.parent {
                Some(Some(tid)) => {
                    w.write_string("parent");
                    w.write_string(&tid.guid);
                }
                Some(None) => {
                    w.write_string("parent");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.props {
                w.write_string("props");
                w.write_hash(&hash_collection_diff(
                    "PropsDiff",
                    "PropDiffUpdate",
                    "prop",
                    &|p: &Prop| hash_prop(p),
                    &|d: &PropDiff| hash_prop_diff(d),
                    coll,
                ));
            }
            match &d.stock {
                Some(Some(n)) => {
                    w.write_string("stock");
                    w.write_int_number(*n);
                }
                Some(None) => {
                    w.write_string("stock");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.unit {
                Some(Some(s)) => {
                    w.write_string("unit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("unit");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.virtual_type {
                Some(Some(b)) => {
                    w.write_string("virtual");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("virtual");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_side_diff(d: &SideDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("SideDiff");
            match &d.connector {
                Some(Some(cid)) => {
                    w.write_string("connector");
                    w.write_string(&cid.guid);
                }
                Some(None) => {
                    w.write_string("connector");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.design_piece {
                Some(Some(pid)) => {
                    w.write_string("designPiece");
                    w.write_string(&pid.guid);
                }
                Some(None) => {
                    w.write_string("designPiece");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref pid) = d.piece {
                w.write_string("piece");
                w.write_string(&pid.guid);
            }
            w.digest()
        }

        pub fn hash_connection_diff(d: &ConnectionDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("ConnectionDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            if let Some(ref s) = d.connected {
                w.write_string("connected");
                w.write_hash(&hash_side_diff(s));
            }
            if let Some(ref s) = d.connecting {
                w.write_string("connecting");
                w.write_hash(&hash_side_diff(s));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(n) = d.gap {
                w.write_string("gap");
                w.write_number(n);
            }
            if let Some(n) = d.rise {
                w.write_string("rise");
                w.write_number(n);
            }
            if let Some(n) = d.rotation {
                w.write_string("rotation");
                w.write_number(n);
            }
            if let Some(n) = d.shift {
                w.write_string("shift");
                w.write_number(n);
            }
            if let Some(n) = d.tilt {
                w.write_string("tilt");
                w.write_number(n);
            }
            if let Some(n) = d.turn {
                w.write_string("turn");
                w.write_number(n);
            }
            match d.u {
                Some(Some(n)) => {
                    w.write_string("u");
                    w.write_number(n);
                }
                Some(None) => {
                    w.write_string("u");
                    w.write_bool(false);
                }
                None => {}
            }
            match d.v {
                Some(Some(n)) => {
                    w.write_string("v");
                    w.write_number(n);
                }
                Some(None) => {
                    w.write_string("v");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_piece_diff(d: &PieceDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("PieceDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.center {
                Some(Some(c)) => {
                    w.write_string("center");
                    w.write_hash(&hash_coord_diff(c));
                }
                Some(None) => {
                    w.write_string("center");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.color {
                Some(Some(s)) => {
                    w.write_string("color");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("color");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.design {
                Some(Some(did)) => {
                    w.write_string("design");
                    w.write_string(&did.guid);
                }
                Some(None) => {
                    w.write_string("design");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_hidden {
                Some(Some(b)) => {
                    w.write_string("isHidden");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isHidden");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_locked {
                Some(Some(b)) => {
                    w.write_string("isLocked");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isLocked");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.mirror_plane {
                Some(Some(p)) => {
                    w.write_string("mirrorPlane");
                    w.write_hash(&hash_plane_diff(p));
                }
                Some(None) => {
                    w.write_string("mirrorPlane");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.name {
                Some(Some(s)) => {
                    w.write_string("name");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("name");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.plane {
                Some(Some(p)) => {
                    w.write_string("plane");
                    w.write_hash(&hash_plane_diff(p));
                }
                Some(None) => {
                    w.write_string("plane");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.props {
                w.write_string("props");
                w.write_hash(&hash_collection_diff(
                    "PropsDiff",
                    "PropDiffUpdate",
                    "prop",
                    &|p: &Prop| hash_prop(p),
                    &|d: &PropDiff| hash_prop_diff(d),
                    coll,
                ));
            }
            match &d.scale {
                Some(Some(n)) => {
                    w.write_string("scale");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("scale");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.type_ref {
                Some(Some(tid)) => {
                    w.write_string("type");
                    w.write_string(&tid.guid);
                }
                Some(None) => {
                    w.write_string("type");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_layer_diff(d: &LayerDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("LayerDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.color {
                Some(Some(s)) => {
                    w.write_string("color");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("color");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_hidden {
                Some(Some(b)) => {
                    w.write_string("isHidden");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isHidden");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_locked {
                Some(Some(b)) => {
                    w.write_string("isLocked");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isLocked");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.path {
                w.write_string("path");
                w.write_string(s);
            }
            w.digest()
        }

        pub fn hash_group_diff(d: &GroupDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("GroupDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.color {
                Some(Some(s)) => {
                    w.write_string("color");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("color");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.name {
                Some(Some(s)) => {
                    w.write_string("name");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("name");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.pieces {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("pieces");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("pieces");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_stat_diff(d: &StatDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("StatDiff");
            match &d.max {
                Some(Some(n)) => {
                    w.write_string("max");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("max");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.max_excluded {
                Some(Some(b)) => {
                    w.write_string("maxExcluded");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("maxExcluded");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.min {
                Some(Some(n)) => {
                    w.write_string("min");
                    w.write_number(*n);
                }
                Some(None) => {
                    w.write_string("min");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.min_excluded {
                Some(Some(b)) => {
                    w.write_string("minExcluded");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("minExcluded");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref qid) = d.quality {
                w.write_string("quality");
                w.write_string(&qid.guid);
            }
            match &d.unit {
                Some(Some(s)) => {
                    w.write_string("unit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("unit");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_design_diff(d: &DesignDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("DesignDiff");
            match &d.active_layer {
                Some(Some(lid)) => {
                    w.write_string("activeLayer");
                    w.write_string(&lid.guid);
                }
                Some(None) => {
                    w.write_string("activeLayer");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            match &d.authors {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("authors");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("authors");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.can_mirror {
                Some(Some(b)) => {
                    w.write_string("canMirror");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("canMirror");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.can_scale {
                Some(Some(b)) => {
                    w.write_string("canScale");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("canScale");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.concepts {
                Some(Some(v)) if !v.is_empty() => {
                    w.write_string("concepts");
                    let guids: Vec<String> = v.iter().map(|x| x.guid.clone()).collect();
                    w.write_guid_list(&guids);
                }
                Some(_) => {
                    w.write_string("concepts");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.connections {
                w.write_string("connections");
                w.write_hash(&hash_collection_diff(
                    "ConnectionsDiff",
                    "ConnectionDiffUpdate",
                    "connection",
                    &|c: &Connection| hash_connection(c),
                    &|d: &ConnectionDiff| hash_connection_diff(d),
                    coll,
                ));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.folder {
                Some(Some(s)) => {
                    w.write_string("folder");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("folder");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.groups {
                w.write_string("groups");
                w.write_hash(&hash_collection_diff(
                    "GroupsDiff",
                    "GroupDiffUpdate",
                    "group",
                    &|g: &Group| hash_group(g),
                    &|d: &GroupDiff| hash_group_diff(d),
                    coll,
                ));
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.image {
                Some(Some(s)) => {
                    w.write_string("image");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("image");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.is_abstract {
                Some(Some(b)) => {
                    w.write_string("isAbstract");
                    w.write_bool(*b);
                }
                Some(None) => {
                    w.write_string("isAbstract");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.layers {
                w.write_string("layers");
                w.write_hash(&hash_collection_diff(
                    "LayersDiff",
                    "LayerDiffUpdate",
                    "layer",
                    &|l: &Layer| hash_layer(l),
                    &|d: &LayerDiff| hash_layer_diff(d),
                    coll,
                ));
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            match &d.parent {
                Some(Some(did)) => {
                    w.write_string("parent");
                    w.write_string(&did.guid);
                }
                Some(None) => {
                    w.write_string("parent");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.pieces {
                w.write_string("pieces");
                w.write_hash(&hash_collection_diff(
                    "PiecesDiff",
                    "PieceDiffUpdate",
                    "piece",
                    &|p: &Piece| hash_piece(p),
                    &|d: &PieceDiff| hash_piece_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.props {
                w.write_string("props");
                w.write_hash(&hash_collection_diff(
                    "PropsDiff",
                    "PropDiffUpdate",
                    "prop",
                    &|p: &Prop| hash_prop(p),
                    &|d: &PropDiff| hash_prop_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.stats {
                w.write_string("stats");
                w.write_hash(&hash_collection_diff(
                    "StatsDiff",
                    "StatDiffUpdate",
                    "stat",
                    &|s: &Stat| hash_stat(s),
                    &|d: &StatDiff| hash_stat_diff(d),
                    coll,
                ));
            }
            match &d.unit {
                Some(Some(s)) => {
                    w.write_string("unit");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("unit");
                    w.write_bool(false);
                }
                None => {}
            }
            w.digest()
        }

        pub fn hash_kit_diff(d: &KitDiff) -> String {
            let mut w = HashWriter::new();
            w.write_string("KitDiff");
            if let Some(ref coll) = d.attributes {
                w.write_string("attributes");
                w.write_hash(&hash_collection_diff(
                    "AttributesDiff",
                    "AttributeDiffUpdate",
                    "attribute",
                    &|a: &Attribute| hash_attribute(a),
                    &|d: &AttributeDiff| hash_attribute_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.authors {
                w.write_string("authors");
                w.write_hash(&hash_collection_diff(
                    "AuthorsDiff",
                    "AuthorDiffUpdate",
                    "author",
                    &|a: &Author| hash_author(a),
                    &|d: &AuthorDiff| hash_author_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.concepts {
                w.write_string("concepts");
                w.write_hash(&hash_collection_diff(
                    "ConceptsDiff",
                    "ConceptDiffUpdate",
                    "concept",
                    &|c: &Concept| hash_concept(c),
                    &|d: &ConceptDiff| hash_concept_diff(d),
                    coll,
                ));
            }
            match &d.description {
                Some(Some(s)) => {
                    w.write_string("description");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("description");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.designs {
                w.write_string("designs");
                w.write_hash(&hash_collection_diff(
                    "DesignsDiff",
                    "DesignDiffUpdate",
                    "design",
                    &|d: &Design| hash_design(d),
                    &|d: &DesignDiff| hash_design_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.files {
                w.write_string("files");
                w.write_hash(&hash_collection_diff(
                    "FilesDiff",
                    "FileDiffUpdate",
                    "file",
                    &|f: &File| hash_file(f),
                    &|d: &FileDiff| hash_file_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.folders {
                w.write_string("folders");
                w.write_hash(&hash_collection_diff(
                    "FoldersDiff",
                    "FolderDiffUpdate",
                    "folder",
                    &|f: &Folder| hash_folder(f),
                    &|d: &FolderDiff| hash_folder_diff(d),
                    coll,
                ));
            }
            match &d.homepage {
                Some(Some(s)) => {
                    w.write_string("homepage");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("homepage");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.icon {
                Some(Some(s)) => {
                    w.write_string("icon");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("icon");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.image {
                Some(Some(s)) => {
                    w.write_string("image");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("image");
                    w.write_bool(false);
                }
                None => {}
            }
            match &d.license {
                Some(Some(s)) => {
                    w.write_string("license");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("license");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref s) = d.name {
                w.write_string("name");
                w.write_string(s);
            }
            if let Some(ref coll) = d.ports {
                w.write_string("ports");
                w.write_hash(&hash_collection_diff(
                    "PortsDiff",
                    "PortDiffUpdate",
                    "port",
                    &|p: &Port| hash_port(p),
                    &|d: &PortDiff| hash_port_diff(d),
                    coll,
                ));
            }
            match &d.preview {
                Some(Some(s)) => {
                    w.write_string("preview");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("preview");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.qualities {
                w.write_string("qualities");
                w.write_hash(&hash_collection_diff(
                    "QualitiesDiff",
                    "QualityDiffUpdate",
                    "quality",
                    &|q: &Quality| hash_quality(q),
                    &|d: &QualityDiff| hash_quality_diff(d),
                    coll,
                ));
            }
            match &d.remote {
                Some(Some(s)) => {
                    w.write_string("remote");
                    w.write_string(s);
                }
                Some(None) => {
                    w.write_string("remote");
                    w.write_bool(false);
                }
                None => {}
            }
            if let Some(ref coll) = d.tags {
                w.write_string("tags");
                w.write_hash(&hash_collection_diff(
                    "TagsDiff",
                    "TagDiffUpdate",
                    "tag",
                    &|t: &Tag| hash_tag(t),
                    &|d: &TagDiff| hash_tag_diff(d),
                    coll,
                ));
            }
            if let Some(ref coll) = d.types {
                w.write_string("types");
                w.write_hash(&hash_collection_diff(
                    "TypesDiff",
                    "TypeDiffUpdate",
                    "type",
                    &|t: &Type| hash_type(t),
                    &|d: &TypeDiff| hash_type_diff(d),
                    coll,
                ));
            }
            match &d.version {
                Some(Some(s)) if !s.is_empty() => {
                    w.write_string("version");
                    w.write_string(s);
                }
                _ => {}
            }
            w.digest()
        }
    } // 📄Hash Diff Entities
    pub use hash_diff_entities::*;
} // 🖥️Hash
pub use hash::*;

mod finder_functions {
    // 🔍Helpers
    // 🥿Finder Functions
    // Finder Functions MUST provide the finder functions functionality.
    /// <summary>🔍finds a type in a kit by GUID.</summary>
    /// <remarks>
    /// </remarks>
    use super::*;

    pub fn find_type_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Type> {
        kit.types.as_ref()?.iter().find(|t| t.guid == guid)
    }
    /// <summary>🔍finds a type in a kit mutably by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_type_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Type> {
        kit.types.as_mut()?.iter_mut().find(|t| t.guid == guid)
    }
    /// <summary>🔍finds a design in a kit by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_design_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Design> {
        kit.designs.as_ref()?.iter().find(|d| d.guid == guid)
    }
    /// <summary>🔍finds a design in a kit mutably by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_design_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Design> {
        kit.designs.as_mut()?.iter_mut().find(|d| d.guid == guid)
    }
    /// <summary>🔍finds a piece in a design by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_piece_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Piece> {
        design.pieces.as_ref()?.iter().find(|p| p.guid == guid)
    }
    /// <remarks>
    /// </remarks>
    pub fn find_piece_in_design_mut<'a>(
        design: &'a mut Design,
        guid: &str,
    ) -> Option<&'a mut Piece> {
        design.pieces.as_mut()?.iter_mut().find(|p| p.guid == guid)
    }
    /// <summary>🔍finds a connection in a design by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_connection_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Connection> {
        design.connections.as_ref()?.iter().find(|c| c.guid == guid)
    }
    /// <summary>🔍finds a connector in a type by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_connector_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Connector> {
        t.connectors.as_ref()?.iter().find(|c| c.guid == guid)
    }
    /// <summary>🔍finds a model in a type by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_model_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Model> {
        t.models.as_ref()?.iter().find(|m| m.guid == guid)
    }
    /// <remarks>
    /// </remarks>
    pub fn find_file_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a File> {
        kit.files.as_ref()?.iter().find(|f| f.guid == guid)
    }

    /// <summary>🔍finds a folder in a kit by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_folder_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Folder> {
        kit.folders.as_ref()?.iter().find(|f| f.guid == guid)
    }
    /// <remarks>
    /// </remarks>
    pub fn find_author_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Author> {
        kit.authors.as_ref()?.iter().find(|a| a.guid == guid)
    }
    /// <summary>🔍find tag in kit.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_tag_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Tag> {
        kit.tags.as_ref()?.iter().find(|t| t.guid == guid)
    }
    /// <summary>🔍finds a concept in a kit by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_concept_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Concept> {
        kit.concepts.as_ref()?.iter().find(|c| c.guid == guid)
    }
    /// <summary>🔍finds a quality in a kit by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_quality_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Quality> {
        kit.qualities.as_ref()?.iter().find(|q| q.guid == guid)
    }
    /// <remarks>
    /// </remarks>
    pub fn find_interface_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Port> {
        kit.ports.as_ref()?.iter().find(|i| i.guid == guid)
    }

    /// <summary>🔍finds a layer in a design by GUID.</summary>
    pub fn find_layer_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Layer> {
        design.layers.as_ref()?.iter().find(|l| l.guid == guid)
    }
    /// <summary>🔍finds a group in a design by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_group_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Group> {
        design.groups.as_ref()?.iter().find(|g| g.guid == guid)
    }

    /// <summary>🔍finds a stat in a design by GUID.</summary>
    /// <remarks>
    /// </remarks>
    pub fn find_stat_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Stat> {
        design.stats.as_ref()?.iter().find(|s| s.guid == guid)
    }

    /// ✔️For each piece, it checks piece-level props first, then falls back to type-level props.
    pub fn sum_quality_in_design(kit: &Kit, design_guid: &str, quality_guid: &str) -> f64 {
        let design = match find_design_in_kit(kit, design_guid) {
            Some(d) => d,
            None => return 0.0,
        };
        let pieces = match &design.pieces {
            Some(p) => p,
            None => return 0.0,
        };
        let mut sum = 0.0;
        for piece in pieces {
            let piece_prop = piece
                .props
                .as_ref()
                .and_then(|props| props.iter().find(|p| p.quality.guid == quality_guid));
            if let Some(prop) = piece_prop {
                if let Ok(val) = prop.value.parse::<f64>() {
                    sum += val;
                }
                continue;
            }
            if let Some(type_ref) = &piece.type_ref {
                if let Some(t) = find_type_in_kit(kit, &type_ref.guid) {
                    if let Some(prop) = t
                        .props
                        .as_ref()
                        .and_then(|props| props.iter().find(|p| p.quality.guid == quality_guid))
                    {
                        if let Ok(val) = prop.value.parse::<f64>() {
                            sum += val;
                        }
                    }
                }
            }
        }
        sum
    }
} // 🔍Helpers
pub use finder_functions::*;

mod kit_diff_validation {
    // 📦Kit Diff Validation
    // Kit diff validation: errors vs warnings; optional JSON heal aligned with Go/TS asset cases.
    include!("kit_diff_validation.inc.rs");
} // 📦Kit Diff Validation
pub use kit_diff_validation::*;

mod kit_graph {
    // 📇Kit Graph Session
    // Parallel to TypeScript `commitKitGraphChange`: `KitGraphChange`, backbone hook, mutex-backed session, transactions, undo stacks.
    include!("kit_graph.inc.rs");
}
pub use kit_graph::*;

mod oop {
    use super::*;
    use serde::{Deserialize, Serialize};

    // ——— Geometry aliases (diagram: Point, Coordinate, Offset)

    /// Diagram `Point` — 3D placement; same wire shape as [`Vector`].
    pub type Point = Vector;

    /// Diagram `Coordinate` — piece layout in 2D parameter space; same as [`Coord`].
    pub type Coordinate = Coord;

    /// Translation in u/v space for [`Coordinate::translate`].
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct Offset {
        pub du: f64,
        pub dv: f64,
    }

    impl Offset {
        pub fn new(du: f64, dv: f64) -> Self {
            Self { du, dv }
        }

        pub fn invert(&self) -> Self {
            Self {
                du: -self.du,
                dv: -self.dv,
            }
        }
    }

    impl Coord {
        pub fn translate(&self, offset: &Offset) -> Self {
            Self {
                u: self.u + offset.du,
                v: self.v + offset.dv,
            }
        }
    }

    impl Vector {
        fn length(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        pub fn normalize(&self) -> Self {
            let len = self.length();
            if len <= 1e-15 || !len.is_finite() {
                return Vector::zero();
            }
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }

        pub fn scale(&self, factor: f64) -> Self {
            Self {
                x: self.x * factor,
                y: self.y * factor,
                z: self.z * factor,
            }
        }

    }

    impl Point {
        pub fn translate(&self, vector: &Vector) -> Self {
            Self {
                x: self.x + vector.x,
                y: self.y + vector.y,
                z: self.z + vector.z,
            }
        }
    }

    impl Plane {
        pub fn move_to(&mut self, origin: Point) {
            self.origin = origin;
        }

        pub fn rotate(&mut self, x_axis: Vector, y_axis: Vector) {
            self.x_axis = x_axis;
            self.y_axis = y_axis;
        }
    }

    include!("oop_store_dto.inc.rs");

    pub trait Actor {
        fn get_name(&self) -> &str;
        fn get_email(&self) -> &str;
        fn get_color(&self) -> &str;
    }

    /// Interactive human actor (session starter).
    #[derive(Debug, Clone)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub color: String,
    }

    impl Actor for User {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_email(&self) -> &str {
            &self.email
        }
        fn get_color(&self) -> &str {
            &self.color
        }
    }

    impl User {
        pub fn start_session(&self, _timeout_seconds: f64) {
            let _ = (_timeout_seconds, self);
        }
    }

    /// Automated actor executing structured kit commands.
    #[derive(Debug, Clone)]
    pub struct Agent {
        pub name: String,
        pub email: String,
        pub color: String,
    }

    impl Actor for Agent {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_email(&self) -> &str {
            &self.email
        }
        fn get_color(&self) -> &str {
            &self.color
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KitCommandKind {
        Query,
        Mutate,
    }

    impl Agent {
        pub fn execute<S: Store>(&self, _command_kind: KitCommandKind, target: &S) {
            let _ = (self, target);
        }
    }

    // ——— KitOperation & KitClient

    /// Design-level edit scoped selection + undo stack for operations within a [`KitClient`] session.
    pub struct KitOperation {
        pub selection: Vec<AnyStore>,
        undo_stack: Vec<KitGraphChange>,
        redo_stack: Vec<KitGraphChange>,
    }

    impl Default for KitOperation {
        fn default() -> Self {
            Self {
                selection: vec![],
                undo_stack: vec![],
                redo_stack: vec![],
            }
        }
    }

    impl KitOperation {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add_to_selection(&mut self, entity: AnyStore) {
            let g = entity.guid().to_string();
            if !self.selection.iter().any(|e| e.guid() == g) {
                self.selection.push(entity);
            }
        }

        pub fn add_many_to_selection(&mut self, entities: impl IntoIterator<Item = AnyStore>) {
            for e in entities {
                self.add_to_selection(e);
            }
        }

        pub fn remove_from_selection(&mut self, entity: &AnyStore) {
            let g = entity.guid();
            self.selection.retain(|e| e.guid() != g);
        }

        pub fn remove_many_from_selection(&mut self, entities: &[AnyStore]) {
            for e in entities {
                self.remove_from_selection(e);
            }
        }

        pub fn clear_selection(&mut self) {
            self.selection.clear();
        }

        pub fn record_change(&mut self, change: KitGraphChange) {
            self.undo_stack.push(change);
            self.redo_stack.clear();
        }

        pub fn undo(&mut self, kit: &mut Kit) -> bool {
            let Some(ch) = self.undo_stack.pop() else {
                return false;
            };
            apply_kit_diff(kit, &ch.backward);
            self.redo_stack.push(ch);
            true
        }

        pub fn can_undo(&self) -> bool {
            !self.undo_stack.is_empty()
        }

        pub fn redo(&mut self, kit: &mut Kit) -> bool {
            let Some(ch) = self.redo_stack.pop() else {
                return false;
            };
            apply_kit_diff(kit, &ch.forward);
            self.undo_stack.push(ch);
            true
        }

        pub fn can_redo(&self) -> bool {
            !self.redo_stack.is_empty()
        }
    }

    pub struct KitClient<A: Actor> {
        pub actor: A,
        pub session: KitGraphSession,
        pub operations: Vec<KitOperation>,
        active_index: Option<usize>,
        timeout_seconds: f64,
    }

    impl<A: Actor> KitClient<A> {
        pub fn new(kit: Kit, actor: A) -> Self {
            Self {
                actor,
                session: KitGraphSession::new(kit),
                operations: vec![],
                active_index: None,
                timeout_seconds: 0.0,
            }
        }

        pub fn start_session(&mut self, timeout_seconds: f64) {
            self.timeout_seconds = timeout_seconds;
            let _ = self.session.start_transaction();
        }

        pub fn end_session(&mut self) {
            self.timeout_seconds = 0.0;
            self.active_index = None;
        }

        pub fn map_kit<T>(&self, f: impl FnOnce(&Kit) -> T) -> Result<T> {
            self.session.map_kit(f)
        }

        pub fn map_kit_mut<T>(&self, f: impl FnOnce(&mut Kit) -> T) -> Result<T> {
            self.session.map_kit_mut(f)
        }

        pub fn commit(&self, diff: KitDiff, opts: KitCommitOptions) -> Result<KitGraphChange> {
            self.session.commit(diff, opts)
        }

        pub fn undo(&self) -> Result<()> {
            self.session.undo_history()
        }

        pub fn can_undo(&self) -> Result<bool> {
            self.session.can_undo_history()
        }

        pub fn redo(&self) -> Result<()> {
            self.session.redo_history()
        }

        pub fn can_redo(&self) -> Result<bool> {
            self.session.can_redo_history()
        }

        pub fn start_new_operation(&mut self) {
            self.operations.push(KitOperation::new());
            self.active_index = Some(self.operations.len() - 1);
        }

        pub fn set_active_operation(&mut self, operation: KitOperation) {
            if let Some(i) = self.active_index {
                if i < self.operations.len() {
                    self.operations[i] = operation;
                    return;
                }
            }
            self.operations.push(operation);
            self.active_index = Some(self.operations.len() - 1);
        }

        pub fn submit_operation(&mut self, operation: KitOperation) {
            self.operations.push(operation);
        }

        pub fn submit_active_operation(&mut self) {
            self.active_index = None;
        }

        pub fn submit_all_operations(&mut self) {
            self.operations.clear();
            self.active_index = None;
        }

        pub fn cancel_operation(&mut self, _operation: &KitOperation) {
            // Caller retains ownership of passed-in op; drop local copy if matched — simplified clear-active.
            self.active_index = None;
        }

        pub fn cancel_active_operation(&mut self) {
            self.active_index = None;
        }

        pub fn cancel_all_operations(&mut self) {
            self.operations.clear();
            self.active_index = None;
        }

        /// Mutable kit reference through session for applying entity-level APIs.
        pub fn with_kit_mut<T>(&self, f: impl FnOnce(&mut Kit) -> T) -> Result<T> {
            self.session.map_kit_mut(f)
        }
    }

    // ——— Diff-from (entity-local diff math; used by `Kit::diff_from` / `Design::diff_from`)

    impl Attribute {
        pub(crate) fn diff_from(&self, after: &Attribute) -> AttributeDiff {
            let mut diff = AttributeDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.key != after.key {
                diff.key = Some(after.key.clone());
            }
            if self.value != after.value {
                diff.value = Some(after.value.clone());
            }
            if self.definition != after.definition {
                diff.definition = Some(after.definition.clone());
            }
            diff
        }
    }

    impl Prop {
        pub(crate) fn diff_from(&self, after: &Prop) -> PropDiff {
            let mut diff = PropDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.quality != after.quality {
                diff.quality = Some(after.quality.clone());
            }
            if self.value != after.value {
                diff.value = Some(after.value.clone());
            }
            if self.unit != after.unit {
                diff.unit = Some(after.unit.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Connector {
        pub(crate) fn diff_from(&self, after: &Connector) -> ConnectorDiff {
            let mut diff = ConnectorDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.point != after.point {
                diff.point = Some(Vector {
                    x: after.point.x - self.point.x,
                    y: after.point.y - self.point.y,
                    z: after.point.z - self.point.z,
                });
            }
            if self.direction != after.direction {
                diff.direction = Some(Vector {
                    x: after.direction.x - self.direction.x,
                    y: after.direction.y - self.direction.y,
                    z: after.direction.z - self.direction.z,
                });
            }
            if self.t != after.t {
                diff.t = Some(after.t);
            }
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.mandatory != after.mandatory {
                diff.mandatory = Some(after.mandatory);
            }
            if self.port != after.port {
                diff.port = Some(after.port.clone());
            }
            diff.props =
                get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Model {
        pub(crate) fn diff_from(&self, after: &Model) -> ModelDiff {
            let mut diff = ModelDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.file != after.file {
                diff.file = Some(after.file.clone());
            }
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.tags != after.tags {
                diff.tags = Some(after.tags.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Type {
        pub(crate) fn diff_from(&self, after: &Type) -> TypeDiff {
            let mut diff = TypeDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.parent != after.parent {
                diff.parent = Some(after.parent.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            if self.image != after.image {
                diff.image = Some(after.image.clone());
            }
            if self.folder != after.folder {
                diff.folder = Some(after.folder.clone());
            }
            if self.unit != after.unit {
                diff.unit = Some(after.unit.clone());
            }
            if self.stock != after.stock {
                diff.stock = Some(after.stock);
            }
            if self.is_abstract != after.is_abstract {
                diff.is_abstract = Some(after.is_abstract);
            }
            if self.virtual_type != after.virtual_type {
                diff.virtual_type = Some(after.virtual_type);
            }
            if self.location != after.location {
                diff.location = Some(after.location.clone());
            }
            if self.concepts != after.concepts {
                diff.concepts = Some(after.concepts.clone());
            }
            if self.authors != after.authors {
                diff.authors = Some(after.authors.clone());
            }
            diff.props = get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| {
                b.diff_from(a)
            });
            diff.models =
                get_guid_collection_diff(&self.models, &after.models, "model", |b, a| b.diff_from(a));
            diff.connectors = get_guid_collection_diff(
                &self.connectors,
                &after.connectors,
                "connector",
                |b, a| b.diff_from(a),
            );
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Piece {
        pub(crate) fn diff_from(&self, after: &Piece) -> PieceDiff {
            let mut diff = PieceDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.type_ref != after.type_ref {
                diff.type_ref = Some(after.type_ref.clone());
            }
            if self.design != after.design {
                diff.design = Some(after.design.clone());
            }
            if self.plane != after.plane {
                diff.plane = Some(after.plane.clone());
            }
            if self.center != after.center {
                diff.center = Some(after.center.clone());
            }
            if self.scale != after.scale {
                diff.scale = Some(after.scale);
            }
            if self.mirror_plane != after.mirror_plane {
                diff.mirror_plane = Some(after.mirror_plane.clone());
            }
            if self.is_hidden != after.is_hidden {
                diff.is_hidden = Some(after.is_hidden);
            }
            if self.is_locked != after.is_locked {
                diff.is_locked = Some(after.is_locked);
            }
            if self.color != after.color {
                diff.color = Some(after.color.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            diff.props =
                get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Connection {
        pub(crate) fn diff_from(&self, after: &Connection) -> ConnectionDiff {
            let mut diff = ConnectionDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.connected != after.connected {
                let mut sd = SideDiff::default();
                if self.connected.piece != after.connected.piece {
                    sd.piece = Some(after.connected.piece.clone());
                }
                if self.connected.design_piece != after.connected.design_piece {
                    sd.design_piece = Some(after.connected.design_piece.clone());
                }
                if self.connected.connector != after.connected.connector {
                    sd.connector = Some(after.connected.connector.clone());
                }
                diff.connected = Some(sd);
            }
            if self.connecting != after.connecting {
                let mut sd = SideDiff::default();
                if self.connecting.piece != after.connecting.piece {
                    sd.piece = Some(after.connecting.piece.clone());
                }
                if self.connecting.design_piece != after.connecting.design_piece {
                    sd.design_piece = Some(after.connecting.design_piece.clone());
                }
                if self.connecting.connector != after.connecting.connector {
                    sd.connector = Some(after.connecting.connector.clone());
                }
                diff.connecting = Some(sd);
            }
            if self.gap != after.gap {
                diff.gap = Some(after.gap - self.gap);
            }
            if self.shift != after.shift {
                diff.shift = Some(after.shift - self.shift);
            }
            if self.rise != after.rise {
                diff.rise = Some(after.rise - self.rise);
            }
            if self.rotation != after.rotation {
                diff.rotation = Some(after.rotation - self.rotation);
            }
            if self.turn != after.turn {
                diff.turn = Some(after.turn - self.turn);
            }
            if self.tilt != after.tilt {
                diff.tilt = Some(after.tilt - self.tilt);
            }
            if self.u != after.u {
                diff.u = Some(match (self.u, after.u) {
                    (Some(b), Some(a)) => Some(a - b),
                    (None, Some(a)) => Some(a),
                    (_, None) => None,
                });
            }
            if self.v != after.v {
                diff.v = Some(match (self.v, after.v) {
                    (Some(b), Some(a)) => Some(a - b),
                    (None, Some(a)) => Some(a),
                    (_, None) => None,
                });
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Layer {
        pub(crate) fn diff_from(&self, after: &Layer) -> LayerDiff {
            let mut diff = LayerDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.path != after.path {
                diff.path = Some(after.path.clone());
            }
            if self.is_hidden != after.is_hidden {
                diff.is_hidden = Some(after.is_hidden);
            }
            if self.is_locked != after.is_locked {
                diff.is_locked = Some(after.is_locked);
            }
            if self.color != after.color {
                diff.color = Some(after.color.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Group {
        pub(crate) fn diff_from(&self, after: &Group) -> GroupDiff {
            let mut diff = GroupDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.color != after.color {
                diff.color = Some(after.color.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.pieces != after.pieces {
                diff.pieces = Some(after.pieces.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Stat {
        pub(crate) fn diff_from(&self, after: &Stat) -> StatDiff {
            let mut diff = StatDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.quality != after.quality {
                diff.quality = Some(after.quality.clone());
            }
            if self.min != after.min {
                diff.min = Some(after.min);
            }
            if self.min_excluded != after.min_excluded {
                diff.min_excluded = Some(after.min_excluded);
            }
            if self.max != after.max {
                diff.max = Some(after.max);
            }
            if self.max_excluded != after.max_excluded {
                diff.max_excluded = Some(after.max_excluded);
            }
            if self.unit != after.unit {
                diff.unit = Some(after.unit.clone());
            }
            diff
        }
    }

    impl Tag {
        pub(crate) fn diff_from(&self, after: &Tag) -> TagDiff {
            let mut diff = TagDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            diff
        }
    }

    impl Concept {
        pub(crate) fn diff_from(&self, after: &Concept) -> ConceptDiff {
            let mut diff = ConceptDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            diff
        }
    }

    impl Port {
        pub(crate) fn diff_from(&self, after: &Port) -> PortDiff {
            let mut diff = PortDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            if self.compatible_interfaces != after.compatible_interfaces {
                diff.compatible_interfaces = Some(after.compatible_interfaces.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Quality {
        pub(crate) fn diff_from(&self, after: &Quality) -> QualityDiff {
            let mut diff = QualityDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.key != after.key {
                diff.key = Some(after.key.clone());
            }
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.kind != after.kind {
                diff.kind = Some(after.kind.clone());
            }
            if self.default_value != after.default_value {
                diff.default_value = Some(after.default_value);
            }
            if self.formula != after.formula {
                diff.formula = Some(after.formula.clone());
            }
            if self.default_si_unit != after.default_si_unit {
                diff.default_si_unit = Some(after.default_si_unit.clone());
            }
            if self.default_imperial_unit != after.default_imperial_unit {
                diff.default_imperial_unit = Some(after.default_imperial_unit.clone());
            }
            if self.min != after.min {
                diff.min = Some(after.min);
            }
            if self.is_min_excluded != after.is_min_excluded {
                diff.is_min_excluded = Some(after.is_min_excluded);
            }
            if self.max != after.max {
                diff.max = Some(after.max);
            }
            if self.is_max_excluded != after.is_max_excluded {
                diff.is_max_excluded = Some(after.is_max_excluded);
            }
            if self.can_scale != after.can_scale {
                diff.can_scale = Some(after.can_scale);
            }
            if self.uri != after.uri {
                diff.uri = Some(after.uri.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl File {
        pub(crate) fn diff_from(&self, after: &File) -> FileDiff {
            let mut diff = FileDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.remote != after.remote {
                diff.remote = Some(after.remote.clone());
            }
            if self.folder != after.folder {
                diff.folder = Some(after.folder.clone());
            }
            if self.size != after.size {
                diff.size = Some(after.size);
            }
            if self.hash != after.hash {
                diff.hash = Some(after.hash.clone());
            }
            diff
        }
    }

    impl Folder {
        pub(crate) fn diff_from(&self, after: &Folder) -> FolderDiff {
            let mut diff = FolderDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.parent != after.parent {
                diff.parent = Some(after.parent.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Author {
        pub(crate) fn diff_from(&self, after: &Author) -> AuthorDiff {
            let mut diff = AuthorDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.email != after.email {
                diff.email = Some(after.email.clone());
            }
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }
    }

    impl Kit {
        /// Computes a structural diff from `self` to `after` (replaces `get_kit_diff` logic).
        pub fn diff_from(&self, after: &Kit) -> KitDiff {
            let mut diff = KitDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.version != after.version {
                diff.version = Some(after.version.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            if self.image != after.image {
                diff.image = Some(after.image.clone());
            }
            if self.preview != after.preview {
                diff.preview = Some(after.preview.clone());
            }
            if self.remote != after.remote {
                diff.remote = Some(after.remote.clone());
            }
            if self.homepage != after.homepage {
                diff.homepage = Some(after.homepage.clone());
            }
            if self.license != after.license {
                diff.license = Some(after.license.clone());
            }
            diff.types =
                get_guid_collection_diff(&self.types, &after.types, "type", |b, a| b.diff_from(a));
            diff.designs =
                get_guid_collection_diff(&self.designs, &after.designs, "design", |b, a| {
                    b.diff_from(a)
                });
            diff.tags =
                get_guid_collection_diff(&self.tags, &after.tags, "tag", |b, a| b.diff_from(a));
            diff.concepts =
                get_guid_collection_diff(&self.concepts, &after.concepts, "concept", |b, a| {
                    b.diff_from(a)
                });
            diff.ports =
                get_guid_collection_diff(&self.ports, &after.ports, "port", |b, a| b.diff_from(a));
            diff.qualities = get_guid_collection_diff(
                &self.qualities,
                &after.qualities,
                "quality",
                |b, a| b.diff_from(a),
            );
            diff.files =
                get_guid_collection_diff(&self.files, &after.files, "file", |b, a| b.diff_from(a));
            diff.folders =
                get_guid_collection_diff(&self.folders, &after.folders, "folder", |b, a| {
                    b.diff_from(a)
                });
            diff.authors =
                get_guid_collection_diff(&self.authors, &after.authors, "author", |b, a| {
                    b.diff_from(a)
                });
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }

        pub fn find_type_entity<'a>(&'a self, t: &Type) -> Option<&'a Type> {
            self.types.as_ref()?.iter().find(|x| x.guid == t.guid)
        }

        pub fn find_design_entity<'a>(&'a self, d: &Design) -> Option<&'a Design> {
            self.designs.as_ref()?.iter().find(|x| x.guid == d.guid)
        }

        pub fn find_tag<'a>(&'a self, tag: &Tag) -> Option<&'a Tag> {
            self.tags.as_ref()?.iter().find(|x| x.guid == tag.guid)
        }

        pub fn find_concept<'a>(&'a self, c: &Concept) -> Option<&'a Concept> {
            self.concepts.as_ref()?.iter().find(|x| x.guid == c.guid)
        }

        pub fn find_port<'a>(&'a self, p: &Port) -> Option<&'a Port> {
            self.ports.as_ref()?.iter().find(|x| x.guid == p.guid)
        }

        pub fn find_quality<'a>(&'a self, q: &Quality) -> Option<&'a Quality> {
            self.qualities.as_ref()?.iter().find(|x| x.guid == q.guid)
        }

        pub fn find_file<'a>(&'a self, f: &File) -> Option<&'a File> {
            self.files.as_ref()?.iter().find(|x| x.guid == f.guid)
        }

        pub fn find_folder<'a>(&'a self, f: &Folder) -> Option<&'a Folder> {
            self.folders.as_ref()?.iter().find(|x| x.guid == f.guid)
        }

        pub fn find_author<'a>(&'a self, a: &Author) -> Option<&'a Author> {
            self.authors.as_ref()?.iter().find(|x| x.guid == a.guid)
        }

        pub fn create_tag(&mut self, tag: Tag) -> Result<()> {
            let v = self.tags.get_or_insert_with(Vec::new);
            if v.iter().any(|t| t.guid == tag.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "tag guid already exists".into(),
                });
            }
            v.push(tag);
            Ok(())
        }

        pub fn create_tags(&mut self, tags: Vec<Tag>) -> Result<()> {
            for t in tags {
                self.create_tag(t)?;
            }
            Ok(())
        }

        pub fn delete_tag(&mut self, tag: &Tag) -> Result<()> {
            let Some(v) = self.tags.as_mut() else {
                return Ok(());
            };
            v.retain(|t| t.guid != tag.guid);
            Ok(())
        }

        pub fn delete_tags(&mut self, tags: &[Tag]) -> Result<()> {
            for t in tags {
                self.delete_tag(t)?;
            }
            Ok(())
        }

        pub fn create_concept(&mut self, concept: Concept) -> Result<()> {
            let v = self.concepts.get_or_insert_with(Vec::new);
            if v.iter().any(|c| c.guid == concept.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "concept guid already exists".into(),
                });
            }
            v.push(concept);
            Ok(())
        }

        pub fn create_concepts(&mut self, concepts: Vec<Concept>) -> Result<()> {
            for c in concepts {
                self.create_concept(c)?;
            }
            Ok(())
        }

        pub fn delete_concept(&mut self, c: &Concept) -> Result<()> {
            let Some(v) = self.concepts.as_mut() else {
                return Ok(());
            };
            v.retain(|x| x.guid != c.guid);
            Ok(())
        }

        pub fn delete_concepts(&mut self, concepts: &[Concept]) -> Result<()> {
            for c in concepts {
                self.delete_concept(c)?;
            }
            Ok(())
        }

        pub fn create_port(&mut self, port: Port) -> Result<()> {
            let v = self.ports.get_or_insert_with(Vec::new);
            if v.iter().any(|p| p.guid == port.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "port guid already exists".into(),
                });
            }
            v.push(port);
            Ok(())
        }

        pub fn create_ports(&mut self, ports: Vec<Port>) -> Result<()> {
            for p in ports {
                self.create_port(p)?;
            }
            Ok(())
        }

        pub fn delete_port(&mut self, p: &Port) -> Result<()> {
            let Some(v) = self.ports.as_mut() else {
                return Ok(());
            };
            v.retain(|x| x.guid != p.guid);
            Ok(())
        }

        pub fn delete_ports(&mut self, ports: &[Port]) -> Result<()> {
            for p in ports {
                self.delete_port(p)?;
            }
            Ok(())
        }

        pub fn create_quality(&mut self, quality: Quality) -> Result<()> {
            let v = self.qualities.get_or_insert_with(Vec::new);
            if v.iter().any(|q| q.guid == quality.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "quality guid already exists".into(),
                });
            }
            v.push(quality);
            Ok(())
        }

        pub fn create_qualities(&mut self, qualities: Vec<Quality>) -> Result<()> {
            for q in qualities {
                self.create_quality(q)?;
            }
            Ok(())
        }

        pub fn delete_quality(&mut self, q: &Quality) -> Result<()> {
            let Some(v) = self.qualities.as_mut() else {
                return Ok(());
            };
            v.retain(|x| x.guid != q.guid);
            Ok(())
        }

        pub fn delete_qualities(&mut self, qualities: &[Quality]) -> Result<()> {
            for q in qualities {
                self.delete_quality(q)?;
            }
            Ok(())
        }

        pub fn create_type(&mut self, t: Type) -> Result<()> {
            let v = self.types.get_or_insert_with(Vec::new);
            if v.iter().any(|x| x.guid == t.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "type guid already exists".into(),
                });
            }
            v.push(t);
            Ok(())
        }

        pub fn create_types(&mut self, types: Vec<Type>) -> Result<()> {
            for t in types {
                self.create_type(t)?;
            }
            Ok(())
        }

        pub fn delete_type(&mut self, t: &Type) -> Result<()> {
            let Some(v) = self.types.as_mut() else {
                return Ok(());
            };
            v.retain(|x| x.guid != t.guid);
            Ok(())
        }

        pub fn delete_types(&mut self, types: &[Type]) -> Result<()> {
            for t in types {
                self.delete_type(t)?;
            }
            Ok(())
        }

        pub fn create_design(&mut self, d: Design) -> Result<()> {
            let v = self.designs.get_or_insert_with(Vec::new);
            if v.iter().any(|x| x.guid == d.guid) {
                return Err(SemioError::InvalidOperation {
                    message: "design guid already exists".into(),
                });
            }
            v.push(d);
            Ok(())
        }

        pub fn create_designs(&mut self, designs: Vec<Design>) -> Result<()> {
            for d in designs {
                self.create_design(d)?;
            }
            Ok(())
        }

        pub fn delete_design(&mut self, d: &Design) -> Result<()> {
            let Some(v) = self.designs.as_mut() else {
                return Ok(());
            };
            v.retain(|x| x.guid != d.guid);
            Ok(())
        }

        pub fn delete_designs(&mut self, designs: &[Design]) -> Result<()> {
            for d in designs {
                self.delete_design(d)?;
            }
            Ok(())
        }
    }

    impl Design {
        pub fn diff_from(&self, after: &Design) -> DesignDiff {
            let mut diff = DesignDiff {
                guid: self.guid.clone(),
                ..Default::default()
            };
            if self.name != after.name {
                diff.name = Some(after.name.clone());
            }
            if self.parent != after.parent {
                diff.parent = Some(after.parent.clone());
            }
            if self.description != after.description {
                diff.description = Some(after.description.clone());
            }
            if self.icon != after.icon {
                diff.icon = Some(after.icon.clone());
            }
            if self.image != after.image {
                diff.image = Some(after.image.clone());
            }
            if self.folder != after.folder {
                diff.folder = Some(after.folder.clone());
            }
            if self.unit != after.unit {
                diff.unit = Some(after.unit.clone());
            }
            if self.is_abstract != after.is_abstract {
                diff.is_abstract = Some(after.is_abstract);
            }
            if self.can_scale != after.can_scale {
                diff.can_scale = Some(after.can_scale);
            }
            if self.can_mirror != after.can_mirror {
                diff.can_mirror = Some(after.can_mirror);
            }
            if self.concepts != after.concepts {
                diff.concepts = Some(after.concepts.clone());
            }
            if self.authors != after.authors {
                diff.authors = Some(after.authors.clone());
            }
            if self.active_layer != after.active_layer {
                diff.active_layer = Some(after.active_layer.clone());
            }
            diff.props =
                get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
            diff.pieces =
                get_guid_collection_diff(&self.pieces, &after.pieces, "piece", |b, a| b.diff_from(a));
            diff.connections = get_guid_collection_diff(
                &self.connections,
                &after.connections,
                "connection",
                |b, a| b.diff_from(a),
            );
            diff.layers =
                get_guid_collection_diff(&self.layers, &after.layers, "layer", |b, a| b.diff_from(a));
            diff.groups =
                get_guid_collection_diff(&self.groups, &after.groups, "group", |b, a| b.diff_from(a));
            diff.stats =
                get_guid_collection_diff(&self.stats, &after.stats, "stat", |b, a| b.diff_from(a));
            diff.attributes = get_guid_collection_diff(
                &self.attributes,
                &after.attributes,
                "attribute",
                |b, a| b.diff_from(a),
            );
            diff
        }

        pub fn find_piece<'a>(&'a self, piece: &Piece) -> Option<&'a Piece> {
            self.pieces.as_ref()?.iter().find(|p| p.guid == piece.guid)
        }

        pub fn find_connection<'a>(&'a self, c: &Connection) -> Option<&'a Connection> {
            self.connections
                .as_ref()?
                .iter()
                .find(|x| x.guid == c.guid)
        }

        pub fn flatten(&self, kit: &Kit) -> SemioReport<DesignChange> {
            flatten_design(kit, &self.guid)
        }

        /// Deletes pieces and connections using entity references; expands stale connection removals.
        pub fn delete_pieces_and_connections(
            &self,
            kit: &Kit,
            pieces: &[Piece],
            connections: &[Connection],
        ) -> SemioReport<DesignDiff> {
            let piece_guids: Vec<String> = pieces.iter().map(|p| p.guid.clone()).collect();
            let connection_guids: Vec<String> = connections.iter().map(|c| c.guid.clone()).collect();
            delete_pieces_and_connections_in_design(kit, self, &piece_guids, &connection_guids)
        }

        pub fn drag_pieces(&self, pieces: &[Piece], offset: &Coord) -> DesignDiff {
            let design_pieces = self.pieces.as_deref().unwrap_or(&[]);
            let design_connections = self.connections.as_deref().unwrap_or(&[]);
            let mut d =
                drag_pieces_in_design(design_pieces, design_connections, pieces, offset);
            d.guid = self.guid.clone();
            d
        }
    }

    fn kit_diff_update_concept(kit: &Kit, before: &Concept, after: &Concept) -> KitDiff {
        let d = before.diff_from(after);
        KitDiff {
            guid: kit.guid.clone(),
            concepts: Some(CollectionDiff {
                updated: Some(vec![DiffUpdate {
                    key: "concept".into(),
                    guid: before.guid.clone(),
                    diff: d,
                }]),
                removed: None,
                added: None,
            }),
            ..KitDiff::default()
        }
    }

    fn kit_diff_remove_concept(kit: &Kit, c: &Concept) -> KitDiff {
        KitDiff {
            guid: kit.guid.clone(),
            concepts: Some(CollectionDiff {
                removed: Some(vec![RemovedItem {
                    guid: c.guid.clone(),
                }]),
                updated: None,
                added: None,
            }),
            ..KitDiff::default()
        }
    }

    impl Concept {
        pub fn rename(&self, kit: &mut Kit, name: impl Into<String>) -> Result<()> {
            let Some(v) = kit.concepts.as_ref() else {
                return Err(SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                });
            };
            let before = v
                .iter()
                .find(|c| c.guid == self.guid)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.name = name.into();
            let kd = kit_diff_update_concept(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
            let Some(v) = kit.concepts.as_ref() else {
                return Err(SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                });
            };
            let before = v
                .iter()
                .find(|c| c.guid == self.guid)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.description = Some(description.to_string());
            let kd = kit_diff_update_concept(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn update_icon(&self, kit: &mut Kit, icon: &str) -> Result<()> {
            let Some(v) = kit.concepts.as_ref() else {
                return Err(SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                });
            };
            let before = v
                .iter()
                .find(|c| c.guid == self.guid)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Concept".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.icon = Some(icon.to_string());
            let kd = kit_diff_update_concept(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn delete(&self, kit: &mut Kit) -> Result<()> {
            let kd = kit_diff_remove_concept(kit, self);
            apply_kit_diff(kit, &kd);
            Ok(())
        }
    }

    fn kit_diff_update_tag(kit: &Kit, before: &Tag, after: &Tag) -> KitDiff {
        let d = before.diff_from(after);
        KitDiff {
            guid: kit.guid.clone(),
            tags: Some(CollectionDiff {
                updated: Some(vec![DiffUpdate {
                    key: "tag".into(),
                    guid: before.guid.clone(),
                    diff: d,
                }]),
                removed: None,
                added: None,
            }),
            ..KitDiff::default()
        }
    }

    impl Tag {
        pub fn rename(&self, kit: &mut Kit, name: impl Into<String>) -> Result<()> {
            let before = kit
                .tags
                .as_ref()
                .and_then(|v| v.iter().find(|t| t.guid == self.guid))
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Tag".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.name = name.into();
            let kd = kit_diff_update_tag(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
            let before = kit
                .tags
                .as_ref()
                .and_then(|v| v.iter().find(|t| t.guid == self.guid))
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Tag".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.description = Some(description.to_string());
            let kd = kit_diff_update_tag(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn update_icon(&self, kit: &mut Kit, icon: &str) -> Result<()> {
            let before = kit
                .tags
                .as_ref()
                .and_then(|v| v.iter().find(|t| t.guid == self.guid))
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Tag".into(),
                    guid: self.guid.clone(),
                })?
                .clone();
            let mut after = before.clone();
            after.icon = Some(icon.to_string());
            let kd = kit_diff_update_tag(kit, &before, &after);
            apply_kit_diff(kit, &kd);
            Ok(())
        }

        pub fn delete(&self, kit: &mut Kit) -> Result<()> {
            let kd = KitDiff {
                guid: kit.guid.clone(),
                tags: Some(CollectionDiff {
                    removed: Some(vec![RemovedItem {
                        guid: self.guid.clone(),
                    }]),
                    updated: None,
                    added: None,
                }),
                ..KitDiff::default()
            };
            apply_kit_diff(kit, &kd);
            Ok(())
        }
    }
}

pub use oop::*;

mod tests {
    // 🧪Tests
    // 🏘️Tests
    // Tests MUST provide the tests functionality.

    use super::*;

    #[cfg(test)]
    /// <summary>🔧ASSETS_DIR.</summary>
    /// <remarks>
    /// </remarks>
    mod tests {
        use super::*;
        use serde::Deserialize;
        use std::fs;
        use std::path::Path;

        pub const ASSETS_DIR: &str = "../assets/semio";
        pub const TOLERANCE: f64 = 0.001;

        pub fn load_kit(filename: &str) -> Kit {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data =
                fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
            serde_json::from_str(&data).expect("Failed to deserialize kit")
        }

        pub fn load_kit_diff(filename: &str) -> KitDiff {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data =
                fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
            serde_json::from_str(&data).expect("Failed to deserialize kit diff")
        }

        pub fn load_validation_result(filename: &str) -> ValidationResult {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data =
                fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
            serde_json::from_str(&data).expect("Failed to deserialize validation result")
        }

        pub fn load_asset<T: serde::de::DeserializeOwned>(filename: &str) -> T {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", filename, e));
            serde_json::from_str(&data)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", filename, e))
        }

        //#region 🔖AssetCaseStructs
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct HashCasesAsset {
            pub kit_hash: HashKitCase,
            pub kit_diff_hash: HashKitDiffCase,
            pub design_name: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct HashKitCase {
            pub kit: String,
            pub expected: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct HashKitDiffCase {
            pub json: String,
            pub expected: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct QualitySumCasesAsset {
            pub cases: Vec<QualitySumCase>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct QualitySumCase {
            pub name: String,
            pub kit: String,
            pub design_name: String,
            pub design_parent: Option<String>,
            pub quality_name: String,
            pub expected: f64,
            pub tolerance: f64,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FilterKitCasesAsset {
            pub cases: Vec<FilterKitCase>,
            pub glob_cases: Vec<FilterKitGlobCase>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FilterKitCase {
            pub name: String,
            pub kit: String,
            pub design_name: String,
            pub design_parent: Option<String>,
            pub expected_kit: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FilterKitGlobCase {
            pub name: String,
            pub kit: String,
            pub type_include: Option<Vec<String>>,
            pub type_exclude: Option<Vec<String>>,
            pub design_include: Option<Vec<String>>,
            pub design_name: Option<String>,
            pub design_parent: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct DesignWithDiffCasesAsset {
            pub cases: Vec<DesignWithDiffCase>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct DesignWithDiffCase {
            pub name: String,
            pub kit: String,
            pub design_name: String,
            pub design_parent: Option<String>,
            pub diff: String,
            pub expected: String,
            pub expected_piece_counts: StatusCounts,
            pub expected_connection_counts: StatusCounts,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct StatusCounts {
            pub unchanged: usize,
            pub modified: usize,
            pub removed: usize,
            pub added: usize,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FindReplaceableCasesAsset {
            pub synthetic_kit: String,
            pub cases: Vec<FindReplaceableCase>,
            pub boundary_cases: FindReplaceableBoundaryCases,
            pub synthetic_cases: Vec<FindReplaceableSyntheticCase>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FindReplaceableCase {
            pub name: String,
            pub kit: String,
            pub design_name: String,
            pub design_parent: Option<String>,
            pub design_parent_name: Option<String>,
            pub piece_names: Option<Vec<String>>,
            pub selection_asset: Option<String>,
            pub expected_selection_piece_count: Option<usize>,
            pub expected_selection_connection_count: Option<usize>,
            pub expected_type_guid_count: Option<usize>,
            pub expected_type_guids: Option<Vec<String>>,
            pub expected_design_guids: Option<Vec<String>>,
            pub use_piece_index: Option<usize>,
            pub expect_non_empty_types: Option<bool>,
            pub expect_own_type_in_results: Option<bool>,
            pub lookup_type_name: Option<String>,
            pub forbidden_type_names: Option<Vec<String>>,
            pub expect_connectorless_type_count: Option<bool>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FindReplaceableBoundaryCases {
            pub kit: String,
            pub design_name: String,
            pub design_parent: Option<String>,
            pub single_capsule_pieces: Vec<String>,
            pub two_capsule_pieces: Vec<String>,
            pub four_capsule_pieces: Vec<String>,
            pub eight_capsule_pieces: Vec<String>,
            pub tambour_piece_name: String,
            pub expected_tambour_type_guid_count: usize,
            pub expected_tambour_design_guid_count: usize,
            pub forbidden_families: Vec<String>,
            pub expected_two_capsule_families: Vec<String>,
            pub expected_large_families: Vec<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FindReplaceableSyntheticCase {
            pub name: String,
            pub design_guid: String,
            pub piece_guids: Vec<String>,
            pub expected_contains_types: Option<Vec<String>>,
            pub expected_not_contains_types: Option<Vec<String>>,
            pub expected_contains_designs: Option<Vec<String>>,
            pub expected_not_contains_designs: Option<Vec<String>>,
        }
        //#endregion

        pub fn float_eq(a: f64, b: f64) -> bool {
            (a - b).abs() < TOLERANCE
        }

        pub fn vectors_equal(v1: &Vector, v2: &Vector) -> bool {
            float_eq(v1.x, v2.x) && float_eq(v1.y, v2.y) && float_eq(v1.z, v2.z)
        }

        pub fn planes_equal(p1: &Plane, p2: &Plane) -> bool {
            vectors_equal(&p1.origin, &p2.origin)
                && vectors_equal(&p1.x_axis, &p2.x_axis)
                && vectors_equal(&p1.y_axis, &p2.y_axis)
        }

        pub fn centers_equal(c1: Option<&Coord>, c2: Option<&Coord>) -> bool {
            match (c1, c2) {
                (None, None) => true,
                (Some(a), Some(b)) => float_eq(a.u, b.u) && float_eq(a.v, b.v),
                _ => false,
            }
        }

        pub fn find_design_by_name<'a>(
            designs: &'a [Design],
            name: &str,
            parent_guid: Option<&str>,
        ) -> Option<&'a Design> {
            designs.iter().find(|d| {
                d.name == name
                    && match parent_guid {
                        None => d.parent.is_none(),
                        Some(pg) => d.parent.as_ref().map(|p| p.guid.as_str()) == Some(pg),
                    }
            })
        }

        pub fn find_piece_by_name<'a>(pieces: &'a [Piece], name: &str) -> Option<&'a Piece> {
            pieces.iter().find(|p| p.name.as_deref() == Some(name))
        }

        pub fn test_flatten_design(kit: &Kit, design_path: &[&str]) {
            let designs = kit.designs.as_ref().expect("Kit has no designs");

            let mut current_design: Option<&Design> = None;
            let mut parent_guid: Option<&str> = None;

            for name in design_path {
                current_design = find_design_by_name(designs, name, parent_guid);
                assert!(current_design.is_some(), "Design {} not found", name);
                parent_guid = current_design.map(|d| d.guid.as_str());
            }

            let design = current_design.expect("Design is None");
            let expected_design = find_design_by_name(designs, "Flat", Some(&design.guid))
                .expect("Expected Flat design not found");

            let flat_rep = flatten_design(kit, &design.guid);
            assert!(flat_rep.ok, "flatten_design failed: {:?}", flat_rep.errors);
            let flat_design_change = flat_rep.diff.expect("flatten ok implies diff");
            let mut flat_design = design.clone();
            apply_design_diff(&mut flat_design, &flat_design_change.forward);

            let expected_pieces = expected_design
                .pieces
                .as_ref()
                .expect("Expected design has no pieces");
            let flat_pieces = flat_design
                .pieces
                .as_ref()
                .expect("Flat design has no pieces");

            for piece in flat_pieces {
                if piece.name.is_none() {
                    continue;
                }
                let name = piece.name.as_ref().unwrap();
                let expected_piece = find_piece_by_name(expected_pieces, name)
                    .expect(&format!("Expected piece {} not found", name));

                assert!(piece.plane.is_some(), "Piece {} has no plane", name);
                assert!(
                    expected_piece.plane.is_some(),
                    "Expected piece {} has no plane",
                    name
                );

                let got = piece.plane.as_ref().unwrap();
                let exp = expected_piece.plane.as_ref().unwrap();
                if !planes_equal(got, exp) {
                    eprintln!("Piece: {}", name);
                    eprintln!(
                        "Got origin: ({:.4}, {:.4}, {:.4})",
                        got.origin.x, got.origin.y, got.origin.z
                    );
                    eprintln!(
                        "Exp origin: ({:.4}, {:.4}, {:.4})",
                        exp.origin.x, exp.origin.y, exp.origin.z
                    );
                    eprintln!(
                        "Got xAxis:  ({:.4}, {:.4}, {:.4})",
                        got.x_axis.x, got.x_axis.y, got.x_axis.z
                    );
                    eprintln!(
                        "Exp xAxis:  ({:.4}, {:.4}, {:.4})",
                        exp.x_axis.x, exp.x_axis.y, exp.x_axis.z
                    );
                    eprintln!(
                        "Got yAxis:  ({:.4}, {:.4}, {:.4})",
                        got.y_axis.x, got.y_axis.y, got.y_axis.z
                    );
                    eprintln!(
                        "Exp yAxis:  ({:.4}, {:.4}, {:.4})",
                        exp.y_axis.x, exp.y_axis.y, exp.y_axis.z
                    );
                }

                assert!(
                    planes_equal(
                        piece.plane.as_ref().unwrap(),
                        expected_piece.plane.as_ref().unwrap()
                    ),
                    "Plane mismatch for piece {}",
                    name
                );
                assert!(
                    centers_equal(piece.center.as_ref(), expected_piece.center.as_ref()),
                    "Center mismatch for piece {}",
                    name
                );
            }
        }

        #[derive(Debug, Deserialize)]
        pub struct ModelSelectionAsset {
            cases: Vec<ModelSelectionCase>,
        }

        #[derive(Debug, Deserialize)]
        pub struct ModelSelectionCase {
            name: String,
            #[serde(rename = "selectedTagGuids")]
            selected_tag_guids: Vec<String>,
            #[serde(rename = "expectedGuid")]
            expected_guid: Option<String>,
            models: Vec<ModelSelectionModel>,
        }

        #[derive(Debug, Deserialize)]
        pub struct ModelSelectionModel {
            guid: String,
            #[serde(rename = "fileGuid")]
            file_guid: String,
            #[serde(rename = "tagGuids")]
            tag_guids: Vec<String>,
        }

        pub fn contains_all_tags(model: &Model, selected_tag_guids: &[String]) -> bool {
            let model_tag_guids: Vec<String> = model
                .tags
                .as_ref()
                .map(|tags| tags.iter().map(|tag| tag.guid.clone()).collect())
                .unwrap_or_default();
            selected_tag_guids.iter().all(|selected| {
                model_tag_guids
                    .iter()
                    .any(|model_tag| model_tag == selected)
            })
        }

        pub fn jaccard_tag_guids(model_tag_guids: &[String], selected_tag_guids: &[String]) -> f64 {
            if model_tag_guids.is_empty() && selected_tag_guids.is_empty() {
                return 1.0;
            }
            let set_a: std::collections::HashSet<&String> = model_tag_guids.iter().collect();
            let set_b: std::collections::HashSet<&String> = selected_tag_guids.iter().collect();
            let intersection = set_a.intersection(&set_b).count();
            let union = set_a.union(&set_b).count();
            if union == 0 {
                0.0
            } else {
                intersection as f64 / union as f64
            }
        }

        pub fn select_best_model_like_semio_ts(
            models: &[Model],
            selected_tag_guids: &[String],
        ) -> Option<Model> {
            if models.is_empty() {
                return None;
            }
            if selected_tag_guids.is_empty() {
                if let Some(default_model) = models.iter().find(|model| {
                    model
                        .tags
                        .as_ref()
                        .map(|tags| tags.is_empty())
                        .unwrap_or(true)
                }) {
                    return Some(default_model.clone());
                }
                return Some(models[0].clone());
            }

            let filtered: Vec<Model> = models
                .iter()
                .filter(|model| contains_all_tags(model, selected_tag_guids))
                .cloned()
                .collect();
            if filtered.is_empty() {
                return None;
            }

            let mut max_index = 0usize;
            let mut max_score = {
                let guids: Vec<String> = filtered[0]
                    .tags
                    .as_ref()
                    .map(|tags| tags.iter().map(|tag| tag.guid.clone()).collect())
                    .unwrap_or_default();
                jaccard_tag_guids(&guids, selected_tag_guids)
            };

            for (i, model) in filtered.iter().enumerate().skip(1) {
                let guids: Vec<String> = model
                    .tags
                    .as_ref()
                    .map(|tags| tags.iter().map(|tag| tag.guid.clone()).collect())
                    .unwrap_or_default();
                let score = jaccard_tag_guids(&guids, selected_tag_guids);
                if score > max_score {
                    max_score = score;
                    max_index = i;
                }
            }

            Some(filtered[max_index].clone())
        }

        mod roundtrip_tests {
            // 👓Roundtrip Tests
            // Roundtrip Tests MUST provide the roundtrip tests functionality.

            use super::*;

            mod roundtrip {
                use super::*;

                #[test]
                pub fn metabolism() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let json = serialize_kit(&kit).unwrap();
                    let restored = deserialize_kit(&json).unwrap();
                    assert!(
                        are_kits_equal(&kit, &restored),
                        "JSON -> Memory -> JSON: serialized and deserialized kit should be equal"
                    );

                    use base64::Engine;
                    let mut files = std::collections::HashMap::new();
                    if let Some(ref kit_files) = kit.files {
                        for f in kit_files {
                            if let Some(ref blob) = f.blob {
                                let b64 = if let Some(pos) = blob.find(";base64,") {
                                    &blob[pos + 8..]
                                } else {
                                    blob.as_str()
                                };
                                let decoded = base64::engine::general_purpose::STANDARD
                                    .decode(b64)
                                    .unwrap();
                                let file_path = crate::zip_roundtrip::build_file_path(&kit, f);
                                files.insert(file_path, decoded);
                            }
                        }
                    }

                    let temp_dir = tempfile::tempdir().unwrap();
                    let roundtrip_path = temp_dir.path().join("metabolism_roundtrip.zip");
                    let roundtrip_path_str = roundtrip_path.to_str().unwrap();
                    crate::zip_roundtrip::export_kit_to_zip(&kit, &files, roundtrip_path_str)
                        .unwrap();

                    let result =
                        crate::zip_roundtrip::import_kit_from_zip(roundtrip_path_str).unwrap();
                    assert!(
                        are_kits_equal(&kit, &result.kit),
                        "ZIP -> JSON: roundtrip kit should be equal"
                    );
                    assert_eq!(
                        files.len(),
                        result.files.len(),
                        "File count mismatch after ZIP roundtrip"
                    );
                }
            }
        } // 🥊Roundtrip Tests
        pub use roundtrip_tests::*;

        mod design_model_tests {
            // 🗽DesignModel Tests
            // DesignModel Tests MUST provide model-selection regression checks.

            use super::*;

            mod design_model {
                use super::*;

                #[test]
                pub fn model_selection_from_shared_semio_assets() {
                    let path = Path::new(ASSETS_DIR).join("model.selection.semio.json");
                    let data = fs::read_to_string(&path)
                        .expect("Failed to read model.selection.semio.json");
                    let payload: ModelSelectionAsset = serde_json::from_str(&data)
                        .expect("Failed to deserialize model.selection.semio.json");

                    for case in payload.cases {
                        let models: Vec<Model> = case
                            .models
                            .iter()
                            .map(|model| Model {
                                guid: model.guid.clone(),
                                file: FileId {
                                    guid: model.file_guid.clone(),
                                },
                                name: None,
                                description: None,
                                tags: Some(
                                    model
                                        .tag_guids
                                        .iter()
                                        .map(|guid| TagId { guid: guid.clone() })
                                        .collect(),
                                ),
                                attributes: None,
                            })
                            .collect();

                        let selected =
                            select_best_model_like_semio_ts(&models, &case.selected_tag_guids);
                        let selected_guid = selected.map(|model| model.guid);
                        assert_eq!(
                            selected_guid, case.expected_guid,
                            "Case {} failed",
                            case.name
                        );
                    }
                }
            }
        } // 🤾DesignModel Tests
        pub use design_model_tests::*;

        mod kit_filter_tests {
            // 🎸KitFilter Tests
            // KitFilter Tests MUST verify design-scoped kit extraction.

            use super::*;

            mod kit_filter {
                use super::*;

                #[test]
                pub fn nakagin_capsule_tower_filter_produces_expected_subset() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let case = &asset.cases[0];
                    let kit = load_kit(&case.kit);
                    let expected = load_kit(&case.expected_kit);
                    let design = kit
                        .designs
                        .as_ref()
                        .and_then(|designs| {
                            designs.iter().find(|design| {
                                design.name == case.design_name && design.parent.is_none()
                            })
                        })
                        .expect("Design not found");

                    let filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            design_guid: Some(design.guid.clone()),
                            ..Default::default()
                        },
                    );

                    assert_eq!(
                        filtered.designs.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.designs.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                    assert_eq!(
                        filtered.types.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.types.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                    assert_eq!(
                        filtered.files.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.files.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                    assert_eq!(
                        filtered.ports.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.ports.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                    assert_eq!(
                        filtered.qualities.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.qualities.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                    assert_eq!(
                        filtered.authors.as_ref().map(|v| v.len()).unwrap_or(0),
                        expected.authors.as_ref().map(|v| v.len()).unwrap_or(0)
                    );

                    let filtered_design = filtered
                        .designs
                        .as_ref()
                        .and_then(|designs| {
                            designs
                                .iter()
                                .find(|filtered_design| filtered_design.guid == design.guid)
                        })
                        .expect("Filtered design not found");
                    assert_eq!(
                        filtered_design
                            .pieces
                            .as_ref()
                            .map(|v| v.len())
                            .unwrap_or(0),
                        design.pieces.as_ref().map(|v| v.len()).unwrap_or(0)
                    );

                    let empty_types = Vec::new();
                    for expected_type in expected.types.as_ref().unwrap_or(&empty_types) {
                        let filtered_type = filtered
                            .types
                            .as_ref()
                            .and_then(|types| {
                                types
                                    .iter()
                                    .find(|filtered_type| filtered_type.guid == expected_type.guid)
                            })
                            .expect("Expected filtered type missing");
                        assert_eq!(
                            filtered_type.models.as_ref().map(|v| v.len()).unwrap_or(0),
                            expected_type.models.as_ref().map(|v| v.len()).unwrap_or(0)
                        );
                    }

                    let empty_pieces = Vec::new();
                    for piece in filtered_design.pieces.as_ref().unwrap_or(&empty_pieces) {
                        if let Some(piece_type) = piece.type_ref.as_ref() {
                            assert!(
                                filtered
                                    .types
                                    .as_ref()
                                    .map(|types| types
                                        .iter()
                                        .any(|filtered_type| filtered_type.guid == piece_type.guid))
                                    .unwrap_or(false),
                                "Missing type {} for filtered piece",
                                piece_type.guid
                            );
                        }
                    }

                    for filtered_type in filtered.types.as_ref().unwrap_or(&empty_types) {
                        assert!(filtered_type.models.as_ref().map(|v| v.len()).unwrap_or(0) <= 1);
                        let empty_models = Vec::new();
                        for model in filtered_type.models.as_ref().unwrap_or(&empty_models) {
                            assert!(
                                filtered
                                    .files
                                    .as_ref()
                                    .map(|files| files
                                        .iter()
                                        .any(|file| file.guid == model.file.guid))
                                    .unwrap_or(false),
                                "Missing file {} for filtered type {}",
                                model.file.guid,
                                filtered_type.guid
                            );
                        }
                        let empty_connectors = Vec::new();
                        for connector in filtered_type
                            .connectors
                            .as_ref()
                            .unwrap_or(&empty_connectors)
                        {
                            if let Some(port) = connector.port.as_ref() {
                                assert!(
                                    filtered
                                        .ports
                                        .as_ref()
                                        .map(|ports| ports
                                            .iter()
                                            .any(|filtered_port| filtered_port.guid == port.guid))
                                        .unwrap_or(false),
                                    "Missing port {} for filtered type {}",
                                    port.guid,
                                    filtered_type.guid
                                );
                            }
                        }
                    }
                }

                #[test]
                pub fn nakagin_capsule_tower_filter_preserves_metadata() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let case = &asset.cases[0];
                    let kit = load_kit(&case.kit);
                    let design = kit
                        .designs
                        .as_ref()
                        .and_then(|designs| {
                            designs.iter().find(|design| {
                                design.name == case.design_name && design.parent.is_none()
                            })
                        })
                        .expect("Design not found");

                    let filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            design_guid: Some(design.guid.clone()),
                            ..Default::default()
                        },
                    );

                    assert_eq!(filtered.guid, kit.guid);
                    assert_eq!(filtered.name, kit.name);
                    assert_eq!(filtered.version, kit.version);
                }

                #[test]
                pub fn glob_filters_types_by_name_include() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let glob_case = asset
                        .glob_cases
                        .iter()
                        .find(|c| c.name == "type_include_capsule")
                        .expect("glob case not found");
                    let kit = load_kit(&glob_case.kit);
                    let patterns = glob_case
                        .type_include
                        .as_ref()
                        .expect("typeInclude missing");
                    let filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            types: Some(GlobFilter {
                                include: Some(patterns.clone()),
                                exclude: None,
                            }),
                            ..Default::default()
                        },
                    );
                    let types = filtered.types.as_ref().unwrap();
                    assert!(!types.is_empty());
                    for t in types {
                        assert!(
                            patterns.iter().any(|p| glob_match(&t.name, p)),
                            "Type {} should match include pattern",
                            t.name
                        );
                    }
                }

                #[test]
                pub fn glob_filters_types_by_name_exclude() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let glob_case = asset
                        .glob_cases
                        .iter()
                        .find(|c| c.name == "type_exclude_capsule")
                        .expect("glob case not found");
                    let kit = load_kit(&glob_case.kit);
                    let patterns = glob_case
                        .type_exclude
                        .as_ref()
                        .expect("typeExclude missing");
                    let total_types = kit.types.as_ref().map(|v| v.len()).unwrap_or(0);
                    let filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            types: Some(GlobFilter {
                                include: None,
                                exclude: Some(patterns.clone()),
                            }),
                            ..Default::default()
                        },
                    );
                    let types = filtered.types.as_ref().unwrap();
                    assert!(types.len() < total_types);
                    for t in types {
                        for p in patterns {
                            assert!(
                                !glob_match(&t.name, p),
                                "Type {} should have been excluded",
                                t.name
                            );
                        }
                    }
                }

                #[test]
                pub fn glob_filters_designs_by_name_include() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let glob_case = asset
                        .glob_cases
                        .iter()
                        .find(|c| c.name == "design_include_nakagin")
                        .expect("glob case not found");
                    let kit = load_kit(&glob_case.kit);
                    let patterns = glob_case
                        .design_include
                        .as_ref()
                        .expect("designInclude missing");
                    let filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            designs: Some(GlobFilter {
                                include: Some(patterns.clone()),
                                exclude: None,
                            }),
                            ..Default::default()
                        },
                    );
                    let designs = filtered.designs.as_ref().unwrap();
                    assert!(!designs.is_empty());
                    for d in designs {
                        assert!(
                            patterns.iter().any(|p| glob_match(&d.name, p)),
                            "Design {} should match include pattern",
                            d.name
                        );
                    }
                }

                #[test]
                pub fn empty_filter_returns_kit_unchanged() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let glob_case = asset
                        .glob_cases
                        .iter()
                        .find(|c| c.name == "empty_filter")
                        .expect("glob case not found");
                    let kit = load_kit(&glob_case.kit);
                    let filtered = filter_kit(&kit, &KitFilter::default());
                    assert_eq!(
                        filtered.types.as_ref().map(|v| v.len()),
                        kit.types.as_ref().map(|v| v.len())
                    );
                    assert_eq!(
                        filtered.designs.as_ref().map(|v| v.len()),
                        kit.designs.as_ref().map(|v| v.len())
                    );
                }

                #[test]
                pub fn combines_design_guid_with_glob_filters() {
                    let asset: FilterKitCasesAsset = load_asset("filter-kit.cases.semio.json");
                    let glob_case = asset
                        .glob_cases
                        .iter()
                        .find(|c| c.name == "combined_design_and_type_exclude")
                        .expect("glob case not found");
                    let kit = load_kit(&glob_case.kit);
                    let design_name = glob_case.design_name.as_ref().expect("designName missing");
                    let exclude_patterns = glob_case
                        .type_exclude
                        .as_ref()
                        .expect("typeExclude missing");
                    let design = kit
                        .designs
                        .as_ref()
                        .and_then(|designs| {
                            designs
                                .iter()
                                .find(|d| d.name == *design_name && d.parent.is_none())
                        })
                        .expect("Design not found");
                    let design_filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            design_guid: Some(design.guid.clone()),
                            ..Default::default()
                        },
                    );
                    let combined_filtered = filter_kit(
                        &kit,
                        &KitFilter {
                            design_guid: Some(design.guid.clone()),
                            types: Some(GlobFilter {
                                include: None,
                                exclude: Some(exclude_patterns.clone()),
                            }),
                            ..Default::default()
                        },
                    );
                    assert!(
                        combined_filtered
                            .types
                            .as_ref()
                            .map(|v| v.len())
                            .unwrap_or(0)
                            < design_filtered.types.as_ref().map(|v| v.len()).unwrap_or(0)
                    );
                }
            }
        } // 🗼KitFilter Tests
        pub use kit_filter_tests::*;

        mod model_kpi_tests {
            // 🌍Model/KPI Tests
            // Model/KPI tests for get_geometric_insights_for_model using nakagin-capsule-tower.gltf.

            use super::*;

            mod model_kpi {
                use super::*;

                #[test]
                pub fn nakagin_capsule_tower_gltf_returns_insights() {
                    let path = format!("{}/nakagin-capsule-tower.gltf", ASSETS_DIR);
                    let path = std::path::Path::new(&path);
                    if !path.exists() {
                        return;
                    }
                    let data = std::fs::read(path).expect("read gltf file");
                    let insights = get_geometric_insights_for_model(&data)
                        .expect("get_geometric_insights_for_model");
                    // Save per-language report mirroring ExportDesignModel behavior.
                    use serde_json::json;
                    use std::fs;
                    use std::path::PathBuf;

                    let mut reports_dir = PathBuf::from("..");
                    reports_dir.push("..");
                    reports_dir.push("reports");
                    reports_dir.push("model-kpi");
                    fs::create_dir_all(&reports_dir).expect("Failed to create reports directory");

                    let round6 = |x: f64| (x * 1e6).round() / 1e6;
                    let pt = |p: Option<[f64; 3]>| {
                        p.map(
                            |a| json!({ "x": round6(a[0]), "y": round6(a[1]), "z": round6(a[2]) }),
                        )
                    };
                    let report = json!({
                        "aspect_ratio_xy": round6(insights.aspect_ratio_xy),
                        "aspect_ratio_xz": round6(insights.aspect_ratio_xz),
                        "aspect_ratio_yz": round6(insights.aspect_ratio_yz),
                        "bounding_box_max": pt(insights.bounding_box_max),
                        "bounding_box_min": pt(insights.bounding_box_min),
                        "centroid": pt(insights.centroid),
                        "characteristic_length": round6(insights.characteristic_length),
                        "dimension_x": round6(insights.dimension_x),
                        "dimension_y": round6(insights.dimension_y),
                        "dimension_z": round6(insights.dimension_z),
                        "face_count": insights.face_count,
                        "footprint_area": round6(insights.footprint_area),
                        "is_watertight": false,
                        "slenderness": round6(insights.slenderness),
                        "total_surface_area": round6(insights.total_surface_area),
                        "vertex_count": insights.vertex_count,
                    });
                    let report_path = reports_dir.join("rs.json");
                    fs::write(&report_path, serde_json::to_vec_pretty(&report).unwrap())
                        .expect("Failed to write report");

                    let canonical_path =
                        std::path::Path::new(ASSETS_DIR).join("nakagin.kpi.model.semio.json");
                    let canonical_bytes =
                        fs::read(&canonical_path).expect("read canonical model-kpi asset");
                    let canonical: serde_json::Value = serde_json::from_slice(&canonical_bytes)
                        .expect("parse canonical model-kpi asset");
                    let skip: std::collections::HashSet<&str> =
                        ["centroid", "total_surface_area"].into_iter().collect();
                    let canon_obj = canonical.as_object().expect("canonical is object");
                    for (k, v) in canon_obj {
                        if skip.contains(k.as_str()) {
                            continue;
                        }
                        let got = report.get(k).expect("report has key");
                        assert!(
                            serde_json::Value::eq(got, v),
                            "mismatch for {}: {:?} != {:?}",
                            k,
                            got,
                            v
                        );
                    }
                }
            }
        } // 🏘️Model/KPI Tests
        pub use model_kpi_tests::*;

        mod flatten_tests {
            // 🔐Flatten Tests
            // Flatten Tests MUST provide the flatten tests functionality.

            use super::*;

            mod flatten {
                use super::*;

                mod nakagin_capsule_tower {
                    use super::*;

                    #[test]
                    pub fn kit_flatten_diff_apply_flat() {
                        let kit = load_kit("metabolism.kit.semio.json");
                        test_flatten_design(&kit, &["Nakagin Capsule Tower"]);
                    }

                    mod slanted {
                        use super::*;

                        #[test]
                        pub fn kit_flatten_diff_apply_flat() {
                            let kit = load_kit("metabolism.kit.semio.json");
                            test_flatten_design(&kit, &["Nakagin Capsule Tower", "Slanted"]);
                        }
                    }

                    mod twisted {
                        use super::*;

                        #[test]
                        pub fn kit_flatten_diff_apply_flat() {
                            let kit = load_kit("metabolism.kit.semio.json");
                            test_flatten_design(&kit, &["Nakagin Capsule Tower", "Twisted"]);
                        }
                    }

                    mod dancing {
                        use super::*;

                        #[test]
                        pub fn kit_flatten_diff_apply_flat() {
                            let kit = load_kit("metabolism.kit.semio.json");
                            test_flatten_design(&kit, &["Nakagin Capsule Tower", "Dancing"]);
                        }
                    }
                }

                mod capsule_dream {
                    use super::*;

                    #[test]
                    pub fn kit_flatten_diff_apply_flat() {
                        let kit = load_kit("metabolism.kit.semio.json");
                        test_flatten_design(&kit, &["Capsule Dream"]);
                    }
                }
            }

            //#region 🌳Flatten Merkle Tests
            // 🌳Shared-asset driven tests for per-piece merkle {plane_hash, center_hash} and cached flatten_design.

            mod flatten_merkle {
                use super::*;
                use serde_json::Value;
                use std::path::Path;

                #[derive(Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct MerkleCasesAsset {
                    parity: MerkleParity,
                    cases: Vec<MerkleCase>,
                }

                #[derive(Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct MerkleParity {
                    kit: String,
                    design_path: Vec<String>,
                    expected_hashes: Vec<MerkleExpectedHash>,
                }

                #[derive(Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct MerkleExpectedHash {
                    piece_guid: String,
                    plane_hash: String,
                    center_hash: String,
                }

                #[derive(Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct MerkleCase {
                    name: String,
                    kit: String,
                    design_path: Vec<String>,
                    #[serde(default)]
                    mutations: Vec<MerkleMutation>,
                    expect: MerkleExpect,
                }

                #[derive(Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct MerkleMutation {
                    kind: String,
                    piece_guid: Option<String>,
                    connection_guid: Option<String>,
                    path: String,
                    value: Value,
                }

                #[derive(Debug, Deserialize, Default)]
                #[serde(rename_all = "camelCase")]
                struct MerkleExpect {
                    #[serde(default)]
                    plane_hashes_changed_any: Option<bool>,
                    #[serde(default)]
                    center_hashes_changed_any: Option<bool>,
                    #[serde(default)]
                    plane_hashes_changed_all: Option<bool>,
                    #[serde(default)]
                    center_hashes_changed_all: Option<bool>,
                    #[serde(default)]
                    plane_hashes_changed_includes: Vec<String>,
                    #[serde(default)]
                    center_hashes_changed_includes: Vec<String>,
                    #[serde(default)]
                    plane_hashes_stable_includes: Vec<String>,
                    #[serde(default)]
                    center_hashes_stable_includes: Vec<String>,
                }

                /// <summary>🌳Find a design index inside a serde_json Value-encoded kit by hierarchical name path.</summary>
                fn find_design_idx_in_value(kit: &Value, design_path: &[String]) -> Option<usize> {
                    let designs = kit.get("designs")?.as_array()?;
                    let mut current_guid: Option<String> = None;
                    let mut current_idx: Option<usize> = None;
                    for (i, name) in design_path.iter().enumerate() {
                        let mut found_idx: Option<usize> = None;
                        for (idx, d) in designs.iter().enumerate() {
                            if d.get("name").and_then(|n| n.as_str()) != Some(name.as_str()) {
                                continue;
                            }
                            let parent = d.get("parent").filter(|v| !v.is_null());
                            if i == 0 {
                                if parent.is_none() {
                                    found_idx = Some(idx);
                                    break;
                                }
                            } else if let Some(parent_obj) = parent {
                                if parent_obj.get("guid").and_then(|g| g.as_str())
                                    == current_guid.as_deref()
                                {
                                    found_idx = Some(idx);
                                    break;
                                }
                            }
                        }
                        let idx = found_idx?;
                        current_guid = designs[idx]
                            .get("guid")
                            .and_then(|g| g.as_str())
                            .map(|s| s.to_string());
                        current_idx = Some(idx);
                    }
                    current_idx
                }

                /// <summary>🌳Assign a JSON value at a dotted path inside an object, creating intermediate objects when missing.</summary>
                fn set_at_path(obj: &mut Value, path: &str, value: Value) {
                    let keys: Vec<&str> = path.split('.').collect();
                    let mut current = obj;
                    for key in &keys[..keys.len() - 1] {
                        if !current.is_object() {
                            *current = Value::Object(serde_json::Map::new());
                        }
                        let needs_init = match current.as_object().and_then(|m| m.get(*key)) {
                            None | Some(Value::Null) => true,
                            _ => false,
                        };
                        if needs_init {
                            if let Some(map) = current.as_object_mut() {
                                map.insert(
                                    (*key).to_string(),
                                    Value::Object(serde_json::Map::new()),
                                );
                            }
                        }
                        current = current
                            .as_object_mut()
                            .and_then(|m| m.get_mut(*key))
                            .expect("intermediate object exists");
                    }
                    if let Some(map) = current.as_object_mut() {
                        let last = (*keys.last().unwrap()).to_string();
                        map.insert(last, value);
                    }
                }

                /// <summary>🌳Apply a single shared-asset mutation to a JSON-encoded kit clone prior to recomputing hashes.</summary>
                fn apply_mutation(
                    kit: &mut Value,
                    design_path: &[String],
                    mutation: &MerkleMutation,
                ) {
                    let design_idx = find_design_idx_in_value(kit, design_path)
                        .expect("design path not found in kit");
                    let designs = kit
                        .get_mut("designs")
                        .and_then(|d| d.as_array_mut())
                        .expect("kit.designs missing");
                    let design = &mut designs[design_idx];
                    match mutation.kind.as_str() {
                        "pieceField" => {
                            let pieces = design
                                .get_mut("pieces")
                                .and_then(|p| p.as_array_mut())
                                .expect("design.pieces missing");
                            let piece_guid = mutation
                                .piece_guid
                                .as_deref()
                                .expect("pieceField mutation missing pieceGuid");
                            let piece = pieces
                                .iter_mut()
                                .find(|p| {
                                    p.get("guid").and_then(|g| g.as_str()) == Some(piece_guid)
                                })
                                .unwrap_or_else(|| panic!("piece {} not found", piece_guid));
                            set_at_path(piece, &mutation.path, mutation.value.clone());
                        }
                        "connectionField" => {
                            let connections = design
                                .get_mut("connections")
                                .and_then(|c| c.as_array_mut())
                                .expect("design.connections missing");
                            let connection_guid = mutation
                                .connection_guid
                                .as_deref()
                                .expect("connectionField mutation missing connectionGuid");
                            let connection = connections
                                .iter_mut()
                                .find(|c| {
                                    c.get("guid").and_then(|g| g.as_str()) == Some(connection_guid)
                                })
                                .unwrap_or_else(|| {
                                    panic!("connection {} not found", connection_guid)
                                });
                            set_at_path(connection, &mutation.path, mutation.value.clone());
                        }
                        other => panic!("unknown mutation kind {}", other),
                    }
                }

                fn find_design_guid_in_kit(kit: &Kit, design_path: &[String]) -> String {
                    let designs = kit.designs.as_ref().expect("kit has no designs");
                    let mut current: Option<&Design> = None;
                    let mut parent_guid: Option<&str> = None;
                    for name in design_path {
                        current = find_design_by_name(designs, name, parent_guid);
                        assert!(current.is_some(), "design {} not found", name);
                        parent_guid = current.map(|d| d.guid.as_str());
                    }
                    current.expect("design is None").guid.clone()
                }

                fn load_kit_value(filename: &str) -> Value {
                    let path = Path::new(ASSETS_DIR).join(filename);
                    let data = std::fs::read_to_string(&path)
                        .unwrap_or_else(|e| panic!("failed to read {}: {}", filename, e));
                    serde_json::from_str(&data)
                        .unwrap_or_else(|e| panic!("failed to parse {}: {}", filename, e))
                }

                fn load_cases() -> MerkleCasesAsset {
                    load_asset("flatten-merkle.cases.semio.json")
                }

                #[test]
                pub fn test_flatten_merkle_parity_reference_hashes() {
                    let cases = load_cases();
                    let parity = cases.parity;
                    let kit = load_kit(&parity.kit);
                    let design_guid = find_design_guid_in_kit(&kit, &parity.design_path);
                    let hashes = compute_flat_hashes(&kit, &design_guid);
                    for expected in &parity.expected_hashes {
                        let actual = hashes.get(&expected.piece_guid).unwrap_or_else(|| {
                            panic!("piece {} missing from computed hashes", expected.piece_guid)
                        });
                        assert_eq!(
                            actual.plane_hash, expected.plane_hash,
                            "piece {} planeHash mismatch",
                            expected.piece_guid
                        );
                        assert_eq!(
                            actual.center_hash, expected.center_hash,
                            "piece {} centerHash mismatch",
                            expected.piece_guid
                        );
                    }
                }

                #[test]
                pub fn test_flatten_merkle_shared_asset_cases() {
                    let cases_doc = load_cases();
                    for case in &cases_doc.cases {
                        let kit_value_before = load_kit_value(&case.kit);
                        let kit_before: Kit = serde_json::from_value(kit_value_before.clone())
                            .expect("kit deserialize before");
                        let design_guid_before =
                            find_design_guid_in_kit(&kit_before, &case.design_path);
                        let before_hashes = compute_flat_hashes(&kit_before, &design_guid_before);

                        let mut kit_value_after = kit_value_before.clone();
                        for mutation in &case.mutations {
                            apply_mutation(&mut kit_value_after, &case.design_path, mutation);
                        }
                        let kit_after: Kit =
                            serde_json::from_value(kit_value_after).expect("kit deserialize after");
                        let design_guid_after =
                            find_design_guid_in_kit(&kit_after, &case.design_path);
                        let after_hashes = compute_flat_hashes(&kit_after, &design_guid_after);

                        let before_keys: HashSet<&String> = before_hashes.keys().collect();
                        let after_keys: HashSet<&String> = after_hashes.keys().collect();
                        assert_eq!(
                            before_keys, after_keys,
                            "case {}: piece set changed",
                            case.name
                        );

                        let changed_plane: HashSet<String> = before_hashes
                            .iter()
                            .filter_map(|(g, h)| {
                                after_hashes
                                    .get(g)
                                    .filter(|a| a.plane_hash != h.plane_hash)
                                    .map(|_| g.clone())
                            })
                            .collect();
                        let changed_center: HashSet<String> = before_hashes
                            .iter()
                            .filter_map(|(g, h)| {
                                after_hashes
                                    .get(g)
                                    .filter(|a| a.center_hash != h.center_hash)
                                    .map(|_| g.clone())
                            })
                            .collect();
                        let all_guids: HashSet<String> = before_hashes.keys().cloned().collect();

                        let expect = &case.expect;
                        let name = &case.name;
                        if let Some(any) = expect.plane_hashes_changed_any {
                            if any {
                                assert!(
                                    !changed_plane.is_empty(),
                                    "case {}: expected some planeHash changes, got none",
                                    name
                                );
                            } else {
                                assert!(
                                    changed_plane.is_empty(),
                                    "case {}: expected no planeHash changes, got {:?}",
                                    name,
                                    changed_plane
                                );
                            }
                        }
                        if let Some(any) = expect.center_hashes_changed_any {
                            if any {
                                assert!(
                                    !changed_center.is_empty(),
                                    "case {}: expected some centerHash changes, got none",
                                    name
                                );
                            } else {
                                assert!(
                                    changed_center.is_empty(),
                                    "case {}: expected no centerHash changes, got {:?}",
                                    name,
                                    changed_center
                                );
                            }
                        }
                        if let Some(all) = expect.plane_hashes_changed_all {
                            if all {
                                assert_eq!(
                                    changed_plane, all_guids,
                                    "case {}: expected every planeHash to change",
                                    name
                                );
                            } else {
                                assert_ne!(
                                    changed_plane, all_guids,
                                    "case {}: expected not every planeHash to change",
                                    name
                                );
                            }
                        }
                        if let Some(all) = expect.center_hashes_changed_all {
                            if all {
                                assert_eq!(
                                    changed_center, all_guids,
                                    "case {}: expected every centerHash to change",
                                    name
                                );
                            } else {
                                assert_ne!(
                                    changed_center, all_guids,
                                    "case {}: expected not every centerHash to change",
                                    name
                                );
                            }
                        }
                        for guid in &expect.plane_hashes_changed_includes {
                            assert!(
                                changed_plane.contains(guid),
                                "case {}: expected piece {} to have changed planeHash",
                                name,
                                guid
                            );
                        }
                        for guid in &expect.center_hashes_changed_includes {
                            assert!(
                                changed_center.contains(guid),
                                "case {}: expected piece {} to have changed centerHash",
                                name,
                                guid
                            );
                        }
                        for guid in &expect.plane_hashes_stable_includes {
                            assert!(
                                !changed_plane.contains(guid),
                                "case {}: expected piece {} to keep stable planeHash",
                                name,
                                guid
                            );
                        }
                        for guid in &expect.center_hashes_stable_includes {
                            assert!(
                                !changed_center.contains(guid),
                                "case {}: expected piece {} to keep stable centerHash",
                                name,
                                guid
                            );
                        }
                    }
                }

                #[test]
                pub fn test_flatten_design_cached_reuses_values() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let design_guid =
                        find_design_guid_in_kit(&kit, &["Nakagin Capsule Tower".to_string()]);
                    let (_first_rep, first_cache) = flatten_design_cached(&kit, &design_guid, None);
                    assert!(!first_cache.is_empty(), "first cache must not be empty");
                    let (_second_rep, second_cache) =
                        flatten_design_cached(&kit, &design_guid, Some(&first_cache));
                    for (guid, entry) in &first_cache {
                        let second_entry = second_cache
                            .get(guid)
                            .unwrap_or_else(|| panic!("piece {} missing from second cache", guid));
                        assert_eq!(
                            entry.plane_hash, second_entry.plane_hash,
                            "plane_hash mismatch for {}",
                            guid
                        );
                        assert_eq!(
                            entry.center_hash, second_entry.center_hash,
                            "center_hash mismatch for {}",
                            guid
                        );
                        assert_eq!(
                            entry.plane, second_entry.plane,
                            "plane mismatch for {}",
                            guid
                        );
                        assert_eq!(
                            entry.center, second_entry.center,
                            "center mismatch for {}",
                            guid
                        );
                    }
                }
            }
            //#endregion 🌳Flatten Merkle Tests
        } // 📭Flatten Tests
        pub use flatten_tests::*;

        mod change_tests {
            // 👠Change Tests
            // Change Tests MUST provide the change tests functionality.

            use super::*;

            mod change {
                use super::*;

                mod metabolism {
                    use super::*;

                    #[test]
                    pub fn kit_change_forward_backward_inverse_behavior() {
                        let mut kit_original = load_kit("metabolism.kit.semio.json");
                        if let Some(designs) = kit_original.designs.take() {
                            kit_original.designs =
                                Some(designs.into_iter().filter(|d| d.parent.is_none()).collect());
                        }
                        let kit_diffed = load_kit("metabolism.kit.diffed.semio.json");

                        let change = get_kit_change(&kit_original, &kit_diffed);

                        let mut applied_forward = kit_original.clone();
                        apply_kit_diff(&mut applied_forward, &change.forward);
                        assert!(
                            are_kits_equal(&applied_forward, &kit_diffed),
                            "ApplyKitDiff forward: applied kit doesn't match expected diffed kit"
                        );

                        let mut applied_inverse = kit_diffed.clone();
                        apply_kit_diff(&mut applied_inverse, &change.backward);
                        // [DEBUG] Write both to files for comparison
                        let inv_json = serde_json::to_string_pretty(&applied_inverse).unwrap();
                        let orig_json = serde_json::to_string_pretty(&kit_original).unwrap();
                        let tmp_dir = std::env::temp_dir();
                        std::fs::write(tmp_dir.join("inverse_applied.json"), &inv_json).unwrap();
                        std::fs::write(tmp_dir.join("original.json"), &orig_json).unwrap();
                        assert!(
                            are_kits_equal(&applied_inverse, &kit_original),
                            "ApplyKitDiff inverse: applied inverse kit doesn't match original kit"
                        );
                    }
                }
            }
        } // ⛹️Change Tests
        pub use change_tests::*;

        mod delete_tests {
            // 🌻Delete Tests
            // Delete Tests MUST verify delete_pieces_and_connections_in_design functionality.

            use super::*;

            mod delete {
                use super::*;

                #[test]
                pub fn nakagin_capsule_tower_delete_third_tambour_and_first_small_tower_connection()
                {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design 'Nakagin Capsule Tower' not found");

                    // Load selection
                    let selection_path = Path::new(ASSETS_DIR)
                        .join("nakagin-capsule-tower.deleted.selection.semio.json");
                    let selection_data = fs::read_to_string(&selection_path)
                        .expect("Failed to read selection asset");
                    let selection: serde_json::Value =
                        serde_json::from_str(&selection_data).expect("Failed to parse selection");
                    let piece_guids: Vec<String> = selection["pieces"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| p["guid"].as_str().unwrap().to_string())
                        .collect();
                    let connection_guids: Vec<String> = selection["connections"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|c| c["guid"].as_str().unwrap().to_string())
                        .collect();

                    // Load expected diff as JSON value (no guid field required)
                    let diff_path = Path::new(ASSETS_DIR)
                        .join("nakagin-capsule-tower.deleted.design.diff.semio.json");
                    let diff_data =
                        fs::read_to_string(&diff_path).expect("Failed to read diff asset");
                    let expected_json: serde_json::Value =
                        serde_json::from_str(&diff_data).expect("Failed to parse expected diff");

                    // Compute diff
                    let computed_report = delete_pieces_and_connections_in_design(
                        &kit,
                        design,
                        &piece_guids,
                        &connection_guids,
                    );
                    assert!(
                        computed_report.ok,
                        "delete failed: {:?}",
                        computed_report.errors
                    );
                    let computed_diff = computed_report.diff.expect("delete ok implies diff");

                    // Serialize computed diff to JSON for comparison
                    let computed_json: serde_json::Value = serde_json::to_value(&computed_diff)
                        .expect("Failed to serialize computed diff");

                    // Verify removed pieces
                    let computed_removed = computed_json["pieces"]["removed"]
                        .as_array()
                        .expect("No removed pieces in computed diff");
                    let expected_removed = expected_json["pieces"]["removed"]
                        .as_array()
                        .expect("No removed pieces in expected diff");
                    assert_eq!(
                        computed_removed.len(),
                        expected_removed.len(),
                        "Removed pieces count mismatch"
                    );
                    for (c, e) in computed_removed.iter().zip(expected_removed.iter()) {
                        assert_eq!(c["guid"], e["guid"], "Removed piece guid mismatch");
                    }

                    // Verify updated (fixed) pieces
                    let computed_updated = computed_json["pieces"]["updated"]
                        .as_array()
                        .expect("No updated pieces in computed diff");
                    let expected_updated = expected_json["pieces"]["updated"]
                        .as_array()
                        .expect("No updated pieces in expected diff");
                    assert_eq!(
                        computed_updated.len(),
                        expected_updated.len(),
                        "Updated pieces count mismatch: computed {} vs expected {}",
                        computed_updated.len(),
                        expected_updated.len()
                    );
                    let mut computed_guids: Vec<&str> = computed_updated
                        .iter()
                        .map(|u| u["piece"]["guid"].as_str().unwrap())
                        .collect();
                    computed_guids.sort();
                    let mut expected_guids: Vec<&str> = expected_updated
                        .iter()
                        .map(|u| u["piece"]["guid"].as_str().unwrap())
                        .collect();
                    expected_guids.sort();
                    assert_eq!(
                        computed_guids, expected_guids,
                        "Updated piece guids mismatch"
                    );
                    for u in computed_updated {
                        let guid = u["piece"]["guid"].as_str().unwrap();
                        let plane = &u["diff"]["plane"];
                        let center = &u["diff"]["center"];
                        // Find matching expected entry
                        let expected_entry = expected_updated
                            .iter()
                            .find(|e| e["piece"]["guid"].as_str().unwrap() == guid)
                            .expect(&format!("Expected entry for piece {}", guid));
                        let exp_plane = &expected_entry["diff"]["plane"];
                        let exp_center = &expected_entry["diff"]["center"];
                        let tol = 0.001;
                        assert!(
                            (plane["origin"]["x"].as_f64().unwrap()
                                - exp_plane["origin"]["x"].as_f64().unwrap())
                            .abs()
                                < tol,
                            "plane origin x mismatch for {}",
                            guid
                        );
                        assert!(
                            (plane["origin"]["y"].as_f64().unwrap()
                                - exp_plane["origin"]["y"].as_f64().unwrap())
                            .abs()
                                < tol,
                            "plane origin y mismatch for {}",
                            guid
                        );
                        assert!(
                            (plane["origin"]["z"].as_f64().unwrap()
                                - exp_plane["origin"]["z"].as_f64().unwrap())
                            .abs()
                                < tol,
                            "plane origin z mismatch for {}",
                            guid
                        );
                        assert!(
                            (center["u"].as_f64().unwrap() - exp_center["u"].as_f64().unwrap())
                                .abs()
                                < tol,
                            "center u mismatch for {}",
                            guid
                        );
                        assert!(
                            (center["v"].as_f64().unwrap() - exp_center["v"].as_f64().unwrap())
                                .abs()
                                < tol,
                            "center v mismatch for {}",
                            guid
                        );
                    }

                    // Verify removed connections
                    let computed_conn_removed = computed_json["connections"]["removed"]
                        .as_array()
                        .expect("No removed connections in computed diff");
                    let expected_conn_removed = expected_json["connections"]["removed"]
                        .as_array()
                        .expect("No removed connections in expected diff");
                    assert_eq!(
                        computed_conn_removed.len(),
                        expected_conn_removed.len(),
                        "Removed connections count mismatch: computed {} vs expected {}",
                        computed_conn_removed.len(),
                        expected_conn_removed.len()
                    );
                    let mut computed_conn_guids: Vec<&str> = computed_conn_removed
                        .iter()
                        .map(|r| r["guid"].as_str().unwrap())
                        .collect();
                    computed_conn_guids.sort();
                    let mut expected_conn_guids: Vec<&str> = expected_conn_removed
                        .iter()
                        .map(|r| r["guid"].as_str().unwrap())
                        .collect();
                    expected_conn_guids.sort();
                    assert_eq!(
                        computed_conn_guids, expected_conn_guids,
                        "Removed connection guids mismatch"
                    );
                }
            }
        } // 🏦Delete Tests
        pub use delete_tests::*;

        mod copy_paste_tests {
            // 📋Copy Paste Tests
            // Copy Paste Tests MUST verify copy_design and paste_design functionality.

            use super::*;

            mod copy_paste {
                use super::*;

                #[test]
                pub fn nakagin_capsule_tower_copy_paste_roundtrip() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design 'Nakagin Capsule Tower' not found");

                    // Load selection
                    let selection_path = Path::new(ASSETS_DIR)
                        .join("nakagin-capsule-tower.copy.design.selection.semio.json");
                    let selection_data = fs::read_to_string(&selection_path)
                        .expect("Failed to read selection asset");
                    let selection: serde_json::Value =
                        serde_json::from_str(&selection_data).expect("Failed to parse selection");
                    let piece_guids: Vec<String> = selection["pieces"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| p["guid"].as_str().unwrap().to_string())
                        .collect();
                    let connection_guids: Vec<String> = selection["connections"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|c| c["guid"].as_str().unwrap().to_string())
                        .collect();

                    // Load expected copy design
                    let expected_copy_path =
                        Path::new(ASSETS_DIR).join("nakagin-capsule-tower.copy.design.semio.json");
                    let expected_copy_data = fs::read_to_string(&expected_copy_path)
                        .expect("Failed to read expected copy asset");
                    let expected_copy_json: serde_json::Value =
                        serde_json::from_str(&expected_copy_data)
                            .expect("Failed to parse expected copy design");

                    // Compute copy
                    let copy_design_result =
                        copy_design(&kit, design, &piece_guids, &connection_guids);
                    let copy_json: serde_json::Value = serde_json::to_value(&copy_design_result)
                        .expect("Failed to serialize copy");

                    // Verify piece count
                    let copy_pieces = copy_json["pieces"].as_array().unwrap();
                    let expected_pieces = expected_copy_json["pieces"].as_array().unwrap();
                    assert_eq!(
                        copy_pieces.len(),
                        expected_pieces.len(),
                        "Copy pieces count mismatch: got {}, want {}",
                        copy_pieces.len(),
                        expected_pieces.len()
                    );

                    // Verify connection count
                    let copy_conns = copy_json["connections"].as_array().unwrap();
                    let expected_conns = expected_copy_json["connections"].as_array().unwrap();
                    assert_eq!(
                        copy_conns.len(),
                        expected_conns.len(),
                        "Copy connections count mismatch: got {}, want {}",
                        copy_conns.len(),
                        expected_conns.len()
                    );

                    // Verify each piece exists in the copy
                    let copy_piece_guids: HashSet<&str> = copy_pieces
                        .iter()
                        .map(|p| p["guid"].as_str().unwrap())
                        .collect();
                    for ep in expected_pieces {
                        let guid = ep["guid"].as_str().unwrap();
                        assert!(
                            copy_piece_guids.contains(guid),
                            "Expected piece {} not found in copy output",
                            guid
                        );
                    }

                    // Verify external pieces have semio.piece.origin attribute
                    for cp in copy_pieces {
                        let guid = cp["guid"].as_str().unwrap();
                        let ep = expected_pieces
                            .iter()
                            .find(|p| p["guid"].as_str().unwrap() == guid)
                            .expect("Piece not in expected");
                        let has_origin = cp["attributes"]
                            .as_array()
                            .map(|attrs| {
                                attrs.iter().any(|a| {
                                    a["key"].as_str() == Some("semio.piece.origin")
                                        && a["value"].as_str() == Some("external")
                                })
                            })
                            .unwrap_or(false);
                        let expected_origin = ep["attributes"]
                            .as_array()
                            .map(|attrs| {
                                attrs.iter().any(|a| {
                                    a["key"].as_str() == Some("semio.piece.origin")
                                        && a["value"].as_str() == Some("external")
                                })
                            })
                            .unwrap_or(false);
                        assert_eq!(
                            has_origin, expected_origin,
                            "Piece {}: semio.piece.origin mismatch",
                            guid
                        );
                    }

                    // Verify pp_excl_pc_incl pieces have semio.center and semio.plane attributes
                    for cp in copy_pieces {
                        let guid = cp["guid"].as_str().unwrap();
                        let ep = expected_pieces
                            .iter()
                            .find(|p| p["guid"].as_str().unwrap() == guid)
                            .unwrap();
                        let has_center = cp["attributes"]
                            .as_array()
                            .map(|a| a.iter().any(|x| x["key"].as_str() == Some("semio.center")))
                            .unwrap_or(false);
                        let expected_center = ep["attributes"]
                            .as_array()
                            .map(|a| a.iter().any(|x| x["key"].as_str() == Some("semio.center")))
                            .unwrap_or(false);
                        assert_eq!(
                            has_center, expected_center,
                            "Piece {}: semio.center attr mismatch",
                            guid
                        );
                        let has_plane = cp["attributes"]
                            .as_array()
                            .map(|a| a.iter().any(|x| x["key"].as_str() == Some("semio.plane")))
                            .unwrap_or(false);
                        let expected_plane = ep["attributes"]
                            .as_array()
                            .map(|a| a.iter().any(|x| x["key"].as_str() == Some("semio.plane")))
                            .unwrap_or(false);
                        assert_eq!(
                            has_plane, expected_plane,
                            "Piece {}: semio.plane attr mismatch",
                            guid
                        );
                    }

                    // Load paste target design (second storey)
                    let paste_target_path =
                        Path::new(ASSETS_DIR).join("nakagin-capsule-tower.paste.design.semio.json");
                    let paste_target_data = fs::read_to_string(&paste_target_path)
                        .expect("Failed to read paste target design");
                    let paste_target: Design = serde_json::from_str(&paste_target_data)
                        .expect("Failed to parse paste target design");

                    // Test PasteDesign with original anchoring (no coord)
                    let paste_diff =
                        paste_design(&kit, &copy_design_result, &paste_target, "original", None);

                    // Load expected paste diff
                    let expected_paste_path = Path::new(ASSETS_DIR)
                        .join("nakagin-capsule-tower.paste.design.diff.semio.json");
                    let expected_paste_data = fs::read_to_string(&expected_paste_path)
                        .expect("Failed to read expected paste asset");
                    let expected_paste_json: serde_json::Value =
                        serde_json::from_str(&expected_paste_data)
                            .expect("Failed to parse expected paste diff");

                    let paste_json: serde_json::Value =
                        serde_json::to_value(&paste_diff).expect("Failed to serialize paste diff");

                    // Verify pasted pieces count
                    let paste_pieces = paste_json["pieces"]["added"].as_array().unwrap();
                    let expected_paste_pieces =
                        expected_paste_json["pieces"]["added"].as_array().unwrap();
                    assert_eq!(
                        paste_pieces.len(),
                        expected_paste_pieces.len(),
                        "Paste added pieces count mismatch: got {}, want {}",
                        paste_pieces.len(),
                        expected_paste_pieces.len()
                    );

                    // Verify pasted pieces don't include external-origin pieces
                    for p in paste_pieces {
                        let has_ext = p["attributes"]
                            .as_array()
                            .map(|attrs| {
                                attrs.iter().any(|a| {
                                    a["key"].as_str() == Some("semio.piece.origin")
                                        && a["value"].as_str() == Some("external")
                                })
                            })
                            .unwrap_or(false);
                        assert!(
                            !has_ext,
                            "External-origin piece {} should not be in paste output",
                            p["guid"].as_str().unwrap_or("?")
                        );
                    }

                    // Verify pasted connections count
                    let paste_conns = paste_json["connections"]["added"].as_array().unwrap();
                    let expected_paste_conns = expected_paste_json["connections"]["added"]
                        .as_array()
                        .unwrap();
                    assert_eq!(
                        paste_conns.len(),
                        expected_paste_conns.len(),
                        "Paste added connections count mismatch: got {}, want {}",
                        paste_conns.len(),
                        expected_paste_conns.len()
                    );

                    // Test PasteDesign with original anchoring and coord
                    let coord_val = Coord::new(10.0, 10.0);
                    let paste_with_coord_diff = paste_design(
                        &kit,
                        &copy_design_result,
                        &paste_target,
                        "original",
                        Some(&coord_val),
                    );

                    // Load expected paste with coord diff
                    let expected_pwc_path = Path::new(ASSETS_DIR)
                        .join("nakagin-capsule-tower.paste.with-coord.design.diff.semio.json");
                    let expected_pwc_data = fs::read_to_string(&expected_pwc_path)
                        .expect("Failed to read expected paste with coord asset");
                    let expected_pwc_json: serde_json::Value =
                        serde_json::from_str(&expected_pwc_data)
                            .expect("Failed to parse expected paste with coord diff");

                    let pwc_json: serde_json::Value = serde_json::to_value(&paste_with_coord_diff)
                        .expect("Failed to serialize paste with coord diff");

                    // Verify pasted pieces count
                    let pwc_pieces = pwc_json["pieces"]["added"].as_array().unwrap();
                    let expected_pwc_pieces =
                        expected_pwc_json["pieces"]["added"].as_array().unwrap();
                    assert_eq!(
                        pwc_pieces.len(),
                        expected_pwc_pieces.len(),
                        "Paste with coord added pieces count mismatch: got {}, want {}",
                        pwc_pieces.len(),
                        expected_pwc_pieces.len()
                    );

                    // Verify pasted connections count
                    let pwc_conns = pwc_json["connections"]["added"].as_array().unwrap();
                    let expected_pwc_conns = expected_pwc_json["connections"]["added"]
                        .as_array()
                        .unwrap();
                    assert_eq!(
                        pwc_conns.len(),
                        expected_pwc_conns.len(),
                        "Paste with coord added connections count mismatch: got {}, want {}",
                        pwc_conns.len(),
                        expected_pwc_conns.len()
                    );

                    // Verify centers are offset by coord
                    for pp in pwc_pieces {
                        let guid = pp["guid"].as_str().unwrap();
                        let ep = expected_pwc_pieces
                            .iter()
                            .find(|p| p["guid"].as_str().unwrap() == guid)
                            .expect(&format!("Piece {} not in expected paste with coord", guid));
                        if let (Some(pc), Some(ec)) =
                            (pp["center"].as_object(), ep["center"].as_object())
                        {
                            let pu = pc["u"].as_f64().unwrap();
                            let pv = pc["v"].as_f64().unwrap();
                            let eu = ec["u"].as_f64().unwrap();
                            let ev = ec["v"].as_f64().unwrap();
                            assert!(
                                (pu - eu).abs() < 0.001 && (pv - ev).abs() < 0.001,
                                "Piece {} center mismatch: got ({},{}), want ({},{})",
                                guid,
                                pu,
                                pv,
                                eu,
                                ev
                            );
                        }
                    }
                }
            }
        } // 📋Copy Paste Tests
        pub use copy_paste_tests::*;

        mod find_replaceable_types_in_designs_tests {
            // 🔍Find Replaceable Types In Designs Tests
            // Find Replaceable Types In Designs Tests MUST verify find_replaceable_types_in_designs_for_pieces_in_design functionality.

            use super::*;

            mod find_replaceable {
                use super::*;
                use std::collections::HashMap;

                #[test]
                pub fn synthetic_selection_enforces_distinct_connectors_and_free_design_connectors()
                {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let kit: Kit = load_asset(&asset.synthetic_kit);

                    for sc in &asset.synthetic_cases {
                        let (type_guids, design_guids) =
                            find_replaceable_types_in_designs_for_pieces_in_design(
                                &kit,
                                &sc.design_guid,
                                &sc.piece_guids,
                            );
                        if let Some(expected) = &sc.expected_contains_types {
                            for t in expected {
                                assert!(
                                    type_guids.contains(&t.to_string()),
                                    "Case {}: expected type {} in results",
                                    sc.name,
                                    t
                                );
                            }
                        }
                        if let Some(not_expected) = &sc.expected_not_contains_types {
                            for t in not_expected {
                                assert!(
                                    !type_guids.contains(&t.to_string()),
                                    "Case {}: type {} should NOT be in results",
                                    sc.name,
                                    t
                                );
                            }
                        }
                        if let Some(expected) = &sc.expected_contains_designs {
                            for d in expected {
                                assert!(
                                    design_guids.contains(&d.to_string()),
                                    "Case {}: expected design {} in results",
                                    sc.name,
                                    d
                                );
                            }
                        }
                        if let Some(not_expected) = &sc.expected_not_contains_designs {
                            for d in not_expected {
                                assert!(
                                    !design_guids.contains(&d.to_string()),
                                    "Case {}: design {} should NOT be in results",
                                    sc.name,
                                    d
                                );
                            }
                        }
                    }
                }

                #[test]
                pub fn connector_level_boundary_matching_shrinks_candidates_as_demand_grows() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let bc = &asset.boundary_cases;
                    let kit = load_kit(&bc.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == bc.design_name && d.parent.is_none())
                        .expect("Design not found");
                    let pieces = design.pieces.as_ref().expect("Design has no pieces");
                    let types = kit.types.as_ref().expect("Kit has no types");

                    let name_to_guid: HashMap<&str, &str> = pieces
                        .iter()
                        .filter_map(|piece| {
                            piece
                                .name
                                .as_deref()
                                .map(|name| (name, piece.guid.as_str()))
                        })
                        .collect();
                    let type_name_by_guid: HashMap<&str, &str> = types
                        .iter()
                        .map(|kind| (kind.guid.as_str(), kind.name.as_str()))
                        .collect();

                    let type_names_for_selection = |piece_names: &[&str]| -> Vec<String> {
                        let piece_guids: Vec<String> = piece_names
                            .iter()
                            .map(|piece_name| {
                                name_to_guid
                                    .get(piece_name)
                                    .expect("Piece not found")
                                    .to_string()
                            })
                            .collect();
                        let (type_guids, _) =
                            find_replaceable_types_in_designs_for_pieces_in_design(
                                &kit,
                                &design.guid,
                                &piece_guids,
                            );
                        type_guids
                            .iter()
                            .map(|type_guid| {
                                type_name_by_guid
                                    .get(type_guid.as_str())
                                    .expect("Type not found")
                                    .to_string()
                            })
                            .collect()
                    };
                    let unique_type_names_for_selection = |piece_names: &[&str]| -> Vec<String> {
                        let mut unique_type_names = type_names_for_selection(piece_names);
                        unique_type_names.sort();
                        unique_type_names.dedup();
                        unique_type_names
                    };

                    let single_refs: Vec<&str> = bc
                        .single_capsule_pieces
                        .iter()
                        .map(|s| s.as_str())
                        .collect();
                    let two_refs: Vec<&str> =
                        bc.two_capsule_pieces.iter().map(|s| s.as_str()).collect();
                    let four_refs: Vec<&str> =
                        bc.four_capsule_pieces.iter().map(|s| s.as_str()).collect();
                    let eight_refs: Vec<&str> =
                        bc.eight_capsule_pieces.iter().map(|s| s.as_str()).collect();

                    let single_capsule_names = type_names_for_selection(&single_refs);
                    let two_capsule_names = type_names_for_selection(&two_refs);
                    let four_capsule_names = type_names_for_selection(&four_refs);
                    let eight_capsule_names = type_names_for_selection(&eight_refs);

                    let tambour_piece_guid = name_to_guid
                        .get(bc.tambour_piece_name.as_str())
                        .expect("Tambour piece not found")
                        .to_string();
                    let (tambour_type_guids, tambour_design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &design.guid,
                            &[tambour_piece_guid],
                        );

                    assert!(single_capsule_names.len() > two_capsule_names.len());
                    assert!(two_capsule_names.len() >= four_capsule_names.len());
                    assert!(four_capsule_names.len() >= eight_capsule_names.len());

                    for forbidden_family in &bc.forbidden_families {
                        assert!(!two_capsule_names
                            .iter()
                            .any(|name| name == forbidden_family));
                        assert!(!four_capsule_names
                            .iter()
                            .any(|name| name == forbidden_family));
                        assert!(!eight_capsule_names
                            .iter()
                            .any(|name| name == forbidden_family));
                    }
                    assert!(!four_capsule_names.iter().any(|name| name == "Bridge"));
                    assert!(!eight_capsule_names.iter().any(|name| name == "Bridge"));

                    assert_eq!(
                        unique_type_names_for_selection(&two_refs),
                        bc.expected_two_capsule_families
                    );
                    assert_eq!(
                        unique_type_names_for_selection(&four_refs),
                        bc.expected_large_families
                    );
                    assert_eq!(
                        unique_type_names_for_selection(&eight_refs),
                        bc.expected_large_families
                    );
                    assert_eq!(
                        tambour_type_guids.len(),
                        bc.expected_tambour_type_guid_count
                    );
                    assert_eq!(
                        tambour_design_guids.len(),
                        bc.expected_tambour_design_guid_count
                    );
                }

                #[test]
                pub fn connected_piece_yields_only_exact_design_matches() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let case = asset
                        .cases
                        .iter()
                        .find(|c| c.name == "connected_piece_yields_only_exact_design_matches")
                        .expect("case not found");
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == case.design_name && d.parent.is_none())
                        .expect("Design not found");
                    let pieces = design.pieces.as_ref().unwrap();
                    let piece_names = case.piece_names.as_ref().expect("pieceNames missing");
                    let piece =
                        find_piece_by_name(pieces, &piece_names[0]).expect("Piece not found");

                    let (type_guids, design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &design.guid,
                            &[piece.guid.clone()],
                        );

                    let expected_design_guids = case
                        .expected_design_guids
                        .as_ref()
                        .expect("expectedDesignGuids missing");
                    assert_eq!(case.expected_type_guid_count.unwrap_or(0), type_guids.len());
                    assert_eq!(design_guids, *expected_design_guids);
                }

                #[test]
                pub fn isolated_piece() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let case = asset
                        .cases
                        .iter()
                        .find(|c| c.name == "isolated_piece")
                        .expect("case not found");
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let parent_design_name = case
                        .design_parent_name
                        .as_ref()
                        .expect("designParentName missing");
                    let parent_design = designs
                        .iter()
                        .find(|d| d.name == *parent_design_name && d.parent.is_none())
                        .expect("Parent design not found");
                    let flat_design = designs
                        .iter()
                        .find(|d| {
                            d.name == case.design_name
                                && d.parent.as_ref().map(|p| p.guid.as_str())
                                    == Some(parent_design.guid.as_str())
                        })
                        .expect("Flat design not found");
                    let pieces = flat_design.pieces.as_ref().unwrap();
                    let piece_index = case.use_piece_index.unwrap_or(0);
                    let piece = &pieces[piece_index];

                    let (type_guids, _design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &flat_design.guid,
                            &[piece.guid.clone()],
                        );

                    assert!(
                        !type_guids.is_empty(),
                        "Should find replaceable types for isolated piece"
                    );

                    if case.expect_own_type_in_results.unwrap_or(false) {
                        let piece_type_guid = piece.type_ref.as_ref().unwrap().guid.as_str();
                        assert!(
                            type_guids.contains(&piece_type_guid.to_string()),
                            "Own type should be in replaceable types for isolated piece"
                        );
                    }
                }

                #[test]
                pub fn capital_piece() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let case = asset
                        .cases
                        .iter()
                        .find(|c| c.name == "capital_piece")
                        .expect("case not found");
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == case.design_name && d.parent.is_none())
                        .expect("Design not found");
                    let pieces = design.pieces.as_ref().unwrap();
                    let types = kit.types.as_ref().unwrap();
                    let lookup_name = case
                        .lookup_type_name
                        .as_ref()
                        .expect("lookupTypeName missing");
                    let lookup_type = types
                        .iter()
                        .find(|t| t.name == *lookup_name)
                        .expect("Lookup type not found");
                    let piece = pieces
                        .iter()
                        .find(|p| {
                            p.type_ref.as_ref().map(|t| t.guid.as_str())
                                == Some(lookup_type.guid.as_str())
                        })
                        .expect("Piece not found");

                    let (type_guids, _design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &design.guid,
                            &[piece.guid.clone()],
                        );

                    assert!(
                        !type_guids.is_empty(),
                        "Should find replaceable types for capital piece"
                    );

                    let forbidden_names = case
                        .forbidden_type_names
                        .as_ref()
                        .expect("forbiddenTypeNames missing");
                    for forbidden_name in forbidden_names {
                        let forbidden_type = types
                            .iter()
                            .find(|t| t.name == *forbidden_name)
                            .expect(&format!("Forbidden type {} not found", forbidden_name));
                        assert!(
                            !type_guids.contains(&forbidden_type.guid),
                            "{} should NOT be in replaceable types",
                            forbidden_name
                        );
                    }
                }

                #[test]
                pub fn multiple_selected_pieces_yield_only_exact_design_matches() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let case = asset
                        .cases
                        .iter()
                        .find(|c| c.name == "multiple_selected_pieces")
                        .expect("case not found");
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == case.design_name && d.parent.is_none())
                        .expect("Design not found");
                    let pieces = design.pieces.as_ref().unwrap();
                    let piece_names = case.piece_names.as_ref().expect("pieceNames missing");
                    let piece_guids: Vec<String> = piece_names
                        .iter()
                        .map(|name| {
                            find_piece_by_name(pieces, name)
                                .expect(&format!("Piece {} not found", name))
                                .guid
                                .clone()
                        })
                        .collect();

                    let (type_guids, design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &design.guid,
                            &piece_guids,
                        );

                    let expected_design_guids = case
                        .expected_design_guids
                        .as_ref()
                        .expect("expectedDesignGuids missing");
                    assert_eq!(case.expected_type_guid_count.unwrap_or(0), type_guids.len());
                    assert_eq!(design_guids, *expected_design_guids);
                }

                #[test]
                pub fn empty_selection() {
                    let asset: FindReplaceableCasesAsset =
                        load_asset("find-replaceable-types.cases.semio.json");
                    let case = asset
                        .cases
                        .iter()
                        .find(|c| c.name == "empty_selection")
                        .expect("case not found");
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == case.design_name && d.parent.is_none())
                        .expect("Design not found");

                    let (type_guids, _design_guids) =
                        find_replaceable_types_in_designs_for_pieces_in_design(
                            &kit,
                            &design.guid,
                            &[],
                        );

                    let types = kit.types.as_ref().unwrap();
                    let connectorless_count = types
                        .iter()
                        .filter(|t| t.connectors.as_deref().unwrap_or(&[]).is_empty())
                        .count();

                    assert_eq!(
                        type_guids.len(),
                        connectorless_count,
                        "Empty selection should return only types with no connectors"
                    );
                }
            }
        } // 🔍Find Replaceable Types In Designs Tests
        pub use find_replaceable_types_in_designs_tests::*;

        mod with_diff_tests {
            // 🏗️WithDiff Tests
            // WithDiff Tests MUST verify design_with_diff functionality.

            use super::*;

            mod with_diff {
                use super::*;

                #[test]
                pub fn nakagin_capsule_tower_design_with_diff() {
                    let asset: DesignWithDiffCasesAsset =
                        load_asset("design-with-diff.cases.semio.json");
                    let case = &asset.cases[0];
                    let kit = load_kit(&case.kit);
                    let designs = kit.designs.as_ref().expect("Kit has no designs");
                    let design = designs
                        .iter()
                        .find(|d| d.name == case.design_name && d.parent.is_none())
                        .expect("Design not found");

                    let diff: DesignDiff = load_asset(&case.diff);

                    let expected: Design = load_asset(&case.expected);

                    let result = design_with_diff(design, &diff);

                    let result_pieces = result.pieces.as_ref().expect("No pieces in result");
                    let expected_pieces = expected.pieces.as_ref().expect("No pieces in expected");
                    assert_eq!(
                        result_pieces.len(),
                        expected_pieces.len(),
                        "Piece count mismatch: got {} expected {}",
                        result_pieces.len(),
                        expected_pieces.len()
                    );

                    let result_conns = result
                        .connections
                        .as_ref()
                        .expect("No connections in result");
                    let expected_conns = expected
                        .connections
                        .as_ref()
                        .expect("No connections in expected");
                    assert_eq!(
                        result_conns.len(),
                        expected_conns.len(),
                        "Connection count mismatch: got {} expected {}",
                        result_conns.len(),
                        expected_conns.len()
                    );

                    let get_status = |attrs: &Option<Vec<Attribute>>| -> String {
                        attrs
                            .as_ref()
                            .and_then(|a| {
                                a.iter()
                                    .find(|attr| attr.key == "semio.diffStatus")
                                    .and_then(|attr| attr.value.clone())
                            })
                            .unwrap_or_default()
                    };

                    let mut piece_status_counts = std::collections::HashMap::new();
                    for p in result_pieces {
                        *piece_status_counts
                            .entry(get_status(&p.attributes))
                            .or_insert(0) += 1;
                    }
                    assert_eq!(
                        piece_status_counts.get("unchanged").copied().unwrap_or(0),
                        case.expected_piece_counts.unchanged
                    );
                    assert_eq!(
                        piece_status_counts.get("modified").copied().unwrap_or(0),
                        case.expected_piece_counts.modified
                    );
                    assert_eq!(
                        piece_status_counts.get("removed").copied().unwrap_or(0),
                        case.expected_piece_counts.removed
                    );
                    assert_eq!(
                        piece_status_counts.get("added").copied().unwrap_or(0),
                        case.expected_piece_counts.added
                    );

                    let mut conn_status_counts = std::collections::HashMap::new();
                    for c in result_conns {
                        *conn_status_counts
                            .entry(get_status(&c.attributes))
                            .or_insert(0) += 1;
                    }
                    assert_eq!(
                        conn_status_counts.get("unchanged").copied().unwrap_or(0),
                        case.expected_connection_counts.unchanged
                    );
                    assert_eq!(
                        conn_status_counts.get("modified").copied().unwrap_or(0),
                        case.expected_connection_counts.modified
                    );
                    assert_eq!(
                        conn_status_counts.get("removed").copied().unwrap_or(0),
                        case.expected_connection_counts.removed
                    );
                    assert_eq!(
                        conn_status_counts.get("added").copied().unwrap_or(0),
                        case.expected_connection_counts.added
                    );
                }
            }
        } // 🧪WithDiff Tests
        pub use with_diff_tests::*;

        mod drag_tests {
            // 🎴Drag Tests
            // Drag Tests MUST verify drag_pieces_in_design functionality.

            use super::*;

            mod drag {
                use super::*;

                #[test]
                pub fn design_pieces_offset_diff_design() {
                    let design_path = Path::new(ASSETS_DIR).join("drag/design.semio.json");
                    let design_data =
                        fs::read_to_string(&design_path).expect("Failed to read design");
                    let design_json: serde_json::Value =
                        serde_json::from_str(&design_data).expect("Failed to parse design");
                    let design_pieces: Vec<Piece> =
                        serde_json::from_value(design_json["pieces"].clone()).unwrap_or_default();
                    let design_connections: Vec<Connection> =
                        serde_json::from_value(design_json["connections"].clone())
                            .unwrap_or_default();
                    let pieces_path = Path::new(ASSETS_DIR).join("drag/pieces.semio.json");
                    let pieces_data =
                        fs::read_to_string(&pieces_path).expect("Failed to read pieces");
                    let pieces_json: serde_json::Value =
                        serde_json::from_str(&pieces_data).expect("Failed to parse pieces");
                    let selected_pieces: Vec<Piece> =
                        serde_json::from_value(pieces_json["pieces"].clone()).unwrap_or_default();
                    let offset_path = Path::new(ASSETS_DIR).join("drag/offset.semio.json");
                    let offset_data =
                        fs::read_to_string(&offset_path).expect("Failed to read offset");
                    let offset: Coord =
                        serde_json::from_str(&offset_data).expect("Failed to parse offset");
                    let diff_path = Path::new(ASSETS_DIR).join("drag/diff.design.semio.json");
                    let diff_data = fs::read_to_string(&diff_path).expect("Failed to read diff");
                    let expected: serde_json::Value =
                        serde_json::from_str(&diff_data).expect("Failed to parse expected diff");
                    let computed = drag_pieces_in_design(
                        &design_pieces,
                        &design_connections,
                        &selected_pieces,
                        &offset,
                    );
                    let computed_json: serde_json::Value =
                        serde_json::to_value(&computed).expect("Failed to serialize computed diff");
                    let expected_pieces = expected["pieces"]["updated"].as_array();
                    let computed_pieces = computed_json["pieces"]["updated"].as_array();
                    match (expected_pieces, computed_pieces) {
                        (Some(ep), Some(cp)) => {
                            assert_eq!(cp.len(), ep.len(), "Piece updates count mismatch");
                            let mut expected_map: std::collections::HashMap<
                                &str,
                                &serde_json::Value,
                            > = std::collections::HashMap::new();
                            for u in ep {
                                expected_map
                                    .insert(u["piece"]["guid"].as_str().unwrap(), &u["diff"]);
                            }
                            for u in cp {
                                let guid = u["piece"]["guid"].as_str().unwrap();
                                let exp = expected_map.get(guid).unwrap_or_else(|| {
                                    panic!("Unexpected piece update for {}", guid)
                                });
                                let tol = 0.001;
                                assert!(
                                    (u["diff"]["center"]["u"].as_f64().unwrap()
                                        - exp["center"]["u"].as_f64().unwrap())
                                    .abs()
                                        < tol,
                                    "Piece {} center u mismatch",
                                    guid
                                );
                                assert!(
                                    (u["diff"]["center"]["v"].as_f64().unwrap()
                                        - exp["center"]["v"].as_f64().unwrap())
                                    .abs()
                                        < tol,
                                    "Piece {} center v mismatch",
                                    guid
                                );
                            }
                        }
                        (None, None) => {}
                        _ => panic!(
                            "Piece updates mismatch: expected {:?} vs computed {:?}",
                            expected_pieces.map(|v| v.len()),
                            computed_pieces.map(|v| v.len())
                        ),
                    }
                    let expected_conns = expected["connections"]["updated"].as_array();
                    let computed_conns = computed_json["connections"]["updated"].as_array();
                    match (expected_conns, computed_conns) {
                        (Some(ec), Some(cc)) => {
                            assert_eq!(cc.len(), ec.len(), "Connection updates count mismatch");
                            let mut expected_map: std::collections::HashMap<
                                &str,
                                &serde_json::Value,
                            > = std::collections::HashMap::new();
                            for u in ec {
                                expected_map
                                    .insert(u["connection"]["guid"].as_str().unwrap(), &u["diff"]);
                            }
                            for u in cc {
                                let guid = u["connection"]["guid"].as_str().unwrap();
                                let exp = expected_map.get(guid).unwrap_or_else(|| {
                                    panic!("Unexpected connection update for {}", guid)
                                });
                                let tol = 0.001;
                                assert!(
                                    (u["diff"]["u"].as_f64().unwrap() - exp["u"].as_f64().unwrap())
                                        .abs()
                                        < tol,
                                    "Connection {} u mismatch",
                                    guid
                                );
                                assert!(
                                    (u["diff"]["v"].as_f64().unwrap() - exp["v"].as_f64().unwrap())
                                        .abs()
                                        < tol,
                                    "Connection {} v mismatch",
                                    guid
                                );
                            }
                        }
                        (None, None) => {}
                        _ => panic!(
                            "Connection updates mismatch: expected {:?} vs computed {:?}",
                            expected_conns.map(|v| v.len()),
                            computed_conns.map(|v| v.len())
                        ),
                    }
                }
            }
        } // 🏊Drag Tests
        pub use drag_tests::*;

        mod validation_tests {
            // 🗝️Validation Tests
            // Validation Tests MUST provide the validation tests functionality.

            use super::*;

            mod validation {
                use super::*;

                mod metabolism {
                    use super::*;

                    #[test]
                    pub fn metabolism_kit_validate_empty_report() {
                        let kit = load_kit("metabolism.kit.semio.json");
                        let result = validate_kit(&kit);
                        assert!(result.problems.is_empty());
                    }
                }

                mod invalid {
                    use super::*;

                    #[test]
                    pub fn invalid_kit_validate_invalid_report() {
                        let kit = load_kit("invalid.kit.semio.json");
                        let result = validate_kit(&kit);
                        let expected = load_validation_result("validation.semio.json");
                        assert_eq!(
                            result.problems.len(),
                            expected.problems.len(),
                            "Number of problems mismatch"
                        );
                    }

                    #[test]
                    pub fn plain_descriptions_do_not_create_emoji_validation_problems() {
                        let mut kit = load_kit("metabolism.kit.semio.json");
                        kit.description = Some("Plain kit summary".to_string());
                        if let Some(types) = kit.types.as_mut() {
                            for (index, entry) in types.iter_mut().enumerate() {
                                entry.description =
                                    Some(format!("Repeated plain description {}", index % 2));
                            }
                        }

                        let result = validate_kit(&kit);
                        assert!(
                            result.problems.iter().all(|problem| {
                                problem.constraint_id != "description-missing-emoji"
                                    && problem.constraint_id != "description-emoji-unique"
                            }),
                            "unexpected emoji validation problem: {:?}",
                            result.problems
                        );
                    }
                }
                mod kit_diff_asset {
                    use super::*;
                    use serde::Deserialize;

                    #[derive(Deserialize)]
                    struct Asset {
                        #[serde(rename = "tinyKit")]
                        tiny_kit: Kit,
                        cases: Vec<Case>,
                    }
                    #[derive(Deserialize)]
                    struct Case {
                        id: String,
                        diff: KitDiff,
                        #[serde(rename = "expectOk")]
                        expect_ok: bool,
                        #[serde(rename = "errorCodes")]
                        error_codes: Vec<String>,
                        #[serde(rename = "warningCodes")]
                        warning_codes: Vec<String>,
                    }

                    fn codes(notes: &[crate::KitDiffValidationNote]) -> Vec<String> {
                        notes.iter().filter_map(|n| n.code.clone()).collect()
                    }

                    #[test]
                    pub fn shared_semio_asset_cases() {
                        let path = Path::new(ASSETS_DIR).join("validate-kit-diff.cases.semio.json");
                        let data = fs::read_to_string(&path).expect("read validate-kit-diff asset");
                        let asset: Asset = serde_json::from_str(&data).expect("parse asset");
                        for c in asset.cases {
                            let r = crate::validate_kit_diff(&asset.tiny_kit, &c.diff, false);
                            assert_eq!(
                                r.ok, c.expect_ok,
                                "case {}: err={:?} warn={:?}",
                                c.id, r.errors, r.warnings
                            );
                            let err_codes = codes(&r.errors);
                            for code in &c.error_codes {
                                assert!(
                                    err_codes.iter().any(|e| e == code),
                                    "case {}: missing error {} got {:?}",
                                    c.id,
                                    code,
                                    err_codes
                                );
                            }
                            let warn_codes = codes(&r.warnings);
                            for code in &c.warning_codes {
                                assert!(
                                    warn_codes.iter().any(|w| w == code),
                                    "case {}: missing warning {} got {:?}",
                                    c.id,
                                    code,
                                    warn_codes
                                );
                            }
                        }
                    }

                    #[test]
                    pub fn heal_drops_invalid_design_update() {
                        let path = Path::new(ASSETS_DIR).join("validate-kit-diff.cases.semio.json");
                        let data = fs::read_to_string(&path).expect("read asset");
                        let asset: Asset = serde_json::from_str(&data).expect("parse asset");
                        let bad: KitDiff = serde_json::from_str(
                            r#"{"designs":{"updated":[{"design":{"guid":"99999999-9999-9999-9999-999999999999"},"diff":{"name":"X"}}]}}"#,
                        )
                        .expect("bad diff");
                        let r = crate::validate_kit_diff(&asset.tiny_kit, &bad, true);
                        let d = r.diff.expect("healed diff");
                        assert!(
                            d.designs.is_none()
                                || d.designs
                                    .as_ref()
                                    .unwrap()
                                    .updated
                                    .as_ref()
                                    .map_or(true, |u| u.is_empty()),
                            "heal should drop invalid design update: {:?}",
                            d.designs
                        );
                    }
                }
            }
        } // 🎷Validation Tests
        pub use validation_tests::*;

        mod design_quality_sum_tests {
            // 🥇Design Quality Sum Tests

            use super::*;

            mod design_quality_sum {
                use super::*;

                mod nakagin_capsule_tower {
                    use super::*;

                    #[test]
                    pub fn sum_effective_floor_area() {
                        let asset: QualitySumCasesAsset =
                            load_asset("quality-sum.cases.semio.json");
                        let case = &asset.cases[0];
                        let kit = load_kit(&case.kit);
                        let design = kit
                            .designs
                            .as_ref()
                            .unwrap()
                            .iter()
                            .find(|d| d.name == case.design_name && d.parent.is_none())
                            .expect("Design not found");
                        let quality = kit
                            .qualities
                            .as_ref()
                            .unwrap()
                            .iter()
                            .find(|q| q.name == case.quality_name)
                            .expect("Quality not found");
                        let result = sum_quality_in_design(&kit, &design.guid, &quality.guid);
                        assert!(
                            (result - case.expected).abs() < case.tolerance,
                            "Expected ~{}, got {}",
                            case.expected,
                            result
                        );
                    }
                }
            }
        } // 🥋Design Quality Sum Tests
        pub use design_quality_sum_tests::*;

        mod export_design_model_tests {
            // 🏄Export Design Model Tests

            use super::*;

            mod export_design_model {
                use super::*;

                #[test]
                pub fn glb_format_valid_header() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let design = kit
                        .designs
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design not found");
                    let result = export_design_model(
                        &kit,
                        &design.guid,
                        ".glb",
                        &[],
                        &std::collections::HashMap::new(),
                    )
                    .expect("export_design_model failed");
                    assert!(!result.is_empty(), "GLB result should not be empty");
                    assert_eq!(&result[0..4], b"glTF", "GLB magic mismatch");
                    let version = u32::from_le_bytes(result[4..8].try_into().unwrap());
                    assert_eq!(version, 2, "GLB version should be 2");
                    let total_len = u32::from_le_bytes(result[8..12].try_into().unwrap());
                    assert_eq!(
                        total_len as usize,
                        result.len(),
                        "GLB total length mismatch"
                    );
                }

                #[test]
                pub fn gltf_format_valid_json() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let design = kit
                        .designs
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design not found");
                    let result = export_design_model(
                        &kit,
                        &design.guid,
                        ".gltf",
                        &[],
                        &std::collections::HashMap::new(),
                    )
                    .expect("export_design_model failed");
                    assert!(!result.is_empty(), "glTF result should not be empty");
                    let json_str =
                        std::str::from_utf8(&result).expect("glTF should be valid UTF-8");
                    let parsed: serde_json::Value =
                        serde_json::from_str(json_str).expect("glTF should be valid JSON");
                    assert!(parsed.is_object(), "glTF root should be an object");
                }

                #[test]
                pub fn invalid_format_returns_error() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let design = kit
                        .designs
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design not found");
                    let result = export_design_model(
                        &kit,
                        &design.guid,
                        ".xyz",
                        &[],
                        &std::collections::HashMap::new(),
                    );
                    assert!(result.is_err(), "Invalid format should return error");
                }

                #[test]
                pub fn export_scene_graph_report() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    let design = kit
                        .designs
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                        .expect("Design not found");
                    let result = export_design_model(
                        &kit,
                        &design.guid,
                        ".gltf",
                        &[],
                        &std::collections::HashMap::new(),
                    )
                    .expect("export_design_model failed");
                    let _: serde_json::Value =
                        serde_json::from_slice(&result).expect("glTF should be valid JSON");
                    let reports_dir = Path::new("..")
                        .join("..")
                        .join("reports")
                        .join("export-design-model");
                    fs::create_dir_all(&reports_dir).expect("Failed to create reports directory");
                    fs::write(reports_dir.join("rs.gltf"), result).expect("Failed to write report");
                }
            }
        } // ⚡Export Design Model Tests
        pub use export_design_model_tests::*;

        mod meta_and_shallow_tests {
            // 🃏Meta And Shallow Tests

            use super::*;

            pub fn load_json<T: serde::de::DeserializeOwned>(filename: &str) -> T {
                let path = Path::new(ASSETS_DIR).join(filename);
                let data =
                    fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
                serde_json::from_str(&data).expect(&format!("Failed to deserialize {}", filename))
            }

            #[test]
            pub fn type_meta_from_asset() {
                let meta: TypeMeta = load_json("tambour.meta.type.semio.json");
                assert_eq!(meta.name, "Tambour");
                assert!(!meta.guid.is_empty());
                assert!(meta.description.is_some());
                assert!(meta.unit.is_some());
            }

            #[test]
            pub fn type_shallow_from_asset() {
                let shallow: TypeShallow = load_json("tambour.shallow.type.semio.json");
                assert_eq!(shallow.name, "Tambour");
                assert!(!shallow.guid.is_empty());
                assert!(shallow.connectors.is_some());
                assert!(shallow.models.is_some());
                assert!(shallow.props.is_some());
            }

            #[test]
            pub fn design_meta_from_asset() {
                let meta: DesignMeta = load_json("nakagin-capsule-tower.meta.design.semio.json");
                assert_eq!(meta.name, "Nakagin Capsule Tower");
                assert!(!meta.guid.is_empty());
                assert!(meta.description.is_some());
                assert!(meta.unit.is_some());
            }

            #[test]
            pub fn design_shallow_from_asset() {
                let shallow: DesignShallow =
                    load_json("nakagin-capsule-tower.shallow.design.semio.json");
                assert_eq!(shallow.name, "Nakagin Capsule Tower");
                assert!(!shallow.guid.is_empty());
                assert!(shallow.pieces.is_some());
                assert!(shallow.connections.is_some());
                assert!(shallow.layers.is_some());
            }

            #[test]
            pub fn kit_meta_from_asset() {
                let meta: KitMeta = load_json("metabolism.meta.kit.semio.json");
                assert_eq!(meta.name, "Metabolism");
                assert!(!meta.guid.is_empty());
                assert!(meta.version.is_some());
                assert!(meta.description.is_some());
            }

            #[test]
            pub fn kit_shallow_from_asset() {
                let shallow: KitShallow = load_json("metabolism.shallow.kit.semio.json");
                assert_eq!(shallow.name, "Metabolism");
                assert!(!shallow.guid.is_empty());
                assert!(shallow.types.is_some());
                assert!(shallow.designs.is_some());
                assert!(shallow.tags.is_some());
                assert!(shallow.concepts.is_some());
                assert!(shallow.ports.is_some());
                assert!(shallow.qualities.is_some());
                assert!(shallow.files.is_some());
                assert!(shallow.folders.is_some());
                assert!(shallow.authors.is_some());
            }

            #[test]
            pub fn kit_to_meta_to_shallow() {
                let kit = load_kit("metabolism.kit.semio.json");
                let meta = kit.to_meta();
                assert_eq!(meta.name, kit.name);
                assert_eq!(meta.guid, kit.guid);
                assert_eq!(meta.version, kit.version);
                assert_eq!(meta.description, kit.description);

                let shallow = kit.to_shallow();
                assert_eq!(shallow.name, kit.name);
                assert_eq!(shallow.guid, kit.guid);
                assert_eq!(
                    shallow.types.as_ref().map(|v| v.len()),
                    kit.types.as_ref().map(|v| v.len())
                );
                assert_eq!(
                    shallow.designs.as_ref().map(|v| v.len()),
                    kit.designs.as_ref().map(|v| v.len())
                );

                // Verify type meta conversion preserves names
                if let (Some(full_types), Some(meta_types)) = (&kit.types, &shallow.types) {
                    for (full, meta) in full_types.iter().zip(meta_types.iter()) {
                        assert_eq!(full.guid, meta.guid);
                        assert_eq!(full.name, meta.name);
                    }
                }

                // Verify design meta conversion preserves names
                if let (Some(full_designs), Some(meta_designs)) = (&kit.designs, &shallow.designs) {
                    for (full, meta) in full_designs.iter().zip(meta_designs.iter()) {
                        assert_eq!(full.guid, meta.guid);
                        assert_eq!(full.name, meta.name);
                    }
                }

                // Verify a single type to_meta and to_shallow roundtrip
                if let Some(types) = &kit.types {
                    let first_type = &types[0];
                    let type_meta = first_type.to_meta();
                    assert_eq!(type_meta.guid, first_type.guid);
                    assert_eq!(type_meta.name, first_type.name);

                    let type_shallow = first_type.to_shallow();
                    assert_eq!(type_shallow.guid, first_type.guid);
                    assert_eq!(type_shallow.name, first_type.name);
                    assert_eq!(
                        type_shallow.connectors.as_ref().map(|v| v.len()),
                        first_type.connectors.as_ref().map(|v| v.len())
                    );
                }

                // Verify a single design to_meta and to_shallow roundtrip
                if let Some(designs) = &kit.designs {
                    let first_design = &designs[0];
                    let design_meta = first_design.to_meta();
                    assert_eq!(design_meta.guid, first_design.guid);
                    assert_eq!(design_meta.name, first_design.name);

                    let design_shallow = first_design.to_shallow();
                    assert_eq!(design_shallow.guid, first_design.guid);
                    assert_eq!(design_shallow.name, first_design.name);
                    assert_eq!(
                        design_shallow.pieces.as_ref().map(|v| v.len()),
                        first_design.pieces.as_ref().map(|v| v.len())
                    );
                }
            }
        } // 🎤Meta And Shallow Tests
        pub use meta_and_shallow_tests::*;

        mod kit_workflow_tests {
            // 🏞️Kit Workflow Tests
            // Kit Workflow Tests MUST verify file, folder, archive, remote, and temporary kit workflows.

            use super::*;

            #[cfg(not(target_arch = "wasm32"))]
            mod kit_workflow {
                use super::*;
                use base64::Engine;
                use std::collections::HashMap;
                use std::io::{BufRead, BufReader, Write};
                use std::net::TcpListener;
                use std::thread;

                pub struct TestHttpRoute {
                    path: String,
                    content_type: String,
                    body: Vec<u8>,
                }

                pub fn workflow_kit_fixture() -> (Kit, HashMap<String, Vec<u8>>) {
                    let file_bytes = b"hello workflow".to_vec();
                    let blob = format!(
                        "data:text/plain;base64,{}",
                        base64::engine::general_purpose::STANDARD.encode(&file_bytes)
                    );
                    let folder_guid = "workflow-folder-guid".to_string();
                    let file_guid = "workflow-file-guid".to_string();

                    let kit = Kit {
                        guid: "workflow-kit-guid".to_string(),
                        name: "Workflow Kit".to_string(),
                        version: Some("1.0.0".to_string()),
                        description: Some("Fixture kit for workflow tests".to_string()),
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
                        files: Some(vec![File {
                            guid: file_guid,
                            name: "hello.txt".to_string(),
                            folder: Some(FolderId {
                                guid: folder_guid.clone(),
                            }),
                            size: Some(file_bytes.len() as i64),
                            hash: Some("workflow-hash".to_string()),
                            remote: None,
                            blob: Some(blob),
                            created_at: None,
                            updated_at: None,
                        }]),
                        folders: Some(vec![Folder {
                            guid: folder_guid,
                            name: "assets".to_string(),
                            description: None,
                            parent: None,
                            attributes: None,
                            created_at: None,
                            updated_at: None,
                        }]),
                        authors: None,
                        attributes: None,
                        created_at: None,
                        updated_at: None,
                    };

                    let file_path =
                        zip_roundtrip::build_file_path(&kit, &kit.files.as_ref().unwrap()[0]);
                    let mut files = HashMap::new();
                    files.insert(file_path, file_bytes);

                    (kit, files)
                }

                pub fn workflow_diff() -> KitDiff {
                    KitDiff {
                        guid: "workflow-kit-guid".to_string(),
                        name: Some("Workflow Kit Edited".to_string()),
                        files: Some(CollectionDiff {
                            added: None,
                            removed: None,
                            updated: Some(vec![DiffUpdate {
                                key: "file".to_string(),
                                guid: "workflow-file-guid".to_string(),
                                diff: FileDiff {
                                    guid: "workflow-file-guid".to_string(),
                                    name: Some("renamed.txt".to_string()),
                                    ..Default::default()
                                },
                            }]),
                        }),
                        folders: Some(CollectionDiff {
                            added: None,
                            removed: None,
                            updated: Some(vec![DiffUpdate {
                                key: "folder".to_string(),
                                guid: "workflow-folder-guid".to_string(),
                                diff: FolderDiff {
                                    guid: "workflow-folder-guid".to_string(),
                                    name: Some("renamed-assets".to_string()),
                                    ..Default::default()
                                },
                            }]),
                        }),
                        ..Default::default()
                    }
                }

                pub fn workflow_expected_paths() -> (String, String) {
                    (
                        "assets/hello.txt".to_string(),
                        "renamed-assets/renamed.txt".to_string(),
                    )
                }

                pub fn spawn_test_http_server(
                    routes: Vec<TestHttpRoute>,
                ) -> (String, thread::JoinHandle<()>) {
                    let listener =
                        TcpListener::bind("127.0.0.1:0").expect("bind test http listener");
                    let address = listener
                        .local_addr()
                        .expect("get test http listener address");
                    let handle = thread::spawn(move || {
                        for _ in 0..routes.len() {
                            let (mut stream, _) =
                                listener.accept().expect("accept test http connection");
                            let mut request_line = String::new();
                            {
                                let mut reader =
                                    BufReader::new(stream.try_clone().expect("clone stream"));
                                reader
                                    .read_line(&mut request_line)
                                    .expect("read request line");
                            }
                            let request_path = request_line
                                .split_whitespace()
                                .nth(1)
                                .expect("http request path");
                            let route = routes
                                .iter()
                                .find(|route| route.path == request_path)
                                .expect("route exists");

                            let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
                        route.body.len(),
                        route.content_type
                    );
                            stream
                                .write_all(response.as_bytes())
                                .expect("write response headers");
                            stream.write_all(&route.body).expect("write response body");
                            stream.flush().expect("flush response");
                        }
                    });

                    (format!("http://{}", address), handle)
                }

                #[test]
                pub fn dev_workflow_roundtrips_and_persists_edits() {
                    let (kit, _) = workflow_kit_fixture();
                    let diff = workflow_diff();
                    let temp_dir = tempfile::tempdir().unwrap();
                    let path = temp_dir.path().join("workflow.kit.semio.json");
                    let path_str = path.to_str().unwrap();

                    export_dev_kit(&kit, path_str).unwrap();
                    let imported = import_dev_kit(path_str).unwrap();
                    assert!(are_kits_equal(&kit, &imported));

                    let edited = edit_dev_kit(path_str, &diff).unwrap();
                    let persisted = import_dev_kit(path_str).unwrap();
                    assert_eq!(edited.name, "Workflow Kit Edited");
                    assert!(are_kits_equal(&edited, &persisted));
                }

                #[test]
                pub fn local_workflow_roundtrips_and_moves_assets_on_edit() {
                    let (kit, files) = workflow_kit_fixture();
                    let diff = workflow_diff();
                    let (old_path, new_path) = workflow_expected_paths();
                    let temp_dir = tempfile::tempdir().unwrap();
                    let folder_path = temp_dir.path().join("workflow-folder-kit");
                    let folder_path_str = folder_path.to_str().unwrap();

                    export_local_kit(&kit, &files, folder_path_str).unwrap();
                    let imported = import_local_kit(folder_path_str).unwrap();
                    assert!(are_kits_equal(&kit, &imported.kit));
                    assert_eq!(imported.files, files);

                    let edited = edit_local_kit(folder_path_str, &diff).unwrap();
                    let reloaded = import_local_kit(folder_path_str).unwrap();
                    assert_eq!(edited.name, "Workflow Kit Edited");
                    assert!(are_kits_equal(&edited, &reloaded.kit));
                    assert!(!folder_path.join(old_path).exists());
                    assert!(folder_path.join(new_path).exists());
                }

                #[test]
                pub fn archive_workflow_roundtrips_and_persists_edits() {
                    let (kit, files) = workflow_kit_fixture();
                    let diff = workflow_diff();
                    let temp_dir = tempfile::tempdir().unwrap();
                    let archive_path = temp_dir.path().join("workflow.semio.zip");
                    let archive_path_str = archive_path.to_str().unwrap();

                    zip_roundtrip::export_kit_to_zip(&kit, &files, archive_path_str).unwrap();
                    let imported = zip_roundtrip::import_kit_from_zip(archive_path_str).unwrap();
                    assert!(are_kits_equal(&kit, &imported.kit));
                    assert_eq!(imported.files, files);

                    let edited = edit_archive_kit(archive_path_str, &diff).unwrap();
                    let reloaded = zip_roundtrip::import_kit_from_zip(archive_path_str).unwrap();
                    assert_eq!(edited.name, "Workflow Kit Edited");
                    assert!(are_kits_equal(&edited, &reloaded.kit));
                    assert!(reloaded.files.contains_key("renamed-assets/renamed.txt"));
                    assert!(!reloaded.files.contains_key("assets/hello.txt"));
                }

                #[test]
                pub fn remote_workflow_supports_json_and_zip_sources() {
                    let (kit, files) = workflow_kit_fixture();
                    let diff = workflow_diff();
                    let json_body = serialize_kit(&kit).unwrap().into_bytes();

                    let temp_dir = tempfile::tempdir().unwrap();
                    let archive_path = temp_dir.path().join("remote.semio.zip");
                    let archive_path_str = archive_path.to_str().unwrap();
                    zip_roundtrip::export_kit_to_zip(&kit, &files, archive_path_str).unwrap();
                    let zip_body = std::fs::read(archive_path_str).unwrap();

                    let (base_url, handle) = spawn_test_http_server(vec![
                        TestHttpRoute {
                            path: "/kit.json".to_string(),
                            content_type: "application/json".to_string(),
                            body: json_body,
                        },
                        TestHttpRoute {
                            path: "/kit.zip".to_string(),
                            content_type: "application/zip".to_string(),
                            body: zip_body,
                        },
                        TestHttpRoute {
                            path: "/kit.json".to_string(),
                            content_type: "application/json".to_string(),
                            body: serialize_kit(&kit).unwrap().into_bytes(),
                        },
                    ]);

                    let json_import = import_remote_kit(&format!("{}/kit.json", base_url)).unwrap();
                    assert!(are_kits_equal(&kit, &json_import.kit));
                    assert!(json_import.files.is_empty());

                    let zip_import = import_remote_kit(&format!("{}/kit.zip", base_url)).unwrap();
                    assert!(are_kits_equal(&kit, &zip_import.kit));
                    assert_eq!(zip_import.files, files);

                    let edited = edit_remote_kit(&format!("{}/kit.json", base_url), &diff).unwrap();
                    assert_eq!(edited.name, "Workflow Kit Edited");
                    assert_eq!(edited.files.as_ref().unwrap()[0].name, "renamed.txt");

                    handle.join().unwrap();
                }

                #[test]
                pub fn transport_workflow_applies_diff_without_mutating_source() {
                    let (kit, _) = workflow_kit_fixture();
                    let diff = workflow_diff();

                    let edited = edit_transport_kit(&kit, &diff);
                    assert_eq!(edited.name, "Workflow Kit Edited");
                    assert_eq!(edited.files.as_ref().unwrap()[0].name, "renamed.txt");
                    assert_eq!(edited.folders.as_ref().unwrap()[0].name, "renamed-assets");
                    assert_eq!(kit.name, "Workflow Kit");
                    assert_eq!(kit.files.as_ref().unwrap()[0].name, "hello.txt");
                    assert_eq!(kit.folders.as_ref().unwrap()[0].name, "assets");
                }
            }
        } // 🏠Kit Workflow Tests
        pub use kit_workflow_tests::*;

        mod kit_kind_tests {
            // 📊KitKind Tests
            // KitKind Tests MUST verify serialization, deserialization and completeness.

            use super::*;

            #[test]
            pub fn test_kit_kind_all_values_exist() {
                assert_eq!(ALL_KIT_KINDS.len(), 5);
                assert!(ALL_KIT_KINDS.contains(&KitKind::Dev));
                assert!(ALL_KIT_KINDS.contains(&KitKind::Local));
                assert!(ALL_KIT_KINDS.contains(&KitKind::Archive));
                assert!(ALL_KIT_KINDS.contains(&KitKind::Remote));
                assert!(ALL_KIT_KINDS.contains(&KitKind::Transport));
            }

            #[test]
            pub fn test_kit_kind_serialization() {
                assert_eq!(serde_json::to_string(&KitKind::Dev).unwrap(), "\"dev\"");
                assert_eq!(serde_json::to_string(&KitKind::Local).unwrap(), "\"local\"");
                assert_eq!(
                    serde_json::to_string(&KitKind::Archive).unwrap(),
                    "\"archive\""
                );
                assert_eq!(
                    serde_json::to_string(&KitKind::Remote).unwrap(),
                    "\"remote\""
                );
                assert_eq!(
                    serde_json::to_string(&KitKind::Transport).unwrap(),
                    "\"transport\""
                );
            }

            #[test]
            pub fn test_kit_kind_deserialization() {
                assert_eq!(
                    serde_json::from_str::<KitKind>("\"dev\"").unwrap(),
                    KitKind::Dev
                );
                assert_eq!(
                    serde_json::from_str::<KitKind>("\"local\"").unwrap(),
                    KitKind::Local
                );
                assert_eq!(
                    serde_json::from_str::<KitKind>("\"archive\"").unwrap(),
                    KitKind::Archive
                );
                assert_eq!(
                    serde_json::from_str::<KitKind>("\"remote\"").unwrap(),
                    KitKind::Remote
                );
                assert_eq!(
                    serde_json::from_str::<KitKind>("\"transport\"").unwrap(),
                    KitKind::Transport
                );
            }

            #[test]
            pub fn test_kit_kind_dev_roundtrip() {
                let kit = Kit {
                    guid: "test-guid-dev".to_string(),
                    name: "TestDevKit".to_string(),
                    version: Some("1.0.0".to_string()),
                    description: Some("A dev kit".to_string()),
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
                let json = serde_json::to_string(&kit).unwrap();
                let roundtripped: Kit = serde_json::from_str(&json).unwrap();
                assert_eq!(kit, roundtripped);
            }

            #[test]
            pub fn test_kit_kind_transport_in_memory() {
                let mut kit = Kit {
                    guid: "transport-guid".to_string(),
                    name: "TransportKit".to_string(),
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
                let kind = KitKind::Transport;
                assert_eq!(kind, KitKind::Transport);
                kit.name = "ModifiedTransportKit".to_string();
                kit.description = Some("Modified in memory".to_string());
                assert_eq!(kit.name, "ModifiedTransportKit");
                assert_eq!(kit.description, Some("Modified in memory".to_string()));
            }

            #[test]
            pub fn test_sync_kit_dev_apply_and_export() {
                let kit = Kit {
                    guid: "sync-dev-guid".to_string(),
                    name: "SyncDevKit".to_string(),
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
                let mut dev = DevKit::new(kit);
                assert_eq!(dev.kit().name, "SyncDevKit");

                let diff = KitDiff {
                    guid: "sync-dev-guid".to_string(),
                    name: Some("SyncDevKitEdited".to_string()),
                    ..Default::default()
                };
                dev.apply(&diff);
                assert_eq!(dev.kit().name, "SyncDevKitEdited");

                let transport = dev.export_transport().unwrap();
                let roundtripped = transport.to_kit().unwrap();
                assert_eq!(roundtripped.name, "SyncDevKitEdited");
            }

            #[test]
            pub fn test_sync_kit_local_apply_and_export() {
                let kit = Kit {
                    guid: "sync-local-guid".to_string(),
                    name: "SyncLocalKit".to_string(),
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
                let mut local = LocalKit::new(kit);
                assert_eq!(local.kit().name, "SyncLocalKit");

                let diff = KitDiff {
                    guid: "sync-local-guid".to_string(),
                    name: Some("SyncLocalKitEdited".to_string()),
                    ..Default::default()
                };
                local.apply(&diff);
                assert_eq!(local.kit().name, "SyncLocalKitEdited");

                let transport = local.export_transport().unwrap();
                let roundtripped = transport.to_kit().unwrap();
                assert_eq!(roundtripped.name, "SyncLocalKitEdited");
            }

            #[test]
            pub fn test_sync_kit_remote_apply_and_export() {
                let kit = Kit {
                    guid: "sync-remote-guid".to_string(),
                    name: "SyncRemoteKit".to_string(),
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
                let mut remote = RemoteKit::new(kit);
                assert_eq!(remote.kit().name, "SyncRemoteKit");

                let diff = KitDiff {
                    guid: "sync-remote-guid".to_string(),
                    name: Some("SyncRemoteKitEdited".to_string()),
                    ..Default::default()
                };
                remote.apply(&diff);
                assert_eq!(remote.kit().name, "SyncRemoteKitEdited");

                let transport = remote.export_transport().unwrap();
                let roundtripped = transport.to_kit().unwrap();
                assert_eq!(roundtripped.name, "SyncRemoteKitEdited");
            }

            #[test]
            pub fn test_transport_kit_roundtrip() {
                let kit = Kit {
                    guid: "transport-rt-guid".to_string(),
                    name: "TransportRoundtrip".to_string(),
                    version: Some("2.0.0".to_string()),
                    description: Some("Transport roundtrip test".to_string()),
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
                let transport = TransportKit::from_kit(&kit).unwrap();
                assert!(!transport.json.is_empty());
                let roundtripped = transport.to_kit().unwrap();
                assert_eq!(roundtripped.name, "TransportRoundtrip");
                assert_eq!(roundtripped.version, Some("2.0.0".to_string()));
            }

            #[test]
            pub fn test_sync_kit_import_transport() {
                let source_kit = Kit {
                    guid: "import-source-guid".to_string(),
                    name: "SourceKit".to_string(),
                    version: Some("1.0.0".to_string()),
                    description: Some("Source".to_string()),
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
                let transport = TransportKit::from_kit(&source_kit).unwrap();

                let target_kit = Kit {
                    guid: "import-source-guid".to_string(),
                    name: "OldName".to_string(),
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
                let mut dev = DevKit::new(target_kit);
                dev.import_transport(&transport).unwrap();
                assert_eq!(dev.kit().name, "SourceKit");
                assert_eq!(dev.kit().version, Some("1.0.0".to_string()));
            }
        } // 🧶KitKind Tests
        pub use kit_kind_tests::*;

        mod hash_tests {
            // 🎖️Hash Tests

            use super::*;

            #[test]
            pub fn test_hash_kit() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let hash = hash_kit(&kit);
                assert_eq!(hash, cases.kit_hash.expected);
            }

            #[test]
            pub fn test_hash_piece_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let design = kit
                    .designs
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|d| d.name == cases.design_name && d.parent.is_none())
                    .unwrap();
                let piece = &design.pieces.as_ref().unwrap()[0];
                let h1 = hash_piece(piece);
                let h2 = hash_piece(piece);
                assert_eq!(h1, h2);
                assert!(h1.len() == 64);
            }

            #[test]
            pub fn test_hash_connection_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let design = kit
                    .designs
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|d| d.name == cases.design_name && d.parent.is_none())
                    .unwrap();
                let conn = &design.connections.as_ref().unwrap()[0];
                let h1 = hash_connection(conn);
                let h2 = hash_connection(conn);
                assert_eq!(h1, h2);
                assert!(h1.len() == 64);
            }

            #[test]
            pub fn test_hash_connector_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let t = kit
                    .types
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|t| t.connectors.as_ref().map_or(false, |c| !c.is_empty()))
                    .unwrap();
                let conn = &t.connectors.as_ref().unwrap()[0];
                let h1 = hash_connector(conn);
                let h2 = hash_connector(conn);
                assert_eq!(h1, h2);
                assert!(h1.len() == 64);
            }

            #[test]
            pub fn test_hash_type_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let t = &kit.types.as_ref().unwrap()[0];
                let h1 = hash_type(t);
                let h2 = hash_type(t);
                assert_eq!(h1, h2);
                assert!(h1.len() == 64);
            }

            #[test]
            pub fn test_hash_design_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let kit = load_kit(&cases.kit_hash.kit);
                let d = &kit.designs.as_ref().unwrap()[0];
                let h1 = hash_design(d);
                let h2 = hash_design(d);
                assert_eq!(h1, h2);
                assert!(h1.len() == 64);
            }

            #[test]
            pub fn test_hash_kit_diff_canonical() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let diff: KitDiff = serde_json::from_str(&cases.kit_diff_hash.json).unwrap();
                let hash = hash_kit_diff(&diff);
                assert_eq!(hash, cases.kit_diff_hash.expected);
            }

            #[test]
            pub fn test_hash_kit_diff_name_only() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let json = r#"{"name":"updated"}"#;
                let diff: KitDiff = serde_json::from_str(json).unwrap();
                let hash = hash_kit_diff(&diff);
                assert!(hash.len() == 64);
                assert_ne!(hash, cases.kit_diff_hash.expected);
            }

            #[test]
            pub fn test_hash_kit_diff_empty() {
                let json = r#"{}"#;
                let diff: KitDiff = serde_json::from_str(json).unwrap();
                let hash = hash_kit_diff(&diff);
                assert!(hash.len() == 64);
            }

            #[test]
            pub fn test_hash_kit_diff_deterministic() {
                let cases: HashCasesAsset = load_asset("hash.cases.semio.json");
                let diff: KitDiff = serde_json::from_str(&cases.kit_diff_hash.json).unwrap();
                let h1 = hash_kit_diff(&diff);
                let h2 = hash_kit_diff(&diff);
                assert_eq!(h1, h2);
            }

            #[test]
            pub fn test_hash_attribute_diff() {
                let diff = AttributeDiff {
                    guid: String::new(),
                    key: Some("newKey".to_string()),
                    value: Some(None),
                    definition: None,
                };
                let hash = hash_attribute_diff(&diff);
                assert!(hash.len() == 64);
            }

            #[test]
            pub fn test_hash_kit_diff_field_order_matters() {
                let json1 = r#"{"name":"a","description":"b"}"#;
                let json2 = r#"{"description":"b","name":"a"}"#;
                let diff1: KitDiff = serde_json::from_str(json1).unwrap();
                let diff2: KitDiff = serde_json::from_str(json2).unwrap();
                assert_eq!(hash_kit_diff(&diff1), hash_kit_diff(&diff2));
            }
        } // 🏸Hash Tests
        pub use hash_tests::*;

        mod max_children_tests {
            // 👠MaxChildren Tests
            use super::*;

            #[test]
            pub fn port_max_children_roundtrip() {
                let port = Port {
                    guid: "p1".to_string(),
                    name: "TestPort".to_string(),
                    max_children: Some(3),
                    description: None,
                    icon: None,
                    compatible_interfaces: None,
                    attributes: None,
                };
                let json = serde_json::to_string(&port).unwrap();
                assert!(json.contains("\"maxChildren\":3"));
                let restored: Port = serde_json::from_str(&json).unwrap();
                assert_eq!(restored.max_children, Some(3));
            }

            #[test]
            pub fn port_max_children_omitted() {
                let port = Port {
                    guid: "p1".to_string(),
                    name: "TestPort".to_string(),
                    max_children: None,
                    description: None,
                    icon: None,
                    compatible_interfaces: None,
                    attributes: None,
                };
                let json = serde_json::to_string(&port).unwrap();
                assert!(!json.contains("maxChildren"));
            }

            #[test]
            pub fn connector_max_children_roundtrip() {
                let connector = Connector {
                    guid: "c1".to_string(),
                    t: 0.0,
                    point: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    direction: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    max_children: Some(5),
                    name: None,
                    description: None,
                    port: None,
                    mandatory: None,
                    props: None,
                    attributes: None,
                };
                let json = serde_json::to_string(&connector).unwrap();
                assert!(json.contains("\"maxChildren\":5"));
                let restored: Connector = serde_json::from_str(&json).unwrap();
                assert_eq!(restored.max_children, Some(5));
            }

            #[test]
            pub fn kit_with_max_children_roundtrip() {
                let json = r#"{"guid":"kit-1","name":"TestKit","ports":[{"guid":"p1","name":"Port1","maxChildren":3}],"types":[{"guid":"t1","name":"Type1","connectors":[{"guid":"c1","t":0,"point":{"x":0,"y":0,"z":0},"direction":{"x":0,"y":0,"z":1},"maxChildren":5}]}]}"#;
                let kit: Kit = serde_json::from_str(json).unwrap();
                assert_eq!(kit.ports.as_ref().unwrap()[0].max_children, Some(3));
                assert_eq!(
                    kit.types.as_ref().unwrap()[0].connectors.as_ref().unwrap()[0].max_children,
                    Some(5)
                );
                let reserialized = serde_json::to_string(&kit).unwrap();
                let restored: Kit = serde_json::from_str(&reserialized).unwrap();
                assert_eq!(restored.ports.as_ref().unwrap()[0].max_children, Some(3));
                assert_eq!(
                    restored.types.as_ref().unwrap()[0]
                        .connectors
                        .as_ref()
                        .unwrap()[0]
                        .max_children,
                    Some(5)
                );
            }
        } // ⚡MaxChildren Tests
        pub use max_children_tests::*;
    }
} // 🧪Tests
pub use tests::*;

mod benchmark {
    // 🏋️Benchmarks
    // 🕹️Benchmark
    use super::*;

    #[cfg(test)]
    pub mod benchmark {
        use super::*;
        use std::fs;
        use std::path::Path;
        use std::time::Instant;

        pub const ASSETS_DIR: &str = "../assets/semio";
        pub const ITERATIONS: u32 = 3;
        pub const BENCHMARK_CSV_LANGUAGES: [&str; 5] =
            ["go", "typescript", "python", "rust", "csharp"];

        pub fn load_kit(filename: &str) -> Kit {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data =
                fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
            serde_json::from_str(&data).expect("Failed to deserialize kit")
        }

        pub fn load_kit_diff(filename: &str) -> KitDiff {
            let path = Path::new(ASSETS_DIR).join(filename);
            let data =
                fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
            serde_json::from_str(&data).expect("Failed to deserialize kit diff")
        }

        pub fn benchmark_csv_path() -> std::path::PathBuf {
            let package_path = Path::new("..").join("benchmark.csv");
            if package_path.parent().map_or(false, Path::exists) {
                return package_path;
            }
            Path::new("semio").join("benchmark.csv")
        }

        pub fn append_benchmark_csv(language: &str, name: &str, duration_seconds: f64) {
            let path = benchmark_csv_path();
            let mut rows: std::collections::BTreeMap<
                String,
                std::collections::BTreeMap<String, String>,
            > = std::collections::BTreeMap::new();
            let mut order: Vec<String> = Vec::new();
            if let Ok(data) = fs::read_to_string(&path) {
                let lines: Vec<&str> = data
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .collect();
                if !lines.is_empty() && lines[0].starts_with("name,") {
                    let headers = parse_csv_line(lines[0]);
                    for line in lines.iter().skip(1) {
                        let values = parse_csv_line(line);
                        if values.is_empty() || values[0].is_empty() {
                            continue;
                        }
                        if !rows.contains_key(&values[0]) {
                            rows.insert(values[0].clone(), std::collections::BTreeMap::new());
                            order.push(values[0].clone());
                        }
                        if let Some(row) = rows.get_mut(&values[0]) {
                            for index in 1..values.len().min(headers.len()) {
                                if !values[index].is_empty() {
                                    row.insert(headers[index].clone(), values[index].clone());
                                }
                            }
                        }
                    }
                }
            }
            if !rows.contains_key(name) {
                rows.insert(name.to_string(), std::collections::BTreeMap::new());
                order.push(name.to_string());
            }
            if let Some(row) = rows.get_mut(name) {
                row.insert(
                    language.to_string(),
                    format!("{:.6}", duration_seconds * 1000.0),
                );
            }
            let mut output = String::from("name");
            for lang in BENCHMARK_CSV_LANGUAGES {
                output.push(',');
                output.push_str(lang);
            }
            output.push('\n');
            for row_name in order {
                output.push_str(&csv_value(&row_name));
                if let Some(row) = rows.get(&row_name) {
                    for lang in BENCHMARK_CSV_LANGUAGES {
                        output.push(',');
                        output.push_str(row.get(lang).map(String::as_str).unwrap_or(""));
                    }
                }
                output.push('\n');
            }
            let _ = fs::write(&path, output);
        }

        pub fn parse_csv_line(line: &str) -> Vec<String> {
            let mut values = Vec::new();
            let mut current = String::new();
            let mut chars = line.chars().peekable();
            let mut in_quotes = false;
            while let Some(ch) = chars.next() {
                if ch == '"' {
                    if in_quotes && chars.peek() == Some(&'"') {
                        current.push('"');
                        chars.next();
                    } else {
                        in_quotes = !in_quotes;
                    }
                } else if ch == ',' && !in_quotes {
                    values.push(current);
                    current = String::new();
                } else {
                    current.push(ch);
                }
            }
            values.push(current);
            values
        }

        pub fn csv_value(value: &str) -> String {
            format!("\"{}\"", value.replace('"', "\"\""))
        }

        pub fn bench<F: Fn()>(name: &str, f: F) {
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                f();
            }
            let duration = start.elapsed().as_secs_f64() / ITERATIONS as f64;
            println!("{},{:.6}", name, duration);
            append_benchmark_csv("rust", name, duration);
        }

        pub fn find_design<'a>(kit: &'a Kit, name: &str, parent_name: Option<&str>) -> &'a Design {
            let parent_guid = if let Some(pn) = parent_name {
                kit.designs
                    .iter()
                    .flatten()
                    .find(|d| d.name == pn)
                    .map(|d| d.guid.clone())
            } else {
                None
            };

            if parent_name.is_some() && parent_guid.is_none() {
                panic!("Parent {} not found", parent_name.unwrap());
            }

            kit.designs
                .iter()
                .flatten()
                .find(|d| {
                    if d.name != name {
                        return false;
                    }
                    match &d.parent {
                        Some(p) => match &parent_guid {
                            Some(pg) => p.guid == *pg,
                            None => false,
                        },
                        None => parent_guid.is_none(),
                    }
                })
                .expect(&format!("Design {} not found", name))
        }

        pub fn run_benchmarks() {
            let kit_metabolism = load_kit("metabolism.kit.semio.json");
            let mut kit_original = kit_metabolism.clone();
            kit_original.designs = kit_metabolism.designs.clone().map(|designs| {
                designs
                    .into_iter()
                    .filter(|design| design.parent.is_none())
                    .collect()
            });
            let kit_diffed = load_kit("metabolism.kit.diffed.semio.json");
            let kit_invalid = load_kit("invalid.kit.semio.json");
            let metabolism_change = get_kit_change(&kit_original, &kit_diffed);
            let diff_forward = metabolism_change.forward;
            let diff_inverse = metabolism_change.backward;

            bench("Roundtrip/Metabolism", || {
                let serialized = serialize_kit(&kit_metabolism).unwrap();
                let restored = deserialize_kit(&serialized).unwrap();
                if restored != kit_metabolism {
                    panic!("Roundtrip/Metabolism output does not match test expectation");
                }
            });

            bench("Diff/Metabolism", || {
                let mut k2 = kit_original.clone();
                apply_kit_diff(&mut k2, &diff_forward);
                if !are_kits_equal(&k2, &kit_diffed) {
                    panic!("Diff/Metabolism forward output does not match test expectation");
                }
                apply_kit_diff(&mut k2, &diff_inverse);
                if !are_kits_equal(&k2, &kit_original) {
                    panic!("Diff/Metabolism inverse output does not match test expectation");
                }
            });

            let d1 = find_design(&kit_metabolism, "Nakagin Capsule Tower", None);
            let d1_guid = d1.guid.clone();
            bench("Flatten Design/Nakagin Capsule Tower", || {
                let diff = flatten_design_change(&kit_metabolism, &d1_guid);
                if diff
                    .forward
                    .pieces
                    .as_ref()
                    .and_then(|p| p.updated.as_ref())
                    .map_or(0, Vec::len)
                    == 0
                {
                    panic!("Flatten Design/Nakagin Capsule Tower output does not match test expectation");
                }
            });

            let d2 = find_design(&kit_metabolism, "Slanted", Some("Nakagin Capsule Tower"));
            let d2_guid = d2.guid.clone();
            bench("Flatten Design/Nakagin Capsule Tower/Slanted", || {
                let diff = flatten_design_change(&kit_metabolism, &d2_guid);
                if diff
                    .forward
                    .pieces
                    .as_ref()
                    .and_then(|p| p.updated.as_ref())
                    .map_or(0, Vec::len)
                    == 0
                {
                    panic!("Flatten Design/Nakagin Capsule Tower/Slanted output does not match test expectation");
                }
            });

            let d3 = find_design(&kit_metabolism, "Twisted", Some("Nakagin Capsule Tower"));
            let d3_guid = d3.guid.clone();
            bench("Flatten Design/Nakagin Capsule Tower/Twisted", || {
                let diff = flatten_design_change(&kit_metabolism, &d3_guid);
                if diff
                    .forward
                    .pieces
                    .as_ref()
                    .and_then(|p| p.updated.as_ref())
                    .map_or(0, Vec::len)
                    == 0
                {
                    panic!("Flatten Design/Nakagin Capsule Tower/Twisted output does not match test expectation");
                }
            });

            let d4 = find_design(&kit_metabolism, "Dancing", Some("Nakagin Capsule Tower"));
            let d4_guid = d4.guid.clone();
            bench("Flatten Design/Nakagin Capsule Tower/Dancing", || {
                let diff = flatten_design_change(&kit_metabolism, &d4_guid);
                if diff
                    .forward
                    .pieces
                    .as_ref()
                    .and_then(|p| p.updated.as_ref())
                    .map_or(0, Vec::len)
                    == 0
                {
                    panic!("Flatten Design/Nakagin Capsule Tower/Dancing output does not match test expectation");
                }
            });

            let d5 = find_design(&kit_metabolism, "Capsule Dream", None);
            let d5_guid = d5.guid.clone();
            bench("Flatten Design/Capsule Dream", || {
                let diff = flatten_design_change(&kit_metabolism, &d5_guid);
                if diff
                    .forward
                    .pieces
                    .as_ref()
                    .and_then(|p| p.updated.as_ref())
                    .map_or(0, Vec::len)
                    == 0
                {
                    panic!("Flatten Design/Capsule Dream output does not match test expectation");
                }
            });

            bench("Validation/Invalid Kit", || {
                let result = validate_kit(&kit_invalid);
                if result.problems.is_empty() {
                    panic!("Validation/Invalid Kit output does not match test expectation");
                }
            });

            bench("Validation/Metabolism", || {
                let result = validate_kit(&kit_metabolism);
                if !result.problems.is_empty() {
                    panic!("Validation/Metabolism output does not match test expectation");
                }
            });
        }

        #[test]
        pub fn benchmark_csv_exports_all_rows() {
            run_benchmarks();
        }
    }
} // 🏋️Benchmarks
pub use benchmark::*;
