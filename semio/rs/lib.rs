// #region 🔖Header
// [👤semio📚rs💻semio](repo://p/u/semio/b/l/rs/f/semio.rs)

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

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚rs💻semio🔖imports](repo://p/u/semio/b/l/rs/f/semio.rs/s/Imports)
// Imports MUST include all required crates and modules for the semio domain library.

#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use base64::Engine;
use nalgebra::{Matrix4, Point3, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use thiserror::Error;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// #endregion 🔖Imports

// #region 🔖Error Types
// [👤semio📚rs💻semio🔖errortypes](repo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types)
// Error Types MUST provide the error types functionality.

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
// [🛠️semio/rs/semio.rs#Error Types§SemioError](repo://definition/semio/rs/semio.rs/ERROR-TYPES/SEMIO-ERROR)
/// <summary>SemioError holds the data fields for a SemioError record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖errortypes🛠️semioerror](repo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types/d/i/SemioError)
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

// [🛠️semio/rs/semio.rs#Error Types§Result](repo://definition/semio/rs/semio.rs/ERROR-TYPES/RESULT)
/// <summary>Result holds the data fields for a Result record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖errortypes🛠️result](repo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types/d/i/Result)
/// </remarks>
pub type Result<T> = std::result::Result<T, SemioError>;

// #endregion 🔖Error Types

// #region 🔖Utility Functions
// [👤semio📚rs💻semio🔖utilityfunctions](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions)
// Utility Functions MUST provide the utility functions functionality.

// [🛠️semio/rs/semio.rs#Utility Functions§Guid](repo://definition/semio/rs/semio.rs/UTILITY-FUNCTIONS/GUID)
/// <summary>Guid holds the data fields for a Guid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️guid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/Guid)
/// </remarks>
pub type Guid = String;

/// <summary>guid holds the data fields for a guid record.</summary>
/// guid MUST perform the guid operation.
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️guid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/guid)
pub fn guid() -> String {
    Uuid::now_v7().to_string()
}

/// <summary>normalize holds the data fields for a normalize record.</summary>
/// normalize MUST perform the normalize operation.
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️normalize](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/normalize)
pub fn normalize(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// <summary>round holds the data fields for a round record.</summary>
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️round](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/round)
/// <remarks>
/// round MUST perform the round operation.
/// </remarks>
pub fn round(value: f64) -> f64 {
    normalize(value, 3)
}

/// <summary>jaccard holds the data fields for a jaccard record.</summary>
/// jaccard MUST perform the jaccard operation.
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️jaccard](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/jaccard)
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

/// <summary>deep_equal holds the data fields for a deep_equal record.</summary>
/// deep_equal MUST perform the deep_equal operation.
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️deepequal](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/deep_equal)
pub fn deep_equal<T: Serialize>(a: &T, b: &T) -> bool {
    fn normalize_json(v: &mut serde_json::Value) {
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
    fn json_approx_eq(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        use serde_json::Value;
        match (a, b) {
            (Value::Number(na), Value::Number(nb)) => match (na.as_f64(), nb.as_f64()) {
                (Some(fa), Some(fb)) => (fa - fb).abs() < 1e-10,
                _ => na == nb,
            },
            (Value::Array(aa), Value::Array(ab)) => {
                aa.len() == ab.len() && aa.iter().zip(ab.iter()).all(|(x, y)| json_approx_eq(x, y))
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

// [🛠️semio/rs/semio.rs#Utility Functions§generate_unique_name](repo://definition/semio/rs/semio.rs/UTILITY-FUNCTIONS/GENERATE-UNIQUE-NAME)
/// <summary>generate_unique_name performs the generate_unique_name operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖utilityfunctions🛠️generateuniquename](repo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/generate_unique_name)
/// generate_unique_name MUST perform the generate_unique_name operation.
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

// #endregion 🔖Utility Functions

// #region 🔖Model Types - Attribute
// [👤semio📚rs💻semio🔖modeltypesattribute](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute)
// Model Types - Attribute MUST provide the model types - attribute functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Attribute holds the data fields for a Attribute record.</summary>
/// Attribute MUST perform the Attribute operation.
/// [👤semio📚rs💻semio🔖modeltypesattribute🛠️attribute](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute/d/i/Attribute)
pub struct Attribute {
    pub guid: Guid,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Attribute§AttributeId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-ATTRIBUTE/ATTRIBUTE-ID)
/// <summary>AttributeId holds the data fields for a AttributeId record.</summary>
/// AttributeId MUST perform the AttributeId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesattribute🛠️attributeid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute/d/i/AttributeId)
/// </remarks>
pub struct AttributeId {
    pub guid: Guid,
}

// #endregion 🔖Model Types - Attribute

// #region 🔖Model Types - Coord
// [👤semio📚rs💻semio🔖modeltypescoord](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord)
// Model Types - Coord MUST provide the model types - coord functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Coord§Coord](repo://definition/semio/rs/semio.rs/MODEL-TYPES-COORD/COORD)
/// <summary>Coord holds the data fields for a Coord record.</summary>
/// Coord MUST perform the Coord operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypescoord🛠️coord](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord/d/i/Coord)
/// </remarks>
pub struct Coord {
    pub u: f64,
    pub v: f64,
}

// [👤semio📚rs💻semio🔖modeltypescoord🛠️coord](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord/d/i/Coord)
/// <summary>Coord holds the data fields for a Coord record.</summary>
/// Coord MUST perform the Coord operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypescoord🛠️coord](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord/d/i/Coord)
/// </remarks>
impl Coord {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

// #endregion 🔖Model Types - Coord

// #region 🔖Model Types - Vector
// [👤semio📚rs💻semio🔖modeltypesvector](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector)
// Model Types - Vector MUST provide the model types - vector functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Vector§Vector](repo://definition/semio/rs/semio.rs/MODEL-TYPES-VECTOR/VECTOR)
/// <summary>Vector holds the data fields for a Vector record.</summary>
/// Vector MUST perform the Vector operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesvector🛠️vector](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector/d/i/Vector)
/// </remarks>
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// <summary>Vector holds the data fields for a Vector record.</summary>
/// Vector MUST perform the Vector operation.
/// [👤semio📚rs💻semio🔖modeltypesvector🛠️vector](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector/d/i/Vector)
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

// #endregion 🔖Model Types - Vector

// #region 🔖Model Types - Plane
// [👤semio📚rs💻semio🔖modeltypesplane](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane)
// Model Types - Plane MUST provide the model types - plane functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Plane holds the data fields for a Plane record.</summary>
/// Plane MUST perform the Plane operation.
/// [👤semio📚rs💻semio🔖modeltypesplane🛠️plane](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Plane)
pub struct Plane {
    pub origin: Vector,
    #[serde(rename = "xAxis")]
    pub x_axis: Vector,
    #[serde(rename = "yAxis")]
    pub y_axis: Vector,
}

/// <summary>Default holds the data fields for a Default record.</summary>
/// Default MUST perform the Default operation.
/// [👤semio📚rs💻semio🔖modeltypesplane🛠️default](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Default)
impl Default for Plane {
    fn default() -> Self {
        Self {
            origin: Vector::zero(),
            x_axis: Vector::unit_x(),
            y_axis: Vector::unit_y(),
        }
    }
}

/// Plane MUST perform the Plane operation.
/// <summary>Plane holds the data fields for a Plane record.</summary>
/// [👤semio📚rs💻semio🔖modeltypesplane🛠️plane](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Plane)
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

// #endregion 🔖Model Types - Plane

// #region 🔖Model Types - Camera
// [👤semio📚rs💻semio🔖modeltypescamera](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera)
// Model Types - Camera MUST provide the model types - camera functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Camera§Camera](repo://definition/semio/rs/semio.rs/MODEL-TYPES-CAMERA/CAMERA)
/// <summary>Camera holds the data fields for a Camera record.</summary>
/// Camera MUST perform the Camera operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypescamera🛠️camera](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera/d/i/Camera)
/// </remarks>
pub struct Camera {
    pub position: Vector,
    pub target: Vector,
    pub up: Vector,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
}

/// <summary>Default holds the data fields for a Default record.</summary>
/// Default MUST perform the Default operation.
/// [👤semio📚rs💻semio🔖modeltypescamera🛠️default](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera/d/i/Default)
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

// #endregion 🔖Model Types - Camera

// #region 🔖Model Types - Location, Author, File, Folder
// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder)
// Model Types - Location, Author, File, Folder MUST provide the model types - location, author, file, folder functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>LocationId holds the data fields for a LocationId record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Location, Author, File, Folder§LocationId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LOCATION-AUTHOR-FILE-FOLDER/LOCATION-ID)
/// <summary>LocationId holds the data fields for a LocationId record.</summary>
/// LocationId MUST perform the LocationId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️locationid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/LocationId)
/// </remarks>
pub struct LocationId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Location holds the data fields for a Location record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️location](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Location)
/// <remarks>
/// Location MUST perform the Location operation.
/// </remarks>
pub struct Location {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>AuthorId holds the data fields for a AuthorId record.</summary>
/// AuthorId MUST perform the AuthorId operation.
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️authorid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/AuthorId)
pub struct AuthorId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Author holds the data fields for a Author record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️author](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Author)
/// <remarks>
/// Author MUST perform the Author operation.
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>FolderId holds the data fields for a FolderId record.</summary>
/// FolderId MUST perform the FolderId operation.
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️folderid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/FolderId)
pub struct FolderId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Location, Author, File, Folder§Folder](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LOCATION-AUTHOR-FILE-FOLDER/FOLDER)
/// <summary>Folder holds the data fields for a Folder record.</summary>
/// Folder MUST perform the Folder operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️folder](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Folder)
/// </remarks>
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
/// <summary>FileId holds the data fields for a FileId record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️fileid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/FileId)
/// <remarks>
/// FileId MUST perform the FileId operation.
/// </remarks>
pub struct FileId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>File holds the data fields for a File record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️file](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/File)
/// <remarks>
/// File MUST perform the File operation.
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

// #endregion 🔖Model Types - Location, Author, File, Folder

// #region 🔖Model Types - Quality, Port, Tag, Concept
// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept)
// Model Types - Quality, Port, Tag, Concept MUST provide the model types - quality, port, tag, concept functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§QualityId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/QUALITY-ID)
/// <summary>QualityId holds the data fields for a QualityId record.</summary>
/// QualityId MUST perform the QualityId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️qualityid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/QualityId)
/// </remarks>
pub struct QualityId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(i32)]
/// <summary>QualityKind holds the data fields for a QualityKind record.</summary>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️qualitykind](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/QualityKind)
pub enum QualityKind {
    #[default]
    Integer = 0,
    Float = 1,
    Boolean = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Quality](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/QUALITY)
/// <summary>Quality holds the data fields for a Quality record.</summary>
/// Quality MUST perform the Quality operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️quality](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Quality)
/// </remarks>
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
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§PortId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/PORT-ID)
/// <summary>PortId holds the data fields for a PortId record.</summary>
/// PortId MUST perform the PortId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️portid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/PortId)
/// </remarks>
pub struct PortId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Port](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/PORT)
/// <summary>Port holds the data fields for a Port record.</summary>
/// Port MUST perform the Port operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️port](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Port)
/// </remarks>
pub struct Port {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "compatiblePorts", skip_serializing_if = "Option::is_none")]
    pub compatible_interfaces: Option<Vec<PortId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§TagId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/TAG-ID)
/// <summary>TagId holds the data fields for a TagId record.</summary>
/// TagId MUST perform the TagId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️tagid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/TagId)
/// </remarks>
pub struct TagId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Tag holds the data fields for a Tag record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Tag](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/TAG)
/// <summary>Tag holds the data fields for a Tag record.</summary>
/// Tag MUST perform the Tag operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️tag](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Tag)
/// </remarks>
pub struct Tag {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§ConceptId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/CONCEPT-ID)
/// <summary>ConceptId holds the data fields for a ConceptId record.</summary>
/// ConceptId MUST perform the ConceptId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️conceptid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/ConceptId)
/// </remarks>
pub struct ConceptId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Concept MUST perform the Concept operation.
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Concept](repo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/CONCEPT)
/// <summary>Concept holds the data fields for a Concept record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️concept](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Concept)
/// </remarks>
/// <remarks>
/// Concept MUST perform the Concept operation.
/// </remarks>
pub struct Concept {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

// #endregion 🔖Model Types - Quality, Port, Tag, Concept

// #region 🔖Model Types - Prop, Model, Connector
// [👤semio📚rs💻semio🔖modeltypespropmodelconnector](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector)
// Model Types - Prop, Model, Connector MUST provide the model types - prop, model, connector functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§PropId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/PROP-ID)
/// <summary>PropId holds the data fields for a PropId record.</summary>
/// PropId MUST perform the PropId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️propid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/PropId)
/// </remarks>
pub struct PropId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Prop holds the data fields for a Prop record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Prop](repo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/PROP)
/// <summary>Prop holds the data fields for a Prop record.</summary>
/// Prop MUST perform the Prop operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️prop](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Prop)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>ModelId holds the data fields for a ModelId record.</summary>
/// ModelId MUST perform the ModelId operation.
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️modelid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/ModelId)
pub struct ModelId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Model MUST perform the Model operation.
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Model](repo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/MODEL)
/// <summary>Model holds the data fields for a Model record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️model](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Model)
/// </remarks>
/// <remarks>
/// Model MUST perform the Model operation.
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>ConnectorId holds the data fields for a ConnectorId record.</summary>
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️connectorid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/ConnectorId)
/// <remarks>
/// ConnectorId MUST perform the ConnectorId operation.
/// </remarks>
pub struct ConnectorId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Connector](repo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/CONNECTOR)
/// <summary>Connector holds the data fields for a Connector record.</summary>
/// Connector MUST perform the Connector operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️connector](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Connector)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

// #endregion 🔖Model Types - Prop, Model, Connector

// #region 🔖Model Types - Type
// [👤semio📚rs💻semio🔖modeltypestype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type)
// Model Types - Type MUST provide the model types - type functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>TypeId holds the data fields for a TypeId record.</summary>
/// TypeId MUST perform the TypeId operation.
/// [👤semio📚rs💻semio🔖modeltypestype🛠️typeid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type/d/i/TypeId)
pub struct TypeId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Type§Type](repo://definition/semio/rs/semio.rs/MODEL-TYPES-TYPE/TYPE)
/// <summary>Type holds the data fields for a Type record.</summary>
/// Type MUST perform the Type operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypestype🛠️type](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type/d/i/Type)
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

// #endregion 🔖Model Types - Type

// #region 🔖Model Types - Layer, Piece, Group, Side, Connection, Stat
// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat)
// Model Types - Layer, Piece, Group, Side, Connection, Stat MUST provide the model types - layer, piece, group, side, connection, stat functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>LayerId holds the data fields for a LayerId record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layerid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/LayerId)
/// <remarks>
/// LayerId MUST perform the LayerId operation.
/// </remarks>
pub struct LayerId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Layer](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/LAYER)
/// <summary>Layer holds the data fields for a Layer record.</summary>
/// Layer MUST perform the Layer operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layer](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Layer)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§PieceId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/PIECE-ID)
/// <summary>PieceId holds the data fields for a PieceId record.</summary>
/// PieceId MUST perform the PieceId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️pieceid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/PieceId)
/// </remarks>
pub struct PieceId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§DesignId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/DESIGN-ID)
/// <summary>DesignId holds the data fields for a DesignId record.</summary>
/// DesignId MUST perform the DesignId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️designid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/DesignId)
/// </remarks>
pub struct DesignId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Piece](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/PIECE)
/// <summary>Piece holds the data fields for a Piece record.</summary>
/// Piece MUST perform the Piece operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️piece](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Piece)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§GroupId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/GROUP-ID)
/// <summary>GroupId holds the data fields for a GroupId record.</summary>
/// GroupId MUST perform the GroupId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️groupid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/GroupId)
/// </remarks>
pub struct GroupId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Group holds the data fields for a Group record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Group](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/GROUP)
/// <summary>Group holds the data fields for a Group record.</summary>
/// Group MUST perform the Group operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️group](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Group)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Side](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/SIDE)
/// <summary>Side holds the data fields for a Side record.</summary>
/// Side MUST perform the Side operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️side](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Side)
/// </remarks>
pub struct Side {
    pub piece: PieceId,
    #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
    pub design_piece: Option<PieceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<ConnectorId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§ConnectionId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/CONNECTION-ID)
