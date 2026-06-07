//! 🧾 Generic scene descriptor JSON (port/edge-agnostic node base).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDescJson {
    pub id: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub draggable: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    /// @emoji 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
    #[serde(default)]
    pub icon_kind: Option<String>,
    /// @emoji 🧩 Semantic node-kind id for compatibility rows at `node` specificity.
    #[serde(default)]
    pub node_kind: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub root: Option<bool>,
    pub shape: Option<String>,
    #[serde(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
}

fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    obj.get("hidden").and_then(|v| v.as_bool())
}

/// 🙈 Resolves fixture element visibility from `hidden` or `visible` JSON fields.
pub fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    match board_json_hidden_flag(obj) {
        Some(hidden) => Some(!hidden),
        None => obj.get("visible").and_then(|v| v.as_bool()),
    }
}

/// 🙈 Returns true when a fixture element is visible (default true when unset).
pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    board_json_visible_option(obj).unwrap_or(true)
}
