// #region 🔖Header

// [👤semio📚rs💻semiors](semiorepo://file/SEMIO/RS/SEMIO.RS)

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

// [👤semio📚rs💻semiors🔖imports](semiorepo://section/semio/rs/semio.rs/imports)
// Imports MUST include all required crates and modules for the semio domain library.

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

// #endregion 🔖Imports

// #region 🔖Error Types
// [👤semio📚rs💻semiors🔖errortypes](semiorepo://section/semio/rs/semio.rs/error-types)
// Error Types MUST provide the error types functionality.

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
// [🛠️semio/rs/semio.rs#Error Types§SemioError](semiorepo://definition/semio/rs/semio.rs/ERROR-TYPES/SEMIO-ERROR)
/// <summary>SemioError holds the data fields for a SemioError record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖errortypes🛠️semioerror](semiorepo://definition/semio/rs/semio.rs/error%20types/semioerror)
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

// [🛠️semio/rs/semio.rs#Error Types§Result](semiorepo://definition/semio/rs/semio.rs/ERROR-TYPES/RESULT)
/// <summary>Result holds the data fields for a Result record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖errortypes🛠️result](semiorepo://definition/semio/rs/semio.rs/error%20types/result)
/// </remarks>
pub type Result<T> = std::result::Result<T, SemioError>;

// #endregion 🔖Error Types

// #region 🔖Utility Functions
// [👤semio📚rs💻semiors🔖utilityfunctions](semiorepo://section/semio/rs/semio.rs/utility-functions)
// Utility Functions MUST provide the utility functions functionality.

// [🛠️semio/rs/semio.rs#Utility Functions§Guid](semiorepo://definition/semio/rs/semio.rs/UTILITY-FUNCTIONS/GUID)
/// <summary>Guid holds the data fields for a Guid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️guid](semiorepo://definition/semio/rs/semio.rs/utility%20functions/guid)
/// </remarks>
pub type Guid = String;

/// <summary>guid holds the data fields for a guid record.</summary>
/// guid MUST perform the guid operation.
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️guid](semiorepo://definition/semio/rs/semio.rs/utility%20functions/guid)
pub fn guid() -> String { Uuid::now_v7().to_string() }

/// <summary>normalize holds the data fields for a normalize record.</summary>
/// normalize MUST perform the normalize operation.
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️normalize](semiorepo://definition/semio/rs/semio.rs/utility%20functions/normalize)
pub fn normalize(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// <summary>round holds the data fields for a round record.</summary>
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️round](semiorepo://definition/semio/rs/semio.rs/utility%20functions/round)
/// <remarks>
/// round MUST perform the round operation.
/// </remarks>
pub fn round(value: f64) -> f64 { normalize(value, 3) }

/// <summary>jaccard holds the data fields for a jaccard record.</summary>
/// jaccard MUST perform the jaccard operation.
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️jaccard](semiorepo://definition/semio/rs/semio.rs/utility%20functions/jaccard)
pub fn jaccard<T: Eq + std::hash::Hash>(a: &HashSet<T>, b: &HashSet<T>) -> f64 {
    if a.is_empty() && b.is_empty() { return 1.0; }
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

/// <summary>deep_equal holds the data fields for a deep_equal record.</summary>
/// deep_equal MUST perform the deep_equal operation.
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️deepequal](semiorepo://definition/semio/rs/semio.rs/utility%20functions/deep_equal)
pub fn deep_equal<T: Serialize>(a: &T, b: &T) -> bool {
    match (serde_json::to_value(a), serde_json::to_value(b)) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

// [🛠️semio/rs/semio.rs#Utility Functions§generate_unique_name](semiorepo://definition/semio/rs/semio.rs/UTILITY-FUNCTIONS/GENERATE-UNIQUE-NAME)
/// <summary>generate_unique_name performs the generate_unique_name operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖utilityfunctions🛠️generateuniquename](semiorepo://definition/semio/rs/semio.rs/utility%20functions/generate_unique_name)
/// generate_unique_name MUST perform the generate_unique_name operation.
/// </remarks>
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

// #endregion 🔖Utility Functions

// #region 🔖Model Types - Attribute
// [👤semio📚rs💻semiors🔖modeltypesattribute](semiorepo://section/semio/rs/semio.rs/model-types-attribute)
// Model Types - Attribute MUST provide the model types - attribute functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Attribute holds the data fields for a Attribute record.</summary>
/// Attribute MUST perform the Attribute operation.
/// [👤semio📚rs💻semiors🔖modeltypesattribute🛠️attribute](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20attribute/attribute)
pub struct Attribute {
    pub guid: Guid,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Attribute§AttributeId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-ATTRIBUTE/ATTRIBUTE-ID)
/// <summary>AttributeId holds the data fields for a AttributeId record.</summary>
/// AttributeId MUST perform the AttributeId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesattribute🛠️attributeid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20attribute/attributeid)
/// </remarks>
pub struct AttributeId { pub guid: Guid }

// #endregion 🔖Model Types - Attribute

// #region 🔖Model Types - Coord
// [👤semio📚rs💻semiors🔖modeltypescoord](semiorepo://section/semio/rs/semio.rs/model-types-coord)
// Model Types - Coord MUST provide the model types - coord functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Coord§Coord](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-COORD/COORD)
/// <summary>Coord holds the data fields for a Coord record.</summary>
/// Coord MUST perform the Coord operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypescoord🛠️coord](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20coord/coord)
/// </remarks>
pub struct Coord { pub u: f64, pub v: f64 }

impl Coord { pub fn new(u: f64, v: f64) -> Self { Self { u, v } } }

// #endregion 🔖Model Types - Coord

// #region 🔖Model Types - Vector
// [👤semio📚rs💻semiors🔖modeltypesvector](semiorepo://section/semio/rs/semio.rs/model-types-vector)
// Model Types - Vector MUST provide the model types - vector functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Vector§Vector](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-VECTOR/VECTOR)
/// <summary>Vector holds the data fields for a Vector record.</summary>
/// Vector MUST perform the Vector operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesvector🛠️vector](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20vector/vector)
/// </remarks>
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

// #endregion 🔖Model Types - Vector

// #region 🔖Model Types - Plane
// [👤semio📚rs💻semiors🔖modeltypesplane](semiorepo://section/semio/rs/semio.rs/model-types-plane)
// Model Types - Plane MUST provide the model types - plane functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Plane holds the data fields for a Plane record.</summary>
/// Plane MUST perform the Plane operation.
/// [👤semio📚rs💻semiors🔖modeltypesplane🛠️plane](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20plane/plane)
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

// #endregion 🔖Model Types - Plane

// #region 🔖Model Types - Camera
// [👤semio📚rs💻semiors🔖modeltypescamera](semiorepo://section/semio/rs/semio.rs/model-types-camera)
// Model Types - Camera MUST provide the model types - camera functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Camera§Camera](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-CAMERA/CAMERA)
/// <summary>Camera holds the data fields for a Camera record.</summary>
/// Camera MUST perform the Camera operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypescamera🛠️camera](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20camera/camera)
/// </remarks>
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

// #endregion 🔖Model Types - Camera

// #region 🔖Model Types - Location, Author, File, Folder
// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder](semiorepo://section/semio/rs/semio.rs/model-types-location-author-file-folder)
// Model Types - Location, Author, File, Folder MUST provide the model types - location, author, file, folder functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>LocationId holds the data fields for a LocationId record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Location, Author, File, Folder§LocationId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LOCATION-AUTHOR-FILE-FOLDER/LOCATION-ID)
/// <summary>LocationId holds the data fields for a LocationId record.</summary>
/// LocationId MUST perform the LocationId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️locationid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/locationid)
/// </remarks>
pub struct LocationId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Location holds the data fields for a Location record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️location](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/location)
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
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️authorid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/authorid)
pub struct AuthorId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Author holds the data fields for a Author record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️author](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/author)
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
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️folderid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/folderid)
pub struct FolderId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Location, Author, File, Folder§Folder](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LOCATION-AUTHOR-FILE-FOLDER/FOLDER)
/// <summary>Folder holds the data fields for a Folder record.</summary>
/// Folder MUST perform the Folder operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️folder](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/folder)
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
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️fileid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/fileid)
/// <remarks>
/// FileId MUST perform the FileId operation.
/// </remarks>
pub struct FileId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>File holds the data fields for a File record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️file](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20location,%20author,%20file,%20folder/file)
/// <remarks>
/// File MUST perform the File operation.
/// </remarks>
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

// #endregion 🔖Model Types - Location, Author, File, Folder

// #region 🔖Model Types - Quality, Port, Tag, Concept
// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept](semiorepo://section/semio/rs/semio.rs/model-types-quality-port-tag-concept)
// Model Types - Quality, Port, Tag, Concept MUST provide the model types - quality, port, tag, concept functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§QualityId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/QUALITY-ID)
/// <summary>QualityId holds the data fields for a QualityId record.</summary>
/// QualityId MUST perform the QualityId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️qualityid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/qualityid)
/// </remarks>
pub struct QualityId { pub guid: Guid }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(i32)]
/// <summary>QualityKind holds the data fields for a QualityKind record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️qualitykind](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/qualitykind)
pub enum QualityKind { #[default] Integer = 0, Float = 1, Boolean = 2 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Quality](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/QUALITY)
/// <summary>Quality holds the data fields for a Quality record.</summary>
/// Quality MUST perform the Quality operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️quality](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/quality)
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
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§PortId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/PORT-ID)
/// <summary>PortId holds the data fields for a PortId record.</summary>
/// PortId MUST perform the PortId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️portid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/portid)
/// </remarks>
pub struct PortId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Port](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/PORT)
/// <summary>Port holds the data fields for a Port record.</summary>
/// Port MUST perform the Port operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️port](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/port)
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
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§TagId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/TAG-ID)
/// <summary>TagId holds the data fields for a TagId record.</summary>
/// TagId MUST perform the TagId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️tagid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/tagid)
/// </remarks>
pub struct TagId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Tag holds the data fields for a Tag record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Tag](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/TAG)
/// <summary>Tag holds the data fields for a Tag record.</summary>
/// Tag MUST perform the Tag operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️tag](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/tag)
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
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§ConceptId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/CONCEPT-ID)
/// <summary>ConceptId holds the data fields for a ConceptId record.</summary>
/// ConceptId MUST perform the ConceptId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️conceptid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/conceptid)
/// </remarks>
pub struct ConceptId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Concept MUST perform the Concept operation.
// [🛠️semio/rs/semio.rs#Model Types - Quality, Port, Tag, Concept§Concept](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-QUALITY-PORT-TAG-CONCEPT/CONCEPT)
/// <summary>Concept holds the data fields for a Concept record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️concept](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20quality,%20port,%20tag,%20concept/concept)
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
// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector](semiorepo://section/semio/rs/semio.rs/model-types-prop-model-connector)
// Model Types - Prop, Model, Connector MUST provide the model types - prop, model, connector functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§PropId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/PROP-ID)
/// <summary>PropId holds the data fields for a PropId record.</summary>
/// PropId MUST perform the PropId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️propid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/propid)
/// </remarks>
pub struct PropId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Prop holds the data fields for a Prop record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Prop](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/PROP)
/// <summary>Prop holds the data fields for a Prop record.</summary>
/// Prop MUST perform the Prop operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️prop](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/prop)
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
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️modelid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/modelid)
pub struct ModelId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Model MUST perform the Model operation.
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Model](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/MODEL)
/// <summary>Model holds the data fields for a Model record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️model](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/model)
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
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️connectorid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/connectorid)
/// <remarks>
/// ConnectorId MUST perform the ConnectorId operation.
/// </remarks>
pub struct ConnectorId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Prop, Model, Connector§Connector](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-PROP-MODEL-CONNECTOR/CONNECTOR)
/// <summary>Connector holds the data fields for a Connector record.</summary>
/// Connector MUST perform the Connector operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️connector](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20prop,%20model,%20connector/connector)
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
// [👤semio📚rs💻semiors🔖modeltypestype](semiorepo://section/semio/rs/semio.rs/model-types-type)
// Model Types - Type MUST provide the model types - type functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>TypeId holds the data fields for a TypeId record.</summary>
/// TypeId MUST perform the TypeId operation.
/// [👤semio📚rs💻semiors🔖modeltypestype🛠️typeid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20type/typeid)
pub struct TypeId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Type§Type](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-TYPE/TYPE)
/// <summary>Type holds the data fields for a Type record.</summary>
/// Type MUST perform the Type operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypestype🛠️type](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20type/type)
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