/// <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/// ConnectionId MUST perform the ConnectionId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connectionid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/ConnectionId)
/// </remarks>
pub struct ConnectionId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Connection](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/CONNECTION)
/// <summary>Connection holds the data fields for a Connection record.</summary>
/// Connection MUST perform the Connection operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connection](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Connection)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§StatId](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/STAT-ID)
/// <summary>StatId holds the data fields for a StatId record.</summary>
/// StatId MUST perform the StatId operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️statid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/StatId)
/// </remarks>
pub struct StatId {
    pub guid: Guid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Stat MUST perform the Stat operation.
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Stat](repo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/STAT)
/// <summary>Stat holds the data fields for a Stat record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️stat](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Stat)
/// </remarks>
/// <remarks>
/// Stat MUST perform the Stat operation.
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

// #endregion 🔖Model Types - Layer, Piece, Group, Side, Connection, Stat

// #region 🔖Model Types - Design
// [👤semio📚rs💻semio🔖modeltypesdesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Design)
// Model Types - Design MUST provide the model types - design functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Design§Design](repo://definition/semio/rs/semio.rs/MODEL-TYPES-DESIGN/DESIGN)
/// <summary>Design holds the data fields for a Design record.</summary>
/// Design MUST perform the Design operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖modeltypesdesign🛠️design](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Design/d/i/Design)
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
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// #endregion 🔖Model Types - Design

// #region 🔖Model Types - Kit
// [👤semio📚rs💻semio🔖modeltypeskit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit)
// Model Types - Kit MUST provide the model types - kit functionality.

// #region 🔖KitKind
// [👤semio📚rs💻semio🔖modeltypeskit🔖kitkind](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit/s/KitKind)
// KitKind discriminates the five persistence/transport forms of a Kit.

/// Discriminator for the five kit persistence/transport forms.
///
/// Specs: Exactly five kit kinds exist:
/// - File: Self-contained JSON file (.kit.json)
/// - Folder: Local folder with .semio/kit.db SQLite file and asset files
/// - Archive: ZIP file packaging a FolderKit structure
/// - Remote: URL-addressable kit served over HTTP(S)
/// - Temporary: In-memory ephemeral kit (no persistence)
/// [👤semio📚rs💻semio🔖modeltypeskit🔖kitkind🛠️kitkind](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit/s/KitKind/d/i/KitKind)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum KitKind {
    File,
    Folder,
    Archive,
    Remote,
    Temporary,
}

/// All valid KitKind values.
/// [👤semio📚rs💻semio🔖modeltypeskit🔖kitkind🪨allkitkinds](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit/s/KitKind/d/i/ALL_KIT_KINDS)
pub const ALL_KIT_KINDS: [KitKind; 5] = [
    KitKind::File,
    KitKind::Folder,
    KitKind::Archive,
    KitKind::Remote,
    KitKind::Temporary,
];

// #endregion 🔖KitKind

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Kit holds the data fields for a Kit record.</summary>
/// [👤semio📚rs💻semio🔖modeltypeskit🛠️kit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit/d/i/Kit)
/// <remarks>
/// Kit MUST perform the Kit operation.
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

// #endregion 🔖Model Types - Kit

// #region 🔖Finder Functions
// [👤semio📚rs💻semio🔖finderfunctions](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions)
// Finder Functions MUST provide the finder functions functionality.

// [🛠️semio/rs/semio.rs#Finder Functions§find_type_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TYPE-IN-KIT)
/// <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/// find_type_in_kit MUST perform the find_type_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findtypeinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_type_in_kit)
/// </remarks>
pub fn find_type_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Type> {
    kit.types.as_ref()?.iter().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_type_in_kit_mut](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TYPE-IN-KIT-MUT)
/// <summary>find_type_in_kit_mut holds the data fields for a find_type_in_kit_mut record.</summary>
/// find_type_in_kit_mut MUST perform the find_type_in_kit_mut operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findtypeinkitmut](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_type_in_kit_mut)
/// </remarks>
pub fn find_type_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Type> {
    kit.types.as_mut()?.iter_mut().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_design_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-DESIGN-IN-KIT)
/// <summary>find_design_in_kit holds the data fields for a find_design_in_kit record.</summary>
/// find_design_in_kit MUST perform the find_design_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️finddesigninkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_design_in_kit)
/// </remarks>
pub fn find_design_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Design> {
    kit.designs.as_ref()?.iter().find(|d| d.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_design_in_kit_mut](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-DESIGN-IN-KIT-MUT)
/// <summary>find_design_in_kit_mut holds the data fields for a find_design_in_kit_mut record.</summary>
/// find_design_in_kit_mut MUST perform the find_design_in_kit_mut operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️finddesigninkitmut](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_design_in_kit_mut)
/// </remarks>
pub fn find_design_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Design> {
    kit.designs.as_mut()?.iter_mut().find(|d| d.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_piece_in_design](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-PIECE-IN-DESIGN)
/// <summary>find_piece_in_design holds the data fields for a find_piece_in_design record.</summary>
/// find_piece_in_design MUST perform the find_piece_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findpieceindesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_piece_in_design)
/// </remarks>
pub fn find_piece_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Piece> {
    design.pieces.as_ref()?.iter().find(|p| p.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_piece_in_design_mut](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-PIECE-IN-DESIGN-MUT)
/// <summary>find_piece_in_design_mut performs the find_piece_in_design_mut operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findpieceindesignmut](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_piece_in_design_mut)
/// find_piece_in_design_mut MUST perform the find_piece_in_design_mut operation.
/// </remarks>
pub fn find_piece_in_design_mut<'a>(design: &'a mut Design, guid: &str) -> Option<&'a mut Piece> {
    design.pieces.as_mut()?.iter_mut().find(|p| p.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_connection_in_design](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONNECTION-IN-DESIGN)
/// <summary>find_connection_in_design holds the data fields for a find_connection_in_design record.</summary>
/// find_connection_in_design MUST perform the find_connection_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findconnectionindesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_connection_in_design)
/// </remarks>
pub fn find_connection_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Connection> {
    design.connections.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_connector_in_type](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONNECTOR-IN-TYPE)
/// <summary>find_connector_in_type holds the data fields for a find_connector_in_type record.</summary>
/// find_connector_in_type MUST perform the find_connector_in_type operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findconnectorintype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_connector_in_type)
/// </remarks>
pub fn find_connector_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Connector> {
    t.connectors.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_model_in_type](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-MODEL-IN-TYPE)
/// <summary>find_model_in_type holds the data fields for a find_model_in_type record.</summary>
/// find_model_in_type MUST perform the find_model_in_type operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findmodelintype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_model_in_type)
/// </remarks>
pub fn find_model_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Model> {
    t.models.as_ref()?.iter().find(|m| m.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_file_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-FILE-IN-KIT)
/// <summary>find_file_in_kit performs the find_file_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findfileinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_file_in_kit)
/// find_file_in_kit MUST perform the find_file_in_kit operation.
/// </remarks>
pub fn find_file_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a File> {
    kit.files.as_ref()?.iter().find(|f| f.guid == guid)
}

/// <summary>find_folder_in_kit holds the data fields for a find_folder_in_kit record.</summary>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findfolderinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_folder_in_kit)
/// <remarks>
/// find_folder_in_kit MUST perform the find_folder_in_kit operation.
/// </remarks>
pub fn find_folder_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Folder> {
    kit.folders.as_ref()?.iter().find(|f| f.guid == guid)
}

/// find_author_in_kit MUST perform the find_author_in_kit operation.
// [🛠️semio/rs/semio.rs#Finder Functions§find_author_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-AUTHOR-IN-KIT)
/// <summary>find_author_in_kit performs the find_author_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findauthorinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_author_in_kit)
/// find_author_in_kit MUST perform the find_author_in_kit operation.
/// </remarks>
pub fn find_author_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Author> {
    kit.authors.as_ref()?.iter().find(|a| a.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_tag_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TAG-IN-KIT)
/// <summary>find_tag_in_kit holds the data fields for a find_tag_in_kit record.</summary>
/// find_tag_in_kit MUST perform the find_tag_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findtaginkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_tag_in_kit)
/// </remarks>
pub fn find_tag_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Tag> {
    kit.tags.as_ref()?.iter().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_concept_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONCEPT-IN-KIT)
/// <summary>find_concept_in_kit holds the data fields for a find_concept_in_kit record.</summary>
/// find_concept_in_kit MUST perform the find_concept_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findconceptinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_concept_in_kit)
/// </remarks>
pub fn find_concept_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Concept> {
    kit.concepts.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_quality_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-QUALITY-IN-KIT)
/// <summary>find_quality_in_kit holds the data fields for a find_quality_in_kit record.</summary>
/// find_quality_in_kit MUST perform the find_quality_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findqualityinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_quality_in_kit)
/// </remarks>
pub fn find_quality_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Quality> {
    kit.qualities.as_ref()?.iter().find(|q| q.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_interface_in_kit](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-INTERFACE-IN-KIT)
/// <summary>find_interface_in_kit performs the find_interface_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findinterfaceinkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_interface_in_kit)
/// find_interface_in_kit MUST perform the find_interface_in_kit operation.
/// </remarks>
pub fn find_interface_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Port> {
    kit.ports.as_ref()?.iter().find(|i| i.guid == guid)
}

/// <summary>find_layer_in_design holds the data fields for a find_layer_in_design record.</summary>
/// find_layer_in_design MUST perform the find_layer_in_design operation.
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findlayerindesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_layer_in_design)
pub fn find_layer_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Layer> {
    design.layers.as_ref()?.iter().find(|l| l.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_group_in_design](repo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-GROUP-IN-DESIGN)
/// <summary>find_group_in_design holds the data fields for a find_group_in_design record.</summary>
/// find_group_in_design MUST perform the find_group_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findgroupindesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_group_in_design)
/// </remarks>
pub fn find_group_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Group> {
    design.groups.as_ref()?.iter().find(|g| g.guid == guid)
}

/// <summary>find_stat_in_design holds the data fields for a find_stat_in_design record.</summary>
/// [👤semio📚rs💻semio🔖finderfunctions🛠️findstatindesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_stat_in_design)
/// <remarks>
/// find_stat_in_design MUST perform the find_stat_in_design operation.
/// </remarks>
pub fn find_stat_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Stat> {
    design.stats.as_ref()?.iter().find(|s| s.guid == guid)
}

/// sum_quality_in_design MUST sum all quality values for a given quality across all pieces in a design.
/// For each piece, it checks piece-level props first, then falls back to type-level props.
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

// #endregion 🔖Finder Functions

// #region 🔖Serialization
// [👤semio📚rs💻semio🔖serialization](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization)
// Serialization MUST provide the serialization functionality.

/// <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/// serialize_kit MUST perform the serialize_kit operation.
/// [👤semio📚rs💻semio🔖serialization🛠️serializekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_kit)
pub fn serialize_kit(kit: &Kit) -> Result<String> {
    serde_json::to_string_pretty(kit).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_kit](repo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-KIT)
/// <summary>deserialize_kit holds the data fields for a deserialize_kit record.</summary>
/// deserialize_kit MUST perform the deserialize_kit operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️deserializekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_kit)
/// </remarks>
pub fn deserialize_kit(json: &str) -> Result<Kit> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

// [🛠️semio/rs/semio.rs#Serialization§serialize_design](repo://definition/semio/rs/semio.rs/SERIALIZATION/SERIALIZE-DESIGN)
/// <summary>serialize_design performs the serialize_design operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️serializedesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_design)
/// serialize_design MUST perform the serialize_design operation.
/// </remarks>
pub fn serialize_design(design: &Design) -> Result<String> {
    serde_json::to_string_pretty(design).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_design](repo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-DESIGN)
/// <summary>deserialize_design holds the data fields for a deserialize_design record.</summary>
/// deserialize_design MUST perform the deserialize_design operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️deserializedesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_design)
/// </remarks>
pub fn deserialize_design(json: &str) -> Result<Design> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

/// <summary>serialize_type holds the data fields for a serialize_type record.</summary>
// [🛠️semio/rs/semio.rs#Serialization§serialize_type](repo://definition/semio/rs/semio.rs/SERIALIZATION/SERIALIZE-TYPE)
/// <summary>serialize_type performs the serialize_type operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️serializetype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_type)
/// serialize_type MUST perform the serialize_type operation.
/// </remarks>
pub fn serialize_type(t: &Type) -> Result<String> {
    serde_json::to_string_pretty(t).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_type](repo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-TYPE)
/// <summary>deserialize_type holds the data fields for a deserialize_type record.</summary>
/// deserialize_type MUST perform the deserialize_type operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️deserializetype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_type)
/// </remarks>
pub fn deserialize_type(json: &str) -> Result<Type> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

/// are_kits_equal MUST perform the are_kits_equal operation.
// [🛠️semio/rs/semio.rs#Serialization§are_kits_equal](repo://definition/semio/rs/semio.rs/SERIALIZATION/ARE-KITS-EQUAL)
/// <summary>are_kits_equal performs the are_kits_equal operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️arekitsequal](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_kits_equal)
/// are_kits_equal MUST perform the are_kits_equal operation.
/// </remarks>
pub fn are_kits_equal(a: &Kit, b: &Kit) -> bool {
    deep_equal(a, b)
}
// [🛠️semio/rs/semio.rs#Serialization§are_designs_equal](repo://definition/semio/rs/semio.rs/SERIALIZATION/ARE-DESIGNS-EQUAL)
/// <summary>are_designs_equal holds the data fields for a are_designs_equal record.</summary>
/// are_designs_equal MUST perform the are_designs_equal operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️aredesignsequal](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_designs_equal)
/// </remarks>
pub fn are_designs_equal(a: &Design, b: &Design) -> bool {
    deep_equal(a, b)
}
/// <summary>are_types_equal holds the data fields for a are_types_equal record.</summary>
/// are_types_equal MUST perform the are_types_equal operation.
/// [👤semio📚rs💻semio🔖serialization🛠️aretypesequal](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_types_equal)
pub fn are_types_equal(a: &Type, b: &Type) -> bool {
    deep_equal(a, b)
}

/// <summary>SUPPORTED_MODEL_EXTENSIONS holds the data fields for a SUPPORTED_MODEL_EXTENSIONS record.</summary>
/// [👤semio📚rs💻semio🔖serialization🛠️supportedmodelextensions](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/SUPPORTED_MODEL_EXTENSIONS)
pub const SUPPORTED_MODEL_EXTENSIONS: &[&str] = &[
    "gltf", "glb", "fbx", "obj", "dae", "3ds", "stl", "ply", "usdz", "vrm", "ifc", "3mf",
];

/// is_supported_model_extension MUST perform the is_supported_model_extension operation.
// [🛠️semio/rs/semio.rs#Serialization§is_supported_model_extension](repo://definition/semio/rs/semio.rs/SERIALIZATION/IS-SUPPORTED-MODEL-EXTENSION)
/// <summary>is_supported_model_extension performs the is_supported_model_extension operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖serialization🛠️issupportedmodelextension](repo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/is_supported_model_extension)
/// is_supported_model_extension MUST perform the is_supported_model_extension operation.
/// </remarks>
pub fn is_supported_model_extension(ext: &str) -> bool {
    SUPPORTED_MODEL_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// #endregion 🔖Serialization

// #region 🔖Diff Types
// [👤semio📚rs💻semio🔖difftypes](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types)
// Diff Types MUST provide the diff types functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
// [🛠️semio/rs/semio.rs#Diff Types§RemovedItem](repo://definition/semio/rs/semio.rs/DIFF-TYPES/REMOVED-ITEM)
/// <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/// RemovedItem MUST perform the RemovedItem operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️removeditem](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/RemovedItem)
/// </remarks>
pub struct RemovedItem {
    pub guid: Guid,
}

#[derive(Debug, Clone, PartialEq)]
// [🛠️semio/rs/semio.rs#Diff Types§DiffUpdate](repo://definition/semio/rs/semio.rs/DIFF-TYPES/DIFF-UPDATE)
/// <summary>DiffUpdate holds the data fields for a DiffUpdate record.</summary>
/// DiffUpdate MUST perform the DiffUpdate operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️diffupdate](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DiffUpdate)
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

        let guid =
            guid.ok_or_else(|| serde::de::Error::custom("Could not find guid in update wrapper"))?;
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

        let diff: D =
            serde_json::from_value(Value::Object(diff_obj)).map_err(serde::de::Error::custom)?;

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
        struct GuidWrapper {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(bound(deserialize = "T: Deserialize<'de>, D: serde::de::DeserializeOwned"))]
// [🛠️semio/rs/semio.rs#Diff Types§CollectionDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/COLLECTION-DIFF)
/// <summary>CollectionDiff holds the data fields for a CollectionDiff record.</summary>
/// CollectionDiff MUST perform the CollectionDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️collectiondiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/CollectionDiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§AttributeDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/ATTRIBUTE-DIFF)
/// <summary>AttributeDiff holds the data fields for a AttributeDiff record.</summary>
/// AttributeDiff MUST perform the AttributeDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️attributediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AttributeDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§PropDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/PROP-DIFF)
/// <summary>PropDiff holds the data fields for a PropDiff record.</summary>
/// PropDiff MUST perform the PropDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️propdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PropDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§ConnectorDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/CONNECTOR-DIFF)
/// <summary>ConnectorDiff holds the data fields for a ConnectorDiff record.</summary>
/// ConnectorDiff MUST perform the ConnectorDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️connectordiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectorDiff)
/// </remarks>
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
    pub port: Option<Option<PortId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CollectionDiff<Prop, PropDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Diff Types§ModelDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/MODEL-DIFF)
/// <summary>ModelDiff holds the data fields for a ModelDiff record.</summary>
/// ModelDiff MUST perform the ModelDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️modeldiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ModelDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§TypeDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/TYPE-DIFF)
/// <summary>TypeDiff holds the data fields for a TypeDiff record.</summary>
/// TypeDiff MUST perform the TypeDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️typediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TypeDiff)
/// </remarks>
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
    pub concepts: Option<Option<Vec<ConceptId>>>,
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
// [🛠️semio/rs/semio.rs#Diff Types§SideDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/SIDE-DIFF)
/// <summary>SideDiff holds the data fields for a SideDiff record.</summary>
/// SideDiff MUST perform the SideDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️sidediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/SideDiff)
/// </remarks>
pub struct SideDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piece: Option<PieceId>,
    #[serde(rename = "designPiece", skip_serializing_if = "Option::is_none")]
    pub design_piece: Option<Option<PieceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<Option<ConnectorId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Diff Types§ConnectionDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/CONNECTION-DIFF)
/// <summary>ConnectionDiff holds the data fields for a ConnectionDiff record.</summary>
/// ConnectionDiff MUST perform the ConnectionDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️connectiondiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectionDiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§PieceDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/PIECE-DIFF)
/// <summary>PieceDiff holds the data fields for a PieceDiff record.</summary>
/// PieceDiff MUST perform the PieceDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️piecediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PieceDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§LayerDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/LAYER-DIFF)
/// <summary>LayerDiff holds the data fields for a LayerDiff record.</summary>
/// LayerDiff MUST perform the LayerDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️layerdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/LayerDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§GroupDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/GROUP-DIFF)
/// <summary>GroupDiff holds the data fields for a GroupDiff record.</summary>
/// GroupDiff MUST perform the GroupDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️groupdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/GroupDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§StatDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/STAT-DIFF)
/// <summary>StatDiff holds the data fields for a StatDiff record.</summary>
/// StatDiff MUST perform the StatDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️statdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/StatDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§DesignDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/DESIGN-DIFF)
/// <summary>DesignDiff holds the data fields for a DesignDiff record.</summary>
/// DesignDiff MUST perform the DesignDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️designdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DesignDiff)
/// </remarks>
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
    pub concepts: Option<Option<Vec<ConceptId>>>,
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
// [🛠️semio/rs/semio.rs#Diff Types§TagDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/TAG-DIFF)
/// <summary>TagDiff holds the data fields for a TagDiff record.</summary>
/// TagDiff MUST perform the TagDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️tagdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TagDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§ConceptDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/CONCEPT-DIFF)
/// <summary>ConceptDiff holds the data fields for a ConceptDiff record.</summary>
/// ConceptDiff MUST perform the ConceptDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️conceptdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConceptDiff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§PortDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/PORT-DIFF)
/// <summary>PortDiff holds the data fields for a PortDiff record.</summary>
/// PortDiff MUST perform the PortDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️portdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PortDiff)
/// </remarks>
pub struct PortDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "compatiblePorts", skip_serializing_if = "Option::is_none")]
    pub compatible_interfaces: Option<Option<Vec<PortId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CollectionDiff<Attribute, AttributeDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Diff Types§QualityDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/QUALITY-DIFF)
/// <summary>QualityDiff holds the data fields for a QualityDiff record.</summary>
/// QualityDiff MUST perform the QualityDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️qualitydiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/QualityDiff)
/// </remarks>
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
    #[serde(
        rename = "defaultImperialUnit",
        skip_serializing_if = "Option::is_none"
    )]
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
// [🛠️semio/rs/semio.rs#Diff Types§FileDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/FILE-DIFF)
/// <summary>FileDiff holds the data fields for a FileDiff record.</summary>
/// FileDiff MUST perform the FileDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️filediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FileDiff)
/// </remarks>
pub struct FileDiff {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
// [🛠️semio/rs/semio.rs#Diff Types§FolderDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/FOLDER-DIFF)
/// <summary>FolderDiff holds the data fields for a FolderDiff record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️folderdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FolderDiff)
/// FolderDiff MUST perform the FolderDiff operation.
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§AuthorDiff](repo://definition/semio/rs/semio.rs/DIFF-TYPES/AUTHOR-DIFF)
/// <summary>AuthorDiff holds the data fields for a AuthorDiff record.</summary>
/// AuthorDiff MUST perform the AuthorDiff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️authordiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AuthorDiff)
/// </remarks>
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
/// <summary>KitDiff holds the data fields for a KitDiff record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️kitdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/KitDiff)
/// <remarks>
/// KitDiff MUST perform the KitDiff operation.
/// </remarks>
pub struct KitDiff {
    #[serde(default)]
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
// [👤semio📚rs💻semio🔖difftypes🛠️change](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/Change)
/// <summary>Change holds the data fields for a Change record.</summary>
/// Change MUST perform the Change operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️change](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/Change)
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

// [👤semio📚rs💻semio🔖difftypes🛠️attributechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AttributeChange)
/// <summary>AttributeChange holds the data fields for a AttributeChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️attributechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AttributeChange)
/// </remarks>
pub type AttributeChange = Change<Attribute, AttributeDiff>;
/// <summary>AuthorChange holds the data fields for a AuthorChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️authorchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AuthorChange)
pub type AuthorChange = Change<Author, AuthorDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️filechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FileChange)
/// <summary>FileChange holds the data fields for a FileChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️filechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FileChange)
/// </remarks>
pub type FileChange = Change<File, FileDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️folderchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FolderChange)
/// <summary>FolderChange holds the data fields for a FolderChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️folderchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FolderChange)
/// </remarks>
pub type FolderChange = Change<Folder, FolderDiff>;
/// <summary>QualityChange holds the data fields for a QualityChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️qualitychange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/QualityChange)
pub type QualityChange = Change<Quality, QualityDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️portchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PortChange)
/// <summary>PortChange holds the data fields for a PortChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️portchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PortChange)
/// </remarks>
pub type PortChange = Change<Port, PortDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️propchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PropChange)
/// <summary>PropChange holds the data fields for a PropChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️propchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PropChange)
/// </remarks>
pub type PropChange = Change<Prop, PropDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️tagchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TagChange)
/// <summary>TagChange holds the data fields for a TagChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️tagchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TagChange)
/// </remarks>
pub type TagChange = Change<Tag, TagDiff>;
/// <summary>ConceptChange holds the data fields for a ConceptChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️conceptchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConceptChange)
pub type ConceptChange = Change<Concept, ConceptDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️modelchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ModelChange)
/// <summary>ModelChange holds the data fields for a ModelChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️modelchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ModelChange)
/// </remarks>
pub type ModelChange = Change<Model, ModelDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️connectorchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectorChange)
/// <summary>ConnectorChange holds the data fields for a ConnectorChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️connectorchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectorChange)
/// </remarks>
pub type ConnectorChange = Change<Connector, ConnectorDiff>;
/// <summary>TypeChange holds the data fields for a TypeChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️typechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TypeChange)
pub type TypeChange = Change<Type, TypeDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️layerchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/LayerChange)
/// <summary>LayerChange holds the data fields for a LayerChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️layerchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/LayerChange)
/// </remarks>
pub type LayerChange = Change<Layer, LayerDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️piecechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PieceChange)
/// <summary>PieceChange holds the data fields for a PieceChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️piecechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PieceChange)
/// </remarks>
pub type PieceChange = Change<Piece, PieceDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️groupchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/GroupChange)
/// <summary>GroupChange holds the data fields for a GroupChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️groupchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/GroupChange)
/// </remarks>
pub type GroupChange = Change<Group, GroupDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️sidechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/SideChange)
/// <summary>SideChange holds the data fields for a SideChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️sidechange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/SideChange)
/// </remarks>
pub type SideChange = Change<Side, SideDiff>;
/// <summary>ConnectionChange holds the data fields for a ConnectionChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️connectionchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectionChange)
pub type ConnectionChange = Change<Connection, ConnectionDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️statchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/StatChange)
/// <summary>StatChange holds the data fields for a StatChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️statchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/StatChange)
/// </remarks>
pub type StatChange = Change<Stat, StatDiff>;
// [👤semio📚rs💻semio🔖difftypes🛠️designchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DesignChange)
/// <summary>DesignChange holds the data fields for a DesignChange record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖difftypes🛠️designchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DesignChange)
/// </remarks>
pub type DesignChange = Change<Design, DesignDiff>;
/// <summary>KitChange holds the data fields for a KitChange record.</summary>
/// [👤semio📚rs💻semio🔖difftypes🛠️kitchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/KitChange)
pub type KitChange = Change<Kit, KitDiff>;

