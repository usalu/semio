//! 🧭 Brep kernel interface: geometry handles and mesh transfer contracts.

mod compute;

pub use compute::{block_on, run_blocking};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// #region 🔖Types
/// 📐 Column vector `[x,y,z]`.
pub type Vec3 = [f64; 3];

/// 📦 Axis-aligned bounds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// 🧭 Geometry entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GeometryKind {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    Compound,
}

/// 🧭 Opaque geometry handle (`solid-3`, `face-7`, …).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GeometryHandle(pub String);

impl GeometryHandle {
    pub fn new(kind: GeometryKind, id: u32) -> Self {
        let prefix = match kind {
            GeometryKind::Vertex => "vertex",
            GeometryKind::Edge => "edge",
            GeometryKind::Wire => "wire",
            GeometryKind::Face => "face",
            GeometryKind::Shell => "shell",
            GeometryKind::Solid => "solid",
            GeometryKind::Compound => "compound",
        };
        Self(format!("{prefix}-{id}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🧩 Triangle index range for one B-Rep face.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaceGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🖼️ Tessellated mesh payload for preview upload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshTransfer {
    pub position: Vec<f32>,
    pub normal: Vec<f32>,
    pub index: Vec<u32>,
    pub edges: Vec<f32>,
    pub face_groups: Vec<FaceGroup>,
}

/// ⚠️ Kernel operation error.
#[derive(Clone, Debug, PartialEq)]
pub enum BrepError {
    InvalidInput(String),
    MissingHandle(String),
    Operation(String),
}

impl std::fmt::Display for BrepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrepError::InvalidInput(message) => write!(f, "invalid input: {message}"),
            BrepError::MissingHandle(handle) => write!(f, "missing handle: {handle}"),
            BrepError::Operation(message) => write!(f, "operation failed: {message}"),
        }
    }
}

impl std::error::Error for BrepError {}
// #endregion 🔖Types

// #region 🔖Kernel
/// 🔌 Model-free BREP kernel interface (fully async).
#[async_trait(?Send)]
pub trait BrepKernel {
    async fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    async fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError>;
    async fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    async fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    async fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError>;
    async fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    async fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    async fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    async fn translate(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError>;
    async fn rotate(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    async fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError>;
    async fn mirror(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError>;
    async fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError>;
    async fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    async fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    async fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError>;
    async fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError>;
    async fn dispose(&mut self, handle: &GeometryHandle);
}
// #endregion 🔖Kernel