// #endregion 🔖Model Types - Type

// #region 🔖Model Types - Layer, Piece, Group, Side, Connection, Stat
// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat](semiorepo://section/semio/rs/semio.rs/model-types-layer-piece-group-side-connection-stat)
// Model Types - Layer, Piece, Group, Side, Connection, Stat MUST provide the model types - layer, piece, group, side, connection, stat functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>LayerId holds the data fields for a LayerId record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layerid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/layerid)
/// <remarks>
/// LayerId MUST perform the LayerId operation.
/// </remarks>
pub struct LayerId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Layer](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/LAYER)
/// <summary>Layer holds the data fields for a Layer record.</summary>
/// Layer MUST perform the Layer operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layer](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/layer)
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
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§PieceId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/PIECE-ID)
/// <summary>PieceId holds the data fields for a PieceId record.</summary>
/// PieceId MUST perform the PieceId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️pieceid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/pieceid)
/// </remarks>
pub struct PieceId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§DesignId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/DESIGN-ID)
/// <summary>DesignId holds the data fields for a DesignId record.</summary>
/// DesignId MUST perform the DesignId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️designid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/designid)
/// </remarks>
pub struct DesignId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Piece](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/PIECE)
/// <summary>Piece holds the data fields for a Piece record.</summary>
/// Piece MUST perform the Piece operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️piece](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/piece)
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
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§GroupId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/GROUP-ID)
/// <summary>GroupId holds the data fields for a GroupId record.</summary>
/// GroupId MUST perform the GroupId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️groupid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/groupid)
/// </remarks>
pub struct GroupId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Group holds the data fields for a Group record.</summary>
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Group](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/GROUP)
/// <summary>Group holds the data fields for a Group record.</summary>
/// Group MUST perform the Group operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️group](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/group)
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
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Side](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/SIDE)
/// <summary>Side holds the data fields for a Side record.</summary>
/// Side MUST perform the Side operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️side](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/side)
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
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§ConnectionId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/CONNECTION-ID)
/// <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/// ConnectionId MUST perform the ConnectionId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connectionid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/connectionid)
/// </remarks>
pub struct ConnectionId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Connection](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/CONNECTION)
/// <summary>Connection holds the data fields for a Connection record.</summary>
/// Connection MUST perform the Connection operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connection](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/connection)
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
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§StatId](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/STAT-ID)
/// <summary>StatId holds the data fields for a StatId record.</summary>
/// StatId MUST perform the StatId operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️statid](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/statid)
/// </remarks>
pub struct StatId { pub guid: Guid }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Stat MUST perform the Stat operation.
// [🛠️semio/rs/semio.rs#Model Types - Layer, Piece, Group, Side, Connection, Stat§Stat](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-LAYER-PIECE-GROUP-SIDE-CONNECTION-STAT/STAT)
/// <summary>Stat holds the data fields for a Stat record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️stat](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20layer,%20piece,%20group,%20side,%20connection,%20stat/stat)
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
// [👤semio📚rs💻semiors🔖modeltypesdesign](semiorepo://section/semio/rs/semio.rs/model-types-design)
// Model Types - Design MUST provide the model types - design functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// [🛠️semio/rs/semio.rs#Model Types - Design§Design](semiorepo://definition/semio/rs/semio.rs/MODEL-TYPES-DESIGN/DESIGN)
/// <summary>Design holds the data fields for a Design record.</summary>
/// Design MUST perform the Design operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖modeltypesdesign🛠️design](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20design/design)
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

// #endregion 🔖Model Types - Design

// #region 🔖Model Types - Kit
// [👤semio📚rs💻semiors🔖modeltypeskit](semiorepo://section/semio/rs/semio.rs/model-types-kit)
// Model Types - Kit MUST provide the model types - kit functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// <summary>Kit holds the data fields for a Kit record.</summary>
/// [👤semio📚rs💻semiors🔖modeltypeskit🛠️kit](semiorepo://definition/semio/rs/semio.rs/model%20types%20-%20kit/kit)
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
// [👤semio📚rs💻semiors🔖finderfunctions](semiorepo://section/semio/rs/semio.rs/finder-functions)
// Finder Functions MUST provide the finder functions functionality.

// [🛠️semio/rs/semio.rs#Finder Functions§find_type_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TYPE-IN-KIT)
/// <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/// find_type_in_kit MUST perform the find_type_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findtypeinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_type_in_kit)
/// </remarks>
pub fn find_type_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Type> {
    kit.types.as_ref()?.iter().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_type_in_kit_mut](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TYPE-IN-KIT-MUT)
/// <summary>find_type_in_kit_mut holds the data fields for a find_type_in_kit_mut record.</summary>
/// find_type_in_kit_mut MUST perform the find_type_in_kit_mut operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findtypeinkitmut](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_type_in_kit_mut)
/// </remarks>
pub fn find_type_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Type> {
    kit.types.as_mut()?.iter_mut().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_design_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-DESIGN-IN-KIT)
/// <summary>find_design_in_kit holds the data fields for a find_design_in_kit record.</summary>
/// find_design_in_kit MUST perform the find_design_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️finddesigninkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_design_in_kit)
/// </remarks>
pub fn find_design_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Design> {
    kit.designs.as_ref()?.iter().find(|d| d.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_design_in_kit_mut](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-DESIGN-IN-KIT-MUT)
/// <summary>find_design_in_kit_mut holds the data fields for a find_design_in_kit_mut record.</summary>
/// find_design_in_kit_mut MUST perform the find_design_in_kit_mut operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️finddesigninkitmut](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_design_in_kit_mut)
/// </remarks>
pub fn find_design_in_kit_mut<'a>(kit: &'a mut Kit, guid: &str) -> Option<&'a mut Design> {
    kit.designs.as_mut()?.iter_mut().find(|d| d.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_piece_in_design](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-PIECE-IN-DESIGN)
/// <summary>find_piece_in_design holds the data fields for a find_piece_in_design record.</summary>
/// find_piece_in_design MUST perform the find_piece_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findpieceindesign](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_piece_in_design)
/// </remarks>
pub fn find_piece_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Piece> {
    design.pieces.as_ref()?.iter().find(|p| p.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_piece_in_design_mut](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-PIECE-IN-DESIGN-MUT)
/// <summary>find_piece_in_design_mut performs the find_piece_in_design_mut operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findpieceindesignmut](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_piece_in_design_mut)
/// find_piece_in_design_mut MUST perform the find_piece_in_design_mut operation.
/// </remarks>
pub fn find_piece_in_design_mut<'a>(design: &'a mut Design, guid: &str) -> Option<&'a mut Piece> {
    design.pieces.as_mut()?.iter_mut().find(|p| p.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_connection_in_design](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONNECTION-IN-DESIGN)
/// <summary>find_connection_in_design holds the data fields for a find_connection_in_design record.</summary>
/// find_connection_in_design MUST perform the find_connection_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findconnectionindesign](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_connection_in_design)
/// </remarks>
pub fn find_connection_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Connection> {
    design.connections.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_connector_in_type](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONNECTOR-IN-TYPE)
/// <summary>find_connector_in_type holds the data fields for a find_connector_in_type record.</summary>
/// find_connector_in_type MUST perform the find_connector_in_type operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findconnectorintype](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_connector_in_type)
/// </remarks>
pub fn find_connector_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Connector> {
    t.connectors.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_model_in_type](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-MODEL-IN-TYPE)
/// <summary>find_model_in_type holds the data fields for a find_model_in_type record.</summary>
/// find_model_in_type MUST perform the find_model_in_type operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findmodelintype](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_model_in_type)
/// </remarks>
pub fn find_model_in_type<'a>(t: &'a Type, guid: &str) -> Option<&'a Model> {
    t.models.as_ref()?.iter().find(|m| m.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_file_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-FILE-IN-KIT)
/// <summary>find_file_in_kit performs the find_file_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findfileinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_file_in_kit)
/// find_file_in_kit MUST perform the find_file_in_kit operation.
/// </remarks>
pub fn find_file_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a File> {
    kit.files.as_ref()?.iter().find(|f| f.guid == guid)
}

/// <summary>find_folder_in_kit holds the data fields for a find_folder_in_kit record.</summary>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findfolderinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_folder_in_kit)
/// <remarks>
/// find_folder_in_kit MUST perform the find_folder_in_kit operation.
/// </remarks>
pub fn find_folder_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Folder> {
    kit.folders.as_ref()?.iter().find(|f| f.guid == guid)
}

