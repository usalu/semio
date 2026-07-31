//! 🩻️ Block 2D app — document entities (constitutional: general). Edits exactly one `NodeKind`: its
//! identity, rim presentation, and the `HandleKind` templates placed on that rim.

use block_shared::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use serde::{Deserialize, Serialize};

pub const BLOCK_2D_SCHEMA: &str = "block.2d";

// #region 🔖️Document
/// 🔵️ The node's own rim presentation — mirrors `Puzzle2dNode`'s shape fields, minus placement (a
/// kind definition has no x/y — those belong to the puzzle assembly, not the definition).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dPresentation {
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

/// 🔘️ One handle-kind catalog row this node kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// 🌱️ One rim-handle template — where a handle of `handle_kind` sits on the node's rim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleTemplate {
    pub id: String,
    #[dsl(refs = "handle_kind")]
    pub handle_kind: String,
    #[dsl(angle = "rad")]
    pub angle: f64,
    pub radius: f64,
}

/// 🩻️ The block-2d projection: a typed single-`NodeKind`-definition document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "block2d", layout = "lines")]
pub struct Block2dDefinition {
    pub schema: String,
    #[dsl(block)]
    pub node_kind: BlockKindIdentity,
    #[dsl(block)]
    #[serde(default)]
    pub presentation: Block2dPresentation,
    #[serde(default)]
    #[dsl(table)]
    pub handle_kinds: Vec<Block2dHandleKind>,
    #[serde(default)]
    #[dsl(table)]
    pub handles: Vec<Block2dHandleTemplate>,
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
    pub meta: BlockMeta,
}

impl Default for Block2dDefinition {
    fn default() -> Self {
        Self {
            schema: BLOCK_2D_SCHEMA.to_string(),
            node_kind: BlockKindIdentity::default(),
            presentation: Block2dPresentation::default(),
            handle_kinds: Vec::new(),
            handles: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            meta: BlockMeta::default(),
        }
    }
}
// #endregion 🔖️Document
