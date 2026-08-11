//! Scratch crate: obj snapshot/diff/mutations logic port for fast iteration (ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION). Mirrors the real
//! crate's trait shapes with local stand-ins (no derive macros, no store/protocol deps).

use serde::{Deserialize, Serialize};

//#region 🔖️LocalTraitStandIns
pub trait MutationDiff<P>: Clone + Default + Serialize {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}
pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;
    fn between(base: &P, other: &P) -> Self;
    fn is_empty(&self) -> bool;
}
pub trait Mutation<P>: Clone {
    type Diff: MutationDiff<P>;
    fn diff(&self, base: &P) -> Self::Diff;
    fn inverse(&self, base: &P) -> Vec<Self> where Self: Sized;
}
//#endregion 🔖️LocalTraitStandIns

pub const STDIO_OBJ_DOCUMENT_SCHEMA: &str = "stdio.obj";

//#region 🔖️MeshModel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoord {
    pub u: f64,
    pub v: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormal {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceVertex {
    pub vertex: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texcoord: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjFace {
    pub vertices: Vec<ObjFaceVertex>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroup {
    pub name: String,
    #[serde(default)]
    pub faces: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjObject {
    pub name: String,
    #[serde(default)]
    pub faces: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjUsemtlRange {
    pub face_index_from: usize,
    pub material: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjSmoothingRange {
    pub face_index_from: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjUnknownStatement {
    pub line_index: usize,
    pub raw: String,
}
//#endregion 🔖️MeshModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjSnapshot {
    pub schema: String,
    #[serde(default)]
    pub vertices: Vec<ObjVertex>,
    #[serde(default)]
    pub texcoords: Vec<ObjTexCoord>,
    #[serde(default)]
    pub normals: Vec<ObjNormal>,
    #[serde(default)]
    pub faces: Vec<ObjFace>,
    #[serde(default)]
    pub groups: Vec<ObjGroup>,
    #[serde(default)]
    pub objects: Vec<ObjObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<String>,
    #[serde(default)]
    pub usemtl: Vec<ObjUsemtlRange>,
    #[serde(default)]
    pub smoothing_groups: Vec<ObjSmoothingRange>,
    #[serde(default)]
    pub unknown_statements: Vec<ObjUnknownStatement>,
}

impl Default for ObjSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
            groups: Vec::new(),
            objects: Vec::new(),
            mtllib: None,
            usemtl: Vec::new(),
            smoothing_groups: Vec::new(),
            unknown_statements: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

pub mod diff;
pub mod mutations;