/// find_author_in_kit MUST perform the find_author_in_kit operation.
// [🛠️semio/rs/semio.rs#Finder Functions§find_author_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-AUTHOR-IN-KIT)
/// <summary>find_author_in_kit performs the find_author_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findauthorinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_author_in_kit)
/// find_author_in_kit MUST perform the find_author_in_kit operation.
/// </remarks>
pub fn find_author_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Author> {
    kit.authors.as_ref()?.iter().find(|a| a.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_tag_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-TAG-IN-KIT)
/// <summary>find_tag_in_kit holds the data fields for a find_tag_in_kit record.</summary>
/// find_tag_in_kit MUST perform the find_tag_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findtaginkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_tag_in_kit)
/// </remarks>
pub fn find_tag_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Tag> {
    kit.tags.as_ref()?.iter().find(|t| t.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_concept_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-CONCEPT-IN-KIT)
/// <summary>find_concept_in_kit holds the data fields for a find_concept_in_kit record.</summary>
/// find_concept_in_kit MUST perform the find_concept_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findconceptinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_concept_in_kit)
/// </remarks>
pub fn find_concept_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Concept> {
    kit.concepts.as_ref()?.iter().find(|c| c.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_quality_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-QUALITY-IN-KIT)
/// <summary>find_quality_in_kit holds the data fields for a find_quality_in_kit record.</summary>
/// find_quality_in_kit MUST perform the find_quality_in_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findqualityinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_quality_in_kit)
/// </remarks>
pub fn find_quality_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Quality> {
    kit.qualities.as_ref()?.iter().find(|q| q.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_interface_in_kit](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-INTERFACE-IN-KIT)
/// <summary>find_interface_in_kit performs the find_interface_in_kit operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findinterfaceinkit](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_interface_in_kit)
/// find_interface_in_kit MUST perform the find_interface_in_kit operation.
/// </remarks>
pub fn find_interface_in_kit<'a>(kit: &'a Kit, guid: &str) -> Option<&'a Port> {
    kit.ports.as_ref()?.iter().find(|i| i.guid == guid)
}

/// <summary>find_layer_in_design holds the data fields for a find_layer_in_design record.</summary>
/// find_layer_in_design MUST perform the find_layer_in_design operation.
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findlayerindesign](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_layer_in_design)
pub fn find_layer_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Layer> {
    design.layers.as_ref()?.iter().find(|l| l.guid == guid)
}

// [🛠️semio/rs/semio.rs#Finder Functions§find_group_in_design](semiorepo://definition/semio/rs/semio.rs/FINDER-FUNCTIONS/FIND-GROUP-IN-DESIGN)
/// <summary>find_group_in_design holds the data fields for a find_group_in_design record.</summary>
/// find_group_in_design MUST perform the find_group_in_design operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findgroupindesign](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_group_in_design)
/// </remarks>
pub fn find_group_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Group> {
    design.groups.as_ref()?.iter().find(|g| g.guid == guid)
}

/// <summary>find_stat_in_design holds the data fields for a find_stat_in_design record.</summary>
/// [👤semio📚rs💻semiors🔖finderfunctions🛠️findstatindesign](semiorepo://definition/semio/rs/semio.rs/finder%20functions/find_stat_in_design)
/// <remarks>
/// find_stat_in_design MUST perform the find_stat_in_design operation.
/// </remarks>
pub fn find_stat_in_design<'a>(design: &'a Design, guid: &str) -> Option<&'a Stat> {
    design.stats.as_ref()?.iter().find(|s| s.guid == guid)
}

// #endregion 🔖Finder Functions

// #region 🔖Serialization
// [👤semio📚rs💻semiors🔖serialization](semiorepo://section/semio/rs/semio.rs/serialization)
// Serialization MUST provide the serialization functionality.

/// <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/// serialize_kit MUST perform the serialize_kit operation.
/// [👤semio📚rs💻semiors🔖serialization🛠️serializekit](semiorepo://definition/semio/rs/semio.rs/serialization/serialize_kit)
pub fn serialize_kit(kit: &Kit) -> Result<String> {
    serde_json::to_string_pretty(kit).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_kit](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-KIT)
/// <summary>deserialize_kit holds the data fields for a deserialize_kit record.</summary>
/// deserialize_kit MUST perform the deserialize_kit operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️deserializekit](semiorepo://definition/semio/rs/semio.rs/serialization/deserialize_kit)
/// </remarks>
pub fn deserialize_kit(json: &str) -> Result<Kit> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

// [🛠️semio/rs/semio.rs#Serialization§serialize_design](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/SERIALIZE-DESIGN)
/// <summary>serialize_design performs the serialize_design operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️serializedesign](semiorepo://definition/semio/rs/semio.rs/serialization/serialize_design)
/// serialize_design MUST perform the serialize_design operation.
/// </remarks>
pub fn serialize_design(design: &Design) -> Result<String> {
    serde_json::to_string_pretty(design).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_design](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-DESIGN)
/// <summary>deserialize_design holds the data fields for a deserialize_design record.</summary>
/// deserialize_design MUST perform the deserialize_design operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️deserializedesign](semiorepo://definition/semio/rs/semio.rs/serialization/deserialize_design)
/// </remarks>
pub fn deserialize_design(json: &str) -> Result<Design> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

/// <summary>serialize_type holds the data fields for a serialize_type record.</summary>
// [🛠️semio/rs/semio.rs#Serialization§serialize_type](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/SERIALIZE-TYPE)
/// <summary>serialize_type performs the serialize_type operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️serializetype](semiorepo://definition/semio/rs/semio.rs/serialization/serialize_type)
/// serialize_type MUST perform the serialize_type operation.
/// </remarks>
pub fn serialize_type(t: &Type) -> Result<String> {
    serde_json::to_string_pretty(t).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

// [🛠️semio/rs/semio.rs#Serialization§deserialize_type](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/DESERIALIZE-TYPE)
/// <summary>deserialize_type holds the data fields for a deserialize_type record.</summary>
/// deserialize_type MUST perform the deserialize_type operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️deserializetype](semiorepo://definition/semio/rs/semio.rs/serialization/deserialize_type)
/// </remarks>
pub fn deserialize_type(json: &str) -> Result<Type> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization { message: e.to_string() })
}

/// are_kits_equal MUST perform the are_kits_equal operation.
// [🛠️semio/rs/semio.rs#Serialization§are_kits_equal](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/ARE-KITS-EQUAL)
/// <summary>are_kits_equal performs the are_kits_equal operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️arekitsequal](semiorepo://definition/semio/rs/semio.rs/serialization/are_kits_equal)
/// are_kits_equal MUST perform the are_kits_equal operation.
/// </remarks>
pub fn are_kits_equal(a: &Kit, b: &Kit) -> bool { deep_equal(a, b) }
// [🛠️semio/rs/semio.rs#Serialization§are_designs_equal](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/ARE-DESIGNS-EQUAL)
/// <summary>are_designs_equal holds the data fields for a are_designs_equal record.</summary>
/// are_designs_equal MUST perform the are_designs_equal operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️aredesignsequal](semiorepo://definition/semio/rs/semio.rs/serialization/are_designs_equal)
/// </remarks>
pub fn are_designs_equal(a: &Design, b: &Design) -> bool { deep_equal(a, b) }
/// <summary>are_types_equal holds the data fields for a are_types_equal record.</summary>
/// are_types_equal MUST perform the are_types_equal operation.
/// [👤semio📚rs💻semiors🔖serialization🛠️aretypesequal](semiorepo://definition/semio/rs/semio.rs/serialization/are_types_equal)
pub fn are_types_equal(a: &Type, b: &Type) -> bool { deep_equal(a, b) }

/// <summary>SUPPORTED_MODEL_EXTENSIONS holds the data fields for a SUPPORTED_MODEL_EXTENSIONS record.</summary>
/// [👤semio📚rs💻semiors🔖serialization🛠️supportedmodelextensions](semiorepo://definition/semio/rs/semio.rs/serialization/supported_model_extensions)
pub const SUPPORTED_MODEL_EXTENSIONS: &[&str] = &["gltf", "glb", "fbx", "obj", "dae", "3ds", "stl", "ply", "usdz", "vrm", "ifc", "3mf"];

/// is_supported_model_extension MUST perform the is_supported_model_extension operation.
// [🛠️semio/rs/semio.rs#Serialization§is_supported_model_extension](semiorepo://definition/semio/rs/semio.rs/SERIALIZATION/IS-SUPPORTED-MODEL-EXTENSION)
/// <summary>is_supported_model_extension performs the is_supported_model_extension operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖serialization🛠️issupportedmodelextension](semiorepo://definition/semio/rs/semio.rs/serialization/is_supported_model_extension)
/// is_supported_model_extension MUST perform the is_supported_model_extension operation.
/// </remarks>
pub fn is_supported_model_extension(ext: &str) -> bool {
    SUPPORTED_MODEL_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// #endregion 🔖Serialization

// #region 🔖Diff Types
// [👤semio📚rs💻semiors🔖difftypes](semiorepo://section/semio/rs/semio.rs/diff-types)
// Diff Types MUST provide the diff types functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
// [🛠️semio/rs/semio.rs#Diff Types§RemovedItem](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/REMOVED-ITEM)
/// <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/// RemovedItem MUST perform the RemovedItem operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️removeditem](semiorepo://definition/semio/rs/semio.rs/diff%20types/removeditem)
/// </remarks>
pub struct RemovedItem { pub guid: Guid }

#[derive(Debug, Clone, PartialEq)]
// [🛠️semio/rs/semio.rs#Diff Types§DiffUpdate](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/DIFF-UPDATE)
/// <summary>DiffUpdate holds the data fields for a DiffUpdate record.</summary>
/// DiffUpdate MUST perform the DiffUpdate operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️diffupdate](semiorepo://definition/semio/rs/semio.rs/diff%20types/diffupdate)
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
        
        let diff_val = map.remove("diff").ok_or_else(|| serde::de::Error::missing_field("diff"))?;
        
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
        
        let guid = guid.ok_or_else(|| serde::de::Error::custom("Could not find guid in update wrapper"))?;
        if key.is_empty() { return Err(serde::de::Error::custom("Could not find entity key")); }

        let mut diff_obj = match diff_val {
            Value::Object(o) => o,
            _ => return Err(serde::de::Error::custom("diff field expected to be an object")),
        };
        
        if !diff_obj.contains_key("guid") {
            diff_obj.insert("guid".to_string(), Value::String(guid.clone()));
        }

        let diff: D = serde_json::from_value(Value::Object(diff_obj)).map_err(serde::de::Error::custom)?;
        
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
        struct GuidWrapper { guid: String }
        map.serialize_entry(&self.key, &GuidWrapper { guid: self.guid.clone() })?;
        
        map.end()
    }
}

impl<D: DiffHasGuid> DiffHasGuid for DiffUpdate<D> {
    fn guid(&self) -> &str { &self.guid }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(bound(deserialize = "T: Deserialize<'de>, D: serde::de::DeserializeOwned"))]
