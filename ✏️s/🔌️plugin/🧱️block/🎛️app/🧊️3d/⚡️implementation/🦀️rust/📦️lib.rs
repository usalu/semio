//! 🏙️ Block 3D app — document entities (constitutional: general). Edits exactly one `ObjectKind`: its
//! identity, representations (meshes at LOD/tags — the semio_compose_rs `type` app's successor), and the
//! `VortexKind` templates placed on its rim.

use block_shared::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use serde::{Deserialize, Serialize};

pub const BLOCK_3D_SCHEMA: &str = "block.3d";

// #region 🔖️Document
/// 🔘️ One vortex-kind catalog row this object kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVortexKind {
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_cable_kind: String,
}

/// 🌱️ One rim-vortex template — where a vortex of `vortex_kind` sits on the object's surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVortexTemplate {
    pub id: String,
    pub vortex_kind: String,
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub direction: [f64; 3],
    #[serde(default)]
    pub radius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🏙️ The block-3d projection: a typed single-`ObjectKind`-definition document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "block3d", layout = "lines")]
pub struct Block3dDefinition {
    pub schema: String,
    #[dsl(block)]
    pub object_kind: BlockKindIdentity,
    #[serde(default)]
    #[dsl(table)]
    pub representations: Vec<BlockRepresentation>,
    #[serde(default)]
    #[dsl(table)]
    pub vortex_kinds: Vec<Block3dVortexKind>,
    #[serde(default)]
    #[dsl(table)]
    pub vortices: Vec<Block3dVortexTemplate>,
    #[serde(default)]
    #[dsl(table)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[serde(default)]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
    #[serde(default)]
    #[dsl(table)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[serde(default)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[serde(default)]
    pub meta: BlockMeta,
}

impl Default for Block3dDefinition {
    fn default() -> Self {
        Self {
            schema: BLOCK_3D_SCHEMA.to_string(),
            object_kind: BlockKindIdentity::default(),
            representations: Vec::new(),
            vortex_kinds: Vec::new(),
            vortices: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}
// #endregion 🔖️Document
