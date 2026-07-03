//! 🔌 Port graph layer: handles and port descriptors on generic graph engine.

pub use mathematical_core::{Directed, Ported};
pub use mathematical_graph::*;

use serde::{Deserialize, Serialize};

// #region 🔖HandleDescJson
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleDescJson {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    #[serde(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub handle_kind: Option<String>,
    /// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
    #[serde(default)]
    pub color: Option<String>,
    /// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    #[serde(default)]
    pub icon_kind: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    pub scale: Option<f64>,
}
// #endregion 🔖HandleDescJson

// #region 🔖HandleKinds
use cavas::Color;

#[derive(Clone, Debug)]
pub struct HandleKindDef {
    pub name: String,
    pub color: Color,
    pub default_wire_kind: Option<String>,
    pub scale: f64,
}

#[derive(Clone, Debug)]
pub struct NodeKindHandleTemplate {
    pub handle_kind: String,
    pub angle: f64,
    pub radius: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct HandleData {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    pub radius: f64,
    pub scale: f64,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub style: Option<String>,
    pub handle_kind: String,
    /// Parsed from descriptor `color` when set (overrides catalog fill).
    pub color_fill: Option<Color>,
    /// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    pub icon_kind: Option<String>,
    pub properties: crate::PropertyBag,
}
// #endregion 🔖HandleKinds