// [🛠️semio/rs/semio.rs#Diff Types§CollectionDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/COLLECTION-DIFF)
/// <summary>CollectionDiff holds the data fields for a CollectionDiff record.</summary>
/// CollectionDiff MUST perform the CollectionDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️collectiondiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/collectiondiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§AttributeDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/ATTRIBUTE-DIFF)
/// <summary>AttributeDiff holds the data fields for a AttributeDiff record.</summary>
/// AttributeDiff MUST perform the AttributeDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️attributediff](semiorepo://definition/semio/rs/semio.rs/diff%20types/attributediff)
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
// [🛠️semio/rs/semio.rs#Diff Types§PropDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/PROP-DIFF)
/// <summary>PropDiff holds the data fields for a PropDiff record.</summary>
/// PropDiff MUST perform the PropDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️propdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/propdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§ConnectorDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/CONNECTOR-DIFF)
/// <summary>ConnectorDiff holds the data fields for a ConnectorDiff record.</summary>
/// ConnectorDiff MUST perform the ConnectorDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️connectordiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/connectordiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§ModelDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/MODEL-DIFF)
/// <summary>ModelDiff holds the data fields for a ModelDiff record.</summary>
/// ModelDiff MUST perform the ModelDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️modeldiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/modeldiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§TypeDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/TYPE-DIFF)
/// <summary>TypeDiff holds the data fields for a TypeDiff record.</summary>
/// TypeDiff MUST perform the TypeDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️typediff](semiorepo://definition/semio/rs/semio.rs/diff%20types/typediff)
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
// [🛠️semio/rs/semio.rs#Diff Types§SideDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/SIDE-DIFF)
/// <summary>SideDiff holds the data fields for a SideDiff record.</summary>
/// SideDiff MUST perform the SideDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️sidediff](semiorepo://definition/semio/rs/semio.rs/diff%20types/sidediff)
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
// [🛠️semio/rs/semio.rs#Diff Types§ConnectionDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/CONNECTION-DIFF)
/// <summary>ConnectionDiff holds the data fields for a ConnectionDiff record.</summary>
/// ConnectionDiff MUST perform the ConnectionDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️connectiondiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/connectiondiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§PieceDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/PIECE-DIFF)
/// <summary>PieceDiff holds the data fields for a PieceDiff record.</summary>
/// PieceDiff MUST perform the PieceDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️piecediff](semiorepo://definition/semio/rs/semio.rs/diff%20types/piecediff)
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
// [🛠️semio/rs/semio.rs#Diff Types§LayerDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/LAYER-DIFF)
/// <summary>LayerDiff holds the data fields for a LayerDiff record.</summary>
/// LayerDiff MUST perform the LayerDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️layerdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/layerdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§GroupDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/GROUP-DIFF)
/// <summary>GroupDiff holds the data fields for a GroupDiff record.</summary>
/// GroupDiff MUST perform the GroupDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️groupdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/groupdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§StatDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/STAT-DIFF)
/// <summary>StatDiff holds the data fields for a StatDiff record.</summary>
/// StatDiff MUST perform the StatDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️statdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/statdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§DesignDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/DESIGN-DIFF)
/// <summary>DesignDiff holds the data fields for a DesignDiff record.</summary>
/// DesignDiff MUST perform the DesignDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️designdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/designdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§TagDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/TAG-DIFF)
/// <summary>TagDiff holds the data fields for a TagDiff record.</summary>
/// TagDiff MUST perform the TagDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️tagdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/tagdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§ConceptDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/CONCEPT-DIFF)
/// <summary>ConceptDiff holds the data fields for a ConceptDiff record.</summary>
/// ConceptDiff MUST perform the ConceptDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️conceptdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/conceptdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§PortDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/PORT-DIFF)
/// <summary>PortDiff holds the data fields for a PortDiff record.</summary>
/// PortDiff MUST perform the PortDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️portdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/portdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§QualityDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/QUALITY-DIFF)
/// <summary>QualityDiff holds the data fields for a QualityDiff record.</summary>
/// QualityDiff MUST perform the QualityDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️qualitydiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/qualitydiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§FileDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/FILE-DIFF)
/// <summary>FileDiff holds the data fields for a FileDiff record.</summary>
/// FileDiff MUST perform the FileDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️filediff](semiorepo://definition/semio/rs/semio.rs/diff%20types/filediff)
/// </remarks>
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
// [🛠️semio/rs/semio.rs#Diff Types§FolderDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/FOLDER-DIFF)
/// <summary>FolderDiff holds the data fields for a FolderDiff record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️folderdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/folderdiff)
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
// [🛠️semio/rs/semio.rs#Diff Types§AuthorDiff](semiorepo://definition/semio/rs/semio.rs/DIFF-TYPES/AUTHOR-DIFF)
/// <summary>AuthorDiff holds the data fields for a AuthorDiff record.</summary>
/// AuthorDiff MUST perform the AuthorDiff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖difftypes🛠️authordiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/authordiff)
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
/// [👤semio📚rs💻semiors🔖difftypes🛠️kitdiff](semiorepo://definition/semio/rs/semio.rs/diff%20types/kitdiff)
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

// #endregion 🔖Diff Types

// #region 🔖HasGuid Trait
// [👤semio📚rs💻semiors🔖hasguidtrait](semiorepo://section/semio/rs/semio.rs/has-guid-trait)
// HasGuid Trait MUST provide the hasguid trait functionality.

// [✂️semio/rs/semio.rs#HasGuid Trait§HasGuid](semiorepo://definition/semio/rs/semio.rs/HAS-GUID-TRAIT/HAS-GUID)
/// <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/// HasGuid MUST perform the HasGuid operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖hasguidtrait🛠️hasguid](semiorepo://definition/semio/rs/semio.rs/hasguid%20trait/hasguid)
/// </remarks>
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
impl HasGuid for Port { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Quality { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for File { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Folder { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Author { fn guid(&self) -> &str { &self.guid } }
impl HasGuid for Kit { fn guid(&self) -> &str { &self.guid } }

// [✂️semio/rs/semio.rs#HasGuid Trait§DiffHasGuid](semiorepo://definition/semio/rs/semio.rs/HAS-GUID-TRAIT/DIFF-HAS-GUID)
/// <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖hasguidtrait🛠️diffhasguid](semiorepo://definition/semio/rs/semio.rs/hasguid%20trait/diffhasguid)
/// DiffHasGuid MUST perform the DiffHasGuid operation.
/// </remarks>
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
impl DiffHasGuid for PortDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for QualityDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for FileDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for FolderDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for AuthorDiff { fn guid(&self) -> &str { &self.guid } }
impl DiffHasGuid for KitDiff { fn guid(&self) -> &str { &self.guid } }

// #endregion 🔖HasGuid Trait

// #region 🔖ApplyDiff
// [👤semio📚rs💻semiors🔖applydiff](semiorepo://section/semio/rs/semio.rs/apply-diff)
// ApplyDiff MUST provide the applydiff functionality.

/// <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applycollectiondiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_collection_diff)
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
            let diff_map: HashMap<_, _> = updated_diffs.iter().map(|d| (d.guid().to_string(), d)).collect();
            for item in &mut new_items {
                if let Some(update) = diff_map.get(item.guid()) {
                    apply_item_diff(item, &update.diff);
                }
            }
        }


        if let Some(added_items) = &diff.added {
            new_items.extend(added_items.clone());
        }

