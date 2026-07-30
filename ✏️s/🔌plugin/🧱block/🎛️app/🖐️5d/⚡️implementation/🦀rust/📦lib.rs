//! 👯 Block 5D app — document entities (constitutional: general). Edits exactly one `PartKind`: its
//! identity, both 2d/3d presentations, its representations, and the `GripKind` templates placed on
//! it in both projections (keep each grip's 2d/3d halves as flat scalar fields — see
//! `s/plugin/puzzle/app/5d/dsl/rs/lib.rs:62` for the known pack table-column bug this dodges).

use block_shared::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use serde::{Deserialize, Serialize};

pub const BLOCK_5D_SCHEMA: &str = "block.5d";

// #region 🔖Document
/// 🔵 The part's 2D-projection presentation (board node).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dPart2d {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
}

/// 🧱 The part's 3D-projection presentation (world object) — pose defaults only; the mesh itself
/// comes from `representations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dPart3d {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
}

/// 🔘 One grip-kind catalog row this part kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dGripKind {
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_rope_kind: String,
}

/// 🌱 One rim-grip template, unified across both projections — flat scalar fields (no nested 2d/3d
/// sub-records) to dodge the pack table-column bug noted on `Block5dPart3d` above.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dGripTemplate {
    pub id: String,
    pub grip_kind: String,
    #[serde(default)]
    pub angle: f64,
    #[serde(default)]
    pub radius_2d: f64,
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub direction: [f64; 3],
    #[serde(default)]
    pub radius_3d: f64,
}

/// 👯 The block-5d projection: a typed single-`PartKind`-definition document unifying both 2d/3d
/// presentations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "block5d", layout = "lines")]
pub struct Block5dDefinition {
    pub schema: String,
    #[dsl(block)]
    pub part_kind: BlockKindIdentity,
    #[dsl(block)]
    #[serde(default, rename = "2d")]
    pub part_2d: Block5dPart2d,
    #[dsl(block)]
    #[serde(default, rename = "3d")]
    pub part_3d: Block5dPart3d,
    #[serde(default)]
    #[dsl(table)]
    pub representations: Vec<BlockRepresentation>,
    #[serde(default)]
    #[dsl(table)]
    pub grip_kinds: Vec<Block5dGripKind>,
    #[serde(default)]
    #[dsl(table)]
    pub grips: Vec<Block5dGripTemplate>,
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
    pub camera2d: BlockCamera2d,
    #[dsl(block)]
    #[serde(default)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[serde(default)]
    pub meta: BlockMeta,
}

impl Default for Block5dDefinition {
    fn default() -> Self {
        Self {
            schema: BLOCK_5D_SCHEMA.to_string(),
            part_kind: BlockKindIdentity::default(),
            part_2d: Block5dPart2d::default(),
            part_3d: Block5dPart3d::default(),
            representations: Vec::new(),
            grip_kinds: Vec::new(),
            grips: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}
// #endregion 🔖Document