// #endregion 🔖Diff Types

// #region 🔖Meta And Shallow Types
// [👤semio📚rs💻semio🔖metaandshallowtypes](repo://p/u/semio/b/l/rs/f/semio.rs/s/Meta%20And%20Shallow%20Types)
// Meta And Shallow Types MUST provide lightweight entity representations.

// #region 🔖Sub-entity Meta Types

/// AttributeMeta is identical to Attribute (no Vec fields to omit).
pub type AttributeMeta = Attribute;

/// StatMeta is identical to Stat (no Vec fields to omit).
pub type StatMeta = Stat;

/// TagMeta is identical to Tag (no Vec fields to omit).
pub type TagMeta = Tag;

/// ConceptMeta is identical to Concept (no Vec fields to omit).
pub type ConceptMeta = Concept;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// PropMeta is Prop without attributes.
pub struct PropMeta {
    pub guid: Guid,
    pub quality: QualityId,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// AuthorMeta is Author without attributes.
pub struct AuthorMeta {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// FileMeta is File without blob.
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
/// FolderMeta is Folder without attributes.
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
/// QualityMeta is Quality without attributes.
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
/// PortMeta is Port without compatible_ports and attributes.
pub struct PortMeta {
    pub guid: Guid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// ModelMeta is Model without tags and attributes.
pub struct ModelMeta {
    pub guid: Guid,
    pub file: FileId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// ConnectorMeta is Connector without props and attributes.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// LayerMeta is Layer without attributes.
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
/// PieceMeta is Piece without props and attributes.
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
/// GroupMeta is Group without pieces and attributes.
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
/// ConnectionMeta is Connection without attributes.
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

// #endregion 🔖Sub-entity Meta Types

// #region 🔖Main Entity Meta Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// TypeMeta is Type with only scalar fields (no models, connectors, props, attributes, authors, concepts).
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
/// TypeShallow is Type with Vec fields replaced by Meta item vectors.
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
/// DesignMeta is Design with only scalar fields.
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
/// DesignShallow is Design with Vec fields replaced by Meta item vectors.
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
/// KitMeta is Kit with only scalar fields.
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
/// KitShallow is Kit with Vec fields replaced by Meta item vectors.
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

// #endregion 🔖Main Entity Meta Types

// #region 🔖Meta And Shallow Conversion Functions

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

// #endregion 🔖Meta And Shallow Conversion Functions

// #endregion 🔖Meta And Shallow Types

// #region 🔖HasGuid Trait
// [👤semio📚rs💻semio🔖hasguidtrait](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait)
// HasGuid Trait MUST provide the hasguid trait functionality.

// [✂️semio/rs/semio.rs#HasGuid Trait§HasGuid](repo://definition/semio/rs/semio.rs/HAS-GUID-TRAIT/HAS-GUID)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
pub trait HasGuid {
    fn guid(&self) -> &str;
}

// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Attribute {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// HasGuid MUST perform the HasGuid operation.
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
impl HasGuid for Prop {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
impl HasGuid for Connector {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Model {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Type {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
impl HasGuid for Piece {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// HasGuid MUST perform the HasGuid operation.
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Connection {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
impl HasGuid for Layer {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Group {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Stat {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
impl HasGuid for Design {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Tag {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Concept {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Port {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Quality {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for File {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Folder {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Author {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/// </remarks>
impl HasGuid for Kit {
    fn guid(&self) -> &str {
        &self.guid
    }
}

// [✂️semio/rs/semio.rs#HasGuid Trait§DiffHasGuid](repo://definition/semio/rs/semio.rs/HAS-GUID-TRAIT/DIFF-HAS-GUID)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// </remarks>
pub trait DiffHasGuid {
    fn guid(&self) -> &str;
}

// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for AttributeDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for PropDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for ConnectorDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for ModelDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for TypeDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for PieceDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for ConnectionDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// DiffHasGuid MUST perform the DiffHasGuid operation.
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
/// <remarks>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// </remarks>
impl DiffHasGuid for LayerDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
impl DiffHasGuid for GroupDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for StatDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for DesignDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for TagDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for ConceptDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for PortDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for QualityDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for FileDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for FolderDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for AuthorDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}
// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/// </remarks>
impl DiffHasGuid for KitDiff {
    fn guid(&self) -> &str {
        &self.guid
    }
}

// #endregion 🔖HasGuid Trait

// #region 🔖ApplyDiff
// [👤semio📚rs💻semio🔖applydiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff)
// ApplyDiff MUST provide the applydiff functionality.

/// <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/// [👤semio📚rs💻semio🔖applydiff🛠️applycollectiondiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_collection_diff)
/// <remarks>
/// apply_collection_diff MUST perform the apply_collection_diff operation.
/// </remarks>
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
            let removed_set: HashSet<_> = removed_items.iter().map(|s| s.guid.clone()).collect();
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

/// apply_attribute_diff MUST perform the apply_attribute_diff operation.
// [🛠️semio/rs/semio.rs#ApplyDiff§apply_attribute_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-ATTRIBUTE-DIFF)
/// <summary>apply_attribute_diff performs the apply_attribute_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyattributediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_attribute_diff)
/// apply_attribute_diff MUST perform the apply_attribute_diff operation.
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_prop_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-PROP-DIFF)
/// <summary>apply_prop_diff holds the data fields for a apply_prop_diff record.</summary>
/// apply_prop_diff MUST perform the apply_prop_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applypropdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_prop_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_connector_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-CONNECTOR-DIFF)
/// <summary>apply_connector_diff holds the data fields for a apply_connector_diff record.</summary>
/// apply_connector_diff MUST perform the apply_connector_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyconnectordiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_connector_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_model_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-MODEL-DIFF)
/// <summary>apply_model_diff performs the apply_model_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applymodeldiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_model_diff)
/// apply_model_diff MUST perform the apply_model_diff operation.
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_type_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-TYPE-DIFF)
/// <summary>apply_type_diff holds the data fields for a apply_type_diff record.</summary>
/// apply_type_diff MUST perform the apply_type_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applytypediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_type_diff)
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

/// <summary>apply_layer_diff holds the data fields for a apply_layer_diff record.</summary>
/// [👤semio📚rs💻semio🔖applydiff🛠️applylayerdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_layer_diff)
/// <remarks>
/// apply_layer_diff MUST perform the apply_layer_diff operation.
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

/// <summary>apply_group_diff holds the data fields for a apply_group_diff record.</summary>
// [🛠️semio/rs/semio.rs#ApplyDiff§apply_group_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-GROUP-DIFF)
/// <summary>apply_group_diff performs the apply_group_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applygroupdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_group_diff)
/// apply_group_diff MUST perform the apply_group_diff operation.
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

/// <summary>apply_stat_diff holds the data fields for a apply_stat_diff record.</summary>
/// apply_stat_diff MUST perform the apply_stat_diff operation.
/// [👤semio📚rs💻semio🔖applydiff🛠️applystatdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_stat_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_piece_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-PIECE-DIFF)
/// <summary>apply_piece_diff holds the data fields for a apply_piece_diff record.</summary>
/// apply_piece_diff MUST perform the apply_piece_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applypiecediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_piece_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_connection_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-CONNECTION-DIFF)
/// <summary>apply_connection_diff holds the data fields for a apply_connection_diff record.</summary>
/// apply_connection_diff MUST perform the apply_connection_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyconnectiondiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_connection_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_design_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-DESIGN-DIFF)
/// <summary>apply_design_diff performs the apply_design_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applydesigndiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_design_diff)
/// apply_design_diff MUST perform the apply_design_diff operation.
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

/// <summary>apply_tag_diff holds the data fields for a apply_tag_diff record.</summary>
/// [👤semio📚rs💻semio🔖applydiff🛠️applytagdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_tag_diff)
/// <remarks>
/// apply_tag_diff MUST perform the apply_tag_diff operation.
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

/// <summary>apply_concept_diff holds the data fields for a apply_concept_diff record.</summary>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyconceptdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_concept_diff)
/// <remarks>
/// apply_concept_diff MUST perform the apply_concept_diff operation.
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

/// <summary>apply_interface_diff holds the data fields for a apply_interface_diff record.</summary>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyinterfacediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_interface_diff)
/// <remarks>
/// apply_interface_diff MUST perform the apply_interface_diff operation.
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

/// <summary>apply_quality_diff holds the data fields for a apply_quality_diff record.</summary>
/// apply_quality_diff MUST perform the apply_quality_diff operation.
/// [👤semio📚rs💻semio🔖applydiff🛠️applyqualitydiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_quality_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_file_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-FILE-DIFF)
/// <summary>apply_file_diff holds the data fields for a apply_file_diff record.</summary>
/// apply_file_diff MUST perform the apply_file_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyfilediff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_file_diff)
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

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_folder_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-FOLDER-DIFF)
/// <summary>apply_folder_diff performs the apply_folder_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applyfolderdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_folder_diff)
/// apply_folder_diff MUST perform the apply_folder_diff operation.
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

/// <summary>apply_author_diff holds the data fields for a apply_author_diff record.</summary>
/// apply_author_diff MUST perform the apply_author_diff operation.
/// [👤semio📚rs💻semio🔖applydiff🛠️applyauthordiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_author_diff)
pub fn apply_author_diff(item: &mut Author, diff: &AuthorDiff) {
    if let Some(value) = &diff.name {
        item.name = value.clone();
    }
    if let Some(value) = &diff.email {
        item.email = value.clone();
    }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_kit_diff](repo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-KIT-DIFF)
/// <summary>apply_kit_diff holds the data fields for a apply_kit_diff record.</summary>
/// apply_kit_diff MUST perform the apply_kit_diff operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖applydiff🛠️applykitdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_kit_diff)
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

// #endregion 🔖ApplyDiff

// #region 🔖Kit Change Helpers
// [👤semio📚rs💻semio🔖kitchangehelpers](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers)
// Kit Change Helpers MUST provide convenience functions for computing kit and design diffs, inverses, and changes.

/// Computes a CollectionDiff between two optional collections of guid-identified items.
/// Uses a caller-provided `compute_diff` function for entity-level diffs.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️getguidcollectiondiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_guid_collection_diff)
fn get_guid_collection_diff<T, D>(
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

// #region 🔖Entity Diff Functions

fn get_attribute_diff(before: &Attribute, after: &Attribute) -> AttributeDiff {
    let mut diff = AttributeDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.key != after.key {
        diff.key = Some(after.key.clone());
    }
    if before.value != after.value {
        diff.value = Some(after.value.clone());
    }
    if before.definition != after.definition {
        diff.definition = Some(after.definition.clone());
    }
    diff
}

fn get_prop_diff(before: &Prop, after: &Prop) -> PropDiff {
    let mut diff = PropDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.quality != after.quality {
        diff.quality = Some(after.quality.clone());
    }
    if before.value != after.value {
        diff.value = Some(after.value.clone());
    }
    if before.unit != after.unit {
        diff.unit = Some(after.unit.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_connector_diff(before: &Connector, after: &Connector) -> ConnectorDiff {
    let mut diff = ConnectorDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.point != after.point {
        diff.point = Some(Vector {
            x: after.point.x - before.point.x,
            y: after.point.y - before.point.y,
            z: after.point.z - before.point.z,
        });
    }
    if before.direction != after.direction {
        diff.direction = Some(Vector {
            x: after.direction.x - before.direction.x,
            y: after.direction.y - before.direction.y,
            z: after.direction.z - before.direction.z,
        });
    }
    if before.t != after.t {
        diff.t = Some(after.t);
    }
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.mandatory != after.mandatory {
        diff.mandatory = Some(after.mandatory);
    }
    if before.port != after.port {
        diff.port = Some(after.port.clone());
    }
    diff.props = get_guid_collection_diff(&before.props, &after.props, "prop", get_prop_diff);
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_model_diff(before: &Model, after: &Model) -> ModelDiff {
    let mut diff = ModelDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.file != after.file {
        diff.file = Some(after.file.clone());
    }
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.tags != after.tags {
        diff.tags = Some(after.tags.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_type_diff(before: &Type, after: &Type) -> TypeDiff {
    let mut diff = TypeDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.parent != after.parent {
        diff.parent = Some(after.parent.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    if before.image != after.image {
        diff.image = Some(after.image.clone());
    }
    if before.folder != after.folder {
        diff.folder = Some(after.folder.clone());
    }
    if before.unit != after.unit {
        diff.unit = Some(after.unit.clone());
    }
    if before.stock != after.stock {
        diff.stock = Some(after.stock);
    }
    if before.is_abstract != after.is_abstract {
        diff.is_abstract = Some(after.is_abstract);
    }
    if before.virtual_type != after.virtual_type {
        diff.virtual_type = Some(after.virtual_type);
    }
    if before.location != after.location {
        diff.location = Some(after.location.clone());
    }
    if before.concepts != after.concepts {
        diff.concepts = Some(after.concepts.clone());
    }
    if before.authors != after.authors {
        diff.authors = Some(after.authors.clone());
    }
    diff.props = get_guid_collection_diff(&before.props, &after.props, "prop", get_prop_diff);
    diff.models = get_guid_collection_diff(&before.models, &after.models, "model", get_model_diff);
    diff.connectors = get_guid_collection_diff(
        &before.connectors,
        &after.connectors,
        "connector",
        get_connector_diff,
    );
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_piece_diff(before: &Piece, after: &Piece) -> PieceDiff {
    let mut diff = PieceDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.type_ref != after.type_ref {
        diff.type_ref = Some(after.type_ref.clone());
    }
    if before.design != after.design {
        diff.design = Some(after.design.clone());
    }
    if before.plane != after.plane {
        diff.plane = Some(after.plane.clone());
    }
    if before.center != after.center {
        diff.center = Some(after.center.clone());
    }
    if before.scale != after.scale {
        diff.scale = Some(after.scale);
    }
    if before.mirror_plane != after.mirror_plane {
        diff.mirror_plane = Some(after.mirror_plane.clone());
    }
    if before.is_hidden != after.is_hidden {
        diff.is_hidden = Some(after.is_hidden);
    }
    if before.is_locked != after.is_locked {
        diff.is_locked = Some(after.is_locked);
    }
    if before.color != after.color {
        diff.color = Some(after.color.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    diff.props = get_guid_collection_diff(&before.props, &after.props, "prop", get_prop_diff);
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_connection_diff(before: &Connection, after: &Connection) -> ConnectionDiff {
    let mut diff = ConnectionDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.connected != after.connected {
        let mut sd = SideDiff::default();
        if before.connected.piece != after.connected.piece {
            sd.piece = Some(after.connected.piece.clone());
        }
        if before.connected.design_piece != after.connected.design_piece {
            sd.design_piece = Some(after.connected.design_piece.clone());
        }
        if before.connected.connector != after.connected.connector {
            sd.connector = Some(after.connected.connector.clone());
        }
        diff.connected = Some(sd);
    }
    if before.connecting != after.connecting {
        let mut sd = SideDiff::default();
        if before.connecting.piece != after.connecting.piece {
            sd.piece = Some(after.connecting.piece.clone());
        }
        if before.connecting.design_piece != after.connecting.design_piece {
            sd.design_piece = Some(after.connecting.design_piece.clone());
        }
        if before.connecting.connector != after.connecting.connector {
            sd.connector = Some(after.connecting.connector.clone());
        }
        diff.connecting = Some(sd);
    }
    if before.gap != after.gap {
        diff.gap = Some(after.gap - before.gap);
    }
    if before.shift != after.shift {
        diff.shift = Some(after.shift - before.shift);
    }
    if before.rise != after.rise {
        diff.rise = Some(after.rise - before.rise);
    }
    if before.rotation != after.rotation {
        diff.rotation = Some(after.rotation - before.rotation);
    }
    if before.turn != after.turn {
        diff.turn = Some(after.turn - before.turn);
    }
    if before.tilt != after.tilt {
        diff.tilt = Some(after.tilt - before.tilt);
    }
    if before.u != after.u {
        diff.u = Some(match (before.u, after.u) {
            (Some(b), Some(a)) => Some(a - b),
            (None, Some(a)) => Some(a),
            (_, None) => None,
        });
    }
    if before.v != after.v {
        diff.v = Some(match (before.v, after.v) {
            (Some(b), Some(a)) => Some(a - b),
            (None, Some(a)) => Some(a),
            (_, None) => None,
        });
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_layer_diff(before: &Layer, after: &Layer) -> LayerDiff {
    let mut diff = LayerDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.path != after.path {
        diff.path = Some(after.path.clone());
    }
    if before.is_hidden != after.is_hidden {
        diff.is_hidden = Some(after.is_hidden);
    }
    if before.is_locked != after.is_locked {
        diff.is_locked = Some(after.is_locked);
    }
    if before.color != after.color {
        diff.color = Some(after.color.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_group_diff(before: &Group, after: &Group) -> GroupDiff {
    let mut diff = GroupDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.color != after.color {
        diff.color = Some(after.color.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.pieces != after.pieces {
        diff.pieces = Some(after.pieces.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_stat_diff(before: &Stat, after: &Stat) -> StatDiff {
    let mut diff = StatDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.quality != after.quality {
        diff.quality = Some(after.quality.clone());
    }
    if before.min != after.min {
        diff.min = Some(after.min);
    }
    if before.min_excluded != after.min_excluded {
        diff.min_excluded = Some(after.min_excluded);
    }
    if before.max != after.max {
        diff.max = Some(after.max);
    }
    if before.max_excluded != after.max_excluded {
        diff.max_excluded = Some(after.max_excluded);
    }
    if before.unit != after.unit {
        diff.unit = Some(after.unit.clone());
    }
    diff
}

fn get_tag_diff(before: &Tag, after: &Tag) -> TagDiff {
    let mut diff = TagDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    diff
}

fn get_concept_diff(before: &Concept, after: &Concept) -> ConceptDiff {
    let mut diff = ConceptDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    diff
}

fn get_port_diff(before: &Port, after: &Port) -> PortDiff {
    let mut diff = PortDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    if before.compatible_interfaces != after.compatible_interfaces {
        diff.compatible_interfaces = Some(after.compatible_interfaces.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_quality_diff(before: &Quality, after: &Quality) -> QualityDiff {
    let mut diff = QualityDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.key != after.key {
        diff.key = Some(after.key.clone());
    }
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.kind != after.kind {
        diff.kind = Some(after.kind.clone());
    }
    if before.default_value != after.default_value {
        diff.default_value = Some(after.default_value);
    }
    if before.formula != after.formula {
        diff.formula = Some(after.formula.clone());
    }
    if before.default_si_unit != after.default_si_unit {
        diff.default_si_unit = Some(after.default_si_unit.clone());
    }
    if before.default_imperial_unit != after.default_imperial_unit {
        diff.default_imperial_unit = Some(after.default_imperial_unit.clone());
    }
    if before.min != after.min {
        diff.min = Some(after.min);
    }
    if before.is_min_excluded != after.is_min_excluded {
        diff.is_min_excluded = Some(after.is_min_excluded);
    }
    if before.max != after.max {
        diff.max = Some(after.max);
    }
    if before.is_max_excluded != after.is_max_excluded {
        diff.is_max_excluded = Some(after.is_max_excluded);
    }
    if before.can_scale != after.can_scale {
        diff.can_scale = Some(after.can_scale);
    }
    if before.uri != after.uri {
        diff.uri = Some(after.uri.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_file_diff(before: &File, after: &File) -> FileDiff {
    let mut diff = FileDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.remote != after.remote {
        diff.remote = Some(after.remote.clone());
    }
    if before.folder != after.folder {
        diff.folder = Some(after.folder.clone());
    }
    if before.size != after.size {
        diff.size = Some(after.size);
    }
    if before.hash != after.hash {
        diff.hash = Some(after.hash.clone());
    }
    diff
}

fn get_folder_diff(before: &Folder, after: &Folder) -> FolderDiff {
    let mut diff = FolderDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.parent != after.parent {
        diff.parent = Some(after.parent.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

fn get_author_diff(before: &Author, after: &Author) -> AuthorDiff {
    let mut diff = AuthorDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.email != after.email {
        diff.email = Some(after.email.clone());
    }
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

// #endregion 🔖Entity Diff Functions

/// Computes the KitDiff that transforms `before` into `after`.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️getkitdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_kit_diff)
pub fn get_kit_diff(before: &Kit, after: &Kit) -> KitDiff {
    let mut diff = KitDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.version != after.version {
        diff.version = Some(after.version.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    if before.image != after.image {
        diff.image = Some(after.image.clone());
    }
    if before.preview != after.preview {
        diff.preview = Some(after.preview.clone());
    }
    if before.remote != after.remote {
        diff.remote = Some(after.remote.clone());
    }
    if before.homepage != after.homepage {
        diff.homepage = Some(after.homepage.clone());
    }
    if before.license != after.license {
        diff.license = Some(after.license.clone());
    }
    diff.types = get_guid_collection_diff(&before.types, &after.types, "type", get_type_diff);
    diff.designs = get_guid_collection_diff(&before.designs, &after.designs, "design", |b, a| {
        get_design_diff(b, a)
    });
    diff.tags = get_guid_collection_diff(&before.tags, &after.tags, "tag", get_tag_diff);
    diff.concepts = get_guid_collection_diff(
        &before.concepts,
        &after.concepts,
        "concept",
        get_concept_diff,
    );
    diff.ports = get_guid_collection_diff(&before.ports, &after.ports, "port", get_port_diff);
    diff.qualities = get_guid_collection_diff(
        &before.qualities,
        &after.qualities,
        "quality",
        get_quality_diff,
    );
    diff.files = get_guid_collection_diff(&before.files, &after.files, "file", get_file_diff);
    diff.folders =
        get_guid_collection_diff(&before.folders, &after.folders, "folder", get_folder_diff);
    diff.authors =
        get_guid_collection_diff(&before.authors, &after.authors, "author", get_author_diff);
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

/// Computes the DesignDiff that transforms `before` into `after`.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️getdesigndiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_design_diff)
pub fn get_design_diff(before: &Design, after: &Design) -> DesignDiff {
    let mut diff = DesignDiff {
        guid: before.guid.clone(),
        ..Default::default()
    };
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.parent != after.parent {
        diff.parent = Some(after.parent.clone());
    }
    if before.description != after.description {
        diff.description = Some(after.description.clone());
    }
    if before.icon != after.icon {
        diff.icon = Some(after.icon.clone());
    }
    if before.image != after.image {
        diff.image = Some(after.image.clone());
    }
    if before.folder != after.folder {
        diff.folder = Some(after.folder.clone());
    }
    if before.unit != after.unit {
        diff.unit = Some(after.unit.clone());
    }
    if before.is_abstract != after.is_abstract {
        diff.is_abstract = Some(after.is_abstract);
    }
    if before.can_scale != after.can_scale {
        diff.can_scale = Some(after.can_scale);
    }
    if before.can_mirror != after.can_mirror {
        diff.can_mirror = Some(after.can_mirror);
    }
    if before.concepts != after.concepts {
        diff.concepts = Some(after.concepts.clone());
    }
    if before.authors != after.authors {
        diff.authors = Some(after.authors.clone());
    }
    if before.active_layer != after.active_layer {
        diff.active_layer = Some(after.active_layer.clone());
    }
    diff.props = get_guid_collection_diff(&before.props, &after.props, "prop", get_prop_diff);
    diff.pieces = get_guid_collection_diff(&before.pieces, &after.pieces, "piece", get_piece_diff);
    diff.connections = get_guid_collection_diff(
        &before.connections,
        &after.connections,
        "connection",
        get_connection_diff,
    );
    diff.layers = get_guid_collection_diff(&before.layers, &after.layers, "layer", get_layer_diff);
    diff.groups = get_guid_collection_diff(&before.groups, &after.groups, "group", get_group_diff);
    diff.stats = get_guid_collection_diff(&before.stats, &after.stats, "stat", get_stat_diff);
    diff.attributes = get_guid_collection_diff(
        &before.attributes,
        &after.attributes,
        "attribute",
        get_attribute_diff,
    );
    diff
}

/// Computes the inverse of a KitDiff given the original Kit state.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️inversekitdiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/inverse_kit_diff)
pub fn inverse_kit_diff(original: &Kit, forward: &KitDiff) -> KitDiff {
    let mut after = original.clone();
    apply_kit_diff(&mut after, forward);
    get_kit_diff(&after, original)
}

/// Computes the inverse of a DesignDiff given the original Design state.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️inversedesigndiff](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/inverse_design_diff)
pub fn inverse_design_diff(original: &Design, forward: &DesignDiff) -> DesignDiff {
    let mut after = original.clone();
    apply_design_diff(&mut after, forward);
    get_design_diff(&after, original)
}

/// Computes a reversible KitChange from two kit states.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️getkitchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_kit_change)
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

/// Computes a reversible DesignChange from two design states.
/// [👤semio📚rs💻semio🔖kitchangehelpers🛠️getdesignchange](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_design_change)
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

// #endregion 🔖Kit Change Helpers

// #region 🔖Filter
// [👤semio📚rs💻semio🔖filter](repo://p/u/semio/b/l/rs/f/semio.rs/s/Filter)
// Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

/// Glob filter with include and exclude patterns for name-based entity filtering.
/// If include is non-empty, only names matching at least one include pattern are kept.
/// Names matching any exclude pattern are always removed.
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

/// Matches a name against a glob pattern supporting * (any chars) and ? (single char). Case-insensitive.
pub fn glob_match(name: &str, pattern: &str) -> bool {
    let name_lower = name.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    let name_bytes = name_lower.as_bytes();
    let pattern_bytes = pattern_lower.as_bytes();

    fn matches(name: &[u8], pattern: &[u8]) -> bool {
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

/// Checks if a name passes a GlobFilter. Returns true if filter is None or name matches.
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

fn select_best_model_for_filter(models: &[Model], selected_tag_guids: &[String]) -> Option<Model> {
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

/// Internal design-based transitive kit filtering.
/// Removes types not used by pieces, designs not used by pieces, ports not used by connectors of used types,
/// files not used by selected models, and keeps at most one model per type according to the optional tags.
/// [👤semio📚rs💻semio🔖filter🛠️filterkitbydesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/Filter/d/i/filter_kit_by_design)
fn filter_kit_by_design(kit: &Kit, design_guid: &str, tags: Option<&[String]>) -> Kit {
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
    fn collect_type_ancestors(
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

/// General-purpose kit filter. Combines optional design-based transitive filtering with glob-based name filtering.
/// When design_guid is set, first performs transitive design-scoped subset extraction.
/// Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
/// [👤semio📚rs💻semio🔖filter🛠️filterkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Filter/d/i/filter_kit)
pub fn filter_kit(kit: &Kit, filter: &KitFilter) -> Kit {
    let base = if let Some(design_guid) = &filter.design_guid {
        filter_kit_by_design(kit, design_guid, filter.model_tags.as_deref())
    } else {
        kit.clone()
    };

    let has_glob_filters = filter.designs.is_some() || filter.types.is_some() || filter.ports.is_some() ||
        filter.files.is_some() || filter.tags.is_some() || filter.concepts.is_some() ||
        filter.qualities.is_some() || filter.authors.is_some() || filter.folders.is_some();

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
        types: Some(base.types.as_deref().unwrap_or(&[]).iter()
            .filter(|t| matches_glob_filter(&t.name, filter.types.as_ref()))
            .cloned().collect()),
        designs: Some(base.designs.as_deref().unwrap_or(&[]).iter()
            .filter(|d| matches_glob_filter(&d.name, filter.designs.as_ref()))
            .cloned().collect()),
        ports: Some(base.ports.as_deref().unwrap_or(&[]).iter()
            .filter(|p| matches_glob_filter(&p.name, filter.ports.as_ref()))
            .cloned().collect()),
        files: Some(base.files.as_deref().unwrap_or(&[]).iter()
            .filter(|f| matches_glob_filter(&f.name, filter.files.as_ref()))
            .cloned().collect()),
        tags: Some(base.tags.as_deref().unwrap_or(&[]).iter()
            .filter(|t| matches_glob_filter(&t.name, filter.tags.as_ref()))
            .cloned().collect()),
        concepts: Some(base.concepts.as_deref().unwrap_or(&[]).iter()
            .filter(|c| matches_glob_filter(&c.name, filter.concepts.as_ref()))
            .cloned().collect()),
        qualities: Some(base.qualities.as_deref().unwrap_or(&[]).iter()
            .filter(|q| matches_glob_filter(&q.name, filter.qualities.as_ref()))
            .cloned().collect()),
        folders: Some(base.folders.as_deref().unwrap_or(&[]).iter()
            .filter(|f| matches_glob_filter(&f.name, filter.folders.as_ref()))
            .cloned().collect()),
        authors: Some(base.authors.as_deref().unwrap_or(&[]).iter()
            .filter(|a| matches_glob_filter(&a.name, filter.authors.as_ref()))
            .cloned().collect()),
        attributes: base.attributes,
        created_at: base.created_at,
        updated_at: base.updated_at,
    }
}

// #endregion 🔖Filter

// #region 🔖FlattenDesign
// [👤semio📚rs💻semio🔖flattendesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign)
// FlattenDesign MUST provide the flattendesign functionality.

/// <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/// FlattenedPiece MUST perform the FlattenedPiece operation.
/// [👤semio📚rs💻semio🔖flattendesign🛠️flattenedpiece](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/FlattenedPiece)
pub struct FlattenedPiece {
    pub piece: Piece,
    pub plane: Plane,
    pub type_guid: Option<String>,
    pub design_guid: Option<String>,
    pub path: Vec<String>,
}

/// <summary>flatten_design holds the data fields for a flatten_design record.</summary>
/// [👤semio📚rs💻semio🔖flattendesign🛠️flattendesign](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/flatten_design)
/// <remarks>
/// flatten_design MUST perform the flatten_design operation.
/// </remarks>
pub fn flatten_design(kit: &Kit, design_guid: &str) -> DesignChange {
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

    let pieces_map: HashMap<&str, &Piece> = pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

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

    const RADIUS: f64 = 2.697;
    const VERTICAL_V_EXTRA: f64 = 1.0;
    const HORIZONTAL_SCALE: f64 = 3.0633;

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

                        let parent_connector =
                            match get_connector_for_side_fast(&types_map, &pieces_map, parent_side)
                            {
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

                        let (child_u, child_v) =
                            if parent_center.u.abs() < 0.0001 && parent_center.v.abs() < 0.0001 {
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
                        let parent_path = piece_paths.get(current_guid).cloned().unwrap_or_default();
                        piece_paths.insert(neighbor_guid, format!("{},{}", parent_path, neighbor_guid));
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
                    (existing.u - center.u).abs() > 0.0001 || (existing.v - center.v).abs() > 0.0001
                }
                None => true,
            };

            if plane_needs_update || center_needs_update {
                let path_attr = piece_paths.get(piece.guid.as_str()).map(|path| {
                    CollectionDiff {
                        added: Some(vec![
                            Attribute { guid: guid(), key: "semio.path".to_string(), value: Some(path.clone()), definition: None },
                        ]),
                        removed: None,
                        updated: None,
                    }
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
    let backward = inverse_design_diff(&before_design, &forward);

    DesignChange {
        forward,
        backward,
        author: None,
        time: None,
        before: Some(before_design),
        after: Some(after_design),
    }
}

// [👤semio📚rs💻semio🔖flattendesign🛠️planesequalapprox](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/planes_equal_approx)
/// <summary>planes_equal_approx holds the data fields for a planes_equal_approx record.</summary>
/// planes_equal_approx MUST perform the planes_equal_approx operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️planesequalapprox](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/planes_equal_approx)
/// </remarks>
fn planes_equal_approx(a: &Plane, b: &Plane) -> bool {
    const TOL: f64 = 0.0001;
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

// [👤semio📚rs💻semio🔖flattendesign🛠️computeconnectionmatrixfast](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_connection_matrix_fast)
/// <summary>compute_connection_matrix_fast holds the data fields for a compute_connection_matrix_fast record.</summary>
/// compute_connection_matrix_fast MUST perform the compute_connection_matrix_fast operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️computeconnectionmatrixfast](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_connection_matrix_fast)
/// </remarks>
fn compute_connection_matrix_fast(
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

// [👤semio📚rs💻semio🔖flattendesign🛠️computechildplanematrix](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_child_plane_matrix)
/// <summary>compute_child_plane_matrix holds the data fields for a compute_child_plane_matrix record.</summary>
/// compute_child_plane_matrix MUST perform the compute_child_plane_matrix operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️computechildplanematrix](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_child_plane_matrix)
/// </remarks>
fn compute_child_plane_matrix(
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

    let rotate_quat =
        UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(parent_dir), -rotation_rad);
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

// [👤semio📚rs💻semio🔖flattendesign🛠️quattomatrix4](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/quat_to_matrix4)
/// <summary>quat_to_matrix4 holds the data fields for a quat_to_matrix4 record.</summary>
/// quat_to_matrix4 MUST perform the quat_to_matrix4 operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️quattomatrix4](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/quat_to_matrix4)
/// </remarks>
fn quat_to_matrix4(q: &nalgebra::UnitQuaternion<f64>) -> Matrix4<f64> {
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

// [👤semio📚rs💻semio🔖flattendesign🛠️maketranslation](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/make_translation)
/// <summary>make_translation holds the data fields for a make_translation record.</summary>
/// make_translation MUST perform the make_translation operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️maketranslation](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/make_translation)
/// </remarks>
fn make_translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    )
}

// [👤semio📚rs💻semio🔖flattendesign🛠️applymatrix4tovec3](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/apply_matrix4_to_vec3)
/// <summary>apply_matrix4_to_vec3 holds the data fields for a apply_matrix4_to_vec3 record.</summary>
/// apply_matrix4_to_vec3 MUST perform the apply_matrix4_to_vec3 operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️applymatrix4tovec3](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/apply_matrix4_to_vec3)
/// </remarks>
fn apply_matrix4_to_vec3(m: &Matrix4<f64>, v: &nalgebra::Vector3<f64>) -> nalgebra::Vector3<f64> {
    nalgebra::Vector3::new(
        m[(0, 0)] * v.x + m[(0, 1)] * v.y + m[(0, 2)] * v.z,
        m[(1, 0)] * v.x + m[(1, 1)] * v.y + m[(1, 2)] * v.z,
        m[(2, 0)] * v.x + m[(2, 1)] * v.y + m[(2, 2)] * v.z,
    )
}

// [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorforsidefast](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_for_side_fast)
/// <summary>get_connector_for_side_fast holds the data fields for a get_connector_for_side_fast record.</summary>
/// get_connector_for_side_fast MUST perform the get_connector_for_side_fast operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorforsidefast](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_for_side_fast)
/// </remarks>
fn get_connector_for_side_fast<'a>(
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

// [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorfromtype](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_from_type)
/// <summary>get_connector_from_type holds the data fields for a get_connector_from_type record.</summary>
/// get_connector_from_type MUST perform the get_connector_from_type operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorfromtype](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_from_type)
/// </remarks>
fn get_connector_from_type<'a>(
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
                    if let Some(c) = get_connector_from_type(types_map, parent, connector_guid) {
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

// [👤semio📚rs💻semio🔖flattendesign🛠️connectortoplane](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/connector_to_plane)
/// <summary>connector_to_plane holds the data fields for a connector_to_plane record.</summary>
/// connector_to_plane MUST perform the connector_to_plane operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖flattendesign🛠️connectortoplane](repo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/connector_to_plane)
/// </remarks>
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

pub fn drag_pieces_in_design(
    design_pieces: &[Piece],
    design_connections: &[Connection],
    selected_pieces: &[Piece],
    offset: &Coord,
) -> DesignDiff {
    let selected_guids: HashSet<&str> = selected_pieces.iter().map(|p| p.guid.as_str()).collect();
    let design_piece_map: HashMap<&str, &Piece> =
        design_pieces.iter().map(|p| (p.guid.as_str(), p)).collect();
    let mut children_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut connection_by_child: HashMap<&str, &Connection> = HashMap::new();
    for conn in design_connections {
        let connecting_guid = conn.connecting.piece.guid.as_str();
        let connected_guid = conn.connected.piece.guid.as_str();
        children_map
            .entry(connecting_guid)
            .or_default()
            .push(connected_guid);
        connection_by_child.insert(connected_guid, conn);
    }
    let root_movers: Vec<&str> = selected_guids
        .iter()
        .filter(|&&guid| {
            design_piece_map
                .get(guid)
                .map_or(false, |p| p.center.is_some())
        })
        .copied()
        .collect();
    let mut moving_set: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = root_movers.iter().copied().collect();
    while let Some(current) = queue.pop_front() {
        if !moving_set.insert(current) {
            continue;
        }
        if let Some(children) = children_map.get(current) {
            for &child in children {
                if !moving_set.contains(child) {
                    queue.push_back(child);
                }
            }
        }
    }
    let piece_updates: Vec<DiffUpdate<PieceDiff>> = root_movers
        .iter()
        .map(|&guid| DiffUpdate {
            key: "piece".to_string(),
            guid: guid.to_string(),
            diff: PieceDiff {
                guid: guid.to_string(),
                center: Some(Some(Coord {
                    u: offset.u,
                    v: offset.v,
                })),
                ..Default::default()
            },
        })
        .collect();
    let connection_updates: Vec<DiffUpdate<ConnectionDiff>> = selected_guids
        .iter()
        .filter(|&&guid| !moving_set.contains(guid) && connection_by_child.contains_key(guid))
        .map(|&guid| {
            let conn = connection_by_child[guid];
            DiffUpdate {
                key: "connection".to_string(),
                guid: conn.guid.clone(),
                diff: ConnectionDiff {
                    guid: conn.guid.clone(),
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

// #endregion 🔖FlattenDesign

// #region 🔖Kit Model Export
// [👤semio📚rs💻semio🔖kitmodelexport](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export)
// Kit Model Export MUST provide GLB/glTF export of a design's assembled 3D model.

/// <summary>Supported 3D model export formats (extension, description).</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️exportmodelformats](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/EXPORT_MODEL_FORMATS)
pub const EXPORT_MODEL_FORMATS: &[(&str, &str)] = &[
    ("glb", "GLB Binary (glTF 2.0)"),
    ("gltf", "glTF JSON (glTF 2.0)"),
];

/// <summary>Decodes a data URI blob (data:mime;base64,...) into raw bytes.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️decodedatauriblob](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/decode_data_uri_blob)
#[cfg(not(target_arch = "wasm32"))]
fn decode_data_uri_blob(blob: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    let b64 = if let Some(pos) = blob.find(";base64,") {
        &blob[pos + 8..]
    } else {
        blob
    };
    base64::engine::general_purpose::STANDARD.decode(b64).ok()
}

/// <summary>Parses a GLB binary into its JSON chunk and BIN chunk.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️parseglb](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/parse_glb)
#[cfg(not(target_arch = "wasm32"))]
fn parse_glb(data: &[u8]) -> Option<(serde_json::Value, Vec<u8>)> {
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

/// <summary>Builds a GLB binary from a glTF JSON value and binary buffer.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️buildglb](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/build_glb)
#[cfg(not(target_arch = "wasm32"))]
fn build_glb(json: &serde_json::Value, bin: &[u8]) -> Vec<u8> {
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

/// <summary>Converts a nalgebra Matrix4 to glTF column-major array of 16 f64.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️matrix4togltfcolumnmajor](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/matrix4_to_gltf_column_major)
fn matrix4_to_gltf_column_major(m: &Matrix4<f64>) -> [f64; 16] {
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

fn semio_matrix_to_gltf_matrix(matrix: &Matrix4<f64>) -> Matrix4<f64> {
    let basis = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let basis_inv = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    basis * matrix * basis_inv
}

/// <summary>Preserves source mesh geometry while clearing unresolved material links.</summary>
fn strip_mesh_material_references(mesh: &mut serde_json::Value) {
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

/// <summary>Assigns a human-readable source file name to a merged mesh.</summary>
fn set_mesh_name(mesh: &mut serde_json::Value, mesh_name: &str) {
    mesh["name"] = serde_json::json!(mesh_name);
}

fn matrix4_to_gltf_column_major_legacy(m: &Matrix4<f64>) -> [f64; 16] {
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

/// <summary>Writes a box placeholder mesh into the combined GLB buffers and returns the mesh index.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️appendboxmesh](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/append_box_mesh)
#[cfg(not(target_arch = "wasm32"))]
fn append_box_mesh(
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

/// <summary>Selects the best model for a type given desired tag guids.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️selectmodelfortype](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/select_model_for_type)
fn select_model_for_type<'a>(t: &'a Type, tags: &[String]) -> Option<&'a Model> {
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

/// <summary>Merges a source GLB's mesh data into the combined GLB builder.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️mergeglbmesh](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/merge_glb_mesh)
#[cfg(not(target_arch = "wasm32"))]
fn merge_glb_mesh(
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
                    if let Some(idx_val) = new_primitive.get("indices").and_then(|v| v.as_u64()) {
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

/// <summary>Exports the 3D model of a design to GLB or glTF format.</summary>
/// [👤semio📚rs💻semio🔖kitmodelexport🛠️exportdesignmodel](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/export_design_model)
/// <remarks>
/// export_design_model MUST assemble all piece instances with their world transforms,
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

    let pieces_map: HashMap<&str, &Piece> = pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

    let files_map: HashMap<&str, &File> = kit
        .files
        .as_ref()
        .map(|files| files.iter().map(|f| (f.guid.as_str(), f)).collect())
        .unwrap_or_default();

    // #region 🔖BFS World Transforms
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
    // #endregion 🔖BFS World Transforms

    // #region 🔖Mesh Assembly
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
                                    && u32::from_le_bytes([data[0], data[1], data[2], data[3]])
                                        == 0x46546C67);
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
    // #endregion 🔖Mesh Assembly

    // #region 🔖Scene Graph
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
    // #endregion 🔖Scene Graph

    // #region 🔖Output
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
    // #endregion 🔖Output
}

// #endregion 🔖Kit Model Export

// #region 🔖Geometric Insights
// [👤semio📚rs💻semio🔖geometricinsights](repo://p/u/semio/b/l/rs/f/semio.rs/s/Geometric%20Insights)
// Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

/// Geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
/// [👤semio📚rs💻semio🔖geometricinsights🪨geometricinsights](repo://p/u/semio/b/l/rs/f/semio.rs/s/Geometric%20Insights/d/i/GeometricInsights)
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
fn read_gltf_mesh_data(
    json: &serde_json::Value,
    bin: &[u8],
) -> Option<(Vec<[f32; 3]>, Vec<u32>, [f32; 3], [f32; 3])> {
    let accessors = json.get("accessors")?.as_array()?;
    let buffer_views = json.get("bufferViews")?.as_array()?;
    let gltf_int = |v: Option<&serde_json::Value>| v.and_then(|x| x.as_u64()).unwrap_or(0) as usize;
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
/// Model MUST be path (&str) or raw bytes (&[u8]). Uses parse_glb for GLB; JSON+buffer for GLTF.
/// [👤semio📚rs💻semio🔖geometricinsights🛠️getgeometricinsightsformodel](repo://p/u/semio/b/l/rs/f/semio.rs/s/Geometric%20Insights/d/i/get_geometric_insights_for_model)
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

    let (positions, indices, _pos_min, _pos_max) =
        read_gltf_mesh_data(&json, &bin).ok_or_else(|| SemioError::InvalidOperation {
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
        area +=
            0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() as f64;
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

/// Loads model from path and returns geometric insights. Model MUST be .glb or .gltf.
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

// #endregion 🔖Geometric Insights

// #region 🔖Validation Types
// [👤semio📚rs💻semio🔖validationtypes](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types)
// Validation Types MUST provide the validation types functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
/// ValidationProblem MUST perform the ValidationProblem operation.
// [🛠️semio/rs/semio.rs#Validation Types§ValidationProblem](repo://definition/semio/rs/semio.rs/VALIDATION-TYPES/VALIDATION-PROBLEM)
/// <summary>ValidationProblem holds the data fields for a ValidationProblem record.</summary>
/// <remarks>
/// ValidationProblem MUST perform the ValidationProblem operation.
/// </remarks>
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️validationproblem](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationProblem)
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
// [🛠️semio/rs/semio.rs#Validation Types§ValidationFix](repo://definition/semio/rs/semio.rs/VALIDATION-TYPES/VALIDATION-FIX)
/// <summary>ValidationFix holds the data fields for a ValidationFix record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️validationfix](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationFix)
/// ValidationFix MUST perform the ValidationFix operation.
/// </remarks>
pub struct ValidationFix {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<KitDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>ValidationResult holds the data fields for a ValidationResult record.</summary>
/// [👤semio📚rs💻semio🔖validationtypes🛠️validationresult](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationResult)
/// <remarks>
/// ValidationResult MUST perform the ValidationResult operation.
/// </remarks>
pub struct ValidationResult {
    pub problems: Vec<ValidationProblem>,
}

/// <summary>validate_kit holds the data fields for a validate_kit record.</summary>
/// validate_kit MUST perform the validate_kit operation.
/// [👤semio📚rs💻semio🔖validationtypes🛠️validatekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/validate_kit)
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

    ValidationResult { problems }
}

// [👤semio📚rs💻semio🔖validationtypes🛠️checkguiduniquenessconstraint](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid_uniqueness_constraint)
/// <summary>check_guid_uniqueness_constraint holds the data fields for a check_guid_uniqueness_constraint record.</summary>
/// check_guid_uniqueness_constraint MUST perform the check_guid_uniqueness_constraint operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkguiduniquenessconstraint](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid_uniqueness_constraint)
/// </remarks>
fn check_guid_uniqueness_constraint(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid)
/// <summary>check_guid holds the data fields for a check_guid record.</summary>
/// check_guid MUST perform the check_guid operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkguid](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid)
/// </remarks>
fn check_guid(
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checktypenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_type_name_uniqueness)
/// <summary>check_type_name_uniqueness holds the data fields for a check_type_name_uniqueness record.</summary>
/// check_type_name_uniqueness MUST perform the check_type_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checktypenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_type_name_uniqueness)
/// </remarks>
fn check_type_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkdesignnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_design_name_uniqueness)
/// <summary>check_design_name_uniqueness holds the data fields for a check_design_name_uniqueness record.</summary>
/// check_design_name_uniqueness MUST perform the check_design_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkdesignnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_design_name_uniqueness)
/// </remarks>
fn check_design_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkpiecenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_piece_name_uniqueness)
/// <summary>check_piece_name_uniqueness holds the data fields for a check_piece_name_uniqueness record.</summary>
/// check_piece_name_uniqueness MUST perform the check_piece_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkpiecenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_piece_name_uniqueness)
/// </remarks>
fn check_piece_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectionnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connection_name_uniqueness)
/// <summary>check_connection_name_uniqueness holds the data fields for a check_connection_name_uniqueness record.</summary>
/// check_connection_name_uniqueness MUST perform the check_connection_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectionnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connection_name_uniqueness)
/// </remarks>
fn check_connection_name_uniqueness(_kit: &Kit, _problems: &mut Vec<ValidationProblem>) {}

// [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectornameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connector_name_uniqueness)
/// <summary>check_connector_name_uniqueness holds the data fields for a check_connector_name_uniqueness record.</summary>
/// check_connector_name_uniqueness MUST perform the check_connector_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectornameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connector_name_uniqueness)
/// </remarks>
fn check_connector_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkmodelnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_model_name_uniqueness)
/// <summary>check_model_name_uniqueness holds the data fields for a check_model_name_uniqueness record.</summary>
/// check_model_name_uniqueness MUST perform the check_model_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkmodelnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_model_name_uniqueness)
/// </remarks>
fn check_model_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checklayerpathuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_layer_path_uniqueness)
/// <summary>check_layer_path_uniqueness holds the data fields for a check_layer_path_uniqueness record.</summary>
/// check_layer_path_uniqueness MUST perform the check_layer_path_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checklayerpathuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_layer_path_uniqueness)
/// </remarks>
fn check_layer_path_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkqualitynameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_quality_name_uniqueness)
/// <summary>check_quality_name_uniqueness holds the data fields for a check_quality_name_uniqueness record.</summary>
/// check_quality_name_uniqueness MUST perform the check_quality_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkqualitynameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_quality_name_uniqueness)
/// </remarks>
fn check_quality_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkportnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_port_name_uniqueness)
/// <summary>check_port_name_uniqueness holds the data fields for a check_port_name_uniqueness record.</summary>
/// check_port_name_uniqueness MUST perform the check_port_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkportnameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_port_name_uniqueness)
/// </remarks>
fn check_port_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkfilenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_file_name_uniqueness)
/// <summary>check_file_name_uniqueness holds the data fields for a check_file_name_uniqueness record.</summary>
/// check_file_name_uniqueness MUST perform the check_file_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkfilenameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_file_name_uniqueness)
/// </remarks>
fn check_file_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// [👤semio📚rs💻semio🔖validationtypes🛠️checkfoldernameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_folder_name_uniqueness)
/// <summary>check_folder_name_uniqueness holds the data fields for a check_folder_name_uniqueness record.</summary>
/// check_folder_name_uniqueness MUST perform the check_folder_name_uniqueness operation.
/// <remarks>
/// [👤semio📚rs💻semio🔖validationtypes🛠️checkfoldernameuniqueness](repo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_folder_name_uniqueness)
/// </remarks>
fn check_folder_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
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

// #endregion 🔖Validation Types

// #region 🔖SQLite Import/Export
// [👤semio📚rs💻semio🔖sqliteimport🔖export](repo://p/u/semio/b/l/rs/f/semio.rs/s/SQLite%20Import/Export)
// SQLite Import/Export MUST provide the sqlite import/export functionality.

#[cfg(not(target_arch = "wasm32"))]
/// <summary>sqlite holds the data fields for a sqlite record.</summary>
/// [👤semio📚rs💻semio🔖sqliteimport🔖export🛠️sqlite](repo://p/u/semio/b/l/rs/f/semio.rs/s/SQLite%20Import/Export/d/i/sqlite)
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

    fn load_tags(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Tag>> {
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

    fn load_concepts(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Concept>> {
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

    fn load_ports(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Port>> {
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

    fn load_folders(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Folder>> {
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

    fn load_files(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<File>> {
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

    fn load_authors(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Author>> {
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

    fn load_types(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Type>> {
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

        let type_data: Vec<_> = rows
            .collect::<std::result::Result<Vec<_>, _>>()
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

    fn load_connectors(conn: &rusqlite::Connection, type_guid: &str) -> Result<Vec<Connector>> {
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

    fn load_models(conn: &rusqlite::Connection, type_guid: &str) -> Result<Vec<Model>> {
        let mut stmt = conn
            .prepare("SELECT guid, file_guid, name, description FROM model WHERE type_guid = ?1")
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

    fn load_designs(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Design>> {
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

    fn load_connections(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Connection>> {
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

    fn load_layers(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Layer>> {
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

    fn load_groups(conn: &rusqlite::Connection, design_guid: &str) -> Result<Vec<Group>> {
        let mut stmt = conn
            .prepare("SELECT guid, name, color, description FROM \"group\" WHERE design_guid = ?1")
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

// #endregion 🔖SQLite Import/Export

// #region 🔖Zip Import/Export
// [👤semio📚rs💻semio🔖zipimport🔖export](repo://p/u/semio/b/l/rs/f/semio.rs/s/Zip%20Import/Export)
// Zip Import/Export MUST provide the zip import/export functionality.

#[cfg(not(target_arch = "wasm32"))]
/// <summary>zip_roundtrip holds the data fields for a zip_roundtrip record.</summary>
/// [👤semio📚rs💻semio🔖zipimport🔖export🛠️ziproundtrip](repo://p/u/semio/b/l/rs/f/semio.rs/s/Zip%20Import/Export/d/i/zip_roundtrip)
fn mime_from_filename(filename: &str) -> &'static str {
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

// #endregion 🔖Zip Import/Export

// #region 🔖Kit Workflow
// [👤semio📚rs💻semio🔖kitworkflow](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow)
// Kit Workflow MUST provide cohesive file, folder, archive, remote, and temporary kit operations.

#[cfg(not(target_arch = "wasm32"))]
const KIT_FOLDER_METADATA_DIRECTORY: &str = ".semio";

#[cfg(not(target_arch = "wasm32"))]
const KIT_FOLDER_DATABASE_FILENAME: &str = "kit.db";

#[cfg(not(target_arch = "wasm32"))]
fn io_semio_error(context: &str, error: impl std::fmt::Display) -> SemioError {
    SemioError::Database {
        message: format!("{}: {}", context, error),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn build_folder_database_path(folder_path: &str) -> std::path::PathBuf {
    std::path::Path::new(folder_path)
        .join(KIT_FOLDER_METADATA_DIRECTORY)
        .join(KIT_FOLDER_DATABASE_FILENAME)
}

#[cfg(not(target_arch = "wasm32"))]
fn decode_blob_bytes(file: &File) -> Result<Option<Vec<u8>>> {
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
fn hydrate_kit_file_blobs(kit: &mut Kit, files: &HashMap<String, Vec<u8>>) {
    let file_paths: Vec<String> = kit
        .files
        .as_ref()
        .map(|kit_files| kit_files.iter().map(|file| zip_roundtrip::build_file_path(kit, file)).collect())
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
fn resolve_exported_file_bytes(
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
fn remap_kit_file_bytes(
    before_kit: &Kit,
    after_kit: &Kit,
    before_files: &HashMap<String, Vec<u8>>,
) -> Result<HashMap<String, Vec<u8>>> {
    let mut before_paths_by_guid = HashMap::new();
    if let Some(before_files_meta) = &before_kit.files {
        for file in before_files_meta {
            before_paths_by_guid.insert(file.guid.clone(), zip_roundtrip::build_file_path(before_kit, file));
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
fn remove_stale_folder_assets(
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
            std::fs::remove_file(&asset_path)
                .map_err(|error| io_semio_error("Failed to remove stale folder asset", error))?;
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
                        "Failed to remove empty folder asset directory",
                        error,
                    ))
                }
            }
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn import_kit_from_zip_bytes(zip_bytes: &[u8]) -> Result<zip_roundtrip::KitImportResult> {
    use std::io::Write;

    let mut temp_file = tempfile::NamedTempFile::new()
        .map_err(|error| io_semio_error("Failed to create temporary zip file", error))?;
    temp_file
        .write_all(zip_bytes)
        .map_err(|error| io_semio_error("Failed to write temporary zip file", error))?;

    let temp_path = temp_file.path().to_str().ok_or(SemioError::InvalidOperation {
        message: "Temporary zip path is not valid UTF-8".to_string(),
    })?;

    zip_roundtrip::import_kit_from_zip(temp_path)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_zip_payload(url: &str, content_type: Option<&str>, bytes: &[u8]) -> bool {
    let lower_url = url.to_lowercase();
    lower_url.ends_with(".zip")
        || content_type
            .map(|value| value.to_ascii_lowercase().contains("zip"))
            .unwrap_or(false)
        || bytes.starts_with(b"PK\x03\x04")
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>import_file_kit holds the data fields for a import_file_kit record.</summary>
/// import_file_kit MUST import a JSON file kit using deserialize_kit.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️importfilekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/import_file_kit)
pub fn import_file_kit(path: &str) -> Result<Kit> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| io_semio_error("Failed to read kit file", error))?;
    deserialize_kit(&content)
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>export_file_kit holds the data fields for a export_file_kit record.</summary>
/// export_file_kit MUST export a JSON file kit using serialize_kit.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️exportfilekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/export_file_kit)
pub fn export_file_kit(kit: &Kit, path: &str) -> Result<()> {
    let json = serialize_kit(kit)?;
    std::fs::write(path, json).map_err(|error| io_semio_error("Failed to write kit file", error))
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>import_folder_kit holds the data fields for a import_folder_kit record.</summary>
/// import_folder_kit MUST import a folder kit using sqlite metadata and filesystem asset files.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️importfolderkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/import_folder_kit)
pub fn import_folder_kit(folder_path: &str) -> Result<zip_roundtrip::KitImportResult> {
    let database_path = build_folder_database_path(folder_path);
    let database_path_str = database_path.to_str().ok_or(SemioError::InvalidOperation {
        message: "Folder kit database path is not valid UTF-8".to_string(),
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
                        "Failed to read folder kit asset file",
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
/// <summary>export_folder_kit holds the data fields for a export_folder_kit record.</summary>
/// export_folder_kit MUST export a folder kit using sqlite metadata and filesystem asset files.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️exportfolderkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/export_folder_kit)
pub fn export_folder_kit(
    kit: &Kit,
    files: &HashMap<String, Vec<u8>>,
    folder_path: &str,
) -> Result<()> {
    let root_path = std::path::Path::new(folder_path);
    std::fs::create_dir_all(root_path)
        .map_err(|error| io_semio_error("Failed to create folder kit root", error))?;

    let metadata_path = root_path.join(KIT_FOLDER_METADATA_DIRECTORY);
    std::fs::create_dir_all(&metadata_path)
        .map_err(|error| io_semio_error("Failed to create folder kit metadata directory", error))?;

    let database_path = metadata_path.join(KIT_FOLDER_DATABASE_FILENAME);
    if database_path.exists() {
        std::fs::remove_file(&database_path)
            .map_err(|error| io_semio_error("Failed to replace existing folder kit database", error))?;
    }
    let database_path_str = database_path.to_str().ok_or(SemioError::InvalidOperation {
        message: "Folder kit database path is not valid UTF-8".to_string(),
    })?;
    sqlite::export_kit_to_sqlite(kit, database_path_str)?;

    let resolved_files = resolve_exported_file_bytes(kit, files)?;
    for (relative_path, data) in resolved_files {
        let asset_path = root_path.join(&relative_path);
        if let Some(parent) = asset_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| io_semio_error("Failed to create folder kit asset directory", error))?;
        }
        std::fs::write(&asset_path, data)
            .map_err(|error| io_semio_error("Failed to write folder kit asset file", error))?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>import_remote_kit holds the data fields for a import_remote_kit record.</summary>
/// import_remote_kit MUST import remote JSON kits and remote ZIP kits over HTTP(S).
/// [👤semio📚rs💻semio🔖kitworkflow🛠️importremotekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/import_remote_kit)
pub fn import_remote_kit(url: &str) -> Result<zip_roundtrip::KitImportResult> {
    let response = reqwest::blocking::get(url).map_err(|error| SemioError::Database {
        message: format!("Failed to fetch remote kit {}: {}", url, error),
    })?;
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());
    let response = response.error_for_status().map_err(|error| SemioError::Database {
        message: format!("Remote kit request failed for {}: {}", url, error),
    })?;
    let bytes = response.bytes().map_err(|error| SemioError::Database {
        message: format!("Failed to read remote kit body from {}: {}", url, error),
    })?;

    if is_zip_payload(url, content_type.as_deref(), bytes.as_ref()) {
        return import_kit_from_zip_bytes(bytes.as_ref());
    }

    let json = String::from_utf8(bytes.to_vec()).map_err(|error| SemioError::Serialization {
        message: format!("Failed to decode remote kit JSON from {}: {}", url, error),
    })?;
    let kit = deserialize_kit(&json)?;
    Ok(zip_roundtrip::KitImportResult {
        kit,
        files: HashMap::new(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>edit_temporary_kit holds the data fields for a edit_temporary_kit record.</summary>
/// edit_temporary_kit MUST clone a kit and apply a diff in memory.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️edittemporarykit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/edit_temporary_kit)
pub fn edit_temporary_kit(kit: &Kit, diff: &KitDiff) -> Kit {
    let mut edited = kit.clone();
    apply_kit_diff(&mut edited, diff);
    edited
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>edit_file_kit holds the data fields for a edit_file_kit record.</summary>
/// edit_file_kit MUST import, edit, and persist a JSON file kit in place.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️editfilekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/edit_file_kit)
pub fn edit_file_kit(path: &str, diff: &KitDiff) -> Result<Kit> {
    let kit = import_file_kit(path)?;
    let edited = edit_temporary_kit(&kit, diff);
    export_file_kit(&edited, path)?;
    Ok(edited)
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>edit_folder_kit holds the data fields for a edit_folder_kit record.</summary>
/// edit_folder_kit MUST import, edit, and persist a folder kit in place.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️editfolderkit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/edit_folder_kit)
pub fn edit_folder_kit(folder_path: &str, diff: &KitDiff) -> Result<Kit> {
    let imported = import_folder_kit(folder_path)?;
    let edited = edit_temporary_kit(&imported.kit, diff);
    let edited_files = remap_kit_file_bytes(&imported.kit, &edited, &imported.files)?;
    remove_stale_folder_assets(folder_path, &imported.files, &edited_files)?;
    export_folder_kit(&edited, &edited_files, folder_path)?;
    Ok(edited)
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>edit_archive_kit holds the data fields for a edit_archive_kit record.</summary>
/// edit_archive_kit MUST import, edit, and persist a ZIP archive kit in place.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️editarchivekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/edit_archive_kit)
pub fn edit_archive_kit(path: &str, diff: &KitDiff) -> Result<Kit> {
    let imported = zip_roundtrip::import_kit_from_zip(path)?;
    let edited = edit_temporary_kit(&imported.kit, diff);
    let edited_files = remap_kit_file_bytes(&imported.kit, &edited, &imported.files)?;
    zip_roundtrip::export_kit_to_zip(&edited, &edited_files, path)?;
    Ok(edited)
}

#[cfg(not(target_arch = "wasm32"))]
/// <summary>edit_remote_kit holds the data fields for a edit_remote_kit record.</summary>
/// edit_remote_kit MUST import a remote kit and apply a temporary diff without persisting remotely.
/// [👤semio📚rs💻semio🔖kitworkflow🛠️editremotekit](repo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Workflow/d/i/edit_remote_kit)
pub fn edit_remote_kit(url: &str, diff: &KitDiff) -> Result<Kit> {
    let imported = import_remote_kit(url)?;
    Ok(edit_temporary_kit(&imported.kit, diff))
}

// #endregion 🔖Kit Workflow

// #region 🔖WASM Bindings
// [👤semio📚rs💻semio🔖wasmbindings](repo://p/u/semio/b/l/rs/f/semio.rs/s/WASM%20Bindings)
// WASM Bindings MUST provide the wasm bindings functionality.

#[cfg(target_arch = "wasm32")]
/// <summary>wasm holds the data fields for a wasm record.</summary>
/// [👤semio📚rs💻semio🔖wasmbindings🛠️wasm](repo://p/u/semio/b/l/rs/f/semio.rs/s/WASM%20Bindings/d/i/wasm)
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

    fn to_js_value<T: Serialize>(result: WasmResult<T>) -> JsValue {
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
                let change = flatten_design(&kit, design_guid);
                to_js_value(WasmResult::success(change))
            }
            Err(e) => to_js_value(WasmResult::<DesignChange>::failure(e.to_string())),
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
            Ok(existing) => to_js_value(WasmResult::success(generate_unique_name(base, &existing))),
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

// #endregion 🔖WASM Bindings

// #region 🔖Tests
// [👤semio📚rs💻semio🔖tests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests)
// Tests MUST provide the tests functionality.

#[cfg(test)]
// [👤semio📚rs💻semio🔖tests🛠️tests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/d/i/tests)
/// <summary>tests holds the data fields for a tests record.</summary>
/// <remarks>
/// [👤semio📚rs💻semio🔖tests🛠️tests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/d/i/tests)
/// </remarks>
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    const ASSETS_DIR: &str = "../assets/semio";
    const TOLERANCE: f64 = 0.001;

    fn load_kit(filename: &str) -> Kit {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize kit")
    }

    fn load_kit_diff(filename: &str) -> KitDiff {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize kit diff")
    }

    fn load_validation_result(filename: &str) -> ValidationResult {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize validation result")
    }

    fn float_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TOLERANCE
    }

    fn vectors_equal(v1: &Vector, v2: &Vector) -> bool {
        float_eq(v1.x, v2.x) && float_eq(v1.y, v2.y) && float_eq(v1.z, v2.z)
    }

    fn planes_equal(p1: &Plane, p2: &Plane) -> bool {
        vectors_equal(&p1.origin, &p2.origin)
            && vectors_equal(&p1.x_axis, &p2.x_axis)
            && vectors_equal(&p1.y_axis, &p2.y_axis)
    }

    fn centers_equal(c1: Option<&Coord>, c2: Option<&Coord>) -> bool {
        match (c1, c2) {
            (None, None) => true,
            (Some(a), Some(b)) => float_eq(a.u, b.u) && float_eq(a.v, b.v),
            _ => false,
        }
    }

    fn find_design_by_name<'a>(
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

    fn find_piece_by_name<'a>(pieces: &'a [Piece], name: &str) -> Option<&'a Piece> {
        pieces.iter().find(|p| p.name.as_deref() == Some(name))
    }

    fn test_flatten_design(kit: &Kit, design_path: &[&str]) {
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

        let flat_design_change = flatten_design(kit, &design.guid);
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
    struct ModelSelectionAsset {
        cases: Vec<ModelSelectionCase>,
    }

    #[derive(Debug, Deserialize)]
    struct ModelSelectionCase {
        name: String,
        #[serde(rename = "selectedTagGuids")]
        selected_tag_guids: Vec<String>,
        #[serde(rename = "expectedGuid")]
        expected_guid: Option<String>,
        models: Vec<ModelSelectionModel>,
    }

    #[derive(Debug, Deserialize)]
    struct ModelSelectionModel {
        guid: String,
        #[serde(rename = "fileGuid")]
        file_guid: String,
        #[serde(rename = "tagGuids")]
        tag_guids: Vec<String>,
    }

    fn contains_all_tags(model: &Model, selected_tag_guids: &[String]) -> bool {
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

    fn jaccard_tag_guids(model_tag_guids: &[String], selected_tag_guids: &[String]) -> f64 {
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

    fn select_best_model_like_semio_ts(
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

    // #region 🔖Roundtrip Tests
    // [👤semio📚rs💻semio🔖tests🔖roundtriptests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Roundtrip%20Tests)
    // Roundtrip Tests MUST provide the roundtrip tests functionality.

    mod roundtrip {
        use super::*;

        #[test]
        fn metabolism() {
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
            crate::zip_roundtrip::export_kit_to_zip(&kit, &files, roundtrip_path_str).unwrap();

            let result = crate::zip_roundtrip::import_kit_from_zip(roundtrip_path_str).unwrap();
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

    // #endregion 🔖Roundtrip Tests

    // #region 🔖DesignModel Tests
    // [👤semio📚rs💻semio🔖tests🔖designmodeltests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/DesignModel%20Tests)
    // DesignModel Tests MUST provide model-selection regression checks.

    mod design_model {
        use super::*;

        #[test]
        fn model_selection_from_shared_semio_assets() {
            let path = Path::new(ASSETS_DIR).join("model.selection.semio.json");
            let data = fs::read_to_string(&path).expect("Failed to read model.selection.semio.json");
            let payload: ModelSelectionAsset =
                serde_json::from_str(&data).expect("Failed to deserialize model.selection.semio.json");

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

                let selected = select_best_model_like_semio_ts(&models, &case.selected_tag_guids);
                let selected_guid = selected.map(|model| model.guid);
                assert_eq!(
                    selected_guid, case.expected_guid,
                    "Case {} failed",
                    case.name
                );
            }
        }
    }

    // #endregion 🔖DesignModel Tests

    // #region 🔖KitFilter Tests
    // [👤semio📚rs💻semio🔖tests🔖kitfiltertests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/KitFilter%20Tests)
    // KitFilter Tests MUST verify design-scoped kit extraction.

    mod kit_filter {
        use super::*;

        #[test]
        fn nakagin_capsule_tower_filter_produces_expected_subset() {
            let kit = load_kit("metabolism.kit.semio.json");
            let expected = load_kit("nakagin-capsule-tower.filtered.kit.semio.json");
            let design = kit
                .designs
                .as_ref()
                .and_then(|designs| {
                    designs.iter().find(|design| {
                        design.name == "Nakagin Capsule Tower" && design.parent.is_none()
                    })
                })
                .expect("Nakagin Capsule Tower design not found");

            let filtered = filter_kit(&kit, &KitFilter { design_guid: Some(design.guid.clone()), ..Default::default() });

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
                            .map(|files| files.iter().any(|file| file.guid == model.file.guid))
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
        fn nakagin_capsule_tower_filter_preserves_metadata() {
            let kit = load_kit("metabolism.kit.semio.json");
            let design = kit
                .designs
                .as_ref()
                .and_then(|designs| {
                    designs.iter().find(|design| {
                        design.name == "Nakagin Capsule Tower" && design.parent.is_none()
                    })
                })
                .expect("Nakagin Capsule Tower design not found");

            let filtered = filter_kit(&kit, &KitFilter { design_guid: Some(design.guid.clone()), ..Default::default() });

            assert_eq!(filtered.guid, kit.guid);
            assert_eq!(filtered.name, kit.name);
            assert_eq!(filtered.version, kit.version);
        }

        #[test]
        fn glob_filters_types_by_name_include() {
            let kit = load_kit("metabolism.kit.semio.json");
            let filtered = filter_kit(&kit, &KitFilter {
                types: Some(GlobFilter { include: Some(vec!["Capsule*".to_string()]), exclude: None }),
                ..Default::default()
            });
            let types = filtered.types.as_ref().unwrap();
            assert!(!types.is_empty());
            for t in types {
                assert!(glob_match(&t.name, "Capsule*"), "Type {} should match Capsule*", t.name);
            }
        }

        #[test]
        fn glob_filters_types_by_name_exclude() {
            let kit = load_kit("metabolism.kit.semio.json");
            let total_types = kit.types.as_ref().map(|v| v.len()).unwrap_or(0);
            let filtered = filter_kit(&kit, &KitFilter {
                types: Some(GlobFilter { include: None, exclude: Some(vec!["Capsule*".to_string()]) }),
                ..Default::default()
            });
            let types = filtered.types.as_ref().unwrap();
            assert!(types.len() < total_types);
            for t in types {
                assert!(!glob_match(&t.name, "Capsule*"), "Type {} should have been excluded", t.name);
            }
        }

        #[test]
        fn empty_filter_returns_kit_unchanged() {
            let kit = load_kit("metabolism.kit.semio.json");
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
        fn combines_design_guid_with_glob_filters() {
            let kit = load_kit("metabolism.kit.semio.json");
            let design = kit.designs.as_ref()
                .and_then(|designs| designs.iter().find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none()))
                .expect("Nakagin Capsule Tower design not found");
            let design_filtered = filter_kit(&kit, &KitFilter {
                design_guid: Some(design.guid.clone()),
                ..Default::default()
            });
            let combined_filtered = filter_kit(&kit, &KitFilter {
                design_guid: Some(design.guid.clone()),
                types: Some(GlobFilter { include: None, exclude: Some(vec!["Capsule*".to_string()]) }),
                ..Default::default()
            });
            assert!(
                combined_filtered.types.as_ref().map(|v| v.len()).unwrap_or(0) <
                design_filtered.types.as_ref().map(|v| v.len()).unwrap_or(0)
            );
        }
    }

    // #endregion 🔖KitFilter Tests

    // #region 🔖Model/KPI Tests
    // [👤semio📚rs💻semio🔖tests🔖modelkpi](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Model%20KPI)
    // Model/KPI tests for get_geometric_insights_for_model using nakagin-capsule-tower.gltf.

    mod model_kpi {
        use super::*;

        #[test]
        fn nakagin_capsule_tower_gltf_returns_insights() {
            let path = format!("{}/nakagin-capsule-tower.gltf", ASSETS_DIR);
            let path = std::path::Path::new(&path);
            if !path.exists() {
                return;
            }
            let data = std::fs::read(path).expect("read gltf file");
            let insights =
                get_geometric_insights_for_model(&data).expect("get_geometric_insights_for_model");
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
                p.map(|a| json!({ "x": round6(a[0]), "y": round6(a[1]), "z": round6(a[2]) }))
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

            let canonical_path = std::path::Path::new(ASSETS_DIR).join("nakagin.kpi.model.semio.json");
            let canonical_bytes =
                fs::read(&canonical_path).expect("read canonical model-kpi asset");
            let canonical: serde_json::Value =
                serde_json::from_slice(&canonical_bytes).expect("parse canonical model-kpi asset");
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

    // #endregion 🔖Model/KPI Tests

    // #region 🔖Flatten Tests
    // [👤semio📚rs💻semio🔖tests🔖flattentests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Flatten%20Tests)
    // Flatten Tests MUST provide the flatten tests functionality.

    mod flatten {
        use super::*;

        mod nakagin_capsule_tower {
            use super::*;

            #[test]
            fn kit_flatten_diff_apply_flat() {
                let kit = load_kit("metabolism.kit.semio.json");
                test_flatten_design(&kit, &["Nakagin Capsule Tower"]);
            }

            mod slanted {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Slanted"]);
                }
            }

            mod twisted {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Twisted"]);
                }
            }

            mod dancing {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("metabolism.kit.semio.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Dancing"]);
                }
            }
        }

        mod capsule_dream {
            use super::*;

            #[test]
            fn kit_flatten_diff_apply_flat() {
                let kit = load_kit("metabolism.kit.semio.json");
                test_flatten_design(&kit, &["Capsule Dream"]);
            }
        }
    }

    // #endregion 🔖Flatten Tests

    // #region 🔖Change Tests
    // [👤semio📚rs💻semio🔖tests🔖changetests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Change%20Tests)
    // Change Tests MUST provide the change tests functionality.

    mod change {
        use super::*;

        mod metabolism {
            use super::*;

            #[test]
            fn kit_change_forward_backward_inverse_behavior() {
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
                std::fs::write("/tmp/inverse_applied.json", &inv_json).unwrap();
                std::fs::write("/tmp/original.json", &orig_json).unwrap();
                assert!(
                    are_kits_equal(&applied_inverse, &kit_original),
                    "ApplyKitDiff inverse: applied inverse kit doesn't match original kit"
                );
            }
        }
    }

    // #endregion 🔖Change Tests

    // #region 🔖Validation Tests
    // [👤semio📚rs💻semio🔖tests🔖validationtests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Validation%20Tests)
    // Validation Tests MUST provide the validation tests functionality.

    mod validation {
        use super::*;

        mod metabolism {
            use super::*;

            #[test]
            fn metabolism_kit_validate_empty_report() {
                let kit = load_kit("metabolism.kit.semio.json");
                let result = validate_kit(&kit);
                assert!(result.problems.is_empty());
            }
        }

        mod invalid {
            use super::*;

            #[test]
            fn invalid_kit_validate_invalid_report() {
                let kit = load_kit("invalid.kit.semio.json");
                let result = validate_kit(&kit);
                let expected = load_validation_result("validation.semio.json");
                assert_eq!(
                    result.problems.len(),
                    expected.problems.len(),
                    "Number of problems mismatch"
                );
            }
        }
    }

    // #endregion 🔖Validation Tests

    // #region 🔖Design Quality Sum Tests

    mod design_quality_sum {
        use super::*;

        mod nakagin_capsule_tower {
            use super::*;

            #[test]
            fn sum_effective_floor_area() {
                let kit = load_kit("metabolism.kit.semio.json");
                let design = kit
                    .designs
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|d| d.name == "Nakagin Capsule Tower" && d.parent.is_none())
                    .expect("Nakagin Capsule Tower design not found");
                let quality = kit
                    .qualities
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|q| q.name == "effective floor area")
                    .expect("effective floor area quality not found");
                let result = sum_quality_in_design(&kit, &design.guid, &quality.guid);
                assert!(
                    (result - 2349.53).abs() < 0.01,
                    "Expected ~2349.53, got {}",
                    result
                );
            }
        }
    }

    // #endregion 🔖Design Quality Sum Tests

    // #region 🔖Export Design Model Tests

    mod export_design_model {
        use super::*;

        #[test]
        fn glb_format_valid_header() {
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
        fn gltf_format_valid_json() {
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
            let json_str = std::str::from_utf8(&result).expect("glTF should be valid UTF-8");
            let parsed: serde_json::Value =
                serde_json::from_str(json_str).expect("glTF should be valid JSON");
            assert!(parsed.is_object(), "glTF root should be an object");
        }

        #[test]
        fn invalid_format_returns_error() {
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
        fn export_scene_graph_report() {
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

    // #endregion 🔖Export Design Model Tests

    // #region 🔖Meta And Shallow Tests

    fn load_json<T: serde::de::DeserializeOwned>(filename: &str) -> T {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect(&format!("Failed to deserialize {}", filename))
    }

    #[test]
    fn type_meta_from_asset() {
        let meta: TypeMeta = load_json("tambour.meta.type.semio.json");
        assert_eq!(meta.name, "Tambour");
        assert!(!meta.guid.is_empty());
        assert!(meta.description.is_some());
        assert!(meta.unit.is_some());
    }

    #[test]
    fn type_shallow_from_asset() {
        let shallow: TypeShallow = load_json("tambour.shallow.type.semio.json");
        assert_eq!(shallow.name, "Tambour");
        assert!(!shallow.guid.is_empty());
        assert!(shallow.connectors.is_some());
        assert!(shallow.models.is_some());
        assert!(shallow.props.is_some());
    }

    #[test]
    fn design_meta_from_asset() {
        let meta: DesignMeta = load_json("nakagin-capsule-tower.meta.design.semio.json");
        assert_eq!(meta.name, "Nakagin Capsule Tower");
        assert!(!meta.guid.is_empty());
        assert!(meta.description.is_some());
        assert!(meta.unit.is_some());
    }

    #[test]
    fn design_shallow_from_asset() {
        let shallow: DesignShallow = load_json("nakagin-capsule-tower.shallow.design.semio.json");
        assert_eq!(shallow.name, "Nakagin Capsule Tower");
        assert!(!shallow.guid.is_empty());
        assert!(shallow.pieces.is_some());
        assert!(shallow.connections.is_some());
        assert!(shallow.layers.is_some());
    }

    #[test]
    fn kit_meta_from_asset() {
        let meta: KitMeta = load_json("metabolism.meta.kit.semio.json");
        assert_eq!(meta.name, "Metabolism");
        assert!(!meta.guid.is_empty());
        assert!(meta.version.is_some());
        assert!(meta.description.is_some());
    }

    #[test]
    fn kit_shallow_from_asset() {
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
    fn kit_to_meta_to_shallow() {
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

    // #endregion 🔖Meta And Shallow Tests

    // #region 🔖Kit Workflow Tests
    // [👤semio📚rs💻semio🔖tests🔖kitworkflowtests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/Kit%20Workflow%20Tests)
    // Kit Workflow Tests MUST verify file, folder, archive, remote, and temporary kit workflows.

    #[cfg(not(target_arch = "wasm32"))]
    mod kit_workflow {
        use super::*;
        use base64::Engine;
        use std::collections::HashMap;
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpListener;
        use std::thread;

        struct TestHttpRoute {
            path: String,
            content_type: String,
            body: Vec<u8>,
        }

        fn workflow_kit_fixture() -> (Kit, HashMap<String, Vec<u8>>) {
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

            let file_path = zip_roundtrip::build_file_path(&kit, &kit.files.as_ref().unwrap()[0]);
            let mut files = HashMap::new();
            files.insert(file_path, file_bytes);

            (kit, files)
        }

        fn workflow_diff() -> KitDiff {
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

        fn workflow_expected_paths() -> (String, String) {
            (
                "assets/hello.txt".to_string(),
                "renamed-assets/renamed.txt".to_string(),
            )
        }

        fn spawn_test_http_server(routes: Vec<TestHttpRoute>) -> (String, thread::JoinHandle<()>) {
            let listener = TcpListener::bind("127.0.0.1:0").expect("bind test http listener");
            let address = listener.local_addr().expect("get test http listener address");
            let handle = thread::spawn(move || {
                for _ in 0..routes.len() {
                    let (mut stream, _) = listener.accept().expect("accept test http connection");
                    let mut request_line = String::new();
                    {
                        let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
                        reader.read_line(&mut request_line).expect("read request line");
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
        fn file_workflow_roundtrips_and_persists_edits() {
            let (kit, _) = workflow_kit_fixture();
            let diff = workflow_diff();
            let temp_dir = tempfile::tempdir().unwrap();
            let path = temp_dir.path().join("workflow.kit.semio.json");
            let path_str = path.to_str().unwrap();

            export_file_kit(&kit, path_str).unwrap();
            let imported = import_file_kit(path_str).unwrap();
            assert!(are_kits_equal(&kit, &imported));

            let edited = edit_file_kit(path_str, &diff).unwrap();
            let persisted = import_file_kit(path_str).unwrap();
            assert_eq!(edited.name, "Workflow Kit Edited");
            assert!(are_kits_equal(&edited, &persisted));
        }

        #[test]
        fn folder_workflow_roundtrips_and_moves_assets_on_edit() {
            let (kit, files) = workflow_kit_fixture();
            let diff = workflow_diff();
            let (old_path, new_path) = workflow_expected_paths();
            let temp_dir = tempfile::tempdir().unwrap();
            let folder_path = temp_dir.path().join("workflow-folder-kit");
            let folder_path_str = folder_path.to_str().unwrap();

            export_folder_kit(&kit, &files, folder_path_str).unwrap();
            let imported = import_folder_kit(folder_path_str).unwrap();
            assert!(are_kits_equal(&kit, &imported.kit));
            assert_eq!(imported.files, files);

            let edited = edit_folder_kit(folder_path_str, &diff).unwrap();
            let reloaded = import_folder_kit(folder_path_str).unwrap();
            assert_eq!(edited.name, "Workflow Kit Edited");
            assert!(are_kits_equal(&edited, &reloaded.kit));
            assert!(!folder_path.join(old_path).exists());
            assert!(folder_path.join(new_path).exists());
        }

        #[test]
        fn archive_workflow_roundtrips_and_persists_edits() {
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
        fn remote_workflow_supports_json_and_zip_sources() {
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
        fn temporary_workflow_applies_diff_without_mutating_source() {
            let (kit, _) = workflow_kit_fixture();
            let diff = workflow_diff();

            let edited = edit_temporary_kit(&kit, &diff);
            assert_eq!(edited.name, "Workflow Kit Edited");
            assert_eq!(edited.files.as_ref().unwrap()[0].name, "renamed.txt");
            assert_eq!(edited.folders.as_ref().unwrap()[0].name, "renamed-assets");
            assert_eq!(kit.name, "Workflow Kit");
            assert_eq!(kit.files.as_ref().unwrap()[0].name, "hello.txt");
            assert_eq!(kit.folders.as_ref().unwrap()[0].name, "assets");
        }
    }

    // #endregion 🔖Kit Workflow Tests

    // #region 🔖KitKind Tests
    // [👤semio📚rs💻semio🔖tests🔖kitkindtests](repo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/s/KitKind%20Tests)
    // KitKind Tests MUST verify serialization, deserialization and completeness.

    #[test]
    fn test_kit_kind_all_values_exist() {
        assert_eq!(ALL_KIT_KINDS.len(), 5);
        assert!(ALL_KIT_KINDS.contains(&KitKind::File));
        assert!(ALL_KIT_KINDS.contains(&KitKind::Folder));
        assert!(ALL_KIT_KINDS.contains(&KitKind::Archive));
        assert!(ALL_KIT_KINDS.contains(&KitKind::Remote));
        assert!(ALL_KIT_KINDS.contains(&KitKind::Temporary));
    }

    #[test]
    fn test_kit_kind_serialization() {
        assert_eq!(serde_json::to_string(&KitKind::File).unwrap(), "\"file\"");
        assert_eq!(serde_json::to_string(&KitKind::Folder).unwrap(), "\"folder\"");
        assert_eq!(serde_json::to_string(&KitKind::Archive).unwrap(), "\"archive\"");
        assert_eq!(serde_json::to_string(&KitKind::Remote).unwrap(), "\"remote\"");
        assert_eq!(serde_json::to_string(&KitKind::Temporary).unwrap(), "\"temporary\"");
    }

    #[test]
    fn test_kit_kind_deserialization() {
        assert_eq!(serde_json::from_str::<KitKind>("\"file\"").unwrap(), KitKind::File);
        assert_eq!(serde_json::from_str::<KitKind>("\"folder\"").unwrap(), KitKind::Folder);
        assert_eq!(serde_json::from_str::<KitKind>("\"archive\"").unwrap(), KitKind::Archive);
        assert_eq!(serde_json::from_str::<KitKind>("\"remote\"").unwrap(), KitKind::Remote);
        assert_eq!(serde_json::from_str::<KitKind>("\"temporary\"").unwrap(), KitKind::Temporary);
    }

    #[test]
    fn test_kit_kind_file_roundtrip() {
        let kit = Kit {
            guid: "test-guid-file".to_string(),
            name: "TestFileKit".to_string(),
            version: Some("1.0.0".to_string()),
            description: Some("A file kit".to_string()),
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
    fn test_kit_kind_temporary_in_memory() {
        let mut kit = Kit {
            guid: "temp-guid".to_string(),
            name: "TempKit".to_string(),
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
        let kind = KitKind::Temporary;
        assert_eq!(kind, KitKind::Temporary);
        kit.name = "ModifiedTempKit".to_string();
        kit.description = Some("Modified in memory".to_string());
        assert_eq!(kit.name, "ModifiedTempKit");
        assert_eq!(kit.description, Some("Modified in memory".to_string()));
    }

    // #endregion 🔖KitKind Tests
}

// #endregion 🔖Tests

// #region 🔖Benchmark
#[cfg(test)]
pub mod benchmark {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::time::Instant;

    const ASSETS_DIR: &str = "../assets/semio";
    const ITERATIONS: u32 = 3;

    fn load_kit(filename: &str) -> Kit {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize kit")
    }

    fn load_kit_diff(filename: &str) -> KitDiff {
        let path = Path::new(ASSETS_DIR).join(filename);
        let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
        serde_json::from_str(&data).expect("Failed to deserialize kit diff")
    }

    fn bench<F: Fn()>(name: &str, f: F) {
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            f();
        }
        let duration = start.elapsed().as_secs_f64() / ITERATIONS as f64;
        println!("{},{:.6}", name, duration);
    }

    fn find_design<'a>(kit: &'a Kit, name: &str, parent_name: Option<&str>) -> &'a Design {
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
        let kit_invalid = load_kit("metabolism.kit.semio.json");
        let metabolism_zip_path = Path::new(ASSETS_DIR).join("metabolism.semio.zip");
        let metabolism_zip_str = metabolism_zip_path.to_str().unwrap();
        // We assume running from rs/semio

        bench("Roundtrip/Metabolism", || {
            let import_result =
                super::zip_roundtrip::import_kit_from_zip(metabolism_zip_str).unwrap();

            let temp_zip = "temp_benchmark_metabolism.zip";
            // Need schema? Rust ExportKitToZip calls KitToSqlite which executes schema.
            // Wait, Rust implementation of export_kit_to_zip in semio.rs...
            // Let's check signature.

            super::zip_roundtrip::export_kit_to_zip(
                &import_result.kit,
                &import_result.files,
                temp_zip,
            )
            .unwrap();
            if Path::new(temp_zip).exists() {
                std::fs::remove_file(temp_zip).unwrap();
            }
        });

        let d1 = find_design(&kit_metabolism, "Nakagin Capsule Tower", None);
        let d1_guid = d1.guid.clone();
        bench("Flatten Design/Nakagin Capsule Tower", || {
            let _ = flatten_design(&kit_metabolism, &d1_guid);
        });

        let d2 = find_design(&kit_metabolism, "Slanted", Some("Nakagin Capsule Tower"));
        let d2_guid = d2.guid.clone();
        bench("Flatten Design/Nakagin Capsule Tower/Slanted", || {
            let _ = flatten_design(&kit_metabolism, &d2_guid);
        });

        let d3 = find_design(&kit_metabolism, "Twisted", Some("Nakagin Capsule Tower"));
        let d3_guid = d3.guid.clone();
        bench("Flatten Design/Nakagin Capsule Tower/Twisted", || {
            let _ = flatten_design(&kit_metabolism, &d3_guid);
        });

        let d4 = find_design(&kit_metabolism, "Dancing", Some("Nakagin Capsule Tower"));
        let d4_guid = d4.guid.clone();
        bench("Flatten Design/Nakagin Capsule Tower/Dancing", || {
            let _ = flatten_design(&kit_metabolism, &d4_guid);
        });

        let d5 = find_design(&kit_metabolism, "Capsule Dream", None);
        let d5_guid = d5.guid.clone();
        bench("Flatten Design/Capsule Dream", || {
            let _ = flatten_design(&kit_metabolism, &d5_guid);
        });

        bench("Validation/Invalid Kit", || {
            let _ = validate_kit(&kit_invalid);
        });

        bench("Validation/Metabolism", || {
            let _ = validate_kit(&kit_metabolism);
        });
    }
}
// #endregion 🔖Benchmark