        *collection = if new_items.is_empty() { None } else { Some(new_items) };
    }
}

/// apply_attribute_diff MUST perform the apply_attribute_diff operation.
// [🛠️semio/rs/semio.rs#ApplyDiff§apply_attribute_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-ATTRIBUTE-DIFF)
/// <summary>apply_attribute_diff performs the apply_attribute_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyattributediff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_attribute_diff)
/// apply_attribute_diff MUST perform the apply_attribute_diff operation.
/// </remarks>
pub fn apply_attribute_diff(item: &mut Attribute, diff: &AttributeDiff) {
    if let Some(value) = &diff.key { item.key = value.clone(); }
    if let Some(value) = &diff.value { item.value = value.clone(); }
    if let Some(value) = &diff.definition { item.definition = value.clone(); }
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_prop_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-PROP-DIFF)
/// <summary>apply_prop_diff holds the data fields for a apply_prop_diff record.</summary>
/// apply_prop_diff MUST perform the apply_prop_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applypropdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_prop_diff)
/// </remarks>
pub fn apply_prop_diff(item: &mut Prop, diff: &PropDiff) {
    if let Some(value) = &diff.quality { item.quality = value.clone(); }
    if let Some(value) = &diff.value { item.value = value.clone(); }
    if let Some(value) = &diff.unit { item.unit = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_connector_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-CONNECTOR-DIFF)
/// <summary>apply_connector_diff holds the data fields for a apply_connector_diff record.</summary>
/// apply_connector_diff MUST perform the apply_connector_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyconnectordiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_connector_diff)
/// </remarks>
pub fn apply_connector_diff(item: &mut Connector, diff: &ConnectorDiff) {
    if let Some(value) = &diff.point { item.point = value.clone(); }
    if let Some(value) = &diff.direction { item.direction = value.clone(); }
    if let Some(value) = &diff.t { item.t = *value; }
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.mandatory { item.mandatory = *value; }
    if let Some(value) = &diff.port { item.port = value.clone(); }
    apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_model_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-MODEL-DIFF)
/// <summary>apply_model_diff performs the apply_model_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applymodeldiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_model_diff)
/// apply_model_diff MUST perform the apply_model_diff operation.
/// </remarks>
pub fn apply_model_diff(item: &mut Model, diff: &ModelDiff) {
    if let Some(value) = &diff.file { item.file = value.clone(); }
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.tags { item.tags = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_type_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-TYPE-DIFF)
/// <summary>apply_type_diff holds the data fields for a apply_type_diff record.</summary>
/// apply_type_diff MUST perform the apply_type_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applytypediff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_type_diff)
/// </remarks>
pub fn apply_type_diff(item: &mut Type, diff: &TypeDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.parent { item.parent = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
    if let Some(value) = &diff.image { item.image = value.clone(); }
    if let Some(value) = &diff.folder { item.folder = value.clone(); }
    if let Some(value) = &diff.unit { item.unit = value.clone(); }
    if let Some(value) = &diff.stock { item.stock = value.clone(); }
    if let Some(value) = &diff.is_abstract { item.is_abstract = *value; }
    if let Some(value) = &diff.virtual_type { item.virtual_type = *value; }
    if let Some(value) = &diff.location { item.location = value.clone(); }
    if let Some(value) = &diff.concepts { item.concepts = value.clone(); }
    if let Some(value) = &diff.authors { item.authors = value.clone(); }
    apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
    apply_collection_diff(&mut item.models, &diff.models, apply_model_diff);
    apply_collection_diff(&mut item.connectors, &diff.connectors, apply_connector_diff);
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_layer_diff holds the data fields for a apply_layer_diff record.</summary>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applylayerdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_layer_diff)
/// <remarks>
/// apply_layer_diff MUST perform the apply_layer_diff operation.
/// </remarks>
pub fn apply_layer_diff(item: &mut Layer, diff: &LayerDiff) {
    if let Some(value) = &diff.path { item.path = value.clone(); }
    if let Some(value) = &diff.is_hidden { item.is_hidden = *value; }
    if let Some(value) = &diff.is_locked { item.is_locked = *value; }
    if let Some(value) = &diff.color { item.color = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_group_diff holds the data fields for a apply_group_diff record.</summary>
// [🛠️semio/rs/semio.rs#ApplyDiff§apply_group_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-GROUP-DIFF)
/// <summary>apply_group_diff performs the apply_group_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applygroupdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_group_diff)
/// apply_group_diff MUST perform the apply_group_diff operation.
/// </remarks>
pub fn apply_group_diff(item: &mut Group, diff: &GroupDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.color { item.color = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.pieces { item.pieces = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_stat_diff holds the data fields for a apply_stat_diff record.</summary>
/// apply_stat_diff MUST perform the apply_stat_diff operation.
/// [👤semio📚rs💻semiors🔖applydiff🛠️applystatdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_stat_diff)
pub fn apply_stat_diff(item: &mut Stat, diff: &StatDiff) {
    if let Some(value) = &diff.quality { item.quality = value.clone(); }
    if let Some(value) = &diff.min { item.min = *value; }
    if let Some(value) = &diff.min_excluded { item.min_excluded = *value; }
    if let Some(value) = &diff.max { item.max = *value; }
    if let Some(value) = &diff.max_excluded { item.max_excluded = *value; }
    if let Some(value) = &diff.unit { item.unit = value.clone(); }
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_piece_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-PIECE-DIFF)
/// <summary>apply_piece_diff holds the data fields for a apply_piece_diff record.</summary>
/// apply_piece_diff MUST perform the apply_piece_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applypiecediff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_piece_diff)
/// </remarks>
pub fn apply_piece_diff(item: &mut Piece, diff: &PieceDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.type_ref { item.type_ref = value.clone(); }
    if let Some(value) = &diff.design { item.design = value.clone(); }
    if let Some(value) = &diff.plane { item.plane = value.clone(); }
    if let Some(value) = &diff.center { item.center = value.clone(); }
    if let Some(value) = &diff.scale { item.scale = *value; }
    if let Some(value) = &diff.mirror_plane { item.mirror_plane = value.clone(); }
    if let Some(value) = &diff.is_hidden { item.is_hidden = *value; }
    if let Some(value) = &diff.is_locked { item.is_locked = *value; }
    if let Some(value) = &diff.color { item.color = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_connection_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-CONNECTION-DIFF)
/// <summary>apply_connection_diff holds the data fields for a apply_connection_diff record.</summary>
/// apply_connection_diff MUST perform the apply_connection_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyconnectiondiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_connection_diff)
/// </remarks>
pub fn apply_connection_diff(item: &mut Connection, diff: &ConnectionDiff) {
    if let Some(value) = &diff.connected {
         if let Some(v) = &value.piece { item.connected.piece = v.clone(); }
         if let Some(v) = &value.design_piece { item.connected.design_piece = v.clone(); }
         if let Some(v) = &value.connector { item.connected.connector = v.clone(); }
    }
    if let Some(value) = &diff.connecting {
         if let Some(v) = &value.piece { item.connecting.piece = v.clone(); }
         if let Some(v) = &value.design_piece { item.connecting.design_piece = v.clone(); }
         if let Some(v) = &value.connector { item.connecting.connector = v.clone(); }
    }
    if let Some(value) = &diff.gap { item.gap = *value; }
    if let Some(value) = &diff.shift { item.shift = *value; }
    if let Some(value) = &diff.rise { item.rise = *value; }
    if let Some(value) = &diff.rotation { item.rotation = *value; }
    if let Some(value) = &diff.turn { item.turn = *value; }
    if let Some(value) = &diff.tilt { item.tilt = *value; }
    if let Some(value) = &diff.u { item.u = *value; }
    if let Some(value) = &diff.v { item.v = *value; }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_design_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-DESIGN-DIFF)
/// <summary>apply_design_diff performs the apply_design_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applydesigndiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_design_diff)
/// apply_design_diff MUST perform the apply_design_diff operation.
/// </remarks>
pub fn apply_design_diff(item: &mut Design, diff: &DesignDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.parent { item.parent = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
    if let Some(value) = &diff.image { item.image = value.clone(); }
    if let Some(value) = &diff.folder { item.folder = value.clone(); }
    if let Some(value) = &diff.unit { item.unit = value.clone(); }
    if let Some(value) = &diff.is_abstract { item.is_abstract = *value; }
    if let Some(value) = &diff.can_scale { item.can_scale = *value; }
    if let Some(value) = &diff.can_mirror { item.can_mirror = *value; }
    if let Some(value) = &diff.concepts { item.concepts = value.clone(); }
    if let Some(value) = &diff.authors { item.authors = value.clone(); }
    if let Some(value) = &diff.active_layer { item.active_layer = value.clone(); }
    apply_collection_diff(&mut item.props, &diff.props, apply_prop_diff);
    apply_collection_diff(&mut item.pieces, &diff.pieces, apply_piece_diff);
    apply_collection_diff(&mut item.connections, &diff.connections, apply_connection_diff);
    apply_collection_diff(&mut item.layers, &diff.layers, apply_layer_diff);
    apply_collection_diff(&mut item.groups, &diff.groups, apply_group_diff);
    apply_collection_diff(&mut item.stats, &diff.stats, apply_stat_diff);
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_tag_diff holds the data fields for a apply_tag_diff record.</summary>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applytagdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_tag_diff)
/// <remarks>
/// apply_tag_diff MUST perform the apply_tag_diff operation.
/// </remarks>
pub fn apply_tag_diff(item: &mut Tag, diff: &TagDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
}

/// <summary>apply_concept_diff holds the data fields for a apply_concept_diff record.</summary>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyconceptdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_concept_diff)
/// <remarks>
/// apply_concept_diff MUST perform the apply_concept_diff operation.
/// </remarks>
pub fn apply_concept_diff(item: &mut Concept, diff: &ConceptDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
}

/// <summary>apply_interface_diff holds the data fields for a apply_interface_diff record.</summary>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyinterfacediff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_interface_diff)
/// <remarks>
/// apply_interface_diff MUST perform the apply_interface_diff operation.
/// </remarks>
pub fn apply_interface_diff(item: &mut Port, diff: &PortDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
    if let Some(value) = &diff.compatible_interfaces { item.compatible_interfaces = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_quality_diff holds the data fields for a apply_quality_diff record.</summary>
/// apply_quality_diff MUST perform the apply_quality_diff operation.
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyqualitydiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_quality_diff)
pub fn apply_quality_diff(item: &mut Quality, diff: &QualityDiff) {
    if let Some(value) = &diff.key { item.key = value.clone(); }
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.kind { item.kind = value.clone(); }
    if let Some(value) = &diff.default_value { item.default_value = *value; }
    if let Some(value) = &diff.formula { item.formula = value.clone(); }
    if let Some(value) = &diff.default_si_unit { item.default_si_unit = value.clone(); }
    if let Some(value) = &diff.default_imperial_unit { item.default_imperial_unit = value.clone(); }
    if let Some(value) = &diff.min { item.min = *value; }
    if let Some(value) = &diff.is_min_excluded { item.is_min_excluded = *value; }
    if let Some(value) = &diff.max { item.max = *value; }
    if let Some(value) = &diff.is_max_excluded { item.is_max_excluded = *value; }
    if let Some(value) = &diff.can_scale { item.can_scale = *value; }
    if let Some(value) = &diff.uri { item.uri = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_file_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-FILE-DIFF)
/// <summary>apply_file_diff holds the data fields for a apply_file_diff record.</summary>
/// apply_file_diff MUST perform the apply_file_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyfilediff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_file_diff)
/// </remarks>
pub fn apply_file_diff(item: &mut File, diff: &FileDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.mime { item.mime = value.clone(); }
    if let Some(value) = &diff.remote { item.remote = value.clone(); }
    if let Some(value) = &diff.folder { item.folder = value.clone(); }
    if let Some(value) = &diff.size { item.size = *value; }
    if let Some(value) = &diff.hash { item.hash = value.clone(); }
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_folder_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-FOLDER-DIFF)
/// <summary>apply_folder_diff performs the apply_folder_diff operation.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyfolderdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_folder_diff)
/// apply_folder_diff MUST perform the apply_folder_diff operation.
/// </remarks>
pub fn apply_folder_diff(item: &mut Folder, diff: &FolderDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.parent { item.parent = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

/// <summary>apply_author_diff holds the data fields for a apply_author_diff record.</summary>
/// apply_author_diff MUST perform the apply_author_diff operation.
/// [👤semio📚rs💻semiors🔖applydiff🛠️applyauthordiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_author_diff)
pub fn apply_author_diff(item: &mut Author, diff: &AuthorDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.email { item.email = value.clone(); }
    apply_collection_diff(&mut item.attributes, &diff.attributes, apply_attribute_diff);
}

// [🛠️semio/rs/semio.rs#ApplyDiff§apply_kit_diff](semiorepo://definition/semio/rs/semio.rs/APPLY-DIFF/APPLY-KIT-DIFF)
/// <summary>apply_kit_diff holds the data fields for a apply_kit_diff record.</summary>
/// apply_kit_diff MUST perform the apply_kit_diff operation.
/// <remarks>
/// [👤semio📚rs💻semiors🔖applydiff🛠️applykitdiff](semiorepo://definition/semio/rs/semio.rs/applydiff/apply_kit_diff)
/// </remarks>
pub fn apply_kit_diff(item: &mut Kit, diff: &KitDiff) {
    if let Some(value) = &diff.name { item.name = value.clone(); }
    if let Some(value) = &diff.version { item.version = value.clone(); }
    if let Some(value) = &diff.description { item.description = value.clone(); }
    if let Some(value) = &diff.icon { item.icon = value.clone(); }
    if let Some(value) = &diff.image { item.image = value.clone(); }
    if let Some(value) = &diff.preview { item.preview = value.clone(); }
    if let Some(value) = &diff.remote { item.remote = value.clone(); }
    if let Some(value) = &diff.homepage { item.homepage = value.clone(); }
    if let Some(value) = &diff.license { item.license = value.clone(); }
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

// #region 🔖FlattenDesign
// [👤semio📚rs💻semiors🔖flattendesign](semiorepo://section/semio/rs/semio.rs/flatten-design)
// FlattenDesign MUST provide the flattendesign functionality.

/// <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/// FlattenedPiece MUST perform the FlattenedPiece operation.
/// [👤semio📚rs💻semiors🔖flattendesign🛠️flattenedpiece](semiorepo://definition/semio/rs/semio.rs/flattendesign/flattenedpiece)
pub struct FlattenedPiece {
    pub piece: Piece,
    pub plane: Plane,
    pub type_guid: Option<String>,
    pub design_guid: Option<String>,
}

/// <summary>flatten_design holds the data fields for a flatten_design record.</summary>
/// [👤semio📚rs💻semiors🔖flattendesign🛠️flattendesign](semiorepo://definition/semio/rs/semio.rs/flattendesign/flatten_design)
/// <remarks>
/// flatten_design MUST perform the flatten_design operation.
/// </remarks>
pub fn flatten_design(kit: &Kit, design_guid: &str) -> DesignDiff {
    let design = match find_design_in_kit(kit, design_guid) {
        Some(d) => d,
        None => return DesignDiff { guid: design_guid.to_string(), ..Default::default() },
    };
    
    let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
    if pieces.is_empty() {
        return DesignDiff { guid: design_guid.to_string(), ..Default::default() };
    }
    
    let connections = design.connections.as_ref().map(|c| c.as_slice()).unwrap_or(&[]);
    
    let types_map: HashMap<&str, &Type> = kit.types.as_ref().map(|types| {
        types.iter().map(|t| (t.guid.as_str(), t)).collect()
    }).unwrap_or_default();

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
    let mut visited: HashSet<&str> = HashSet::with_capacity(pieces.len());
    let mut queue: VecDeque<&str> = VecDeque::with_capacity(pieces.len());
    
    const RADIUS: f64 = 2.697;
    const VERTICAL_V_EXTRA: f64 = 1.0;
    const HORIZONTAL_SCALE: f64 = 3.0633;
    
    for piece in pieces {
        if !visited.contains(piece.guid.as_str()) {
            let initial_matrix = piece.plane.as_ref()
                .map(|p| p.to_matrix())
                .unwrap_or_else(Matrix4::identity);
            piece_planes.insert(piece.guid.as_str(), initial_matrix);
            
            let initial_center = piece.center.clone().unwrap_or(Coord { u: 0.0, v: 0.0 });
            piece_centers.insert(piece.guid.as_str(), initial_center);
            
            visited.insert(piece.guid.as_str());
            queue.push_back(piece.guid.as_str());
            
            while let Some(current_guid) = queue.pop_front() {
                let current_matrix = *piece_planes.get(current_guid).unwrap();
                let parent_center = piece_centers.get(current_guid).cloned().unwrap_or(Coord { u: 0.0, v: 0.0 });
                
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
                        
                        let parent_connector = match get_connector_for_side_fast(&types_map, &pieces_map, parent_side) {
                            Some(c) => c,
                            None => continue,
                        };
                        
                        let connection_matrix = match compute_connection_matrix_fast(&types_map, &pieces_map, conn, is_connected) {
                            Some(m) => m,
                            None => continue,
                        };
                        
                        let new_matrix = current_matrix * connection_matrix;
                        
                        let conn_u = conn.u.unwrap_or(0.0);
                        let conn_v = conn.v.unwrap_or(0.0);
                        
                        let (child_u, child_v) = if parent_center.u.abs() < 0.0001 && parent_center.v.abs() < 0.0001 {
                            let angle = 2.0 * PI * parent_connector.t;
                            (RADIUS * angle.sin(), RADIUS * angle.cos())
                        } else {
                            let is_vertical = parent_connector.direction.z.abs() > 0.5;
                            if is_vertical {
                                (parent_center.u + conn_u, parent_center.v + conn_v + VERTICAL_V_EXTRA)
                            } else {
                                (parent_center.u + conn_u * HORIZONTAL_SCALE, parent_center.v + conn_v * HORIZONTAL_SCALE)
                            }
                        };
                        
                        let child_center = Coord { 
                            u: (child_u * 1_000_000.0).round() / 1_000_000.0,
                            v: (child_v * 1_000_000.0).round() / 1_000_000.0 
                        };
                        
                        piece_planes.insert(neighbor_guid, new_matrix);
                        piece_centers.insert(neighbor_guid, child_center);
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
                Some(existing) => (existing.u - center.u).abs() > 0.0001 || (existing.v - center.v).abs() > 0.0001,
                None => true,
            };
            
            if plane_needs_update || center_needs_update {
                updated_pieces.push(DiffUpdate {
                    key: "piece".to_string(),
                    guid: piece.guid.clone(),
                    diff: PieceDiff {
                        guid: piece.guid.clone(),
                        plane: if plane_needs_update { Some(Some(new_plane)) } else { None },
                        center: if center_needs_update { Some(Some(center.clone())) } else { None },
                        ..Default::default()
                    },
                });
            }
        }
    }
    
    let mut result = DesignDiff {
        guid: design_guid.to_string(),
        ..Default::default()
    };
    
    if !updated_pieces.is_empty() {
        result.pieces = Some(CollectionDiff {
            added: None,
            removed: None,
            updated: Some(updated_pieces),
        });
    }
    
    result
}

fn planes_equal_approx(a: &Plane, b: &Plane) -> bool {
    const TOL: f64 = 0.0001;
    (a.origin.x - b.origin.x).abs() < TOL &&
    (a.origin.y - b.origin.y).abs() < TOL &&
    (a.origin.z - b.origin.z).abs() < TOL &&
    (a.x_axis.x - b.x_axis.x).abs() < TOL &&
    (a.x_axis.y - b.x_axis.y).abs() < TOL &&
    (a.x_axis.z - b.x_axis.z).abs() < TOL &&
    (a.y_axis.x - b.y_axis.x).abs() < TOL &&
    (a.y_axis.y - b.y_axis.y).abs() < TOL &&
    (a.y_axis.z - b.y_axis.z).abs() < TOL
}

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
    
    Some(compute_child_plane_matrix(&parent_connector, &child_connector, conn))
}

fn compute_child_plane_matrix(parent_connector: &Connector, child_connector: &Connector, conn: &Connection) -> Matrix4<f64> {
    use nalgebra::{UnitQuaternion, Vector3};
    
    let parent_point = Vector3::new(parent_connector.point.x, parent_connector.point.y, parent_connector.point.z);
    let mut parent_dir = Vector3::new(parent_connector.direction.x, parent_connector.direction.y, parent_connector.direction.z);
    if parent_dir.norm() > 0.0001 { parent_dir = parent_dir.normalize(); }
    
    let child_point = Vector3::new(child_connector.point.x, child_connector.point.y, child_connector.point.z);
    let mut child_dir = Vector3::new(child_connector.direction.x, child_connector.direction.y, child_connector.direction.z);
    if child_dir.norm() > 0.0001 { child_dir = child_dir.normalize(); }
    
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
                    UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(axis.normalize()), PI)
                } else {
                    UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI)
                }
            }
        } else {
            UnitQuaternion::rotation_between(&reverse_child_dir, &parent_dir).unwrap_or_else(UnitQuaternion::identity)
        }
    };
    let direction_t = quat_to_matrix4(&align_quat);
    
    let y_axis = Vector3::new(0.0, 1.0, 0.0);
    let parent_connector_quat = {
        let dot = y_axis.dot(&parent_dir);
        if (dot + 1.0).abs() < 0.0001 {
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI)
        } else {
            UnitQuaternion::rotation_between(&y_axis, &parent_dir).unwrap_or_else(UnitQuaternion::identity)
        }
    };
    let parent_rotation_t = quat_to_matrix4(&parent_connector_quat);
    
    let gap_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 1.0, 0.0));
    let shift_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(1.0, 0.0, 0.0));
    let raise_dir = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 0.0, 1.0));
    let mut turn_axis = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(0.0, 0.0, 1.0));
    let mut tilt_axis = apply_matrix4_to_vec3(&parent_rotation_t, &Vector3::new(1.0, 0.0, 0.0));
    
    let mut orientation_t = direction_t;
    
    let rotate_quat = UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(parent_dir), -rotation_rad);
    let rotate_t = quat_to_matrix4(&rotate_quat);
    orientation_t = rotate_t * orientation_t;
    
    turn_axis = apply_matrix4_to_vec3(&rotate_t, &turn_axis);
    tilt_axis = apply_matrix4_to_vec3(&rotate_t, &tilt_axis);
    
    if turn_axis.norm() > 0.0001 {
        let turn_quat = UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(turn_axis.normalize()), turn_rad);
        let turn_t = quat_to_matrix4(&turn_quat);
        orientation_t = turn_t * orientation_t;
    }
    
    if tilt_axis.norm() > 0.0001 {
        let tilt_quat = UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(tilt_axis.normalize()), tilt_rad);
        let tilt_t = quat_to_matrix4(&tilt_quat);
        orientation_t = tilt_t * orientation_t;
    }
    
    let center_child_t = make_translation(-child_point.x, -child_point.y, -child_point.z);
    let mut transform = orientation_t * center_child_t;
    
    let gap_transform = make_translation(gap_dir.x * gap, gap_dir.y * gap, gap_dir.z * gap);
    let shift_transform = make_translation(shift_dir.x * shift, shift_dir.y * shift, shift_dir.z * shift);
    let raise_transform = make_translation(raise_dir.x * rise, raise_dir.y * rise, raise_dir.z * rise);
    
    let translation_t = raise_transform * (shift_transform * gap_transform);
    transform = translation_t * transform;
    
    let move_to_parent_t = make_translation(parent_point.x, parent_point.y, parent_point.z);
    transform = move_to_parent_t * transform;
    
    transform
}

fn quat_to_matrix4(q: &nalgebra::UnitQuaternion<f64>) -> Matrix4<f64> {
    let rot = q.to_rotation_matrix();
    let m = rot.matrix();
    Matrix4::new(
        m[(0,0)], m[(0,1)], m[(0,2)], 0.0,
        m[(1,0)], m[(1,1)], m[(1,2)], 0.0,
        m[(2,0)], m[(2,1)], m[(2,2)], 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn make_translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    )
}

fn apply_matrix4_to_vec3(m: &Matrix4<f64>, v: &nalgebra::Vector3<f64>) -> nalgebra::Vector3<f64> {
    nalgebra::Vector3::new(
        m[(0,0)] * v.x + m[(0,1)] * v.y + m[(0,2)] * v.z,
        m[(1,0)] * v.x + m[(1,1)] * v.y + m[(1,2)] * v.z,
        m[(2,0)] * v.x + m[(2,1)] * v.y + m[(2,2)] * v.z
    )
}

fn get_connector_for_side_fast<'a>(
    types_map: &HashMap<&str, &'a Type>,
    pieces_map: &HashMap<&str, &Piece>,
    side: &Side
) -> Option<Connector> {
    let piece = pieces_map.get(side.piece.guid.as_str())?;
    let type_id = piece.type_ref.as_ref()?;
    let t = types_map.get(type_id.guid.as_str())?;
    let connector_guid = side.connector.as_ref().map(|c| c.guid.as_str());
    get_connector_from_type(types_map, t, connector_guid).map(|c| c.clone())
}

fn get_connector_from_type<'a>(
    types_map: &HashMap<&str, &'a Type>,
    t: &'a Type, 
    connector_guid: Option<&str>
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

// #endregion 🔖FlattenDesign

// #region 🔖Validation Types
// [👤semio📚rs💻semiors🔖validationtypes](semiorepo://section/semio/rs/semio.rs/validation-types)
// Validation Types MUST provide the validation types functionality.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
/// ValidationProblem MUST perform the ValidationProblem operation.
// [🛠️semio/rs/semio.rs#Validation Types§ValidationProblem](semiorepo://definition/semio/rs/semio.rs/VALIDATION-TYPES/VALIDATION-PROBLEM)
/// <summary>ValidationProblem holds the data fields for a ValidationProblem record.</summary>
/// <remarks>
/// ValidationProblem MUST perform the ValidationProblem operation.
/// </remarks>
/// <remarks>
/// [👤semio📚rs💻semiors🔖validationtypes🛠️validationproblem](semiorepo://definition/semio/rs/semio.rs/validation%20types/validationproblem)
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
// [🛠️semio/rs/semio.rs#Validation Types§ValidationFix](semiorepo://definition/semio/rs/semio.rs/VALIDATION-TYPES/VALIDATION-FIX)
/// <summary>ValidationFix holds the data fields for a ValidationFix record.</summary>
/// <remarks>
/// [👤semio📚rs💻semiors🔖validationtypes🛠️validationfix](semiorepo://definition/semio/rs/semio.rs/validation%20types/validationfix)
/// ValidationFix MUST perform the ValidationFix operation.
/// </remarks>
pub struct ValidationFix {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<KitDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
/// <summary>ValidationResult holds the data fields for a ValidationResult record.</summary>
/// [👤semio📚rs💻semiors🔖validationtypes🛠️validationresult](semiorepo://definition/semio/rs/semio.rs/validation%20types/validationresult)
/// <remarks>
/// ValidationResult MUST perform the ValidationResult operation.
/// </remarks>
pub struct ValidationResult {
    pub problems: Vec<ValidationProblem>,
}

/// <summary>validate_kit holds the data fields for a validate_kit record.</summary>
/// validate_kit MUST perform the validate_kit operation.
/// [👤semio📚rs💻semiors🔖validationtypes🛠️validatekit](semiorepo://definition/semio/rs/semio.rs/validation%20types/validate_kit)
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

fn check_guid_uniqueness_constraint(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let mut guids: HashSet<String> = HashSet::new();
    guids.insert(kit.guid.clone());
    
    if let Some(ref types) = kit.types {
        for t in types {
            check_guid(&t.guid, "Type", &mut guids, problems);
            if let Some(ref connectors) = t.connectors {
                for c in connectors { check_guid(&c.guid, "Connector", &mut guids, problems); }
            }
            if let Some(ref models) = t.models {
                for m in models { check_guid(&m.guid, "Model", &mut guids, problems); }
            }
        }
    }
    
    if let Some(ref designs) = kit.designs {
        for d in designs {
            check_guid(&d.guid, "Design", &mut guids, problems);
            if let Some(ref pieces) = d.pieces {
                for p in pieces { check_guid(&p.guid, "Piece", &mut guids, problems); }
            }
            if let Some(ref connections) = d.connections {
                for c in connections { check_guid(&c.guid, "Connection", &mut guids, problems); }
            }
            if let Some(ref layers) = d.layers {
                for l in layers { check_guid(&l.guid, "Layer", &mut guids, problems); }
            }
        }
    }
    
    if let Some(ref tags) = kit.tags {
        for t in tags { check_guid(&t.guid, "Tag", &mut guids, problems); }
    }
    if let Some(ref concepts) = kit.concepts {
        for c in concepts { check_guid(&c.guid, "Concept", &mut guids, problems); }
    }
    if let Some(ref files) = kit.files {
        for f in files { check_guid(&f.guid, "File", &mut guids, problems); }
    }
    if let Some(ref folders) = kit.folders {
        for f in folders { check_guid(&f.guid, "Folder", &mut guids, problems); }
    }
    if let Some(ref authors) = kit.authors {
        for a in authors { check_guid(&a.guid, "Author", &mut guids, problems); }
    }
    if let Some(ref ports) = kit.ports {
        for p in ports { check_guid(&p.guid, "Port", &mut guids, problems); }
    }
    if let Some(ref qualities) = kit.qualities {
        for q in qualities { check_guid(&q.guid, "Quality", &mut guids, problems); }
    }
}

fn check_guid(guid: &str, kind: &str, guids: &mut HashSet<String>, problems: &mut Vec<ValidationProblem>) {
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

fn check_design_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref designs) = kit.designs else { return };
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

fn check_piece_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref designs) = kit.designs else { return };
    for design in designs {
        let Some(ref pieces) = design.pieces else { continue };
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
                        message: format!("Duplicate piece name \"{}\" inside design \"{}\".", name, design.name),
                        entity_kind: Some("Piece".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
}

fn check_connection_name_uniqueness(_kit: &Kit, _problems: &mut Vec<ValidationProblem>) {

}

fn check_connector_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref types) = kit.types else { return };
    for typ in types {
        let Some(ref connectors) = typ.connectors else { continue };
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
                        message: format!("Duplicate connector name \"{}\" inside type \"{}\".", name, typ.name),
                        entity_kind: Some("Connector".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
}

fn check_model_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref types) = kit.types else { return };
    for typ in types {
        let Some(ref models) = typ.models else { continue };
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
                        message: format!("Duplicate model name \"{}\" inside type \"{}\".", name, typ.name),
                        entity_kind: Some("Model".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
}

fn check_layer_path_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref designs) = kit.designs else { return };
    for design in designs {
        let Some(ref layers) = design.layers else { continue };
        let mut paths: HashMap<&str, Vec<&Layer>> = HashMap::new();
        for l in layers {
            paths.entry(&l.path).or_default().push(l);
        }
        for (path, dups) in paths {
            if dups.len() > 1 {
                for dup in dups.iter().skip(1) {
                    problems.push(ValidationProblem {
                        constraint_id: "layer-path-unique".to_string(),
                        message: format!("Duplicate layer path \"{}\" inside design \"{}\".", path, design.name),
                        entity_kind: Some("Layer".to_string()),
                        entity_guid: Some(dup.guid.clone()),
                        fixes: vec![],
                    });
                }
            }
        }
    }
}

fn check_quality_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref qualities) = kit.qualities else { return };
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

fn check_folder_name_uniqueness(kit: &Kit, problems: &mut Vec<ValidationProblem>) {
    let Some(ref folders) = kit.folders else { return };
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
// [👤semio📚rs💻semiors🔖sqliteimport🔖export](semiorepo://section/semio/rs/semio.rs/sq-lite-import-export)
// SQLite Import/Export MUST provide the sqlite import/export functionality.

#[cfg(not(target_arch = "wasm32"))]
/// <summary>sqlite holds the data fields for a sqlite record.</summary>
/// [👤semio📚rs💻semiors🔖sqliteimport🔖export🛠️sqlite](semiorepo://definition/semio/rs/semio.rs/sqlite%20import/export/sqlite)
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
    
    fn load_ports(conn: &rusqlite::Connection, kit_guid: &str) -> Result<Vec<Port>> {
        let mut stmt = conn.prepare("SELECT guid, name, description, icon FROM port WHERE kit_guid = ?1")
            .map_err(|e| SemioError::Database { message: e.to_string() })?;
        let rows = stmt.query_map([kit_guid], |row| {
            Ok(Port { guid: row.get(0)?, name: row.get(1)?, description: row.get(2)?, icon: row.get(3)?, compatible_interfaces: None, attributes: None })
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
                port: row.get::<_, Option<String>>(10)?.map(|g| PortId { guid: g }),
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

// #endregion 🔖SQLite Import/Export

// #region 🔖Zip Import/Export
// [👤semio📚rs💻semiors🔖zipimport🔖export](semiorepo://section/semio/rs/semio.rs/zip-import-export)
// Zip Import/Export MUST provide the zip import/export functionality.

#[cfg(not(target_arch = "wasm32"))]
/// <summary>zip_roundtrip holds the data fields for a zip_roundtrip record.</summary>
/// [👤semio📚rs💻semiors🔖zipimport🔖export🛠️ziproundtrip](semiorepo://definition/semio/rs/semio.rs/zip%20import/export/zip_roundtrip)
pub mod zip_roundtrip {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Write;
    
    
    pub struct KitImportResult {
        pub kit: Kit,
        pub files: HashMap<String, Vec<u8>>,
    }
    
    pub fn import_kit_from_zip(zip_path: &str) -> Result<KitImportResult> {
        let file = fs::File::open(zip_path)
            .map_err(|e| SemioError::Database { message: format!("Failed to open zip: {}", e) })?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| SemioError::Database { message: format!("Failed to read zip: {}", e) })?;
        
        let temp_dir = tempfile::tempdir()
            .map_err(|e| SemioError::Database { message: format!("Failed to create temp dir: {}", e) })?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| SemioError::Database { message: format!("Failed to read zip entry: {}", e) })?;
            let outpath = temp_dir.path().join(file.name());
            
            if file.is_dir() {
                fs::create_dir_all(&outpath).ok();
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p).ok();
                }
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| SemioError::Database { message: format!("Failed to create file: {}", e) })?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| SemioError::Database { message: format!("Failed to write file: {}", e) })?;
            }
        }
        
        let db_path = temp_dir.path().join(".semio").join("kit.db");
        if !db_path.exists() {
            return Err(SemioError::Database { message: "kit.db not found in zip".to_string() });
        }
        
        let kit = sqlite::import_kit_from_sqlite(db_path.to_str().unwrap())?;
        
        let mut files = HashMap::new();
        for entry in walkdir::WalkDir::new(temp_dir.path()).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(temp_dir.path()).unwrap().to_string_lossy().replace("\\", "/");
                if !rel_path.starts_with(".semio/") {
                    let data = fs::read(entry.path())
                        .map_err(|e| SemioError::Database { message: format!("Failed to read file: {}", e) })?;
                    files.insert(rel_path, data);
                }
            }
        }
        
        Ok(KitImportResult { kit, files })
    }
    
    pub fn export_kit_to_zip(kit: &Kit, files: &HashMap<String, Vec<u8>>, zip_path: &str) -> Result<()> {
        let temp_dir = tempfile::tempdir()
            .map_err(|e| SemioError::Database { message: format!("Failed to create temp dir: {}", e) })?;
        
        let semio_dir = temp_dir.path().join(".semio");
        fs::create_dir_all(&semio_dir)
            .map_err(|e| SemioError::Database { message: format!("Failed to create .semio dir: {}", e) })?;
        
        let db_path = semio_dir.join("kit.db");
        sqlite::export_kit_to_sqlite(kit, db_path.to_str().unwrap())?;
        
        for (rel_path, data) in files {
            let full_path = temp_dir.path().join(rel_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::write(&full_path, data)
                .map_err(|e| SemioError::Database { message: format!("Failed to write file: {}", e) })?;
        }
        
        let zip_file = fs::File::create(zip_path)
            .map_err(|e| SemioError::Database { message: format!("Failed to create zip: {}", e) })?;
        let mut zip_writer = zip::ZipWriter::new(zip_file);
        let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        
        for entry in walkdir::WalkDir::new(temp_dir.path()).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let rel_path = path.strip_prefix(temp_dir.path()).unwrap().to_string_lossy().replace("\\", "/");
            
            if path.is_file() {
                zip_writer.start_file(&rel_path, options)
                    .map_err(|e| SemioError::Database { message: format!("Failed to start zip file: {}", e) })?;
                let data = fs::read(path)
                    .map_err(|e| SemioError::Database { message: format!("Failed to read file: {}", e) })?;
                zip_writer.write_all(&data)
                    .map_err(|e| SemioError::Database { message: format!("Failed to write to zip: {}", e) })?;
            } else if !rel_path.is_empty() {
                zip_writer.add_directory(&rel_path, options)
                    .map_err(|e| SemioError::Database { message: format!("Failed to add directory: {}", e) })?;
            }
        }
        
        zip_writer.finish()
            .map_err(|e| SemioError::Database { message: format!("Failed to finish zip: {}", e) })?;
        
        Ok(())
    }
}

// #endregion 🔖Zip Import/Export

// #region 🔖WASM Bindings
// [👤semio📚rs💻semiors🔖wasmbindings](semiorepo://section/semio/rs/semio.rs/wasm-bindings)
// WASM Bindings MUST provide the wasm bindings functionality.

#[cfg(target_arch = "wasm32")]
/// <summary>wasm holds the data fields for a wasm record.</summary>
/// [👤semio📚rs💻semiors🔖wasmbindings🛠️wasm](semiorepo://definition/semio/rs/semio.rs/wasm%20bindings/wasm)
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
            Ok(kit) => {
                let diff = flatten_design(&kit, design_guid);
                to_js_value(WasmResult::success(diff))
            },
            Err(e) => to_js_value(WasmResult::<DesignDiff>::failure(e.to_string())),
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

// #endregion 🔖WASM Bindings

// #region 🔖Tests
// [👤semio📚rs💻semiors🔖tests](semiorepo://section/semio/rs/semio.rs/tests)
// Tests MUST provide the tests functionality.

#[cfg(test)]
mod tests {
    use super::*;
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
        vectors_equal(&p1.origin, &p2.origin) &&
        vectors_equal(&p1.x_axis, &p2.x_axis) &&
        vectors_equal(&p1.y_axis, &p2.y_axis)
    }

    fn centers_equal(c1: Option<&Coord>, c2: Option<&Coord>) -> bool {
        match (c1, c2) {
            (None, None) => true,
            (Some(a), Some(b)) => float_eq(a.u, b.u) && float_eq(a.v, b.v),
            _ => false,
        }
    }

    fn find_design_by_name<'a>(designs: &'a [Design], name: &str, parent_guid: Option<&str>) -> Option<&'a Design> {
        designs.iter().find(|d| {
            d.name == name && match parent_guid {
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

        let flat_design_diff = flatten_design(kit, &design.guid);
        let mut flat_design = design.clone();
        apply_design_diff(&mut flat_design, &flat_design_diff);

        let expected_pieces = expected_design.pieces.as_ref().expect("Expected design has no pieces");
        let flat_pieces = flat_design.pieces.as_ref().expect("Flat design has no pieces");

        for piece in flat_pieces {
            if piece.name.is_none() { continue; }
            let name = piece.name.as_ref().unwrap();
            let expected_piece = find_piece_by_name(expected_pieces, name)
                .expect(&format!("Expected piece {} not found", name));
            
            assert!(piece.plane.is_some(), "Piece {} has no plane", name);
            assert!(expected_piece.plane.is_some(), "Expected piece {} has no plane", name);
            
            let got = piece.plane.as_ref().unwrap();
            let exp = expected_piece.plane.as_ref().unwrap();
            if !planes_equal(got, exp) {
                eprintln!("Piece: {}", name);
                eprintln!("Got origin: ({:.4}, {:.4}, {:.4})", got.origin.x, got.origin.y, got.origin.z);
                eprintln!("Exp origin: ({:.4}, {:.4}, {:.4})", exp.origin.x, exp.origin.y, exp.origin.z);
                eprintln!("Got xAxis:  ({:.4}, {:.4}, {:.4})", got.x_axis.x, got.x_axis.y, got.x_axis.z);
                eprintln!("Exp xAxis:  ({:.4}, {:.4}, {:.4})", exp.x_axis.x, exp.x_axis.y, exp.x_axis.z);
                eprintln!("Got yAxis:  ({:.4}, {:.4}, {:.4})", got.y_axis.x, got.y_axis.y, got.y_axis.z);
                eprintln!("Exp yAxis:  ({:.4}, {:.4}, {:.4})", exp.y_axis.x, exp.y_axis.y, exp.y_axis.z);
            }
            
            assert!(
                planes_equal(piece.plane.as_ref().unwrap(), expected_piece.plane.as_ref().unwrap()),
                "Plane mismatch for piece {}", name
            );
            assert!(
                centers_equal(piece.center.as_ref(), expected_piece.center.as_ref()),
                "Center mismatch for piece {}", name
            );
        }
    }

    // #region 🔖Roundtrip Tests
// [👤semio📚rs💻semiors🔖tests🔖roundtriptests](semiorepo://section/semio/rs/semio.rs/tests/roundtrip-tests)
// Roundtrip Tests MUST provide the roundtrip tests functionality.

    mod roundtrip {
        use super::*;

        mod json {
            use super::*;

            #[test]
            fn metabolism_kit_json_kit() {
                let kit = load_kit("kit_metabolism.json");
                let json = serialize_kit(&kit).unwrap();
                let restored = deserialize_kit(&json).unwrap();
                assert!(are_kits_equal(&kit, &restored));
            }
        }

        mod zip {
            use super::*;
            use crate::zip_roundtrip::import_kit_from_zip;

            #[test]
            fn metabolism_zip_kit_zip_kit() {
                let zip_path = Path::new(ASSETS_DIR).join("metabolism.zip");
                let zip_path_str = zip_path.to_str().expect("Invalid path");
                
                let result = import_kit_from_zip(zip_path_str).expect("Failed to import kit");
                let kit = result.kit;
                let files = result.files;
                assert!(!kit.guid.is_empty());
                assert_eq!(kit.name, "Metabolism");
                assert!(kit.types.as_ref().map(|t: &Vec<Type>| !t.is_empty()).unwrap_or(false));
                assert!(kit.designs.as_ref().map(|d: &Vec<Design>| !d.is_empty()).unwrap_or(false));
                assert!(!files.is_empty());
            }
        }
    }

    // #endregion 🔖Roundtrip Tests

    // #region 🔖Flatten Tests
// [👤semio📚rs💻semiors🔖tests🔖flattentests](semiorepo://section/semio/rs/semio.rs/tests/flatten-tests)
// Flatten Tests MUST provide the flatten tests functionality.

    mod flatten {
        use super::*;

        mod nakagin_capsule_tower {
            use super::*;

            #[test]
            fn kit_flatten_diff_apply_flat() {
                let kit = load_kit("kit_metabolism.json");
                test_flatten_design(&kit, &["Nakagin Capsule Tower"]);
            }

            mod slanted {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("kit_metabolism.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Slanted"]);
                }
            }

            mod twisted {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("kit_metabolism.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Twisted"]);
                }
            }

            mod dancing {
                use super::*;

                #[test]
                fn kit_flatten_diff_apply_flat() {
                    let kit = load_kit("kit_metabolism.json");
                    test_flatten_design(&kit, &["Nakagin Capsule Tower", "Dancing"]);
                }
            }
        }

        mod capsule_dream {
            use super::*;

            #[test]
            fn kit_flatten_diff_apply_flat() {
                let kit = load_kit("kit_metabolism.json");
                test_flatten_design(&kit, &["Capsule Dream"]);
            }
        }
    }

    // #endregion 🔖Flatten Tests

    // #region 🔖Diff Tests
// [👤semio📚rs💻semiors🔖tests🔖difftests](semiorepo://section/semio/rs/semio.rs/tests/diff-tests)
// Diff Tests MUST provide the diff tests functionality.

    mod diff {
        use super::*;

        mod metabolism {
            use super::*;

            #[test]
            fn kit_diff_diffedkit_inversediff_kit() {
                // TODO: Implement when get_kit_diff, inverse_kit_diff, are_kit_diffs_equal are available
                let kit_original = load_kit("kit_metabolism.json");
                let _kit_diff = load_kit_diff("diff_kit_metabolism.json");
                let _kit_diffed = load_kit("kit_metabolism_diffed.json");
                assert!(!kit_original.guid.is_empty());
            }
        }
    }

    // #endregion 🔖Diff Tests

    // #region 🔖Validation Tests
// [👤semio📚rs💻semiors🔖tests🔖validationtests](semiorepo://section/semio/rs/semio.rs/tests/validation-tests)
// Validation Tests MUST provide the validation tests functionality.

    mod validation {
        use super::*;

        mod metabolism {
            use super::*;

            #[test]
            fn metabolism_kit_validate_empty_report() {
                let kit = load_kit("kit_metabolism.json");
                let result = validate_kit(&kit);
                assert!(result.problems.is_empty());
            }
        }

        mod invalid {
            use super::*;

            #[test]
            fn invalid_kit_validate_invalid_report() {
                let kit = load_kit("kit_invalid.json");
                let result = validate_kit(&kit);
                let expected = load_validation_result("validation.json");
                assert_eq!(result.problems.len(), expected.problems.len(), "Number of problems mismatch");
            }
        }
    }

    // #endregion 🔖Validation Tests
}

// #endregion 🔖Tests
